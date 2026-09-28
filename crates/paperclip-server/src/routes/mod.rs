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

use std::path::PathBuf;
use axum::{
    routing::{get, post},
    Router,
};
use tower_http::cors::CorsLayer;
use tower_http::services::{ServeDir, ServeFile};
use crate::state::AppState;

pub fn build_router(state: AppState) -> Router {
    let api_routes = Router::new()
        // Health & Info
        .route("/health", get(health::health_check))
        .route("/info", get(health::system_info))
        // Companies
        .route("/companies", get(companies::list_companies).post(companies::create_company))
        .route("/companies/stats", get(companies::get_companies_stats))
        .route("/companies/seed", post(companies::seed_company))
        .route("/companies/{id}", get(companies::get_company))
        .route("/companies/{id}/org-chart", get(companies::get_org_chart))
        // Agents
        .route("/agents", get(agents::list_agents).post(agents::create_agent))
        .route("/agents/{id}", get(agents::get_agent))
        .route("/agents/{id}/pause", post(agents::pause_agent))
        .route("/agents/{id}/resume", post(agents::resume_agent))
        // Goals
        .route("/goals", get(goals::list_goals).post(goals::create_goal))
        // Projects
        .route("/projects", get(projects::list_projects).post(projects::create_project))
        .route("/projects/{id}", get(projects::get_project))
        // Issues
        .route("/issues", get(issues::list_issues).post(issues::create_issue))
        .route("/issues/{id}", get(issues::get_issue).patch(issues::update_issue))
        // Approvals
        .route("/approvals", get(approvals::list_approvals))
        .route("/approvals/{id}/approve", post(approvals::approve_action))
        .route("/approvals/{id}/reject", post(approvals::reject_action))
        // Costs
        .route("/costs/summary/{company_id}", get(costs::get_cost_summary))
        // Activity
        .route("/activity", get(activity::list_activity))
        // Heartbeat
        .route("/heartbeat/tick/{company_id}", post(heartbeat::trigger_heartbeat_tick));

    let mut router = Router::new()
        .route("/health", get(health::health_check))
        .route("/legacy-dashboard", get(crate::ui::dashboard_html))
        .nest("/api", api_routes.clone())
        .nest("/api/v1", api_routes);

    let ui_dir = std::env::var("PAPERCLIP_UI_DIR")
        .map(PathBuf::from)
        .unwrap_or_else(|_| PathBuf::from("ui/dist"));

    if ui_dir.exists() && ui_dir.join("index.html").exists() {
        let serve_dir = ServeDir::new(&ui_dir)
            .fallback(ServeFile::new(ui_dir.join("index.html")));
        router = router.fallback_service(serve_dir);
    } else {
        router = router
            .route("/", get(crate::ui::dashboard_html))
            .route("/dashboard", get(crate::ui::dashboard_html))
            .fallback(crate::ui::dashboard_html);
    }

    router
        .layer(CorsLayer::permissive())
        .with_state(state)
}
