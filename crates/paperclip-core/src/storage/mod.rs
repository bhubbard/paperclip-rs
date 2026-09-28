use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::RwLock;
use serde::{Deserialize, Serialize};

use crate::error::PaperclipError;
use crate::models::*;

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
struct StorageSnapshot {
    companies: HashMap<String, Company>,
    agents: HashMap<String, Agent>,
    goals: HashMap<String, Goal>,
    projects: HashMap<String, Project>,
    issues: HashMap<String, Issue>,
    approvals: HashMap<String, Approval>,
    cost_events: Vec<CostEvent>,
    activity_logs: Vec<ActivityLog>,
    heartbeat_runs: HashMap<String, HeartbeatRun>,
}

pub struct Storage {
    snapshot: RwLock<StorageSnapshot>,
    persist_path: Option<PathBuf>,
}

impl Storage {
    pub fn new_in_memory() -> Self {
        Self {
            snapshot: RwLock::new(StorageSnapshot::default()),
            persist_path: None,
        }
    }

    pub fn new_persistent(path: impl AsRef<Path>) -> Result<Self, PaperclipError> {
        let path = path.as_ref().to_path_buf();
        let mut snapshot: StorageSnapshot = if path.exists() {
            let data = std::fs::read_to_string(&path)?;
            serde_json::from_str(&data)?
        } else {
            StorageSnapshot::default()
        };

        for company in snapshot.companies.values_mut() {
            company.normalize_defaults();
        }

        Ok(Self {
            snapshot: RwLock::new(snapshot),
            persist_path: Some(path),
        })
    }

    fn persist_if_needed(&self) -> Result<(), PaperclipError> {
        if let Some(path) = &self.persist_path {
            if let Some(parent) = path.parent() {
                std::fs::create_dir_all(parent)?;
            }
            let data = {
                let guard = self.snapshot.read().unwrap();
                serde_json::to_string_pretty(&*guard)?
            };
            std::fs::write(path, data)?;
        }
        Ok(())
    }

    // Company CRUD
    pub fn create_company(&self, company: Company) -> Result<Company, PaperclipError> {
        let mut guard = self.snapshot.write().unwrap();
        let id = company.id.clone();
        guard.companies.insert(id, company.clone());
        drop(guard);
        self.persist_if_needed()?;
        Ok(company)
    }

    pub fn get_company(&self, id: &str) -> Result<Company, PaperclipError> {
        let guard = self.snapshot.read().unwrap();
        guard
            .companies
            .get(id)
            .cloned()
            .ok_or_else(|| PaperclipError::NotFound(format!("Company {}", id)))
    }

    pub fn list_companies(&self) -> Vec<Company> {
        let guard = self.snapshot.read().unwrap();
        guard.companies.values().cloned().collect()
    }

    pub fn update_company<F>(&self, id: &str, f: F) -> Result<Company, PaperclipError>
    where
        F: FnOnce(&mut Company),
    {
        let mut guard = self.snapshot.write().unwrap();
        let company = guard
            .companies
            .get_mut(id)
            .ok_or_else(|| PaperclipError::NotFound(format!("Company {}", id)))?;
        f(company);
        let updated = company.clone();
        drop(guard);
        self.persist_if_needed()?;
        Ok(updated)
    }

    // Agent CRUD
    pub fn create_agent(&self, agent: Agent) -> Result<Agent, PaperclipError> {
        let mut guard = self.snapshot.write().unwrap();
        let id = agent.id.clone();
        guard.agents.insert(id, agent.clone());
        drop(guard);
        self.persist_if_needed()?;
        Ok(agent)
    }

    pub fn get_agent(&self, id: &str) -> Result<Agent, PaperclipError> {
        let guard = self.snapshot.read().unwrap();
        guard
            .agents
            .get(id)
            .cloned()
            .ok_or_else(|| PaperclipError::NotFound(format!("Agent {}", id)))
    }

    pub fn list_agents(&self, company_id: Option<&str>) -> Vec<Agent> {
        let guard = self.snapshot.read().unwrap();
        guard
            .agents
            .values()
            .filter(|a| company_id.map_or(true, |cid| a.company_id == cid))
            .cloned()
            .collect()
    }

    pub fn update_agent<F>(&self, id: &str, f: F) -> Result<Agent, PaperclipError>
    where
        F: FnOnce(&mut Agent),
    {
        let mut guard = self.snapshot.write().unwrap();
        let agent = guard
            .agents
            .get_mut(id)
            .ok_or_else(|| PaperclipError::NotFound(format!("Agent {}", id)))?;
        f(agent);
        let updated = agent.clone();
        drop(guard);
        self.persist_if_needed()?;
        Ok(updated)
    }

