use std::sync::Arc;
use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use tower::ServiceExt; // for `oneshot`
use paperclip_core::budget::BudgetEnforcer;
use paperclip_core::storage::Storage;
use paperclip_runner::{AgentSupervisor, HeartbeatDispatcher, MockProvider};
use paperclip_server::{build_router, AppState};

#[tokio::test]
async fn test_health_and_info_endpoints() {
    let storage = Arc::new(Storage::new_in_memory());
    let _company = storage
        .seed_startup_template("Acme Labs", "Autonomous multi-agent enterprise", 25_000.0)
        .expect("Seed template");

    let provider = Arc::new(MockProvider::default());
    let supervisor = Arc::new(AgentSupervisor::new());
    let dispatcher = Arc::new(HeartbeatDispatcher::new(
        storage.clone(),
        provider,
        supervisor,
        BudgetEnforcer::default(),
    ));

    let state = AppState::new(storage, dispatcher);
    let app = build_router(state);

    // Test /health
    let response = app
        .clone()
        .oneshot(Request::builder().uri("/health").body(Body::empty()).unwrap())
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    // Test /api/v1/info
    let response = app
        .oneshot(Request::builder().uri("/api/v1/info").body(Body::empty()).unwrap())
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
}
