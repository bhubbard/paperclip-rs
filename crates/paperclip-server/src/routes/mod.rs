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
        .route("/companies/{id}/org", get(companies::get_company_org))
        .route("/companies/{id}/org-chart", get(companies::get_org_chart))
        .route("/companies/{id}/dashboard", get(companies::get_company_dashboard))
        .route("/companies/{id}/agents", get(companies::get_company_agents))
        .route("/companies/{id}/projects", get(companies::get_company_projects))
        .route("/companies/{id}/issues", get(companies::get_company_issues))
        .route("/companies/{id}/approvals", get(companies::get_company_approvals))
        .route("/companies/{id}/activity", get(companies::get_company_activity))
        .route("/companies/{id}/skills", get(companies::get_company_skills))
        .route("/companies/{id}/routines", get(companies::get_company_routines))
        .route("/companies/{id}/live-runs", get(companies::get_company_live_runs))
        .route("/companies/{id}/heartbeat-runs", get(companies::get_company_heartbeat_runs))
        .route("/companies/{id}/user-directory", get(companies::get_company_user_directory))
        .route("/companies/{id}/members", get(companies::get_company_members))
        .route("/companies/{id}/inbox-dismissals", get(companies::get_company_inbox_dismissals))
        .route("/companies/{id}/resource-memberships/me", get(companies::get_company_resource_memberships_me))
        .route("/companies/{id}/join-requests", get(companies::get_company_join_requests))
        .route("/companies/{id}/labels", get(companies::get_company_labels))
        .route("/companies/{id}/goals", get(companies::get_company_goals))
        .route("/companies/{id}/events/ws", get(companies::ws_events_handler))
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
        .route("/heartbeat/tick/{company_id}", post(heartbeat::trigger_heartbeat_tick))
        // Auth session
        .route("/auth/get-session", get(get_session))
        // Adapters
        .route("/adapters", get(get_adapters))
        // UI Plugins & Announcements
        .route("/plugins/ui-contributions", get(get_empty_array))
        .route("/announcements/current", get(get_current_announcements))
        // Instance Settings
        .route("/instance/settings", get(get_instance_settings))
        .route("/instance/settings/general", get(get_instance_settings))
        .route("/instance/settings/experimental", get(get_experimental_settings))
        .fallback(api_not_found);

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

async fn get_session() -> axum::response::Json<serde_json::Value> {
    axum::response::Json(serde_json::json!({
        "session": {
            "id": "local-session",
            "userId": "local-user"
        },
        "user": {
            "id": "local-user",
            "email": "operator@paperclip.local",
            "name": "Local Operator",
            "image": null
        },
        "sentryDsn": null,
        "sentryEnvironment": null
    }))
}

async fn get_adapters() -> axum::response::Json<serde_json::Value> {
    axum::response::Json(serde_json::json!([]))
}

async fn get_empty_array() -> axum::response::Json<serde_json::Value> {
    axum::response::Json(serde_json::json!([]))
}

async fn get_current_announcements() -> axum::response::Json<serde_json::Value> {
    axum::response::Json(serde_json::json!({
        "announcements": []
    }))
}

async fn get_experimental_settings() -> axum::response::Json<serde_json::Value> {
    axum::response::Json(serde_json::json!({
        "enableRunnerPreviewIngress": false,
        "enableWorktreeRunExecution": false,
        "worktreeRunExecutionActivatedAt": null,
        "worktreeRunExecutionActivationInstanceId": null
    }))
}

async fn get_instance_settings() -> axum::response::Json<serde_json::Value> {
    axum::response::Json(serde_json::json!({
        "id": "local-instance",
        "defaultEnvironmentId": null,
        "general": {},
        "experimental": {
            "enableRunnerPreviewIngress": false,
            "enableWorktreeRunExecution": false,
            "worktreeRunExecutionActivatedAt": null,
            "worktreeRunExecutionActivationInstanceId": null
        },
        "createdAt": "2026-09-28T00:00:00Z",
        "updatedAt": "2026-09-28T00:00:00Z"
    }))
}

async fn api_not_found() -> (axum::http::StatusCode, axum::response::Json<serde_json::Value>) {
    (
        axum::http::StatusCode::NOT_FOUND,
        axum::response::Json(serde_json::json!({
            "error": "API route not found"
        }))
    )
}

