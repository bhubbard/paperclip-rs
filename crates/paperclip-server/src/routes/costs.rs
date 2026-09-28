use axum::{
    extract::{Path, State},
    response::Json,
};
use paperclip_core::models::CostSummary;
use crate::state::AppState;

pub async fn get_cost_summary(
    State(state): State<AppState>,
    Path(company_id): Path<String>,
) -> Json<CostSummary> {
    Json(state.storage.get_cost_summary(&company_id))
}
