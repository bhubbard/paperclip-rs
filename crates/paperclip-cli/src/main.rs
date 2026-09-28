use std::net::SocketAddr;
use std::path::PathBuf;
use std::sync::Arc;
use clap::{Args, Parser, Subcommand};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use paperclip_core::budget::BudgetEnforcer;
use paperclip_core::models::*;
use paperclip_core::storage::Storage;
use paperclip_runner::{AgentSupervisor, HeartbeatDispatcher, MockProvider};
use paperclip_server::{run_server, AppState};

#[derive(Parser, Debug)]
#[command(name = "paperclip")]
#[command(author = "Brandon Hubbard")]
#[command(version = "0.1.0")]
#[command(about = "High-performance Rust control plane and orchestrator for autonomous AI agent companies", long_about = None)]
struct Cli {
    #[arg(short, long, global = true, help = "Path to data store file (default: .paperclip/data.json)")]
    data_path: Option<PathBuf>,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    #[command(about = "Onboard a new virtual AI company from a template")]
    Onboard(OnboardArgs),

    #[command(about = "Start the Paperclip REST API server and control plane")]
    Serve(ServeArgs),

    #[command(about = "Manage companies and view org charts")]
    Company {
        #[command(subcommand)]
        cmd: CompanyCommands,
    },

    #[command(about = "Manage AI agents, roles, and status")]
    Agent {
        #[command(subcommand)]
        cmd: AgentCommands,
    },

    #[command(about = "Manage company goals and target metrics")]
    Goal {
        #[command(subcommand)]
        cmd: GoalCommands,
    },

    #[command(about = "Manage projects")]
    Project {
        #[command(subcommand)]
        cmd: ProjectCommands,
    },

    #[command(about = "Manage Kanban issues and task backlogs")]
    Issue {
        #[command(subcommand)]
        cmd: IssueCommands,
    },

    #[command(about = "Execute an orchestration heartbeat tick across agents")]
    Heartbeat(HeartbeatArgs),

    #[command(about = "Review and resolve human-in-the-loop approvals")]
    Approval {
        #[command(subcommand)]
        cmd: ApprovalCommands,
    },

    #[command(about = "View token costs, spend analytics, and budget status")]
    Cost(CostArgs),

    #[command(about = "View company activity stream and audit logs")]
    Activity(ActivityArgs),
}

#[derive(Args, Debug)]
struct OnboardArgs {
    #[arg(short, long, default_value = "Paperclip Labs")]
    name: String,

    #[arg(short, long, default_value = "Autonomous multi-agent enterprise")]
    mission: String,

    #[arg(short, long, default_value_t = 50000.0)]
    budget: f64,
}

#[derive(Args, Debug)]
struct ServeArgs {
    #[arg(short = 'H', long, default_value = "127.0.0.1")]
    host: String,

    #[arg(short, long, default_value_t = 3100)]
    port: u16,
}

#[derive(Subcommand, Debug)]
enum CompanyCommands {
    #[command(about = "List all companies")]
    List,
    #[command(about = "Show company details")]
    Show { id: String },
    #[command(about = "Render the company ASCII org chart hierarchy")]
    Org { id: String },
}

#[derive(Subcommand, Debug)]
enum AgentCommands {
    #[command(about = "List agents")]
    List {
        #[arg(short, long)]
        company_id: Option<String>,
    },
    #[command(about = "Hire a new agent")]
    Hire {
        #[arg(short, long)]
        company_id: String,
        #[arg(short, long)]
        name: String,
        #[arg(short, long)]
        role: String,
        #[arg(short, long, default_value = "Execute tasks assigned to this role.")]
        prompt: String,
        #[arg(long)]
        reports_to: Option<String>,
        #[arg(short, long, default_value_t = 5000.0)]
        budget: f64,
    },
    #[command(about = "Pause an agent")]
    Pause { id: String },
    #[command(about = "Resume an agent")]
    Resume { id: String },
}