    // Goal CRUD
    pub fn create_goal(&self, goal: Goal) -> Result<Goal, PaperclipError> {
        let mut guard = self.snapshot.write().unwrap();
        let id = goal.id.clone();
        guard.goals.insert(id, goal.clone());
        drop(guard);
        self.persist_if_needed()?;
        Ok(goal)
    }

    pub fn list_goals(&self, company_id: Option<&str>) -> Vec<Goal> {
        let guard = self.snapshot.read().unwrap();
        guard
            .goals
            .values()
            .filter(|g| company_id.map_or(true, |cid| g.company_id == cid))
            .cloned()
            .collect()
    }

    pub fn update_goal<F>(&self, id: &str, f: F) -> Result<Goal, PaperclipError>
    where
        F: FnOnce(&mut Goal),
    {
        let mut guard = self.snapshot.write().unwrap();
        let goal = guard
            .goals
            .get_mut(id)
            .ok_or_else(|| PaperclipError::NotFound(format!("Goal {}", id)))?;
        f(goal);
        let updated = goal.clone();
        drop(guard);
        self.persist_if_needed()?;
        Ok(updated)
    }

    // Project CRUD
    pub fn create_project(&self, project: Project) -> Result<Project, PaperclipError> {
        let mut guard = self.snapshot.write().unwrap();
        let id = project.id.clone();
        guard.projects.insert(id, project.clone());
        drop(guard);
        self.persist_if_needed()?;
        Ok(project)
    }

    pub fn list_projects(&self, company_id: Option<&str>) -> Vec<Project> {
        let guard = self.snapshot.read().unwrap();
        guard
            .projects
            .values()
            .filter(|p| company_id.map_or(true, |cid| p.company_id == cid))
            .cloned()
            .collect()
    }

    pub fn get_project(&self, id: &str) -> Result<Project, PaperclipError> {
        let guard = self.snapshot.read().unwrap();
        guard
            .projects
            .get(id)
            .cloned()
            .ok_or_else(|| PaperclipError::NotFound(format!("Project {}", id)))
    }

    // Issue CRUD
    pub fn create_issue(&self, issue: Issue) -> Result<Issue, PaperclipError> {
        let mut guard = self.snapshot.write().unwrap();
        let id = issue.id.clone();
        guard.issues.insert(id, issue.clone());
        drop(guard);
        self.persist_if_needed()?;
        Ok(issue)
    }

    pub fn get_issue(&self, id: &str) -> Result<Issue, PaperclipError> {
        let guard = self.snapshot.read().unwrap();
        guard
            .issues
            .get(id)
            .cloned()
            .ok_or_else(|| PaperclipError::NotFound(format!("Issue {}", id)))
    }

    pub fn list_issues(&self, project_id: Option<&str>, status: Option<IssueStatus>) -> Vec<Issue> {
        let guard = self.snapshot.read().unwrap();
        guard
            .issues
            .values()
            .filter(|i| project_id.map_or(true, |pid| i.project_id == pid))
            .filter(|i| status.as_ref().map_or(true, |s| &i.status == s))
            .cloned()
            .collect()
    }

    pub fn update_issue<F>(&self, id: &str, f: F) -> Result<Issue, PaperclipError>
    where
        F: FnOnce(&mut Issue),
    {
        let mut guard = self.snapshot.write().unwrap();
        let issue = guard
            .issues
            .get_mut(id)
            .ok_or_else(|| PaperclipError::NotFound(format!("Issue {}", id)))?;
        f(issue);
        let updated = issue.clone();
        drop(guard);
        self.persist_if_needed()?;
        Ok(updated)
    }

    // Approval CRUD
    pub fn create_approval(&self, approval: Approval) -> Result<Approval, PaperclipError> {
        let mut guard = self.snapshot.write().unwrap();
        let id = approval.id.clone();
        guard.approvals.insert(id, approval.clone());
        drop(guard);
        self.persist_if_needed()?;
        Ok(approval)
    }

    pub fn list_approvals(&self, company_id: Option<&str>, status: Option<ApprovalStatus>) -> Vec<Approval> {
        let guard = self.snapshot.read().unwrap();
        guard
            .approvals
            .values()
            .filter(|a| company_id.map_or(true, |cid| a.company_id == cid))
            .filter(|a| status.as_ref().map_or(true, |s| &a.status == s))
            .cloned()
            .collect()
    }

