use std::fmt;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "snake_case")]
pub enum IssueStatus {
    Backlog,
    Todo,
    InProgress,
    InReview,
    Done,
    Blocked,
}

impl fmt::Display for IssueStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            IssueStatus::Backlog => write!(f, "Backlog"),
            IssueStatus::Todo => write!(f, "Todo"),
            IssueStatus::InProgress => write!(f, "In Progress"),
            IssueStatus::InReview => write!(f, "In Review"),
            IssueStatus::Done => write!(f, "Done"),
            IssueStatus::Blocked => write!(f, "Blocked"),
        }
    }
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum IssuePriority {
    Low,
    Medium,
    High,
    Urgent,
}

impl fmt::Display for IssuePriority {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            IssuePriority::Low => write!(f, "Low"),
            IssuePriority::Medium => write!(f, "Medium"),
            IssuePriority::High => write!(f, "High"),
            IssuePriority::Urgent => write!(f, "Urgent"),
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Issue {
    pub id: String,
    pub project_id: String,
    pub title: String,
    pub description: String,
    pub assignee_agent_id: Option<String>,
    pub reporter_agent_id: Option<String>,
    pub status: IssueStatus,
    pub priority: IssuePriority,
    pub parent_issue_id: Option<String>,
    pub labels: Vec<String>,
    pub execution_log: Vec<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Issue {
    pub fn new(
        project_id: impl Into<String>,
        title: impl Into<String>,
        description: impl Into<String>,
        priority: IssuePriority,
    ) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4().to_string(),
            project_id: project_id.into(),
            title: title.into(),
            description: description.into(),
            assignee_agent_id: None,
            reporter_agent_id: None,
            status: IssueStatus::Todo,
            priority,
            parent_issue_id: None,
            labels: Vec::new(),
            execution_log: Vec::new(),
            created_at: now,
            updated_at: now,
        }
    }

    pub fn with_assignee(mut self, agent_id: impl Into<String>) -> Self {
        self.assignee_agent_id = Some(agent_id.into());
        self
    }

    pub fn with_labels(mut self, labels: Vec<String>) -> Self {
        self.labels = labels;
        self
    }

    pub fn set_status(&mut self, status: IssueStatus) {
        self.status = status;
        self.updated_at = Utc::now();
    }

    pub fn add_log(&mut self, msg: impl Into<String>) {
        self.execution_log.push(msg.into());
        self.updated_at = Utc::now();
    }
}