#[derive(Subcommand, Debug)]
enum GoalCommands {
    #[command(about = "List goals")]
    List {
        #[arg(short, long)]
        company_id: Option<String>,
    },
    #[command(about = "Add a company goal")]
    Add {
        #[arg(short, long)]
        company_id: String,
        #[arg(short, long)]
        title: String,
        #[arg(short, long)]
        description: String,
        #[arg(long)]
        metric: Option<String>,
    },
}

#[derive(Subcommand, Debug)]
enum ProjectCommands {
    #[command(about = "List projects")]
    List {
        #[arg(short, long)]
        company_id: Option<String>,
    },
    #[command(about = "Add a project")]
    Add {
        #[arg(short, long)]
        company_id: String,
        #[arg(short, long)]
        name: String,
        #[arg(short, long)]
        description: String,
        #[arg(long)]
        goal_id: Option<String>,
    },
}

#[derive(Subcommand, Debug)]
enum IssueCommands {
    #[command(about = "List issues")]
    List {
        #[arg(short, long)]
        project_id: Option<String>,
        #[arg(short, long)]
        status: Option<String>,
    },
    #[command(about = "Create a new issue")]
    Add {
        #[arg(short, long)]
        project_id: String,
        #[arg(short, long)]
        title: String,
        #[arg(short, long)]
        description: String,
        #[arg(short, long)]
        assignee: Option<String>,
    },
    #[command(about = "Move an issue to a new status (Kanban transition)")]
    Move {
        id: String,
        #[arg(help = "New status: backlog, todo, in_progress, in_review, done, blocked")]
        status: String,
    },
}

#[derive(Args, Debug)]
struct HeartbeatArgs {
    #[arg(short, long, help = "Company ID to execute heartbeat tick for (defaults to first company if omitted)")]
    company_id: Option<String>,
}

#[derive(Subcommand, Debug)]
enum ApprovalCommands {
    #[command(about = "List pending approvals")]
    List {
        #[arg(short, long)]
        company_id: Option<String>,
    },
    #[command(about = "Approve a pending request")]
    Approve {
        id: String,
        #[arg(short, long, default_value = "Admin")]
        reviewer: String,
    },
    #[command(about = "Reject a pending request")]
    Reject {
        id: String,
        #[arg(short, long, default_value = "Admin")]
        reviewer: String,
    },
}

#[derive(Args, Debug)]
struct CostArgs {
    #[arg(short, long, help = "Company ID to display cost report for")]
    company_id: Option<String>,
}

#[derive(Args, Debug)]
struct ActivityArgs {
    #[arg(short, long)]
    company_id: Option<String>,
    #[arg(short, long, default_value_t = 20)]
    limit: usize,
}