    pub fn review_approval(&self, id: &str, approve: bool, reviewer: &str) -> Result<Approval, PaperclipError> {
        let mut guard = self.snapshot.write().unwrap();
        let approval = guard
            .approvals
            .get_mut(id)
            .ok_or_else(|| PaperclipError::NotFound(format!("Approval {}", id)))?;
        if approve {
            approval.approve(reviewer);
        } else {
            approval.reject(reviewer);
        }
        let updated = approval.clone();
        drop(guard);
        self.persist_if_needed()?;
        Ok(updated)
    }

    // Cost tracking
    pub fn record_cost(&self, event: CostEvent) -> Result<(), PaperclipError> {
        let mut guard = self.snapshot.write().unwrap();
        let amount = event.cost_usd;
        let company_id = event.company_id.clone();
        let agent_id = event.agent_id.clone();

        if let Some(company) = guard.companies.get_mut(&company_id) {
            company.record_spend(amount);
        }
        if let Some(agent) = guard.agents.get_mut(&agent_id) {
            agent.record_spend(amount);
        }
        guard.cost_events.push(event);
        drop(guard);
        self.persist_if_needed()?;
        Ok(())
    }

    pub fn get_cost_summary(&self, company_id: &str) -> CostSummary {
        let guard = self.snapshot.read().unwrap();
        let events: Vec<&CostEvent> = guard
            .cost_events
            .iter()
            .filter(|e| e.company_id == company_id)
            .collect();

        let total_cost_usd: f64 = events.iter().map(|e| e.cost_usd).sum();
        let total_input_tokens: u64 = events.iter().map(|e| e.input_tokens).sum();
        let total_output_tokens: u64 = events.iter().map(|e| e.output_tokens).sum();

        let mut agent_costs: HashMap<String, (f64, u64)> = HashMap::new();
        for e in &events {
            let entry = agent_costs.entry(e.agent_id.clone()).or_insert((0.0, 0));
            entry.0 += e.cost_usd;
            entry.1 += e.total_tokens;
        }

        let agents = guard
            .agents
            .values()
            .filter(|a| a.company_id == company_id);

        let mut breakdown = Vec::new();
        for agent in agents {
            let (cost, tokens) = agent_costs.get(&agent.id).cloned().unwrap_or((agent.budget_spent_usd, 0));
            let pct = if agent.budget_limit_usd > 0.0 {
                (cost / agent.budget_limit_usd) * 100.0
            } else {
                0.0
            };
            breakdown.push(AgentCostBreakdown {
                agent_id: agent.id.clone(),
                agent_name: agent.name.clone(),
                role: agent.role.to_string(),
                cost_usd: cost,
                budget_limit_usd: agent.budget_limit_usd,
                budget_percent: pct,
                total_tokens: tokens,
            });
        }

        CostSummary {
            company_id: company_id.to_string(),
            total_cost_usd,
            total_input_tokens,
            total_output_tokens,
            total_tokens: total_input_tokens + total_output_tokens,
            agent_breakdown: breakdown,
        }
    }

    // Activity Logs
    pub fn log_activity(&self, log: ActivityLog) -> Result<(), PaperclipError> {
        let mut guard = self.snapshot.write().unwrap();
        guard.activity_logs.push(log);
        drop(guard);
        self.persist_if_needed()?;
        Ok(())
    }

    pub fn list_activity(&self, company_id: Option<&str>, limit: usize) -> Vec<ActivityLog> {
        let guard = self.snapshot.read().unwrap();
        guard
            .activity_logs
            .iter()
            .filter(|l| company_id.map_or(true, |cid| l.company_id == cid))
            .rev()
            .take(limit)
            .cloned()
            .collect()
    }

    // Heartbeat Runs
    pub fn record_heartbeat(&self, run: HeartbeatRun) -> Result<(), PaperclipError> {
        let mut guard = self.snapshot.write().unwrap();
        guard.heartbeat_runs.insert(run.id.clone(), run);
        drop(guard);
        self.persist_if_needed()?;
        Ok(())
    }

    pub fn list_heartbeats(&self, company_id: Option<&str>) -> Vec<HeartbeatRun> {
        let guard = self.snapshot.read().unwrap();
        guard
            .heartbeat_runs
            .values()
            .filter(|h| company_id.map_or(true, |cid| h.company_id == cid))
            .cloned()
            .collect()
    }

