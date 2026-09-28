use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::Json,
};
use serde::Deserialize;
use serde_json::{json, Value};
use paperclip_core::models::Project;
use crate::state::AppState;

#[derive(Deserialize)]
pub struct ListProjectsQuery {
    pub company_id: Option<String>,
}

#[derive(Deserialize)]
pub struct CreateProjectPayload {
    pub company_id: String,
    pub name: String,
    pub description: String,
    pub goal_id: Option<String>,
    pub lead_agent_id: Option<String>,
}

pub async fn list_projects(
    State(state): State<AppState>,
    Query(query): Query<ListProjectsQuery>,
) -> Json<Vec<Project>> {
    Json(state.storage.list_projects(query.company_id.as_deref()))
}

pub async fn get_project(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<Project>, (StatusCode, Json<Value>)> {
    state
        .storage
        .get_project(&id)
        .map(Json)
        .map_err(|e| (StatusCode::NOT_FOUND, Json(json!({ "error": e.to_string() }))))
}

pub async fn create_project(
    State(state): State<AppState>,
    Json(payload): Json<CreateProjectPayload>,
) -> Result<(StatusCode, Json<Project>), (StatusCode, Json<Value>)> {
    let mut project = Project::new(payload.company_id, payload.name, payload.description);
    if let Some(gid) = payload.goal_id {
        project = project.with_goal(gid);
    }
    if let Some(lead) = payload.lead_agent_id {
        project = project.with_lead(lead);
    }

    state
        .storage
        .create_project(project)
        .map(|p| (StatusCode::CREATED, Json(p)))
        .map_err(|e| (StatusCode::BAD_REQUEST, Json(json!({ "error": e.to_string() }))))
}
