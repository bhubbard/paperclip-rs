use paperclip_core::*;

#[test]
fn test_org_chart_hierarchy_and_ascii() {
    let company = Company::new("Acme AI Inc.", "Build helpful software", 10_000.0);
    let ceo = Agent::new(&company.id, "CEO Alice", AgentRole::Ceo, AgentAdapterType::Mock, "Strategy", None, 3000.0);
    let cto = Agent::new(&company.id, "CTO Bob", AgentRole::Cto, AgentAdapterType::Mock, "Engineering", Some(ceo.id.clone()), 2500.0);
    let eng1 = Agent::new(&company.id, "Eng Charlie", AgentRole::LeadEngineer, AgentAdapterType::Mock, "Code", Some(cto.id.clone()), 1500.0);
    let eng2 = Agent::new(&company.id, "Eng Diana", AgentRole::QaEngineer, AgentAdapterType::Mock, "Testing", Some(cto.id.clone()), 1000.0);

    let agents = vec![ceo.clone(), cto.clone(), eng1.clone(), eng2.clone()];
    let org_chart = OrgChart::build(&agents).expect("Should build org chart");
    assert_eq!(org_chart.roots.len(), 1);
    assert_eq!(org_chart.roots[0].agent_id, ceo.id);
    assert_eq!(org_chart.roots[0].children.len(), 1);
    assert_eq!(org_chart.roots[0].children[0].agent_id, cto.id);
    assert_eq!(org_chart.roots[0].children[0].children.len(), 2);

    let ascii = org_chart.render_ascii();
    assert!(ascii.contains("CEO Alice (CEO)"));
    assert!(ascii.contains("CTO Bob (CTO)"));
    assert!(ascii.contains("Eng Charlie (Lead Engineer)"));
}

#[test]
fn test_org_chart_cycle_detection() {
    let company = Company::new("Acme AI Inc.", "Test", 1000.0);
    let a1 = Agent::new(&company.id, "A1", AgentRole::Ceo, AgentAdapterType::Mock, "Test", None, 100.0);
    let a2 = Agent::new(&company.id, "A2", AgentRole::Cto, AgentAdapterType::Mock, "Test", Some(a1.id.clone()), 100.0);
    let mut a1_cyclic = a1.clone();
    a1_cyclic.reports_to = Some(a2.id.clone()); // Cycle: A1 -> A2 -> A1

    let res = OrgChart::build(&[a1_cyclic, a2]);
    assert!(res.is_err());
}

#[test]
fn test_budget_enforcement() {
    let company = Company::new("Acme AI Inc.", "Test", 100.0);
    let agent = Agent::new(&company.id, "Agent 1", AgentRole::LeadEngineer, AgentAdapterType::Mock, "Prompt", None, 50.0);

    let enforcer = BudgetEnforcer::new(BudgetPolicy::HardStop);
    assert!(enforcer.check_spend(&company, &agent, 20.0).is_ok());

    // Spend exceeds agent budget
    let result = enforcer.check_spend(&company, &agent, 60.0);
    assert!(result.is_err());

    // Test RequireApproval policy
    let approval_enforcer = BudgetEnforcer::new(BudgetPolicy::RequireApproval);
    let approval_opt = approval_enforcer.check_spend(&company, &agent, 60.0).unwrap();
    assert!(approval_opt.is_some());
    let approval = approval_opt.unwrap();
    assert_eq!(approval.action, ApprovalAction::BudgetIncrease);
}

#[test]
fn test_storage_template_seeding() {
    let storage = Storage::new_in_memory();
    let company = storage
        .seed_startup_template("Paperclip Labs", "Autonomous multi-agent enterprise", 50_000.0)
        .expect("Seed template should succeed");

    assert_eq!(company.name, "Paperclip Labs");
    let agents = storage.list_agents(Some(&company.id));
    assert_eq!(agents.len(), 5);

    let projects = storage.list_projects(Some(&company.id));
    assert_eq!(projects.len(), 1);

    let issues = storage.list_issues(Some(&projects[0].id), None);
    assert_eq!(issues.len(), 2);

    let summary = storage.get_cost_summary(&company.id);
    assert_eq!(summary.agent_breakdown.len(), 5);
}