    // Onboarding / Seeding Template
    pub fn seed_startup_template(
        &self,
        company_name: &str,
        mission: &str,
        budget_limit_usd: f64,
    ) -> Result<Company, PaperclipError> {
        let company = self.create_company(Company::new(
            company_name,
            mission,
            budget_limit_usd,
        ))?;
        let cid = &company.id;

        // 1. CEO
        let ceo = self.create_agent(Agent::new(
            cid,
            "Nova (CEO)",
            AgentRole::Ceo,
            AgentAdapterType::Mock,
            "You are the autonomous CEO responsible for setting strategy, reviewing milestones, and approving budgets.",
            None,
            budget_limit_usd * 0.3,
        ).with_capabilities(vec!["governance".into(), "planning".into(), "delegation".into()]))?;

        // 2. CTO
        let cto = self.create_agent(Agent::new(
            cid,
            "Atlas (CTO)",
            AgentRole::Cto,
            AgentAdapterType::Mock,
            "You are the Chief Technology Officer responsible for system architecture, technical velocity, and code quality.",
            Some(ceo.id.clone()),
            budget_limit_usd * 0.25,
        ).with_capabilities(vec!["architecture".into(), "code_review".into(), "infrastructure".into()]))?;

        // 3. Lead Engineer
        let engineer = self.create_agent(Agent::new(
            cid,
            "Cipher (Lead Engineer)",
            AgentRole::LeadEngineer,
            AgentAdapterType::Mock,
            "You are the Lead Software Engineer responsible for implementing core services, fixing bugs, and writing tests.",
            Some(cto.id.clone()),
            budget_limit_usd * 0.2,
        ).with_capabilities(vec!["rust".into(), "typescript".into(), "system_design".into()]))?;

        // 4. QA Engineer
        let qa = self.create_agent(Agent::new(
            cid,
            "Sentinel (QA Engineer)",
            AgentRole::QaEngineer,
            AgentAdapterType::Mock,
            "You are the QA Engineer verifying test suites, reliability criteria, regression prevention, and edge cases.",
            Some(cto.id.clone()),
            budget_limit_usd * 0.15,
        ).with_capabilities(vec!["testing".into(), "fuzzing".into(), "benchmarking".into()]))?;

        // 5. Product Manager
        let _pm = self.create_agent(Agent::new(
            cid,
            "Iris (Product Manager)",
            AgentRole::ProductManager,
            AgentAdapterType::Mock,
            "You are the Product Manager turning strategic business goals into sprint backlogs and specifications.",
            Some(ceo.id.clone()),
            budget_limit_usd * 0.1,
        ).with_capabilities(vec!["specs".into(), "user_stories".into(), "prioritization".into()]))?;

        // Primary Goal
        let goal = self.create_goal(
            Goal::new(
                cid,
                "Launch Autonomous SaaS Engine to 10k Users",
                "Deploy production-grade multi-agent orchestrator with robust budget governance and sub-second task execution.",
                GoalPriority::Urgent,
            )
            .with_target_metric("10,000 active users; 99.99% reliability")
            .with_owner(ceo.id.clone()),
        )?;

        // Primary Project
        let project = self.create_project(
            Project::new(
                cid,
                "Core Engine & API Infrastructure",
                "Build and harden the native orchestrator core, runner supervisor, and REST endpoints.",
            )
            .with_goal(goal.id.clone())
            .with_lead(cto.id.clone()),
        )?;

        // Initial Issues
        self.create_issue(
            Issue::new(
                project.id.clone(),
                "Implement Process Supervisor & Agent Heartbeat Tick",
                "Create supervisor loop to poll assigned issues and execute provider tasks with timeout watchdog.",
                IssuePriority::High,
            )
            .with_assignee(engineer.id.clone())
            .with_labels(vec!["core".into(), "runner".into()]),
        )?;

        self.create_issue(
            Issue::new(
                project.id.clone(),
                "Automate End-to-End Conformance & Regression Test Suite",
                "Add integration verification checking org chart recursion, budget caps, and simulated agent turn limits.",
                IssuePriority::Medium,
            )
            .with_assignee(qa.id.clone())
            .with_labels(vec!["qa".into(), "conformance".into()]),
        )?;

        self.log_activity(ActivityLog::new(
            cid,
            &ceo.id,
            ActorType::Agent,
            "company_initialized",
            format!("Company '{}' onboarded with 5 agents and initial project backlog.", company_name),
        ))?;

        Ok(company)
    }
}
