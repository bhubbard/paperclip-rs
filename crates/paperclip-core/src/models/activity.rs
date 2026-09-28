use std::fmt;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ActorType {
    Agent,
    Human,
    System,
}

impl fmt::Display for ActorType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ActorType::Agent => write!(f, "Agent"),
            ActorType::Human => write!(f, "Human"),
            ActorType::System => write!(f, "System"),
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct ActivityLog {
    pub id: String,
    pub company_id: String,
    pub actor_id: String,
    pub actor_type: ActorType,
    pub action: String,
    pub details: String,
    pub timestamp: DateTime<Utc>,
}

impl ActivityLog {
    pub fn new(
        company_id: impl Into<String>,
        actor_id: impl Into<String>,
        actor_type: ActorType,
        action: impl Into<String>,
        details: impl Into<String>,
    ) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            company_id: company_id.into(),
            actor_id: actor_id.into(),
            actor_type,
            action: action.into(),
            details: details.into(),
            timestamp: Utc::now(),
        }
    }
}
