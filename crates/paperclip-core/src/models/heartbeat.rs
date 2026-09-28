use std::fmt;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum HeartbeatStatus {
    Scheduled,
    Running,
    Succeeded,
    Failed,
    Cancelled,
}

impl fmt::Display for HeartbeatStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            HeartbeatStatus::Scheduled => write!(f, "Scheduled"),
            HeartbeatStatus::Running => write!(f, "Running"),
            HeartbeatStatus::Succeeded => write!(f, "Succeeded"),
            HeartbeatStatus::Failed => write!(f, "Failed"),
            HeartbeatStatus::Cancelled => write!(f, "Cancelled"),
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct HeartbeatRun {
    pub id: String,
    pub company_id: String,
    pub agent_id: String,
    pub issue_id: Option<String>,
    pub status: HeartbeatStatus,
    pub duration_ms: Option<u64>,
    pub logs: Vec<String>,
    pub error: Option<String>,
    pub started_at: DateTime<Utc>,
    pub completed_at: Option<DateTime<Utc>>,
}

impl HeartbeatRun {
    pub fn new(company_id: impl Into<String>, agent_id: impl Into<String>) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            company_id: company_id.into(),
            agent_id: agent_id.into(),
            issue_id: None,
            status: HeartbeatStatus::Running,
            duration_ms: None,
            logs: Vec::new(),
            error: None,
            started_at: Utc::now(),
            completed_at: None,
        }
    }

    pub fn with_issue(mut self, issue_id: impl Into<String>) -> Self {
        self.issue_id = Some(issue_id.into());
        self
    }

    pub fn complete_success(&mut self, duration_ms: u64) {
        self.status = HeartbeatStatus::Succeeded;
        self.duration_ms = Some(duration_ms);
        self.completed_at = Some(Utc::now());
    }

    pub fn complete_failure(&mut self, error: impl Into<String>, duration_ms: u64) {
        self.status = HeartbeatStatus::Failed;
        self.error = Some(error.into());
        self.duration_ms = Some(duration_ms);
        self.completed_at = Some(Utc::now());
    }

    pub fn add_log(&mut self, line: impl Into<String>) {
        self.logs.push(line.into());
    }
}
