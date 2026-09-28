use axum::{
    extract::{Query, State},
    response::Json,
};
use serde::Deserialize;
use paperclip_core::models::ActivityLog;
use crate::state::AppState;

#[derive(Deserialize)]
pub struct ListActivityQuery {
    pub company_id: Option<String>,
    pub limit: Option<usize>,
}

pub async fn list_activity(
    State(state): State<AppState>,
    Query(query): Query<ListActivityQuery>,
) -> Json<Vec<ActivityLog>> {
    let limit = query.limit.unwrap_or(50);
    Json(state.storage.list_activity(query.company_id.as_deref(), limit))
}