fn get_storage(data_path: Option<PathBuf>) -> Arc<Storage> {
    let path = data_path.unwrap_or_else(|| PathBuf::from(".paperclip/data.json"));
    let storage = Storage::new_persistent(&path).unwrap_or_else(|_| Storage::new_in_memory());
    Arc::new(storage)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| "info".into()))
        .with(tracing_subscriber::fmt::layer().compact())
        .init();

    let cli = Cli::parse();
    let storage = get_storage(cli.data_path);
    let provider = Arc::new(MockProvider::default());
    let supervisor = Arc::new(AgentSupervisor::new());
    let dispatcher = Arc::new(HeartbeatDispatcher::new(
        storage.clone(),
        provider,
        supervisor,
        BudgetEnforcer::default(),
    ));

    match cli.command {
        Commands::Onboard(args) => {
            println!("🚀 Initializing virtual company '{}'...", args.name);
            let company = storage.seed_startup_template(&args.name, &args.mission, args.budget)?;
            let agents = storage.list_agents(Some(&company.id));
            let chart = OrgChart::build(&agents)?;

            println!("\n✅ Successfully onboarded company!");
            println!("   Company ID:    {}", company.id);
            println!("   Name:          {}", company.name);
            println!("   Mission:       {}", company.mission);
            println!("   Budget Cap:    ${:.2} {}", company.budget_limit_usd, company.currency);
            println!("   Hired Agents:  {}", agents.len());
            println!("\n📊 Org Chart Hierarchy:\n");
            println!("{}", chart.render_ascii());
        }

        Commands::Serve(args) => {
            let state = AppState::new(storage, dispatcher);
            let addr: SocketAddr = format!("{}:{}", args.host, args.port).parse()?;
            run_server(state, addr).await?;
        }

        Commands::Company { cmd } => match cmd {
            CompanyCommands::List => {
                let companies = storage.list_companies();
                if companies.is_empty() {
                    println!("No companies found. Run 'paperclip onboard' to create one.");
                } else {
                    println!("\n{:<38} {:<24} {:<16} {:<16}", "COMPANY ID", "NAME", "SPEND", "BUDGET CAP");
                    println!("{:-<96}", "");
                    for c in companies {
                        println!(
                            "{:<38} {:<24} ${:<15.2} ${:<15.2}",
                            c.id, c.name, c.budget_spent_usd, c.budget_limit_usd
                        );
                    }
                }
            }
            CompanyCommands::Show { id } => {
                let c = storage.get_company(&id)?;
                println!("\nCompany Details:");
                println!("  ID:          {}", c.id);
                println!("  Name:        {}", c.name);
                println!("  Mission:     {}", c.mission);
                println!("  Spend:       ${:.2} / ${:.2}", c.budget_spent_usd, c.budget_limit_usd);
                println!("  Remaining:   ${:.2}", c.remaining_budget());
            }
            CompanyCommands::Org { id } => {
                let agents = storage.list_agents(Some(&id));
                let chart = OrgChart::build(&agents)?;
                println!("\n📊 Org Chart Hierarchy:\n");
                println!("{}", chart.render_ascii());
            }
        },

        Commands::Agent { cmd } => match cmd {
            AgentCommands::List { company_id } => {
                let agents = storage.list_agents(company_id.as_deref());
                if agents.is_empty() {
                    println!("No agents found.");
                } else {
                    println!("\n{:<38} {:<28} {:<18} {:<10} {:<16}", "AGENT ID", "NAME", "ROLE", "STATUS", "SPEND / BUDGET");
                    println!("{:-<114}", "");
                    for a in agents {
                        println!(
                            "{:<38} {:<28} {:<18} {:<10} ${:.2} / ${:.2}",
                            a.id, a.name, a.role.to_string(), a.status.to_string(), a.budget_spent_usd, a.budget_limit_usd
                        );
                    }
                }
            }
            AgentCommands::Hire {
                company_id,
                name,
                role,
                prompt,
                reports_to,
                budget,
            } => {
                let agent_role = match role.to_lowercase().as_str() {
                    "ceo" => AgentRole::Ceo,
                    "cto" => AgentRole::Cto,
                    "lead_engineer" | "leadengineer" | "engineer" => AgentRole::LeadEngineer,
                    "product_manager" | "productmanager" | "pm" => AgentRole::ProductManager,
                    "qa_engineer" | "qaengineer" | "qa" => AgentRole::QaEngineer,
                    "growth_marketer" | "marketer" => AgentRole::GrowthMarketer,
                    "designer" => AgentRole::Designer,
                    other => AgentRole::Custom(other.to_string()),
                };

                let agent = Agent::new(
                    company_id,
                    name,
                    agent_role,
                    AgentAdapterType::Mock,
                    prompt,
                    reports_to,
                    budget,
                );
                let created = storage.create_agent(agent)?;
                println!("✅ Hired agent '{}' (ID: {})", created.name, created.id);
            }
            AgentCommands::Pause { id } => {
                let updated = storage.update_agent(&id, |a| a.set_status(AgentStatus::Paused))?;
                println!("⏸  Paused agent '{}' ({})", updated.name, updated.id);
            }
            AgentCommands::Resume { id } => {
                let updated = storage.update_agent(&id, |a| a.set_status(AgentStatus::Idle))?;
                println!("▶  Resumed agent '{}' ({})", updated.name, updated.id);
            }
        },

        Commands::Goal { cmd } => match cmd {
            GoalCommands::List { company_id } => {
                let goals = storage.list_goals(company_id.as_deref());
                if goals.is_empty() {
                    println!("No goals found.");
                } else {
                    println!("\n{:<38} {:<36} {:<12} {:<10}", "GOAL ID", "TITLE", "STATUS", "PRIORITY");
                    println!("{:-<100}", "");
                    for g in goals {
                        println!("{:<38} {:<36} {:<12} {:<10}", g.id, g.title, g.status.to_string(), g.priority.to_string());
                    }
                }
            }
            GoalCommands::Add { company_id, title, description, metric } => {
                let mut goal = Goal::new(company_id, title, description, GoalPriority::High);
                if let Some(m) = metric {
                    goal = goal.with_target_metric(m);
                }
                let created = storage.create_goal(goal)?;
                println!("✅ Created goal '{}' (ID: {})", created.title, created.id);
            }
        },

        Commands::Project { cmd } => match cmd {
            ProjectCommands::List { company_id } => {
                let projects = storage.list_projects(company_id.as_deref());
                if projects.is_empty() {
                    println!("No projects found.");
                } else {
                    println!("\n{:<38} {:<32} {:<12}", "PROJECT ID", "NAME", "STATUS");
                    println!("{:-<84}", "");
                    for p in projects {
                        println!("{:<38} {:<32} {:<12}", p.id, p.name, p.status.to_string());
                    }
                }
            }
            ProjectCommands::Add { company_id, name, description, goal_id } => {
                let mut project = Project::new(company_id, name, description);
                if let Some(gid) = goal_id {
                    project = project.with_goal(gid);
                }
                let created = storage.create_project(project)?;
                println!("✅ Created project '{}' (ID: {})", created.name, created.id);
            }
        },

        Commands::Issue { cmd } => match cmd {
            IssueCommands::List { project_id, status } => {
                let status_filter = status.as_deref().and_then(|s| match s.to_lowercase().as_str() {
                    "backlog" => Some(IssueStatus::Backlog),
                    "todo" => Some(IssueStatus::Todo),
                    "in_progress" | "inprogress" => Some(IssueStatus::InProgress),
                    "in_review" | "inreview" => Some(IssueStatus::InReview),
                    "done" => Some(IssueStatus::Done),
                    "blocked" => Some(IssueStatus::Blocked),
                    _ => None,
                });

                let issues = storage.list_issues(project_id.as_deref(), status_filter);
                if issues.is_empty() {
                    println!("No issues found.");
                } else {
                    println!("\n{:<38} {:<40} {:<14} {:<10}", "ISSUE ID", "TITLE", "STATUS", "PRIORITY");
                    println!("{:-<106}", "");
                    for i in issues {
                        println!("{:<38} {:<40} {:<14} {:<10}", i.id, i.title, i.status.to_string(), i.priority.to_string());
                    }
                }
            }
            IssueCommands::Add { project_id, title, description, assignee } => {
                let mut issue = Issue::new(project_id, title, description, IssuePriority::Medium);
                if let Some(a) = assignee {
                    issue = issue.with_assignee(a);
                }
                let created = storage.create_issue(issue)?;
                println!("✅ Created issue '{}' (ID: {})", created.title, created.id);
            }
            IssueCommands::Move { id, status } => {
                let new_status = match status.to_lowercase().as_str() {
                    "backlog" => IssueStatus::Backlog,
                    "todo" => IssueStatus::Todo,
                    "in_progress" | "inprogress" => IssueStatus::InProgress,
                    "in_review" | "inreview" => IssueStatus::InReview,
                    "done" => IssueStatus::Done,
                    "blocked" => IssueStatus::Blocked,
                    other => {
                        eprintln!("Invalid status '{}'. Use: backlog, todo, in_progress, in_review, done, blocked", other);
                        return Ok(());
                    }
                };
                let updated = storage.update_issue(&id, |i| i.set_status(new_status))?;
                println!("✅ Transitioned issue '{}' to {}", updated.title, updated.status);
            }
        },

        Commands::Heartbeat(args) => {
            let company_id = match args.company_id {
                Some(id) => id,
                None => {
                    let companies = storage.list_companies();
                    if companies.is_empty() {
                        eprintln!("No companies found. Run 'paperclip onboard' first.");
                        return Ok(());
                    }
                    companies[0].id.clone()
                }
            };

            println!("⚡ Executing orchestration heartbeat tick for company {}...", company_id);
            let ticks = dispatcher.tick(&company_id).await?;
            if ticks.is_empty() {
                println!("   No eligible tasks found for active agents.");
            } else {
                for t in &ticks {
                    if t.success {
                        println!("   ✔ [{}] Task '{}': {} (${:.4})", t.agent_name, t.issue_title, t.summary, t.cost_usd);
                    } else {
                        println!("   ✖ [{}] Task '{}': {}", t.agent_name, t.issue_title, t.summary);
                    }
                }
                let total_cost: f64 = ticks.iter().map(|t| t.cost_usd).sum();
                println!("\n✨ Heartbeat finished! {} tasks executed (Total tick cost: ${:.4})", ticks.len(), total_cost);
            }
        }

        Commands::Approval { cmd } => match cmd {
            ApprovalCommands::List { company_id } => {
                let approvals = storage.list_approvals(company_id.as_deref(), Some(ApprovalStatus::Pending));
                if approvals.is_empty() {
                    println!("No pending approvals.");
                } else {
                    println!("\n{:<38} {:<24} {:<40}", "APPROVAL ID", "ACTION", "DESCRIPTION");
                    println!("{:-<104}", "");
                    for a in approvals {
                        println!("{:<38} {:<24} {:<40}", a.id, a.action.to_string(), a.description);
                    }
                }
            }
            ApprovalCommands::Approve { id, reviewer } => {
                let updated = storage.review_approval(&id, true, &reviewer)?;
                println!("✅ Approved request {} (Reviewer: {})", updated.id, reviewer);
            }
            ApprovalCommands::Reject { id, reviewer } => {
                let updated = storage.review_approval(&id, false, &reviewer)?;
                println!("❌ Rejected request {} (Reviewer: {})", updated.id, reviewer);
            }
        },

        Commands::Cost(args) => {
            let company_id = match args.company_id {
                Some(id) => id,
                None => {
                    let companies = storage.list_companies();
                    if companies.is_empty() {
                        eprintln!("No companies found. Run 'paperclip onboard' first.");
                        return Ok(());
                    }
                    companies[0].id.clone()
                }
            };

            let summary = storage.get_cost_summary(&company_id);
            println!("\n💰 Cost & Token Spend Summary:");
            println!("   Company ID:           {}", summary.company_id);
            println!("   Total Spend:          ${:.4}", summary.total_cost_usd);
            println!("   Total Tokens:         {} (Input: {}, Output: {})", summary.total_tokens, summary.total_input_tokens, summary.total_output_tokens);
            println!("\n{:<28} {:<18} {:<12} {:<14} {:<14}", "AGENT", "ROLE", "SPEND", "BUDGET CAP", "% CONSUMED");
            println!("{:-<90}", "");
            for a in summary.agent_breakdown {
                println!(
                    "{:<28} {:<18} ${:<11.4} ${:<13.2} {:.1}%",
                    a.agent_name, a.role, a.cost_usd, a.budget_limit_usd, a.budget_percent
                );
            }
        }

        Commands::Activity(args) => {
            let logs = storage.list_activity(args.company_id.as_deref(), args.limit);
            if logs.is_empty() {
                println!("No activity logs recorded yet.");
            } else {
                println!("\n📜 Recent Activity Feed (Latest {} events):", logs.len());
                println!("{:-<100}", "");
                for l in logs {
                    println!("[{}] <{}> {}: {}", l.timestamp.format("%Y-%m-%d %H:%M:%S"), l.actor_type, l.action, l.details);
                }
            }
        }
    }

    Ok(())
}
