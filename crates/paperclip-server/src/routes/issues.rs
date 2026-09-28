use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::Json,
};
use serde::Deserialize;
use serde_json::{json, Value};
use paperclip_core::models::{Issue, IssuePriority, IssueStatus};
use crate::state::AppState;

#[derive(Deserialize)]
pub struct ListIssuesQuery {
    pub project_id: Option<String>,
    pub status: Option<String>,
}

#[derive(Deserialize)]
pub struct CreateIssuePayload {
    pub project_id: String,
    pub title: String,
    pub description: String,
    pub priority: Option<String>,
    pub assignee_agent_id: Option<String>,
    pub labels: Option<Vec<String>>,
}

#[derive(Deserialize)]
pub struct UpdateIssuePayload {
    pub status: Option<String>,
    pub assignee_agent_id: Option<String>,
    pub priority: Option<String>,
    pub add_log: Option<String>,
}

pub async fn list_issues(
    State(state): State<AppState>,
    Query(query): Query<ListIssuesQuery>,
) -> Json<Vec<Issue>> {
    let status_filter = query.status.as_deref().and_then(|s| match s.to_lowercase().as_str() {
        "backlog" => Some(IssueStatus::Backlog),
        "todo" => Some(IssueStatus::Todo),
        "in_progress" | "inprogress" => Some(IssueStatus::InProgress),
        "in_review" | "inreview" => Some(IssueStatus::InReview),
        "done" => Some(IssueStatus::Done),
        "blocked" => Some(IssueStatus::Blocked),
        _ => None,
    });

    Json(state.storage.list_issues(query.project_id.as_deref(), status_filter))
}

pub async fn get_issue(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<Issue>, (StatusCode, Json<Value>)> {
    state
        .storage
        .get_issue(&id)
        .map(Json)
        .map_err(|e| (StatusCode::NOT_FOUND, Json(json!({ "error": e.to_string() }))))
}

pub async fn create_issue(
    State(state): State<AppState>,
    Json(payload): Json<CreateIssuePayload>,
) -> Result<(StatusCode, Json<Issue>), (StatusCode, Json<Value>)> {
    let priority = match payload.priority.as_deref().unwrap_or("medium") {
        "low" => IssuePriority::Low,
        "high" => IssuePriority::High,
        "urgent" => IssuePriority::Urgent,
        _ => IssuePriority::Medium,
    };

    let mut issue = Issue::new(payload.project_id, payload.title, payload.description, priority);
    if let Some(assignee) = payload.assignee_agent_id {
        issue = issue.with_assignee(assignee);
    }
    if let Some(labels) = payload.labels {
        issue = issue.with_labels(labels);
    }

    state
        .storage
        .create_issue(issue)
        .map(|i| (StatusCode::CREATED, Json(i)))
        .map_err(|e| (StatusCode::BAD_REQUEST, Json(json!({ "error": e.to_string() }))))
}

pub async fn update_issue(
    State(state): State<AppState>,
    Path(id): Path<String>,
    Json(payload): Json<UpdateIssuePayload>,
) -> Result<Json<Issue>, (StatusCode, Json<Value>)> {
    state
        .storage
        .update_issue(&id, |i| {
            if let Some(s) = payload.status.as_deref() {
                match s.to_lowercase().as_str() {
                    "backlog" => i.set_status(IssueStatus::Backlog),
                    "todo" => i.set_status(IssueStatus::Todo),
                    "in_progress" | "inprogress" => i.set_status(IssueStatus::InProgress),
                    "in_review" | "inreview" => i.set_status(IssueStatus::InReview),
                    "done" => i.set_status(IssueStatus::Done),
                    "blocked" => i.set_status(IssueStatus::Blocked),
                    _ => {}
                }
            }
            if let Some(assignee) = payload.assignee_agent_id {
                i.assignee_agent_id = Some(assignee);
            }
            if let Some(p) = payload.priority.as_deref() {
                match p.to_lowercase().as_str() {
                    "low" => i.priority = IssuePriority::Low,
                    "high" => i.priority = IssuePriority::High,
                    "urgent" => i.priority = IssuePriority::Urgent,
                    "medium" => i.priority = IssuePriority::Medium,
                    _ => {}
                }
            }
            if let Some(log) = payload.add_log {
                i.add_log(log);
            }
        })
        .map(Json)
        .map_err(|e| (StatusCode::NOT_FOUND, Json(json!({ "error": e.to_string() }))))
}
