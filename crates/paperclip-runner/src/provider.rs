use std::process::Stdio;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use tokio::io::AsyncWriteExt;
use tokio::process::Command;
use tokio::time::{timeout, Duration};

use paperclip_core::models::{Agent, Issue, IssueStatus};
use crate::error::RunnerError;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ExecutionContext {
    pub company_id: String,
    pub agent: Agent,
    pub issue: Issue,
    pub previous_logs: Vec<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ExecutionResult {
    pub success: bool,
    pub new_status: IssueStatus,
    pub summary: String,
    pub logs: Vec<String>,
    pub input_tokens: u64,
    pub output_tokens: u64,
    pub cost_usd: f64,
}

#[async_trait]
pub trait Provider: Send + Sync {
    async fn execute(&self, ctx: &ExecutionContext) -> Result<ExecutionResult, RunnerError>;
}

/// Simulated mock provider for deterministic offline testing and demos
pub struct MockProvider {
    pub auto_resolve: bool,
}

impl Default for MockProvider {
    fn default() -> Self {
        Self { auto_resolve: true }
    }
}

#[async_trait]
impl Provider for MockProvider {
    async fn execute(&self, ctx: &ExecutionContext) -> Result<ExecutionResult, RunnerError> {
        let agent_name = &ctx.agent.name;
        let issue_title = &ctx.issue.title;

        let input_tokens = 450 + (issue_title.len() as u64 * 8);
        let output_tokens = 220 + (agent_name.len() as u64 * 12);
        // Cost based roughly on Claude 3.5 Sonnet / GPT-4o pricing ($3 / 1M in, $15 / 1M out)
        let cost_usd = (input_tokens as f64 * 0.000003) + (output_tokens as f64 * 0.000015);

        let new_status = if self.auto_resolve {
            match ctx.issue.status {
                IssueStatus::Todo => IssueStatus::InProgress,
                IssueStatus::InProgress => IssueStatus::InReview,
                IssueStatus::InReview => IssueStatus::Done,
                IssueStatus::Backlog => IssueStatus::Todo,
                other => other,
            }
        } else {
            IssueStatus::InProgress
        };

        let log1 = format!("[{}] Loaded context for task: '{}'", agent_name, issue_title);
        let log2 = format!("[{}] Evaluated requirements against role prompt: '{}'", agent_name, ctx.agent.role);
        let log3 = format!("[{}] Executed work batch and transitioned status to {}", agent_name, new_status);

        Ok(ExecutionResult {
            success: true,
            new_status,
            summary: format!("Agent '{}' processed task '{}'", agent_name, issue_title),
            logs: vec![log1, log2, log3],
            input_tokens,
            output_tokens,
            cost_usd,
        })
    }
}

/// Subprocess provider executing local agent CLI binary or script (e.g. claude, codex, agy, or shell command)
pub struct ProcessProvider {
    pub binary: String,
    pub args: Vec<String>,
    pub timeout_secs: u64,
}

impl ProcessProvider {
    pub fn new(binary: impl Into<String>, args: Vec<String>, timeout_secs: u64) -> Self {
        Self {
            binary: binary.into(),
            args,
            timeout_secs,
        }
    }
}

#[async_trait]
impl Provider for ProcessProvider {
    async fn execute(&self, ctx: &ExecutionContext) -> Result<ExecutionResult, RunnerError> {
        let payload = serde_json::to_string(ctx)?;

        let mut child = Command::new(&self.binary)
            .args(&self.args)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .kill_on_drop(true)
            .spawn()
            .map_err(|e| RunnerError::Provider(format!("Failed to spawn process '{}': {}", self.binary, e)))?;

        if let Some(mut stdin) = child.stdin.take() {
            stdin.write_all(payload.as_bytes()).await?;
        }

        let output = match timeout(Duration::from_secs(self.timeout_secs), child.wait_with_output()).await {
            Ok(res) => res.map_err(|e| RunnerError::Provider(format!("Process execution error: {}", e)))?,
            Err(_) => return Err(RunnerError::Timeout(self.timeout_secs)),
        };

        let stdout = String::from_utf8_lossy(&output.stdout).to_string();
        let stderr = String::from_utf8_lossy(&output.stderr).to_string();

        if !output.status.success() {
            return Err(RunnerError::Provider(format!(
                "Process exited with code {:?}. Stderr: {}",
                output.status.code(),
                stderr
            )));
        }

        // Attempt to parse JSON ExecutionResult from stdout; fall back to plain text
        if let Ok(res) = serde_json::from_str::<ExecutionResult>(&stdout) {
            Ok(res)
        } else {
            let logs = stdout.lines().map(|s| s.to_string()).collect();
            Ok(ExecutionResult {
                success: true,
                new_status: IssueStatus::InProgress,
                summary: format!("Executed process '{}' successfully", self.binary),
                logs,
                input_tokens: 500,
                output_tokens: 300,
                cost_usd: 0.005,
            })
        }
    }
}

/// HTTP Provider calling OpenAI / Anthropic / Ollama compatible endpoint
pub struct HttpLlmProvider {
    pub endpoint_url: String,
    pub api_key: Option<String>,
    pub model: String,
}

impl HttpLlmProvider {
    pub fn new(endpoint_url: impl Into<String>, api_key: Option<String>, model: impl Into<String>) -> Self {
        Self {
            endpoint_url: endpoint_url.into(),
            api_key,
            model: model.into(),
        }
    }
}

#[async_trait]
impl Provider for HttpLlmProvider {
    async fn execute(&self, ctx: &ExecutionContext) -> Result<ExecutionResult, RunnerError> {
        let client = reqwest::Client::new();
        let mut req = client.post(&self.endpoint_url).json(&serde_json::json!({
            "model": self.model,
            "messages": [
                {
                    "role": "system",
                    "content": format!("Role: {}. Instructions: {}", ctx.agent.role, ctx.agent.system_prompt)
                },
                {
                    "role": "user",
                    "content": format!("Work on issue: {}\nDescription: {}", ctx.issue.title, ctx.issue.description)
                }
            ],
            "temperature": 0.2
        }));

        if let Some(key) = &self.api_key {
            req = req.bearer_auth(key);
        }

        let resp = req
            .send()
            .await
            .map_err(|e| RunnerError::Provider(format!("HTTP request failed: {}", e)))?;

        let status = resp.status();
        if !status.is_success() {
            let body = resp.text().await.unwrap_or_default();
            return Err(RunnerError::Provider(format!("API error {}: {}", status, body)));
        }

        let json: serde_json::Value = resp
            .json()
            .await
            .map_err(|e| RunnerError::Provider(format!("Failed to parse JSON response: {}", e)))?;

        let content = json["choices"][0]["message"]["content"]
            .as_str()
            .unwrap_or("No content generated")
            .to_string();

        let in_tokens = json["usage"]["prompt_tokens"].as_u64().unwrap_or(400);
        let out_tokens = json["usage"]["completion_tokens"].as_u64().unwrap_or(200);
        let cost_usd = (in_tokens as f64 * 0.000003) + (out_tokens as f64 * 0.000015);

        Ok(ExecutionResult {
            success: true,
            new_status: IssueStatus::InProgress,
            summary: format!("LLM response received from {}", self.model),
            logs: content.lines().map(|s| s.to_string()).collect(),
            input_tokens: in_tokens,
            output_tokens: out_tokens,
            cost_usd,
        })
    }
}
