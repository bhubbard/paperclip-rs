use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct CostEvent {
    pub id: String,
    pub company_id: String,
    pub agent_id: String,
    pub issue_id: Option<String>,
    pub model: String,
    pub input_tokens: u64,
    pub output_tokens: u64,
    pub total_tokens: u64,
    pub cost_usd: f64,
    pub timestamp: DateTime<Utc>,
}

impl CostEvent {
    pub fn new(
        company_id: impl Into<String>,
        agent_id: impl Into<String>,
        model: impl Into<String>,
        input_tokens: u64,
        output_tokens: u64,
        cost_usd: f64,
    ) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            company_id: company_id.into(),
            agent_id: agent_id.into(),
            issue_id: None,
            model: model.into(),
            input_tokens,
            output_tokens,
            total_tokens: input_tokens + output_tokens,
            cost_usd,
            timestamp: Utc::now(),
        }
    }

    pub fn with_issue(mut self, issue_id: impl Into<String>) -> Self {
        self.issue_id = Some(issue_id.into());
        self
    }
}

#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct CostSummary {
    pub company_id: String,
    pub total_cost_usd: f64,
    pub total_input_tokens: u64,
    pub total_output_tokens: u64,
    pub total_tokens: u64,
    pub agent_breakdown: Vec<AgentCostBreakdown>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct AgentCostBreakdown {
    pub agent_id: String,
    pub agent_name: String,
    pub role: String,
    pub cost_usd: f64,
    pub budget_limit_usd: f64,
    pub budget_percent: f64,
    pub total_tokens: u64,
}
