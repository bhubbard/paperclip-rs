pub mod activity;
pub mod agents;
pub mod approvals;
pub mod companies;
pub mod costs;
pub mod goals;
pub mod health;
pub mod heartbeat;
pub mod issues;
pub mod projects;

use axum::{
    routing::{get, post},
    Router,
};
use tower_http::cors::CorsLayer;
use crate::state::AppState;

pub fn build_router(state: AppState) -> Router {
    Router::new()
        // Dashboard Web App UI
        .route("/", get(crate::ui::dashboard_html))
        .route("/dashboard", get(crate::ui::dashboard_html))
        // Health & Info
        .route("/health", get(health::health_check))
        .route("/api/v1/info", get(health::system_info))
        // Companies
        .route("/api/v1/companies", get(companies::list_companies).post(companies::create_company))
        .route("/api/v1/companies/seed", post(companies::seed_company))
        .route("/api/v1/companies/{id}", get(companies::get_company))
        .route("/api/v1/companies/{id}/org-chart", get(companies::get_org_chart))
        // Agents
        .route("/api/v1/agents", get(agents::list_agents).post(agents::create_agent))
        .route("/api/v1/agents/{id}", get(agents::get_agent))
        .route("/api/v1/agents/{id}/pause", post(agents::pause_agent))
        .route("/api/v1/agents/{id}/resume", post(agents::resume_agent))
        // Goals
        .route("/api/v1/goals", get(goals::list_goals).post(goals::create_goal))
        // Projects
        .route("/api/v1/projects", get(projects::list_projects).post(projects::create_project))
        .route("/api/v1/projects/{id}", get(projects::get_project))
        // Issues
        .route("/api/v1/issues", get(issues::list_issues).post(issues::create_issue))
        .route("/api/v1/issues/{id}", get(issues::get_issue).patch(issues::update_issue))
        // Approvals
        .route("/api/v1/approvals", get(approvals::list_approvals))
        .route("/api/v1/approvals/{id}/approve", post(approvals::approve_action))
        .route("/api/v1/approvals/{id}/reject", post(approvals::reject_action))
        // Costs
        .route("/api/v1/costs/summary/{company_id}", get(costs::get_cost_summary))
        // Activity
        .route("/api/v1/activity", get(activity::list_activity))
        // Heartbeat
        .route("/api/v1/heartbeat/tick/{company_id}", post(heartbeat::trigger_heartbeat_tick))
        .layer(CorsLayer::permissive())
        .with_state(state)
}
