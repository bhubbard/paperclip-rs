use axum::{extract::State, response::Json};
use serde_json::{json, Value};
use crate::state::AppState;

pub async fn health_check() -> Json<Value> {
    Json(json!({
        "status": "ok",
        "service": "paperclip-rs",
        "version": env!("CARGO_PKG_VERSION"),
    }))
}

pub async fn system_info(State(state): State<AppState>) -> Json<Value> {
    let companies = state.storage.list_companies();
    let agents = state.storage.list_agents(None);
    let projects = state.storage.list_projects(None);
    let issues = state.storage.list_issues(None, None);

    Json(json!({
        "total_companies": companies.len(),
        "total_agents": agents.len(),
        "total_projects": projects.len(),
        "total_issues": issues.len(),
        "version": env!("CARGO_PKG_VERSION"),
    }))
}
