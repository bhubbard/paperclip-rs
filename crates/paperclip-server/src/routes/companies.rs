use axum::{
    extract::{
        ws::{Message, WebSocket, WebSocketUpgrade},
        Path, State,
    },
    http::StatusCode,
    response::{IntoResponse, Json},
};
use paperclip_core::models::{
    AgentStatus, ApprovalStatus, Company, IssuePriority, IssueStatus, OrgChart, ProjectStatus,
};
use serde::Deserialize;
use serde_json::{json, Value};
use std::collections::HashSet;
use crate::state::AppState;

#[derive(Deserialize)]
pub struct CreateCompanyPayload {
    pub name: String,
    pub mission: String,
    pub budget_limit_usd: f64,
}

#[derive(Deserialize)]
pub struct SeedCompanyPayload {
    pub name: String,
    pub mission: String,
    pub budget_limit_usd: f64,
}

pub async fn list_companies(State(state): State<AppState>) -> Json<Vec<Company>> {
    Json(state.storage.list_companies())
}

pub async fn get_company(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<Company>, (StatusCode, Json<Value>)> {
    state
        .storage
        .get_company(&id)
        .map(Json)
        .map_err(|e| (StatusCode::NOT_FOUND, Json(json!({ "error": e.to_string() }))))
}

pub async fn create_company(
    State(state): State<AppState>,
    Json(payload): Json<CreateCompanyPayload>,
) -> Result<(StatusCode, Json<Company>), (StatusCode, Json<Value>)> {
    let company = Company::new(payload.name, payload.mission, payload.budget_limit_usd);
    state
        .storage
        .create_company(company)
        .map(|c| (StatusCode::CREATED, Json(c)))
        .map_err(|e| (StatusCode::BAD_REQUEST, Json(json!({ "error": e.to_string() }))))
}

pub async fn seed_company(
    State(state): State<AppState>,
    Json(payload): Json<SeedCompanyPayload>,
) -> Result<(StatusCode, Json<Company>), (StatusCode, Json<Value>)> {
    state
        .storage
        .seed_startup_template(&payload.name, &payload.mission, payload.budget_limit_usd)
        .map(|c| (StatusCode::CREATED, Json(c)))
        .map_err(|e| (StatusCode::BAD_REQUEST, Json(json!({ "error": e.to_string() }))))
}

pub async fn get_org_chart(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<Value>, (StatusCode, Json<Value>)> {
    let agents = state.storage.list_agents(Some(&id));
    let chart = OrgChart::build(&agents)
        .map_err(|e| (StatusCode::BAD_REQUEST, Json(json!({ "error": e.to_string() }))))?;

    let ascii = chart.render_ascii();
    Ok(Json(json!({
        "company_id": id,
        "ascii": ascii,
        "roots": chart.roots,
    })))
}

pub async fn get_companies_stats(State(state): State<AppState>) -> Json<Value> {
    let mut stats = serde_json::Map::new();
    for company in state.storage.list_companies() {
        let agent_count = state.storage.list_agents(Some(&company.id)).len();
        let issue_count = state.storage.list_issues(Some(&company.id), None).len();
        stats.insert(
            company.id,
            json!({
                "agentCount": agent_count,
                "issueCount": issue_count,
            }),
        );
    }
    Json(Value::Object(stats))
}

pub async fn get_company_dashboard(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<Value>, (StatusCode, Json<Value>)> {
    let company = state
        .storage
        .get_company(&id)
        .map_err(|e| (StatusCode::NOT_FOUND, Json(json!({ "error": e.to_string() }))))?;

    let agents = state.storage.list_agents(Some(&id));
    let projects = state.storage.list_projects(Some(&id));
    let project_ids: HashSet<_> = projects.iter().map(|p| p.id.as_str()).collect();
    let all_issues = state.storage.list_issues(None, None);
    let company_issues: Vec<_> = all_issues
        .into_iter()
        .filter(|i| project_ids.contains(i.project_id.as_str()))
        .collect();
    let approvals = state.storage.list_approvals(Some(&id), None);

    let active_agents = agents
        .iter()
        .filter(|a| matches!(a.status, AgentStatus::Running | AgentStatus::Idle))
        .count();
    let running_agents = agents
        .iter()
        .filter(|a| matches!(a.status, AgentStatus::Running))
        .count();
    let paused_agents = agents
        .iter()
        .filter(|a| matches!(a.status, AgentStatus::Paused))
        .count();
    let error_agents = agents
        .iter()
        .filter(|a| matches!(a.status, AgentStatus::Terminated))
        .count();

    let open_tasks = company_issues
        .iter()
        .filter(|i| matches!(i.status, IssueStatus::Todo | IssueStatus::Backlog))
        .count();
    let in_progress_tasks = company_issues
        .iter()
        .filter(|i| matches!(i.status, IssueStatus::InProgress | IssueStatus::InReview))
        .count();
    let blocked_tasks = company_issues
        .iter()
        .filter(|i| matches!(i.status, IssueStatus::Blocked))
        .count();
    let done_tasks = company_issues
        .iter()
        .filter(|i| matches!(i.status, IssueStatus::Done))
        .count();

    let budget_cents = company.budget_monthly_cents.unwrap_or(250_000);
    let spend_cents = company.spent_monthly_cents.unwrap_or(0);
    let util_pct = if budget_cents > 0 {
        (spend_cents * 100) / budget_cents
    } else {
        0
    };
    let pending_approvals = approvals
        .iter()
        .filter(|a| matches!(a.status, ApprovalStatus::Pending))
        .count();

    Ok(Json(json!({
        "companyId": id,
        "agents": {
            "active": active_agents,
            "running": running_agents,
            "paused": paused_agents,
            "error": error_agents,
        },
        "tasks": {
            "open": open_tasks,
            "inProgress": in_progress_tasks,
            "blocked": blocked_tasks,
            "done": done_tasks,
        },
        "costs": {
            "monthSpendCents": spend_cents,
            "monthBudgetCents": budget_cents,
            "monthUtilizationPercent": util_pct,
        },
        "pendingApprovals": pending_approvals,
        "budgets": {
            "activeIncidents": 0,
            "pendingApprovals": pending_approvals,
            "pausedAgents": paused_agents,
            "pausedProjects": 0,
        },
        "runActivity": [],
    })))
}

pub async fn get_company_agents(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Json<Value> {
    let agents = state.storage.list_agents(Some(&id));
    let formatted: Vec<_> = agents
        .into_iter()
        .map(|a| {
            let budget_cents = (a.budget_limit_usd * 100.0) as i64;
            let spent_cents = (a.budget_spent_usd * 100.0) as i64;
            let status_str = match a.status {
                AgentStatus::Idle => "idle",
                AgentStatus::Running => "running",
                AgentStatus::Paused => "paused",
                AgentStatus::Terminated => "terminated",
            };
            let role_str = a.role.to_string().to_lowercase();
            json!({
                "id": a.id,
                "companyId": a.company_id,
                "name": a.name,
                "urlKey": a.name.to_lowercase().replace(' ', "-"),
                "role": role_str,
                "title": a.name,
                "icon": "bot",
                "status": status_str,
                "reportsTo": a.reports_to,
                "capabilities": a.capabilities.join(", "),
                "adapterType": format!("{:?}", a.adapter_type).to_lowercase(),
                "adapterConfig": {},
                "runtimeConfig": {},
                "budgetMonthlyCents": budget_cents,
                "spentMonthlyCents": spent_cents,
                "pauseReason": null,
                "pausedAt": null,
                "permissions": { "canCreateAgents": false },
                "lastHeartbeatAt": a.updated_at.to_rfc3339(),
                "metadata": null,
                "createdAt": a.created_at.to_rfc3339(),
                "updatedAt": a.updated_at.to_rfc3339(),
            })
        })
        .collect();
    Json(json!(formatted))
}

pub async fn get_company_projects(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Json<Value> {
    let projects = state.storage.list_projects(Some(&id));
    let formatted: Vec<_> = projects
        .into_iter()
        .map(|p| {
            let status_str = match p.status {
                ProjectStatus::Planning => "planning",
                ProjectStatus::Active => "active",
                ProjectStatus::Paused => "paused",
                ProjectStatus::Completed => "completed",
            };
            json!({
                "id": p.id,
                "companyId": p.company_id,
                "goalId": p.goal_id,
                "name": p.name,
                "description": p.description,
                "leadAgentId": p.lead_agent_id,
                "status": status_str,
                "workspaces": [],
                "createdAt": p.created_at.to_rfc3339(),
                "updatedAt": p.updated_at.to_rfc3339(),
            })
        })
        .collect();
    Json(json!(formatted))
}

pub async fn get_company_issues(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Json<Value> {
    let projects = state.storage.list_projects(Some(&id));
    let project_ids: HashSet<_> = projects.iter().map(|p| p.id.as_str()).collect();
    let all_issues = state.storage.list_issues(None, None);
    let company = state.storage.get_company(&id).ok();
    let prefix = company
        .map(|c| c.issue_prefix)
        .unwrap_or_else(|| "PAP".to_string());

    let mut num = 1;
    let mut formatted = Vec::new();
    for i in all_issues {
        if project_ids.contains(i.project_id.as_str()) {
            let status_str = match i.status {
                IssueStatus::Backlog => "backlog",
                IssueStatus::Todo => "todo",
                IssueStatus::InProgress => "in_progress",
                IssueStatus::InReview => "in_review",
                IssueStatus::Done => "done",
                IssueStatus::Blocked => "blocked",
            };
            let priority_str = match i.priority {
                IssuePriority::Low => "low",
                IssuePriority::Medium => "medium",
                IssuePriority::High => "high",
                IssuePriority::Urgent => "urgent",
            };
            formatted.push(json!({
                "id": i.id,
                "companyId": id,
                "projectId": i.project_id,
                "projectWorkspaceId": null,
                "identifier": format!("{}-{}", prefix, num),
                "issueNumber": num,
                "title": i.title,
                "description": i.description,
                "status": status_str,
                "priority": priority_str,
                "assigneeAgentId": i.assignee_agent_id,
                "assigneeUserId": null,
                "reporterAgentId": i.reporter_agent_id,
                "parentIssueId": i.parent_issue_id,
                "labels": i.labels,
                "labelIds": [],
                "blockedBy": [],
                "blocks": [],
                "workProducts": [],
                "isUnreadForMe": false,
                "createdAt": i.created_at.to_rfc3339(),
                "updatedAt": i.updated_at.to_rfc3339(),
            }));
            num += 1;
        }
    }
    Json(json!(formatted))
}

pub async fn get_company_approvals(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Json<Value> {
    let approvals = state.storage.list_approvals(Some(&id), None);
    let formatted: Vec<_> = approvals
        .into_iter()
        .map(|a| {
            let status_str = match a.status {
                ApprovalStatus::Pending => "pending",
                ApprovalStatus::Approved => "approved",
                ApprovalStatus::Rejected => "rejected",
            };
            json!({
                "id": a.id,
                "companyId": a.company_id,
                "type": a.action.to_string().to_lowercase().replace(' ', "_"),
                "requestedByAgentId": a.agent_id,
                "requestedByUserId": null,
                "status": status_str,
                "payload": {},
                "decisionNote": a.description,
                "decidedByUserId": a.reviewed_by,
                "decidedAt": a.reviewed_at.map(|t| t.to_rfc3339()),
                "createdAt": a.created_at.to_rfc3339(),
                "updatedAt": a.created_at.to_rfc3339(),
            })
        })
        .collect();
    Json(json!(formatted))
}

pub async fn get_company_activity(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Json<Value> {
    let logs = state.storage.list_activity(Some(&id), 50);
    let formatted: Vec<_> = logs
        .into_iter()
        .map(|l| {
            let actor_type_str = match l.actor_type {
                paperclip_core::models::ActorType::Agent => "agent",
                paperclip_core::models::ActorType::Human => "user",
                paperclip_core::models::ActorType::System => "system",
            };
            json!({
                "id": l.id,
                "companyId": l.company_id,
                "actorType": actor_type_str,
                "actorId": l.actor_id,
                "action": l.action,
                "entityType": "agent",
                "entityId": l.actor_id,
                "agentId": l.actor_id,
                "runId": null,
                "details": { "summary": l.details },
                "createdAt": l.timestamp.to_rfc3339(),
            })
        })
        .collect();
    Json(json!(formatted))
}

pub async fn get_company_skills() -> Json<Value> {
    Json(json!([]))
}

pub async fn get_company_routines() -> Json<Value> {
    Json(json!([]))
}

pub async fn get_company_live_runs() -> Json<Value> {
    Json(json!([]))
}

pub async fn get_company_heartbeat_runs() -> Json<Value> {
    Json(json!([]))
}

pub async fn get_company_user_directory() -> Json<Value> {
    Json(json!([
        {
            "id": "local-user",
            "name": "Local Operator",
            "email": "operator@paperclip.local",
            "role": "owner"
        }
    ]))
}

pub async fn get_company_members(Path(id): Path<String>) -> Json<Value> {
    Json(json!({
        "members": [
            {
                "id": "local-member",
                "companyId": id,
                "userId": "local-user",
                "role": "owner"
            }
        ]
    }))
}

pub async fn get_company_inbox_dismissals() -> Json<Value> {
    Json(json!([]))
}

pub async fn get_company_resource_memberships_me() -> Json<Value> {
    Json(json!({
        "projectMemberships": {},
        "agentMemberships": {},
        "starredProjectIds": [],
        "starredAgentIds": [],
        "starredDocumentIds": [],
        "projectStarredAt": {},
        "agentStarredAt": {},
        "documentStarredAt": {},
        "updatedAt": null,
    }))
}

pub async fn get_company_org(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Json<Value> {
    let agents = state.storage.list_agents(Some(&id));
    let mut by_id = std::collections::HashMap::new();
    let mut children_map: std::collections::HashMap<Option<String>, Vec<paperclip_core::models::Agent>> = std::collections::HashMap::new();
    for a in &agents {
        by_id.insert(a.id.clone(), a.clone());
        children_map.entry(a.reports_to.clone()).or_default().push(a.clone());
    }

    fn build_org_node(
        agent: &paperclip_core::models::Agent,
        children_map: &std::collections::HashMap<Option<String>, Vec<paperclip_core::models::Agent>>,
    ) -> Value {
        let child_agents = children_map.get(&Some(agent.id.clone())).cloned().unwrap_or_default();
        let reports: Vec<Value> = child_agents
            .iter()
            .map(|c| build_org_node(c, children_map))
            .collect();
        let status_str = match agent.status {
            AgentStatus::Idle => "idle",
            AgentStatus::Running => "running",
            AgentStatus::Paused => "paused",
            AgentStatus::Terminated => "terminated",
        };
        json!({
            "id": agent.id,
            "name": agent.name,
            "role": agent.role.to_string().to_lowercase(),
            "status": status_str,
            "reports": reports,
        })
    }

    let mut roots = Vec::new();
    for a in &agents {
        let is_root = match &a.reports_to {
            None => true,
            Some(pid) => !by_id.contains_key(pid),
        };
        if is_root {
            roots.push(build_org_node(a, &children_map));
        }
    }
    Json(json!(roots))
}

pub async fn get_company_join_requests() -> Json<Value> {
    Json(json!([]))
}

pub async fn get_company_labels() -> Json<Value> {
    Json(json!([]))
}

pub async fn get_company_goals(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Json<Value> {
    let goals = state.storage.list_goals(Some(&id));
    let formatted: Vec<_> = goals
        .into_iter()
        .map(|g| {
            json!({
                "id": g.id,
                "companyId": g.company_id,
                "title": g.title,
                "description": g.description,
                "level": "company",
                "status": "active",
                "parentId": null,
                "ownerAgentId": null,
                "createdAt": g.created_at.to_rfc3339(),
                "updatedAt": g.updated_at.to_rfc3339(),
            })
        })
        .collect();
    Json(json!(formatted))
}

pub async fn ws_events_handler(
    ws: WebSocketUpgrade,
    Path(id): Path<String>,
) -> impl IntoResponse {
    ws.on_upgrade(move |socket| handle_events_socket(socket, id))
}

async fn handle_events_socket(mut socket: WebSocket, _company_id: String) {
    while let Some(Ok(msg)) = socket.recv().await {
        match msg {
            Message::Close(_) => break,
            Message::Ping(payload) => {
                if socket.send(Message::Pong(payload)).await.is_err() {
                    break;
                }
            }
            _ => {}
        }
    }
}
