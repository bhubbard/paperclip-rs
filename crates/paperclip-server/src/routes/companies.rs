use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::Json,
};
use serde::Deserialize;
use serde_json::{json, Value};
use paperclip_core::models::{Company, OrgChart};
use crate::state::AppState;

#[derive(Deserialize)]
pub struct CreateCompanyPayload {
    pub name: String,
    pub mission: String,
    pub budget_limit_usd: f64,
}

#[derive(Deserialize)]
pub struct SeedCompanyPayload {
    pub name: String,
    pub mission: String,
    pub budget_limit_usd: f64,
}

pub async fn list_companies(State(state): State<AppState>) -> Json<Vec<Company>> {
    Json(state.storage.list_companies())
}

pub async fn get_company(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<Company>, (StatusCode, Json<Value>)> {
    state
        .storage
        .get_company(&id)
        .map(Json)
        .map_err(|e| (StatusCode::NOT_FOUND, Json(json!({ "error": e.to_string() }))))
}

pub async fn create_company(
    State(state): State<AppState>,
    Json(payload): Json<CreateCompanyPayload>,
) -> Result<(StatusCode, Json<Company>), (StatusCode, Json<Value>)> {
    let company = Company::new(payload.name, payload.mission, payload.budget_limit_usd);
    state
        .storage
        .create_company(company)
        .map(|c| (StatusCode::CREATED, Json(c)))
        .map_err(|e| (StatusCode::BAD_REQUEST, Json(json!({ "error": e.to_string() }))))
}

pub async fn seed_company(
    State(state): State<AppState>,
    Json(payload): Json<SeedCompanyPayload>,
) -> Result<(StatusCode, Json<Company>), (StatusCode, Json<Value>)> {
    state
        .storage
        .seed_startup_template(&payload.name, &payload.mission, payload.budget_limit_usd)
        .map(|c| (StatusCode::CREATED, Json(c)))
        .map_err(|e| (StatusCode::BAD_REQUEST, Json(json!({ "error": e.to_string() }))))
}

pub async fn get_org_chart(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<Value>, (StatusCode, Json<Value>)> {
    let agents = state.storage.list_agents(Some(&id));
    let chart = OrgChart::build(&agents)
        .map_err(|e| (StatusCode::BAD_REQUEST, Json(json!({ "error": e.to_string() }))))?;

    let ascii = chart.render_ascii();
    Ok(Json(json!({
        "company_id": id,
        "ascii": ascii,
        "roots": chart.roots,
    })))
}

pub async fn get_companies_stats(State(state): State<AppState>) -> Json<Value> {
    let mut stats = serde_json::Map::new();
    for company in state.storage.list_companies() {
        let agent_count = state.storage.list_agents(Some(&company.id)).len();
        let issue_count = state.storage.list_issues(Some(&company.id), None).len();
        stats.insert(
            company.id,
            json!({
                "agentCount": agent_count,
                "issueCount": issue_count,
            }),
        );
    }
    Json(Value::Object(stats))
}

