use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::Json,
};
use serde_json::{json, Value};
use crate::state::AppState;

pub async fn trigger_heartbeat_tick(
    State(state): State<AppState>,
    Path(company_id): Path<String>,
) -> Result<Json<Value>, (StatusCode, Json<Value>)> {
    let ticks = state
        .dispatcher
        .tick(&company_id)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({ "error": e.to_string() }))))?;

    let executed_count = ticks.len();
    let total_cost: f64 = ticks.iter().map(|t| t.cost_usd).sum();

    let details: Vec<Value> = ticks
        .into_iter()
        .map(|t| {
            json!({
                "agent_id": t.agent_id,
                "agent_name": t.agent_name,
                "issue_id": t.issue_id,
                "issue_title": t.issue_title,
                "success": t.success,
                "summary": t.summary,
                "cost_usd": t.cost_usd,
            })
        })
        .collect();

    Ok(Json(json!({
        "status": "completed",
        "company_id": company_id,
        "ticks_executed": executed_count,
        "total_cost_usd": total_cost,
        "details": details,
    })))
}
