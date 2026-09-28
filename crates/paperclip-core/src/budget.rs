use serde::{Deserialize, Serialize};
use crate::error::PaperclipError;
use crate::models::{Agent, Approval, ApprovalAction, Company};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum BudgetPolicy {
    HardStop,
    RequireApproval,
    WarnOnly,
}

pub struct BudgetEnforcer {
    pub policy: BudgetPolicy,
}

impl Default for BudgetEnforcer {
    fn default() -> Self {
        Self {
            policy: BudgetPolicy::RequireApproval,
        }
    }
}

impl BudgetEnforcer {
    pub fn new(policy: BudgetPolicy) -> Self {
        Self { policy }
    }

    pub fn check_spend(
        &self,
        company: &Company,
        agent: &Agent,
        estimated_cost_usd: f64,
    ) -> Result<Option<Approval>, PaperclipError> {
        let projected_company_spend = company.budget_spent_usd + estimated_cost_usd;
        let projected_agent_spend = agent.budget_spent_usd + estimated_cost_usd;

        let company_exceeded = projected_company_spend > company.budget_limit_usd;
        let agent_exceeded = projected_agent_spend > agent.budget_limit_usd;

        if !company_exceeded && !agent_exceeded {
            return Ok(None);
        }

        match self.policy {
            BudgetPolicy::HardStop => {
                let reason = if company_exceeded {
                    format!(
                        "Company '{}' budget cap of ${:.2} exceeded (projected: ${:.2})",
                        company.name, company.budget_limit_usd, projected_company_spend
                    )
                } else {
                    format!(
                        "Agent '{}' budget cap of ${:.2} exceeded (projected: ${:.2})",
                        agent.name, agent.budget_limit_usd, projected_agent_spend
                    )
                };
                Err(PaperclipError::BudgetExceeded(reason))
            }
            BudgetPolicy::RequireApproval => {
                let desc = if company_exceeded {
                    format!(
                        "Action requires company budget increase. Limit: ${:.2}, Spend: ${:.2}, Added: ${:.2}",
                        company.budget_limit_usd, company.budget_spent_usd, estimated_cost_usd
                    )
                } else {
                    format!(
                        "Action requires agent budget increase. Limit: ${:.2}, Spend: ${:.2}, Added: ${:.2}",
                        agent.budget_limit_usd, agent.budget_spent_usd, estimated_cost_usd
                    )
                };

                let mut approval = Approval::new(
                    company.id.clone(),
                    agent.id.clone(),
                    ApprovalAction::BudgetIncrease,
                    desc,
                );
                approval.requested_amount_usd = Some(estimated_cost_usd);
                Ok(Some(approval))
            }
            BudgetPolicy::WarnOnly => Ok(None),
        }
    }
}
