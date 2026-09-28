use std::sync::Arc;
use paperclip_core::budget::BudgetEnforcer;
use paperclip_core::storage::Storage;
use paperclip_runner::*;

#[tokio::test]
async fn test_mock_provider_and_heartbeat_tick() {
    let storage = Arc::new(Storage::new_in_memory());
    let company = storage
        .seed_startup_template("Test Corp", "Testing automated runner", 10_000.0)
        .expect("Seed template");

    let provider = Arc::new(MockProvider::default());
    let supervisor = Arc::new(AgentSupervisor::new());
    let dispatcher = HeartbeatDispatcher::new(
        storage.clone(),
        provider,
        supervisor,
        BudgetEnforcer::default(),
    );

    let ticks = dispatcher.tick(&company.id).await.expect("Heartbeat tick");
    assert!(!ticks.is_empty(), "Should execute at least one tick on pending issues");

    for t in &ticks {
        assert!(t.success);
        assert!(t.cost_usd > 0.0);
    }

    let summary = storage.get_cost_summary(&company.id);
    assert!(summary.total_cost_usd > 0.0);
    assert!(summary.total_tokens > 0);

    let activity = storage.list_activity(Some(&company.id), 10);
    assert!(activity.iter().any(|a| a.action == "task_executed"));
}

#[tokio::test]
async fn test_supervisor_registration() {
    let supervisor = AgentSupervisor::new();
    let task = SupervisedTask {
        run_id: "run-123".into(),
        agent_id: "agent-123".into(),
        issue_id: "issue-123".into(),
    };

    supervisor.register_run(task);
    assert_eq!(supervisor.active_count(), 1);
    assert!(supervisor.is_agent_busy("agent-123"));

    let removed = supervisor.unregister_run("run-123");
    assert!(removed.is_some());
    assert_eq!(supervisor.active_count(), 0);
    assert!(!supervisor.is_agent_busy("agent-123"));
}
