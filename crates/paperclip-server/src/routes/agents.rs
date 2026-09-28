use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::Json,
};
use serde::Deserialize;
use serde_json::{json, Value};
use paperclip_core::models::{Agent, AgentAdapterType, AgentRole, AgentStatus};
use crate::state::AppState;

#[derive(Deserialize)]
pub struct ListAgentsQuery {
    pub company_id: Option<String>,
}

#[derive(Deserialize)]
pub struct CreateAgentPayload {
    pub company_id: String,
    pub name: String,
    pub role: String,
    pub system_prompt: String,
    pub reports_to: Option<String>,
    pub budget_limit_usd: f64,
    pub adapter_type: Option<String>,
    pub capabilities: Option<Vec<String>>,
}

pub async fn list_agents(
    State(state): State<AppState>,
    Query(query): Query<ListAgentsQuery>,
) -> Json<Vec<Agent>> {
    Json(state.storage.list_agents(query.company_id.as_deref()))
}

pub async fn get_agent(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<Agent>, (StatusCode, Json<Value>)> {
    state
        .storage
        .get_agent(&id)
        .map(Json)
        .map_err(|e| (StatusCode::NOT_FOUND, Json(json!({ "error": e.to_string() }))))
}

pub async fn create_agent(
    State(state): State<AppState>,
    Json(payload): Json<CreateAgentPayload>,
) -> Result<(StatusCode, Json<Agent>), (StatusCode, Json<Value>)> {
    let role = match payload.role.to_lowercase().as_str() {
        "ceo" => AgentRole::Ceo,
        "cto" => AgentRole::Cto,
        "lead_engineer" | "leadengineer" | "engineer" => AgentRole::LeadEngineer,
        "product_manager" | "productmanager" | "pm" => AgentRole::ProductManager,
        "qa_engineer" | "qaengineer" | "qa" => AgentRole::QaEngineer,
        "growth_marketer" | "marketer" => AgentRole::GrowthMarketer,
        "designer" => AgentRole::Designer,
        other => AgentRole::Custom(other.to_string()),
    };

    let adapter = match payload.adapter_type.as_deref().unwrap_or("mock") {
        "process" => AgentAdapterType::Process,
        "httpllm" | "llm" => AgentAdapterType::HttpLlm,
        "sidecar" => AgentAdapterType::Sidecar,
        _ => AgentAdapterType::Mock,
    };

    let mut agent = Agent::new(
        payload.company_id,
        payload.name,
        role,
        adapter,
        payload.system_prompt,
        payload.reports_to,
        payload.budget_limit_usd,
    );

    if let Some(caps) = payload.capabilities {
        agent = agent.with_capabilities(caps);
    }

    state
        .storage
        .create_agent(agent)
        .map(|a| (StatusCode::CREATED, Json(a)))
        .map_err(|e| (StatusCode::BAD_REQUEST, Json(json!({ "error": e.to_string() }))))
}

pub async fn pause_agent(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<Agent>, (StatusCode, Json<Value>)> {
    state
        .storage
        .update_agent(&id, |a| a.set_status(AgentStatus::Paused))
        .map(Json)
        .map_err(|e| (StatusCode::NOT_FOUND, Json(json!({ "error": e.to_string() }))))
}

pub async fn resume_agent(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<Agent>, (StatusCode, Json<Value>)> {
    state
        .storage
        .update_agent(&id, |a| a.set_status(AgentStatus::Idle))
        .map(Json)
        .map_err(|e| (StatusCode::NOT_FOUND, Json(json!({ "error": e.to_string() }))))
}
