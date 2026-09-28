use axum::response::Html;

pub async fn dashboard_html() -> Html<&'static str> {
    Html(r#"<!DOCTYPE html>
<html lang="en">
<head>
  <meta charset="UTF-8">
  <meta name="viewport" content="width=device-width, initial-scale=1.0">
  <title>Paperclip Control Plane</title>
  <link rel="preconnect" href="https://fonts.googleapis.com">
  <link rel="preconnect" href="https://fonts.gstatic.com" crossorigin>
  <link href="https://fonts.googleapis.com/css2?family=Inter:wght@400;500;600;700&family=JetBrains+Mono:wght@400;500;600&display=swap" rel="stylesheet">
  <script src="https://cdn.tailwindcss.com"></script>
  <script>
    tailwind.config = {
      darkMode: 'class',
      theme: {
        extend: {
          fontFamily: {
            sans: ['Inter', 'sans-serif'],
            mono: ['JetBrains Mono', 'monospace'],
          },
          colors: {
            brand: {
              50: '#fff7ed',
              500: '#f97316',
              600: '#ea580c',
              700: '#c2410c',
            },
            dark: {
              900: '#090d16',
              800: '#0f172a',
              700: '#1e293b',
              600: '#334155',
            }
          }
        }
      }
    }
  </script>
  <style>
    body { background-color: #090d16; color: #f8fafc; font-family: 'Inter', sans-serif; }
    .card { background: rgba(15, 23, 42, 0.75); border: 1px solid rgba(51, 65, 85, 0.6); backdrop-filter: blur(12px); }
    .card-hover:hover { border-color: rgba(249, 115, 22, 0.5); }
    .tab-active { background: rgba(249, 115, 22, 0.15); color: #f97316; border-left: 3px solid #f97316; }
    .tree-branch { border-left: 2px dashed #334155; }
    ::-webkit-scrollbar { width: 6px; height: 6px; }
    ::-webkit-scrollbar-track { background: #090d16; }
    ::-webkit-scrollbar-thumb { background: #334155; border-radius: 3px; }
    ::-webkit-scrollbar-thumb:hover { background: #475569; }
  </style>
</head>
<body class="min-h-screen flex flex-col antialiased selection:bg-orange-500 selection:text-white">
  <!-- Top Navigation -->
  <header class="border-b border-slate-800 bg-slate-950/80 sticky top-0 z-50 backdrop-blur-md px-6 py-3.5 flex items-center justify-between">
    <div class="flex items-center gap-3">
      <div class="w-9 h-9 rounded-xl bg-gradient-to-br from-orange-500 to-amber-600 flex items-center justify-center font-bold text-lg shadow-lg shadow-orange-500/20">
        📎
      </div>
      <div>
        <div class="flex items-center gap-2">
          <span class="font-bold text-lg tracking-tight">Paperclip</span>
          <span class="text-xs uppercase tracking-wider font-mono px-2 py-0.5 rounded bg-orange-500/10 text-orange-400 border border-orange-500/20 font-semibold">Rust Core</span>
        </div>
        <p class="text-xs text-slate-400" id="header-company-name">Autonomous AI Agent Organization</p>
      </div>
    </div>

    <div class="flex items-center gap-3">
      <div class="flex items-center gap-2 bg-slate-900 border border-slate-800 px-3 py-1.5 rounded-lg text-xs font-mono text-slate-300">
        <span class="w-2 h-2 rounded-full bg-emerald-500 animate-pulse"></span>
        <span id="header-spend">$0.00 / $0.00</span>
      </div>
      <button onclick="triggerHeartbeat()" id="btn-heartbeat" class="flex items-center gap-2 px-4 py-1.5 rounded-lg bg-gradient-to-r from-orange-500 to-amber-500 hover:from-orange-600 hover:to-amber-600 text-white font-medium text-sm shadow-md shadow-orange-500/20 transition-all active:scale-95">
        <span>⚡</span> Run Heartbeat
      </button>
    </div>
  </header>

  <!-- Main Layout -->
  <div class="flex-1 flex overflow-hidden">
    <!-- Sidebar -->
    <aside class="w-64 border-r border-slate-800 bg-slate-950/40 p-4 flex flex-col justify-between shrink-0">
      <nav class="space-y-1">
        <button onclick="switchTab('overview')" id="tab-overview" class="w-full flex items-center gap-3 px-3 py-2.5 rounded-lg text-sm font-medium transition-colors text-slate-300 hover:bg-slate-900 tab-active">
          <span>📊</span> Overview
        </button>
        <button onclick="switchTab('kanban')" id="tab-kanban" class="w-full flex items-center gap-3 px-3 py-2.5 rounded-lg text-sm font-medium transition-colors text-slate-300 hover:bg-slate-900">
          <span>📋</span> Kanban Board
        </button>
        <button onclick="switchTab('org')" id="tab-org" class="w-full flex items-center gap-3 px-3 py-2.5 rounded-lg text-sm font-medium transition-colors text-slate-300 hover:bg-slate-900">
          <span>🏢</span> Org Chart
        </button>
        <button onclick="switchTab('agents')" id="tab-agents" class="w-full flex items-center gap-3 px-3 py-2.5 rounded-lg text-sm font-medium transition-colors text-slate-300 hover:bg-slate-900">
          <span>🤖</span> Agents Roster
        </button>
        <button onclick="switchTab('costs')" id="tab-costs" class="w-full flex items-center gap-3 px-3 py-2.5 rounded-lg text-sm font-medium transition-colors text-slate-300 hover:bg-slate-900">
          <span>💰</span> Budgets & Tokens
        </button>
        <button onclick="switchTab('approvals')" id="tab-approvals" class="w-full flex items-center gap-3 px-3 py-2.5 rounded-lg text-sm font-medium transition-colors text-slate-300 hover:bg-slate-900">
          <span>🛡️</span> Approvals
          <span id="badge-approvals" class="ml-auto text-xs px-1.5 py-0.5 rounded bg-orange-500/20 text-orange-400 hidden">0</span>
        </button>
        <button onclick="switchTab('activity')" id="tab-activity" class="w-full flex items-center gap-3 px-3 py-2.5 rounded-lg text-sm font-medium transition-colors text-slate-300 hover:bg-slate-900">
          <span>📜</span> Activity Feed
        </button>
      </nav>

      <div class="p-3 rounded-lg bg-slate-900/60 border border-slate-800 text-xs text-slate-400 space-y-1 font-mono">
        <div class="flex justify-between"><span>Core engine:</span> <span class="text-orange-400">Rust 1.98</span></div>
        <div class="flex justify-between"><span>Web framework:</span> <span class="text-slate-300">Axum 0.8</span></div>
        <div class="flex justify-between"><span>Status:</span> <span class="text-emerald-400">Ready</span></div>
      </div>
    </aside>

    <!-- Main Content Area -->
    <main class="flex-1 overflow-y-auto p-6 bg-slate-900/20">
      <!-- Live Heartbeat Console Toast / Panel -->
      <div id="heartbeat-banner" class="hidden mb-6 p-4 rounded-xl card border-orange-500/40 bg-orange-950/20 space-y-2 transition-all">
        <div class="flex items-center justify-between">
          <div class="flex items-center gap-2 text-sm font-semibold text-orange-400">
            <span class="animate-spin text-base">⚙️</span> Heartbeat Execution In Progress...
          </div>
          <span id="heartbeat-stats" class="text-xs font-mono text-slate-400"></span>
        </div>
        <div id="heartbeat-logs" class="font-mono text-xs bg-slate-950/80 p-3 rounded-lg text-slate-300 max-h-40 overflow-y-auto space-y-1"></div>
      </div>

      <!-- VIEW: OVERVIEW -->
      <section id="view-overview" class="space-y-6">
        <div class="flex items-start justify-between">
          <div>
            <h1 class="text-2xl font-bold tracking-tight text-white" id="ov-company-name">Company Overview</h1>
            <p class="text-sm text-slate-400 mt-1" id="ov-company-mission">Loading company mission...</p>
          </div>
          <div class="flex gap-2">
            <button onclick="openNewIssueModal()" class="px-3.5 py-1.5 rounded-lg bg-slate-800 hover:bg-slate-700 text-sm font-medium border border-slate-700 text-slate-200">
              + New Issue
            </button>
          </div>
        </div>

        <!-- Metrics Grid -->
        <div class="grid grid-cols-1 md:grid-cols-4 gap-4">
          <div class="card p-5 rounded-xl space-y-2">
            <p class="text-xs font-medium uppercase tracking-wider text-slate-400">Total Spend / Budget</p>
            <h3 class="text-2xl font-bold font-mono text-white" id="stat-spend">$0.00</h3>
            <div class="w-full bg-slate-800 rounded-full h-1.5 overflow-hidden">
              <div id="stat-spend-bar" class="bg-orange-500 h-1.5 rounded-full" style="width: 0%"></div>
            </div>
            <p class="text-xs text-slate-500" id="stat-spend-sub">Cap: $0.00</p>
          </div>

          <div class="card p-5 rounded-xl space-y-2">
            <p class="text-xs font-medium uppercase tracking-wider text-slate-400">Active Agents</p>
            <h3 class="text-2xl font-bold font-mono text-white" id="stat-agents">0</h3>
            <p class="text-xs text-emerald-400 flex items-center gap-1">
              <span class="w-2 h-2 rounded-full bg-emerald-500"></span> Fully Supervised
            </p>
          </div>

          <div class="card p-5 rounded-xl space-y-2">
            <p class="text-xs font-medium uppercase tracking-wider text-slate-400">Active Tasks</p>
            <h3 class="text-2xl font-bold font-mono text-white" id="stat-issues">0</h3>
            <p class="text-xs text-slate-400" id="stat-issues-sub">0 completed</p>
          </div>

          <div class="card p-5 rounded-xl space-y-2">
            <p class="text-xs font-medium uppercase tracking-wider text-slate-400">Tokens Processed</p>
            <h3 class="text-2xl font-bold font-mono text-white" id="stat-tokens">0</h3>
            <p class="text-xs text-slate-400">Zero human intervention</p>
          </div>
        </div>

        <!-- Primary Goal & Project Card -->
        <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
          <div class="card p-5 rounded-xl space-y-3">
            <div class="flex items-center justify-between">
              <span class="text-xs font-semibold uppercase tracking-wider text-orange-400">🎯 Primary Goal</span>
              <span class="text-xs font-mono px-2 py-0.5 rounded bg-emerald-500/10 text-emerald-400 border border-emerald-500/20" id="ov-goal-status">Active</span>
            </div>
            <h4 class="text-lg font-semibold text-white" id="ov-goal-title">Goal title</h4>
            <p class="text-xs text-slate-400" id="ov-goal-desc">Goal description</p>
            <div class="p-2.5 rounded bg-slate-900/80 border border-slate-800 text-xs font-mono text-slate-300" id="ov-goal-metric">
              Target Metric: --
            </div>
          </div>

          <div class="card p-5 rounded-xl space-y-3">
            <div class="flex items-center justify-between">
              <span class="text-xs font-semibold uppercase tracking-wider text-amber-400">📁 Active Project</span>
              <span class="text-xs font-mono px-2 py-0.5 rounded bg-blue-500/10 text-blue-400 border border-blue-500/20" id="ov-project-status">Active</span>
            </div>
            <h4 class="text-lg font-semibold text-white" id="ov-project-name">Project name</h4>
            <p class="text-xs text-slate-400" id="ov-project-desc">Project description</p>
            <div class="text-xs text-slate-400 flex items-center gap-2">
              <span>Lead:</span>
              <span class="font-mono text-slate-200" id="ov-project-lead">Atlas (CTO)</span>
            </div>
          </div>
        </div>

        <!-- Recent Activity Mini Feed -->
        <div class="card p-5 rounded-xl space-y-3">
          <div class="flex items-center justify-between">
            <h4 class="text-sm font-semibold uppercase tracking-wider text-slate-300">⚡ Live Audit Trail</h4>
            <button onclick="switchTab('activity')" class="text-xs text-orange-400 hover:underline">View All →</button>
          </div>
          <div id="ov-activity-list" class="space-y-2 text-xs font-mono"></div>
        </div>
      </section>

      <!-- VIEW: KANBAN BOARD -->
      <section id="view-kanban" class="hidden space-y-4">
        <div class="flex items-center justify-between">
          <div>
            <h2 class="text-xl font-bold text-white">Kanban Task Board</h2>
            <p class="text-xs text-slate-400">Agent assigned issues, execution status, and turn cycles</p>
          </div>
          <button onclick="openNewIssueModal()" class="px-3.5 py-1.5 rounded-lg bg-orange-500 hover:bg-orange-600 text-sm font-medium text-white shadow-sm shadow-orange-500/20">
            + New Issue
          </button>
        </div>

        <div class="grid grid-cols-1 md:grid-cols-5 gap-3 overflow-x-auto pb-4">
          <!-- Col: Todo -->
          <div class="space-y-2">
            <div class="flex items-center justify-between text-xs font-semibold text-slate-400 px-2 py-1 bg-slate-900/80 rounded border border-slate-800">
              <span>TODO</span>
              <span id="count-todo" class="font-mono px-1.5 py-0.2 rounded bg-slate-800">0</span>
            </div>
            <div id="col-todo" class="space-y-2 min-h-[400px]"></div>
          </div>

          <!-- Col: In Progress -->
          <div class="space-y-2">
            <div class="flex items-center justify-between text-xs font-semibold text-amber-400 px-2 py-1 bg-amber-950/30 rounded border border-amber-800/40">
              <span>IN PROGRESS</span>
              <span id="count-in_progress" class="font-mono px-1.5 py-0.2 rounded bg-amber-900/50">0</span>
            </div>
            <div id="col-in_progress" class="space-y-2 min-h-[400px]"></div>
          </div>

          <!-- Col: In Review -->
          <div class="space-y-2">
            <div class="flex items-center justify-between text-xs font-semibold text-blue-400 px-2 py-1 bg-blue-950/30 rounded border border-blue-800/40">
              <span>IN REVIEW</span>
              <span id="count-in_review" class="font-mono px-1.5 py-0.2 rounded bg-blue-900/50">0</span>
            </div>
            <div id="col-in_review" class="space-y-2 min-h-[400px]"></div>
          </div>

          <!-- Col: Done -->
          <div class="space-y-2">
            <div class="flex items-center justify-between text-xs font-semibold text-emerald-400 px-2 py-1 bg-emerald-950/30 rounded border border-emerald-800/40">
              <span>DONE</span>
              <span id="count-done" class="font-mono px-1.5 py-0.2 rounded bg-emerald-900/50">0</span>
            </div>
            <div id="col-done" class="space-y-2 min-h-[400px]"></div>
          </div>

          <!-- Col: Blocked -->
          <div class="space-y-2">
            <div class="flex items-center justify-between text-xs font-semibold text-rose-400 px-2 py-1 bg-rose-950/30 rounded border border-rose-800/40">
              <span>BLOCKED</span>
              <span id="count-blocked" class="font-mono px-1.5 py-0.2 rounded bg-rose-900/50">0</span>
            </div>
            <div id="col-blocked" class="space-y-2 min-h-[400px]"></div>
          </div>
        </div>
      </section>

      <!-- VIEW: ORG CHART -->
      <section id="view-org" class="hidden space-y-4">
        <div>
          <h2 class="text-xl font-bold text-white">Virtual Company Org Chart</h2>
          <p class="text-xs text-slate-400">Hierarchical reporting structure, responsibility chains, and individual agent caps</p>
        </div>

        <div class="card p-6 rounded-xl space-y-4">
          <div class="flex items-center justify-between border-b border-slate-800 pb-3">
            <span class="text-xs font-semibold uppercase tracking-wider text-slate-400">Interactive Tree View</span>
            <span class="text-xs font-mono text-slate-500">Acyclic Verified</span>
          </div>
          <div id="org-tree-container" class="space-y-4"></div>
        </div>

        <div class="card p-5 rounded-xl space-y-2">
          <h3 class="text-xs font-semibold uppercase tracking-wider text-slate-400">Raw ASCII Hierarchy</h3>
          <pre id="org-ascii" class="p-4 rounded-lg bg-slate-950 font-mono text-xs text-orange-300 overflow-x-auto"></pre>
        </div>
      </section>

      <!-- VIEW: AGENTS -->
      <section id="view-agents" class="hidden space-y-4">
        <div class="flex items-center justify-between">
          <div>
            <h2 class="text-xl font-bold text-white">Agents Roster</h2>
            <p class="text-xs text-slate-400">Supervised AI employees, assigned roles, and runtime adapters</p>
          </div>
        </div>
        <div id="agents-grid" class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4"></div>
      </section>

      <!-- VIEW: BUDGETS & COSTS -->
      <section id="view-costs" class="hidden space-y-4">
        <div>
          <h2 class="text-xl font-bold text-white">Budgets, Tokens & Cost Accounting</h2>
          <p class="text-xs text-slate-400">Granular token consumption and budget policy governance</p>
        </div>

        <div class="card p-5 rounded-xl space-y-4">
          <h3 class="text-sm font-semibold uppercase tracking-wider text-slate-300">Agent Spend Breakdown</h3>
          <div class="overflow-x-auto">
            <table class="w-full text-left text-xs font-mono">
              <thead>
                <tr class="border-b border-slate-800 text-slate-400">
                  <th class="py-2.5">Agent</th>
                  <th class="py-2.5">Role</th>
                  <th class="py-2.5">Total Tokens</th>
                  <th class="py-2.5">Current Spend</th>
                  <th class="py-2.5">Budget Cap</th>
                  <th class="py-2.5">% Consumed</th>
                </tr>
              </thead>
              <tbody id="cost-table-body" class="divide-y divide-slate-800/60 text-slate-300"></tbody>
            </table>
          </div>
        </div>
      </section>

      <!-- VIEW: APPROVALS -->
      <section id="view-approvals" class="hidden space-y-4">
        <div>
          <h2 class="text-xl font-bold text-white">Human-in-the-Loop Approvals</h2>
          <p class="text-xs text-slate-400">Governance authorization for budget expansions, dangerous tools, or releases</p>
        </div>
        <div id="approvals-list" class="space-y-3"></div>
      </section>

      <!-- VIEW: ACTIVITY FEED -->
      <section id="view-activity" class="hidden space-y-4">
        <div>
          <h2 class="text-xl font-bold text-white">Audit Log & Event Stream</h2>
          <p class="text-xs text-slate-400">Immutable ledger of agent decisions, tool invocations, and heartbeat runs</p>
        </div>
        <div class="card p-5 rounded-xl">
          <div id="activity-full-list" class="space-y-3 font-mono text-xs"></div>
        </div>
      </section>
    </main>
  </div>

  <!-- Modal: Create Issue -->
  <div id="modal-issue" class="fixed inset-0 bg-black/70 backdrop-blur-sm z-50 flex items-center justify-center hidden">
    <div class="card w-full max-w-lg p-6 rounded-2xl border-slate-700 bg-slate-900 space-y-4">
      <div class="flex items-center justify-between border-b border-slate-800 pb-3">
        <h3 class="text-lg font-bold text-white">Create New Task Issue</h3>
        <button onclick="closeNewIssueModal()" class="text-slate-400 hover:text-white">✕</button>
      </div>
      <div class="space-y-3 text-sm">
        <div>
          <label class="block text-xs font-semibold text-slate-400 uppercase mb-1">Title</label>
          <input id="new-issue-title" type="text" placeholder="e.g. Implement OAuth Flow" class="w-full bg-slate-950 border border-slate-800 rounded-lg p-2.5 text-white focus:outline-none focus:border-orange-500">
        </div>
        <div>
          <label class="block text-xs font-semibold text-slate-400 uppercase mb-1">Description</label>
          <textarea id="new-issue-desc" rows="3" placeholder="Context and acceptance criteria for agent..." class="w-full bg-slate-950 border border-slate-800 rounded-lg p-2.5 text-white focus:outline-none focus:border-orange-500"></textarea>
        </div>
        <div class="grid grid-cols-2 gap-3">
          <div>
            <label class="block text-xs font-semibold text-slate-400 uppercase mb-1">Assignee</label>
            <select id="new-issue-assignee" class="w-full bg-slate-950 border border-slate-800 rounded-lg p-2.5 text-white focus:outline-none focus:border-orange-500"></select>
          </div>
          <div>
            <label class="block text-xs font-semibold text-slate-400 uppercase mb-1">Priority</label>
            <select id="new-issue-priority" class="w-full bg-slate-950 border border-slate-800 rounded-lg p-2.5 text-white focus:outline-none focus:border-orange-500">
              <option value="medium">Medium</option>
              <option value="high">High</option>
              <option value="urgent">Urgent</option>
              <option value="low">Low</option>
            </select>
          </div>
        </div>
      </div>
      <div class="flex justify-end gap-2 pt-2 border-t border-slate-800">
        <button onclick="closeNewIssueModal()" class="px-4 py-2 rounded-lg bg-slate-800 hover:bg-slate-700 text-sm font-medium text-slate-300">Cancel</button>
        <button onclick="submitNewIssue()" class="px-4 py-2 rounded-lg bg-orange-500 hover:bg-orange-600 text-sm font-medium text-white">Create Issue</button>
      </div>
    </div>
  </div>

  <script>
    let currentCompany = null;
    let cachedAgents = [];
    let cachedProjects = [];

    async function loadData() {
      try {
        const companies = await fetch('/api/v1/companies').then(r => r.json());
        if (!companies || companies.length === 0) {
          // If no company exists yet, seed one
          const seeded = await fetch('/api/v1/companies/seed', {
            method: 'POST',
            headers: {'Content-Type': 'application/json'},
            body: JSON.stringify({
              name: 'Apollo AI',
              mission: 'Build the leading autonomous software suite',
              budget_limit_usd: 100000.0
            })
          }).then(r => r.json());
          currentCompany = seeded;
        } else {
          currentCompany = companies[0];
        }

        document.getElementById('header-company-name').textContent = currentCompany.name;
        document.getElementById('ov-company-name').textContent = currentCompany.name;
        document.getElementById('ov-company-mission').textContent = currentCompany.mission;

        await Promise.all([
          loadAgents(),
          loadProjectsAndGoals(),
          loadIssues(),
          loadCosts(),
          loadApprovals(),
          loadActivity(),
          loadOrgChart(),
        ]);
      } catch (err) {
        console.error('Failed to load data:', err);
      }
    }

    async function loadAgents() {
      const agents = await fetch(`/api/v1/agents?company_id=${currentCompany.id}`).then(r => r.json());
      cachedAgents = agents;
      document.getElementById('stat-agents').textContent = agents.length;

      const grid = document.getElementById('agents-grid');
      const select = document.getElementById('new-issue-assignee');
      grid.innerHTML = '';
      select.innerHTML = '<option value="">Unassigned</option>';

      agents.forEach(a => {
        const opt = document.createElement('option');
        opt.value = a.id;
        opt.textContent = `${a.name} (${a.role})`;
        select.appendChild(opt);

        const card = document.createElement('div');
        card.className = 'card p-5 rounded-xl space-y-3';
        card.innerHTML = `
          <div class="flex items-start justify-between">
            <div>
              <h4 class="font-bold text-white text-base">${a.name}</h4>
              <span class="text-xs font-mono px-2 py-0.5 rounded bg-orange-500/10 text-orange-400 border border-orange-500/20">${a.role}</span>
            </div>
            <span class="text-xs font-mono px-2 py-0.5 rounded ${a.status === 'Idle' || a.status === 'Running' ? 'bg-emerald-500/10 text-emerald-400' : 'bg-rose-500/10 text-rose-400'}">${a.status}</span>
          </div>
          <p class="text-xs text-slate-400 line-clamp-2">${a.system_prompt}</p>
          <div class="pt-2 border-t border-slate-800 text-xs font-mono flex justify-between text-slate-400">
            <span>Spend: $${a.budget_spent_usd.toFixed(2)}</span>
            <span>Cap: $${a.budget_limit_usd.toFixed(2)}</span>
          </div>
          <div class="flex gap-2 pt-1">
            ${a.status === 'Paused' 
              ? `<button onclick="toggleAgent('${a.id}', 'resume')" class="w-full py-1 text-xs rounded bg-slate-800 hover:bg-slate-700 text-slate-200">Resume</button>`
              : `<button onclick="toggleAgent('${a.id}', 'pause')" class="w-full py-1 text-xs rounded bg-slate-800 hover:bg-slate-700 text-slate-400">Pause</button>`
            }
          </div>
        `;
        grid.appendChild(card);
      });
    }

    async function loadProjectsAndGoals() {
      const goals = await fetch(`/api/v1/goals?company_id=${currentCompany.id}`).then(r => r.json());
      const projects = await fetch(`/api/v1/projects?company_id=${currentCompany.id}`).then(r => r.json());
      cachedProjects = projects;

      if (goals.length > 0) {
        const g = goals[0];
        document.getElementById('ov-goal-title').textContent = g.title;
        document.getElementById('ov-goal-desc').textContent = g.description;
        document.getElementById('ov-goal-status').textContent = g.status;
        document.getElementById('ov-goal-metric').textContent = `Target: ${g.target_metric || 'N/A'}`;
      }

      if (projects.length > 0) {
        const p = projects[0];
        document.getElementById('ov-project-name').textContent = p.name;
        document.getElementById('ov-project-desc').textContent = p.description;
        document.getElementById('ov-project-status').textContent = p.status;
      }
    }

    async function loadIssues() {
      const issues = await fetch('/api/v1/issues').then(r => r.json());
      document.getElementById('stat-issues').textContent = issues.length;
      const completed = issues.filter(i => i.status === 'Done').length;
      document.getElementById('stat-issues-sub').textContent = `${completed} completed`;

      const cols = {
        'Todo': document.getElementById('col-todo'),
        'InProgress': document.getElementById('col-in_progress'),
        'InReview': document.getElementById('col-in_review'),
        'Done': document.getElementById('col-done'),
        'Blocked': document.getElementById('col-blocked'),
      };

      const counts = {
        'Todo': document.getElementById('count-todo'),
        'InProgress': document.getElementById('count-in_progress'),
        'InReview': document.getElementById('count-in_review'),
        'Done': document.getElementById('count-done'),
        'Blocked': document.getElementById('count-blocked'),
      };

      Object.values(cols).forEach(c => c.innerHTML = '');
      const countNums = { 'Todo': 0, 'InProgress': 0, 'InReview': 0, 'Done': 0, 'Blocked': 0 };

      issues.forEach(issue => {
        const col = cols[issue.status] || cols['Todo'];
        countNums[issue.status] = (countNums[issue.status] || 0) + 1;

        const assignee = cachedAgents.find(a => a.id === issue.assignee_agent_id);
        const card = document.createElement('div');
        card.className = 'card p-3 rounded-lg space-y-2 card-hover transition-all';
        card.innerHTML = `
          <div class="flex items-start justify-between gap-2">
            <h5 class="text-xs font-semibold text-white leading-snug">${issue.title}</h5>
            <span class="text-[10px] font-mono px-1.5 py-0.5 rounded uppercase ${
              issue.priority === 'Urgent' ? 'bg-rose-500/20 text-rose-400' :
              issue.priority === 'High' ? 'bg-orange-500/20 text-orange-400' : 'bg-slate-800 text-slate-400'
            }">${issue.priority}</span>
          </div>
          <p class="text-[11px] text-slate-400 line-clamp-2">${issue.description}</p>
          <div class="flex items-center justify-between pt-1 border-t border-slate-800 text-[10px] font-mono text-slate-400">
            <span>👤 ${assignee ? assignee.name.split(' ')[0] : 'Unassigned'}</span>
            <div class="flex gap-1">
              ${issue.status !== 'Done' ? `<button onclick="moveIssue('${issue.id}', 'Done')" class="px-1.5 py-0.5 rounded bg-emerald-950 text-emerald-400 border border-emerald-800 hover:bg-emerald-900">Done</button>` : ''}
              ${issue.status === 'Todo' ? `<button onclick="moveIssue('${issue.id}', 'InProgress')" class="px-1.5 py-0.5 rounded bg-amber-950 text-amber-400 border border-amber-800 hover:bg-amber-900">Start</button>` : ''}
            </div>
          </div>
        `;
        col.appendChild(card);
      });

      for (let k in countNums) {
        if (counts[k]) counts[k].textContent = countNums[k];
      }
    }

    async function loadCosts() {
      const summary = await fetch(`/api/v1/costs/summary/${currentCompany.id}`).then(r => r.json());
      const spent = summary.total_cost_usd || 0;
      const cap = currentCompany.budget_limit_usd || 100000;
      const pct = (spent / cap) * 100;

      document.getElementById('header-spend').textContent = `$${spent.toFixed(2)} / $${cap.toFixed(0)}`;
      document.getElementById('stat-spend').textContent = `$${spent.toFixed(4)}`;
      document.getElementById('stat-spend-sub').textContent = `Budget Cap: $${cap.toFixed(2)} (${pct.toFixed(2)}% used)`;
      document.getElementById('stat-spend-bar').style.width = `${Math.min(pct, 100)}%`;
      document.getElementById('stat-tokens').textContent = (summary.total_tokens || 0).toLocaleString();

      const tbody = document.getElementById('cost-table-body');
      tbody.innerHTML = '';
      (summary.agent_breakdown || []).forEach(a => {
        const tr = document.createElement('tr');
        tr.innerHTML = `
          <td class="py-2.5 font-medium text-white">${a.agent_name}</td>
          <td class="py-2.5 text-slate-400">${a.role}</td>
          <td class="py-2.5 font-mono">${(a.total_tokens || 0).toLocaleString()}</td>
          <td class="py-2.5 font-mono text-orange-400">$${a.cost_usd.toFixed(4)}</td>
          <td class="py-2.5 font-mono text-slate-400">$${a.budget_limit_usd.toFixed(2)}</td>
          <td class="py-2.5 font-mono">${a.budget_percent.toFixed(2)}%</td>
        `;
        tbody.appendChild(tr);
      });
    }

    async function loadApprovals() {
      const approvals = await fetch(`/api/v1/approvals?company_id=${currentCompany.id}&status=pending`).then(r => r.json());
      const badge = document.getElementById('badge-approvals');
      if (approvals.length > 0) {
        badge.textContent = approvals.length;
        badge.classList.remove('hidden');
      } else {
        badge.classList.add('hidden');
      }

      const list = document.getElementById('approvals-list');
      list.innerHTML = '';
      if (approvals.length === 0) {
        list.innerHTML = '<div class="card p-8 rounded-xl text-center text-slate-400 text-sm">✅ No pending approval requests. Everything is operating smoothly.</div>';
      } else {
        approvals.forEach(a => {
          const card = document.createElement('div');
          card.className = 'card p-4 rounded-xl flex items-center justify-between border-amber-500/40 bg-amber-950/10';
          card.innerHTML = `
            <div class="space-y-1">
              <div class="flex items-center gap-2">
                <span class="text-xs font-bold uppercase tracking-wider text-amber-400">${a.action}</span>
                <span class="text-xs font-mono text-slate-400">${a.created_at.split('T')[0]}</span>
              </div>
              <p class="text-sm font-medium text-white">${a.description}</p>
            </div>
            <div class="flex items-center gap-2">
              <button onclick="reviewApproval('${a.id}', true)" class="px-3 py-1.5 rounded-lg bg-emerald-500 hover:bg-emerald-600 text-white font-medium text-xs">Approve</button>
              <button onclick="reviewApproval('${a.id}', false)" class="px-3 py-1.5 rounded-lg bg-rose-500/20 hover:bg-rose-500/30 text-rose-300 font-medium text-xs">Reject</button>
            </div>
          `;
          list.appendChild(card);
        });
      }
    }

    async function loadActivity() {
      const logs = await fetch(`/api/v1/activity?company_id=${currentCompany.id}&limit=20`).then(r => r.json());
      const ovList = document.getElementById('ov-activity-list');
      const fullList = document.getElementById('activity-full-list');

      ovList.innerHTML = '';
      fullList.innerHTML = '';

      logs.slice(0, 4).forEach(l => {
        const item = document.createElement('div');
        item.className = 'p-2 rounded bg-slate-900/60 border border-slate-800 text-slate-300 flex justify-between';
        item.innerHTML = `<span>⚡ ${l.details}</span> <span class="text-slate-500 text-[10px]">${l.timestamp.split('T')[1].slice(0, 8)}</span>`;
        ovList.appendChild(item);
      });

      logs.forEach(l => {
        const row = document.createElement('div');
        row.className = 'p-3 rounded-lg bg-slate-900/40 border border-slate-800/80 flex items-start justify-between gap-4';
        row.innerHTML = `
          <div>
            <div class="flex items-center gap-2">
              <span class="text-orange-400 font-semibold">&lt;${l.actor_type}&gt;</span>
              <span class="text-slate-400">${l.action}</span>
            </div>
            <p class="text-white mt-0.5 text-xs font-sans">${l.details}</p>
          </div>
          <span class="text-slate-500 text-[11px] shrink-0 font-mono">${l.timestamp.replace('T', ' ').slice(0, 19)}</span>
        `;
        fullList.appendChild(row);
      });
    }

    async function loadOrgChart() {
      const data = await fetch(`/api/v1/companies/${currentCompany.id}/org-chart`).then(r => r.json());
      document.getElementById('org-ascii').textContent = data.ascii;

      const container = document.getElementById('org-tree-container');
      container.innerHTML = '';

      function renderNode(node, depth = 0) {
        const div = document.createElement('div');
        div.className = depth > 0 ? 'ml-6 pl-4 border-l-2 border-slate-800 space-y-3' : 'space-y-3';
        div.innerHTML = `
          <div class="card p-3 rounded-lg inline-flex items-center gap-4 border-slate-700 bg-slate-900/90 shadow-md">
            <div class="w-8 h-8 rounded-lg bg-orange-500/20 text-orange-400 flex items-center justify-center font-bold text-sm">
              ${node.role.slice(0, 2).toUpperCase()}
            </div>
            <div>
              <div class="font-bold text-white text-sm">${node.name}</div>
              <div class="text-[11px] text-slate-400">${node.role} &middot; $${node.budget_spent_usd.toFixed(2)} / $${node.budget_limit_usd.toFixed(2)}</div>
            </div>
          </div>
        `;
        if (node.children && node.children.length > 0) {
          const childrenContainer = document.createElement('div');
          childrenContainer.className = 'space-y-3';
          node.children.forEach(c => childrenContainer.appendChild(renderNode(c, depth + 1)));
          div.appendChild(childrenContainer);
        }
        return div;
      }

      (data.roots || []).forEach(root => container.appendChild(renderNode(root, 0)));
    }

    async function triggerHeartbeat() {
      const btn = document.getElementById('btn-heartbeat');
      const banner = document.getElementById('heartbeat-banner');
      const logs = document.getElementById('heartbeat-logs');
      const stats = document.getElementById('heartbeat-stats');

      btn.disabled = true;
      btn.classList.add('opacity-50');
      banner.classList.remove('hidden');
      logs.innerHTML = '<div>⚡ Heartbeat tick triggered. Dispatching tasks across active agents...</div>';

      try {
        const res = await fetch(`/api/v1/heartbeat/tick/${currentCompany.id}`, { method: 'POST' }).then(r => r.json());
        stats.textContent = `${res.ticks_executed} tasks executed · $${res.total_cost_usd.toFixed(4)}`;
        
        if (res.details && res.details.length > 0) {
          res.details.forEach(d => {
            const entry = document.createElement('div');
            entry.className = d.success ? 'text-emerald-400' : 'text-rose-400';
            entry.textContent = `✔ [${d.agent_name}] '${d.issue_title}': ${d.summary} ($${d.cost_usd.toFixed(4)})`;
            logs.appendChild(entry);
          });
        } else {
          logs.innerHTML += '<div class="text-slate-400">No pending tasks found for active agents.</div>';
        }

        await loadData();
      } catch (err) {
        logs.innerHTML += `<div class="text-rose-400">Error: ${err.message}</div>`;
      } finally {
        btn.disabled = false;
        btn.classList.remove('opacity-50');
        setTimeout(() => banner.classList.add('hidden'), 10000);
      }
    }

    async function moveIssue(id, status) {
      await fetch(`/api/v1/issues/${id}`, {
        method: 'PATCH',
        headers: {'Content-Type': 'application/json'},
        body: JSON.stringify({ status })
      });
      await loadIssues();
    }

    async function reviewApproval(id, approve) {
      const endpoint = approve ? 'approve' : 'reject';
      await fetch(`/api/v1/approvals/${id}/${endpoint}`, {
        method: 'POST',
        headers: {'Content-Type': 'application/json'},
        body: JSON.stringify({ reviewer: 'Human Admin' })
      });
      await Promise.all([loadApprovals(), loadData()]);
    }

    async function toggleAgent(id, action) {
      await fetch(`/api/v1/agents/${id}/${action}`, { method: 'POST' });
      await loadAgents();
    }

    function openNewIssueModal() {
      document.getElementById('modal-issue').classList.remove('hidden');
    }

    function closeNewIssueModal() {
      document.getElementById('modal-issue').classList.add('hidden');
    }

    async function submitNewIssue() {
      const title = document.getElementById('new-issue-title').value.trim();
      const description = document.getElementById('new-issue-desc').value.trim();
      const assignee_agent_id = document.getElementById('new-issue-assignee').value || null;
      const priority = document.getElementById('new-issue-priority').value;

      if (!title || cachedProjects.length === 0) return;

      await fetch('/api/v1/issues', {
        method: 'POST',
        headers: {'Content-Type': 'application/json'},
        body: JSON.stringify({
          project_id: cachedProjects[0].id,
          title,
          description,
          assignee_agent_id,
          priority
        })
      });

      closeNewIssueModal();
      document.getElementById('new-issue-title').value = '';
      document.getElementById('new-issue-desc').value = '';
      await loadIssues();
    }

    function switchTab(tab) {
      const tabs = ['overview', 'kanban', 'org', 'agents', 'costs', 'approvals', 'activity'];
      tabs.forEach(t => {
        const btn = document.getElementById(`tab-${t}`);
        const view = document.getElementById(`view-${t}`);
        if (t === tab) {
          btn.classList.add('tab-active');
          view.classList.remove('hidden');
        } else {
          btn.classList.remove('tab-active');
          view.classList.add('hidden');
        }
      });
    }

    document.addEventListener('DOMContentLoaded', loadData);
  </script>
</body>
</html>
"#)
}
