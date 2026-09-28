use std::sync::Arc;
use std::time::Instant;
use tracing::{info, warn};

use paperclip_core::budget::BudgetEnforcer;
use paperclip_core::models::*;
use paperclip_core::storage::Storage;
use crate::error::RunnerError;
use crate::provider::{ExecutionContext, Provider};
use crate::supervisor::{AgentSupervisor, SupervisedTask};

pub struct TickResult {
    pub agent_id: String,
    pub agent_name: String,
    pub issue_id: String,
    pub issue_title: String,
    pub success: bool,
    pub summary: String,
    pub cost_usd: f64,
}

pub struct HeartbeatDispatcher {
    storage: Arc<Storage>,
    provider: Arc<dyn Provider>,
    supervisor: Arc<AgentSupervisor>,
    budget_enforcer: BudgetEnforcer,
}

impl HeartbeatDispatcher {
    pub fn new(
        storage: Arc<Storage>,
        provider: Arc<dyn Provider>,
        supervisor: Arc<AgentSupervisor>,
        budget_enforcer: BudgetEnforcer,
    ) -> Self {
        Self {
            storage,
            provider,
            supervisor,
            budget_enforcer,
        }
    }

    pub async fn tick(&self, company_id: &str) -> Result<Vec<TickResult>, RunnerError> {
        let company = self.storage.get_company(company_id)?;
        let agents = self.storage.list_agents(Some(company_id));
        let active_agents: Vec<Agent> = agents
            .into_iter()
            .filter(|a| a.status == AgentStatus::Idle || a.status == AgentStatus::Running)
            .collect();

        let mut results = Vec::new();

        for agent in active_agents {
            if self.supervisor.is_agent_busy(&agent.id) {
                continue;
            }

            // Find an issue assigned to this agent in Todo or InProgress
            let issues = self.storage.list_issues(None, None);
            let target_issue = issues.into_iter().find(|i| {
                i.assignee_agent_id.as_deref() == Some(&agent.id)
                    && (i.status == IssueStatus::Todo || i.status == IssueStatus::InProgress)
            });

            let Some(issue) = target_issue else {
                continue;
            };

            let start_time = Instant::now();
            let mut heartbeat = HeartbeatRun::new(company_id, &agent.id).with_issue(&issue.id);
            self.storage.record_heartbeat(heartbeat.clone())?;

            let task = SupervisedTask {
                run_id: heartbeat.id.clone(),
                agent_id: agent.id.clone(),
                issue_id: issue.id.clone(),
            };
            self.supervisor.register_run(task);

            // Pre-execution budget check (estimate $0.01 per turn)
            let estimate_usd = 0.01;
            match self.budget_enforcer.check_spend(&company, &agent, estimate_usd) {
                Ok(Some(approval)) => {
                    self.storage.create_approval(approval.clone())?;
                    self.storage.update_issue(&issue.id, |i| {
                        i.set_status(IssueStatus::Blocked);
                        i.add_log(format!("Blocked pending approval: {}", approval.description));
                    })?;
                    self.supervisor.unregister_run(&heartbeat.id);
                    results.push(TickResult {
                        agent_id: agent.id.clone(),
                        agent_name: agent.name.clone(),
                        issue_id: issue.id.clone(),
                        issue_title: issue.title.clone(),
                        success: false,
                        summary: format!("Execution blocked: approval required for budget cap"),
                        cost_usd: 0.0,
                    });
                    continue;
                }
                Err(e) => {
                    self.storage.update_issue(&issue.id, |i| {
                        i.set_status(IssueStatus::Blocked);
                        i.add_log(format!("Budget halted: {}", e));
                    })?;
                    self.supervisor.unregister_run(&heartbeat.id);
                    results.push(TickResult {
                        agent_id: agent.id.clone(),
                        agent_name: agent.name.clone(),
                        issue_id: issue.id.clone(),
                        issue_title: issue.title.clone(),
                        success: false,
                        summary: format!("Budget cap reached: {}", e),
                        cost_usd: 0.0,
                    });
                    continue;
                }
                Ok(None) => {}
            }

            let ctx = ExecutionContext {
                company_id: company_id.to_string(),
                agent: agent.clone(),
                issue: issue.clone(),
                previous_logs: issue.execution_log.clone(),
            };

            let exec_result = self.provider.execute(&ctx).await;
            let duration = start_time.elapsed().as_millis() as u64;
            self.supervisor.unregister_run(&heartbeat.id);

            match exec_result {
                Ok(res) => {
                    let cost_usd = res.cost_usd;
                    // Record cost
                    let cost_event = CostEvent::new(
                        company_id,
                        &agent.id,
                        format!("{:?}", agent.adapter_type),
                        res.input_tokens,
                        res.output_tokens,
                        cost_usd,
                    ).with_issue(&issue.id);
                    self.storage.record_cost(cost_event)?;

                    // Update issue
                    self.storage.update_issue(&issue.id, |i| {
                        i.set_status(res.new_status);
                        for l in &res.logs {
                            i.add_log(l);
                        }
                    })?;

                    // Record heartbeat run completion
                    heartbeat.complete_success(duration);
                    for l in &res.logs {
                        heartbeat.add_log(l);
                    }
                    self.storage.record_heartbeat(heartbeat)?;

                    // Log activity
                    self.storage.log_activity(ActivityLog::new(
                        company_id,
                        &agent.id,
                        ActorType::Agent,
                        "task_executed",
                        format!("Completed turn on issue '{}' ({})", issue.title, res.summary),
                    ))?;

                    info!(
                        "Agent '{}' executed tick on issue '{}' successfully (${:.4})",
                        agent.name, issue.title, cost_usd
                    );

                    results.push(TickResult {
                        agent_id: agent.id,
                        agent_name: agent.name,
                        issue_id: issue.id,
                        issue_title: issue.title,
                        success: true,
                        summary: res.summary,
                        cost_usd,
                    });
                }
                Err(err) => {
                    warn!("Agent '{}' failed tick on issue '{}': {}", agent.name, issue.title, err);
                    heartbeat.complete_failure(err.to_string(), duration);
                    self.storage.record_heartbeat(heartbeat)?;

                    results.push(TickResult {
                        agent_id: agent.id,
                        agent_name: agent.name,
                        issue_id: issue.id,
                        issue_title: issue.title,
                        success: false,
                        summary: format!("Execution failed: {}", err),
                        cost_usd: 0.0,
                    });
                }
            }
        }

        Ok(results)
    }
}
