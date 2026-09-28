use std::fmt;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "snake_case")]
pub enum AgentRole {
    Ceo,
    Cto,
    LeadEngineer,
    ProductManager,
    QaEngineer,
    GrowthMarketer,
    Designer,
    Custom(String),
}

impl fmt::Display for AgentRole {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AgentRole::Ceo => write!(f, "CEO"),
            AgentRole::Cto => write!(f, "CTO"),
            AgentRole::LeadEngineer => write!(f, "Lead Engineer"),
            AgentRole::ProductManager => write!(f, "Product Manager"),
            AgentRole::QaEngineer => write!(f, "QA Engineer"),
            AgentRole::GrowthMarketer => write!(f, "Growth Marketer"),
            AgentRole::Designer => write!(f, "Designer"),
            AgentRole::Custom(s) => write!(f, "{}", s),
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum AgentStatus {
    Idle,
    Running,
    Paused,
    Terminated,
}

impl fmt::Display for AgentStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AgentStatus::Idle => write!(f, "Idle"),
            AgentStatus::Running => write!(f, "Running"),
            AgentStatus::Paused => write!(f, "Paused"),
            AgentStatus::Terminated => write!(f, "Terminated"),
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum AgentAdapterType {
    Mock,
    Process,
    HttpLlm,
    Sidecar,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Agent {
    pub id: String,
    pub company_id: String,
    pub name: String,
    pub role: AgentRole,
    pub adapter_type: AgentAdapterType,
    pub system_prompt: String,
    pub reports_to: Option<String>,
    pub status: AgentStatus,
    pub budget_limit_usd: f64,
    pub budget_spent_usd: f64,
    pub capabilities: Vec<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Agent {
    pub fn new(
        company_id: impl Into<String>,
        name: impl Into<String>,
        role: AgentRole,
        adapter_type: AgentAdapterType,
        system_prompt: impl Into<String>,
        reports_to: Option<String>,
        budget_limit_usd: f64,
    ) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4().to_string(),
            company_id: company_id.into(),
            name: name.into(),
            role,
            adapter_type,
            system_prompt: system_prompt.into(),
            reports_to,
            status: AgentStatus::Idle,
            budget_limit_usd,
            budget_spent_usd: 0.0,
            capabilities: Vec::new(),
            created_at: now,
            updated_at: now,
        }
    }

    pub fn with_capabilities(mut self, caps: Vec<String>) -> Self {
        self.capabilities = caps;
        self
    }

    pub fn record_spend(&mut self, amount_usd: f64) {
        self.budget_spent_usd += amount_usd;
        self.updated_at = Utc::now();
    }

    pub fn remaining_budget(&self) -> f64 {
        (self.budget_limit_usd - self.budget_spent_usd).max(0.0)
    }

    pub fn is_over_budget(&self) -> bool {
        self.budget_spent_usd >= self.budget_limit_usd
    }

    pub fn set_status(&mut self, status: AgentStatus) {
        self.status = status;
        self.updated_at = Utc::now();
    }
}
