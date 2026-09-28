use std::fmt;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum GoalStatus {
    Active,
    InProgress,
    Achieved,
    Blocked,
    Archived,
}

impl fmt::Display for GoalStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            GoalStatus::Active => write!(f, "Active"),
            GoalStatus::InProgress => write!(f, "In Progress"),
            GoalStatus::Achieved => write!(f, "Achieved"),
            GoalStatus::Blocked => write!(f, "Blocked"),
            GoalStatus::Archived => write!(f, "Archived"),
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum GoalPriority {
    Low,
    Medium,
    High,
    Urgent,
}

impl fmt::Display for GoalPriority {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            GoalPriority::Low => write!(f, "Low"),
            GoalPriority::Medium => write!(f, "Medium"),
            GoalPriority::High => write!(f, "High"),
            GoalPriority::Urgent => write!(f, "Urgent"),
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Goal {
    pub id: String,
    pub company_id: String,
    pub title: String,
    pub description: String,
    pub target_metric: Option<String>,
    pub status: GoalStatus,
    pub priority: GoalPriority,
    pub owner_agent_id: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Goal {
    pub fn new(
        company_id: impl Into<String>,
        title: impl Into<String>,
        description: impl Into<String>,
        priority: GoalPriority,
    ) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4().to_string(),
            company_id: company_id.into(),
            title: title.into(),
            description: description.into(),
            target_metric: None,
            status: GoalStatus::Active,
            priority,
            owner_agent_id: None,
            created_at: now,
            updated_at: now,
        }
    }

    pub fn with_target_metric(mut self, metric: impl Into<String>) -> Self {
        self.target_metric = Some(metric.into());
        self
    }

    pub fn with_owner(mut self, agent_id: impl Into<String>) -> Self {
        self.owner_agent_id = Some(agent_id.into());
        self
    }

    pub fn set_status(&mut self, status: GoalStatus) {
        self.status = status;
        self.updated_at = Utc::now();
    }
}
