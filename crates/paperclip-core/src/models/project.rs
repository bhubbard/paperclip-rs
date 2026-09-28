use std::fmt;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ProjectStatus {
    Planning,
    Active,
    Paused,
    Completed,
}

impl fmt::Display for ProjectStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ProjectStatus::Planning => write!(f, "Planning"),
            ProjectStatus::Active => write!(f, "Active"),
            ProjectStatus::Paused => write!(f, "Paused"),
            ProjectStatus::Completed => write!(f, "Completed"),
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Project {
    pub id: String,
    pub company_id: String,
    pub goal_id: Option<String>,
    pub name: String,
    pub description: String,
    pub lead_agent_id: Option<String>,
    pub status: ProjectStatus,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Project {
    pub fn new(
        company_id: impl Into<String>,
        name: impl Into<String>,
        description: impl Into<String>,
    ) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4().to_string(),
            company_id: company_id.into(),
            goal_id: None,
            name: name.into(),
            description: description.into(),
            lead_agent_id: None,
            status: ProjectStatus::Active,
            created_at: now,
            updated_at: now,
        }
    }

    pub fn with_goal(mut self, goal_id: impl Into<String>) -> Self {
        self.goal_id = Some(goal_id.into());
        self
    }

    pub fn with_lead(mut self, agent_id: impl Into<String>) -> Self {
        self.lead_agent_id = Some(agent_id.into());
        self
    }
}
