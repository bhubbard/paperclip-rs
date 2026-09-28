use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::Json,
};
use serde::Deserialize;
use serde_json::{json, Value};
use paperclip_core::models::{Approval, ApprovalStatus};
use crate::state::AppState;

#[derive(Deserialize)]
pub struct ListApprovalsQuery {
    pub company_id: Option<String>,
    pub status: Option<String>,
}

#[derive(Deserialize)]
pub struct ReviewPayload {
    pub reviewer: String,
}

pub async fn list_approvals(
    State(state): State<AppState>,
    Query(query): Query<ListApprovalsQuery>,
) -> Json<Vec<Approval>> {
    let status_filter = query.status.as_deref().and_then(|s| match s.to_lowercase().as_str() {
        "pending" => Some(ApprovalStatus::Pending),
        "approved" => Some(ApprovalStatus::Approved),
        "rejected" => Some(ApprovalStatus::Rejected),
        _ => None,
    });

    Json(state.storage.list_approvals(query.company_id.as_deref(), status_filter))
}

pub async fn approve_action(
    State(state): State<AppState>,
    Path(id): Path<String>,
    Json(payload): Json<ReviewPayload>,
) -> Result<Json<Approval>, (StatusCode, Json<Value>)> {
    state
        .storage
        .review_approval(&id, true, &payload.reviewer)
        .map(Json)
        .map_err(|e| (StatusCode::NOT_FOUND, Json(json!({ "error": e.to_string() }))))
}

pub async fn reject_action(
    State(state): State<AppState>,
    Path(id): Path<String>,
    Json(payload): Json<ReviewPayload>,
) -> Result<Json<Approval>, (StatusCode, Json<Value>)> {
    state
        .storage
        .review_approval(&id, false, &payload.reviewer)
        .map(Json)
        .map_err(|e| (StatusCode::NOT_FOUND, Json(json!({ "error": e.to_string() }))))
}
