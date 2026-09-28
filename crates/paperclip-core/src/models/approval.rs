use std::fmt;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ApprovalAction {
    BudgetIncrease,
    SensitiveToolExecution,
    ExternalDeployment,
    ProductionRelease,
    Custom(String),
}

impl fmt::Display for ApprovalAction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ApprovalAction::BudgetIncrease => write!(f, "Budget Increase"),
            ApprovalAction::SensitiveToolExecution => write!(f, "Sensitive Tool Execution"),
            ApprovalAction::ExternalDeployment => write!(f, "External Deployment"),
            ApprovalAction::ProductionRelease => write!(f, "Production Release"),
            ApprovalAction::Custom(s) => write!(f, "{}", s),
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ApprovalStatus {
    Pending,
    Approved,
    Rejected,
}

impl fmt::Display for ApprovalStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ApprovalStatus::Pending => write!(f, "Pending"),
            ApprovalStatus::Approved => write!(f, "Approved"),
            ApprovalStatus::Rejected => write!(f, "Rejected"),
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Approval {
    pub id: String,
    pub company_id: String,
    pub agent_id: String,
    pub issue_id: Option<String>,
    pub action: ApprovalAction,
    pub description: String,
    pub requested_amount_usd: Option<f64>,
    pub status: ApprovalStatus,
    pub reviewed_by: Option<String>,
    pub reviewed_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
}

impl Approval {
    pub fn new(
        company_id: impl Into<String>,
        agent_id: impl Into<String>,
        action: ApprovalAction,
        description: impl Into<String>,
    ) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            company_id: company_id.into(),
            agent_id: agent_id.into(),
            issue_id: None,
            action,
            description: description.into(),
            requested_amount_usd: None,
            status: ApprovalStatus::Pending,
            reviewed_by: None,
            reviewed_at: None,
            created_at: Utc::now(),
        }
    }

    pub fn with_issue(mut self, issue_id: impl Into<String>) -> Self {
        self.issue_id = Some(issue_id.into());
        self
    }

    pub fn with_amount(mut self, amount: f64) -> Self {
        self.requested_amount_usd = Some(amount);
        self
    }

    pub fn approve(&mut self, reviewer: impl Into<String>) {
        self.status = ApprovalStatus::Approved;
        self.reviewed_by = Some(reviewer.into());
        self.reviewed_at = Some(Utc::now());
    }

    pub fn reject(&mut self, reviewer: impl Into<String>) {
        self.status = ApprovalStatus::Rejected;
        self.reviewed_by = Some(reviewer.into());
        self.reviewed_at = Some(Utc::now());
    }
}
