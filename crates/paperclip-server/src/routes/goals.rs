use axum::{
    extract::{Query, State},
    http::StatusCode,
    response::Json,
};
use serde::Deserialize;
use serde_json::{json, Value};
use paperclip_core::models::{Goal, GoalPriority};
use crate::state::AppState;

#[derive(Deserialize)]
pub struct ListGoalsQuery {
    pub company_id: Option<String>,
}

#[derive(Deserialize)]
pub struct CreateGoalPayload {
    pub company_id: String,
    pub title: String,
    pub description: String,
    pub priority: Option<String>,
    pub target_metric: Option<String>,
    pub owner_agent_id: Option<String>,
}

pub async fn list_goals(
    State(state): State<AppState>,
    Query(query): Query<ListGoalsQuery>,
) -> Json<Vec<Goal>> {
    Json(state.storage.list_goals(query.company_id.as_deref()))
}

pub async fn create_goal(
    State(state): State<AppState>,
    Json(payload): Json<CreateGoalPayload>,
) -> Result<(StatusCode, Json<Goal>), (StatusCode, Json<Value>)> {
    let priority = match payload.priority.as_deref().unwrap_or("medium") {
        "low" => GoalPriority::Low,
        "high" => GoalPriority::High,
        "urgent" => GoalPriority::Urgent,
        _ => GoalPriority::Medium,
    };

    let mut goal = Goal::new(payload.company_id, payload.title, payload.description, priority);
    if let Some(metric) = payload.target_metric {
        goal = goal.with_target_metric(metric);
    }
    if let Some(owner) = payload.owner_agent_id {
        goal = goal.with_owner(owner);
    }

    state
        .storage
        .create_goal(goal)
        .map(|g| (StatusCode::CREATED, Json(g)))
        .map_err(|e| (StatusCode::BAD_REQUEST, Json(json!({ "error": e.to_string() }))))
}
