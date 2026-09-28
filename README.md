# paperclip-rs 📎🦀

A high-performance, memory-safe Rust fork and control plane for [Paperclip](https://github.com/paperclipai/paperclip) — the open-source orchestration system that manages teams of autonomous AI agents like a real company.

> *"If OpenClaw is an employee, Paperclip is the company."*

---

## ⚡ Why a Rust Fork?

- **Blazingly Fast Heartbeat Ticks**: Dispatches concurrent agent turns, evaluates budgets, and updates issue state machines in sub-millisecond latencies.
- **Robust Process Supervision**: Native process isolation with `kill_on_drop`, watchdog cancellation tokens, and timeouts for CLI-driven agent runtimes (Claude Code, OpenAI Codex, AGY, Ollama).
- **Zero Overhead & Embedded Storage**: Lightweight in-memory and persistent atomic storage without requiring complex external database servers for local setups.
- **Strict Governance & Budget Enforcement**: First-class support for `HardStop`, `RequireApproval`, and `WarnOnly` spend policies with human-in-the-loop signoffs.
- **Single Static Binary**: Compiles to a single standalone executable (`paperclip`) for both the CLI and Axum HTTP control plane.

---

## 📦 Workspace Crates

| Crate | Description |
| :--- | :--- |
| [**`paperclip-core`**](crates/paperclip-core) | Core domain entities (`Company`, `Agent`, `Goal`, `Project`, `Issue`, `Approval`, `CostEvent`), acyclic Org Chart hierarchy engine with ASCII visualization, budget policy enforcer, and persistent thread-safe storage. |
| [**`paperclip-runner`**](crates/paperclip-runner) | Agent execution engine, process supervisor, pluggable adapters (`MockProvider`, `ProcessProvider`, `HttpLlmProvider`), and heartbeat dispatcher. |
| [**`paperclip-server`**](crates/paperclip-server) | High-throughput async REST API server built on **Axum** with CORS and graceful shutdown. |
| [**`paperclip-cli`**](crates/paperclip-cli) | Native CLI toolkit for onboarding virtual companies, managing Kanban issues, triggering heartbeat ticks, inspecting spend, and viewing ASCII org charts. |

---

## 🚀 Quickstart

### 1. Build the Workspace

```bash
cargo build --release
```

The compiled binary will be located at `target/release/paperclip`.

### 2. Onboard a Virtual AI Company

Initialize an autonomous company from the startup template:

```bash
cargo run -p paperclip-cli -- onboard \
  --name "Apollo AI" \
  --mission "Build the leading autonomous software suite" \
  --budget 100000
```

Output:
```text
🚀 Initializing virtual company 'Apollo AI'...

✅ Successfully onboarded company!
   Company ID:    9730e6f0-9248-4ff4-9ef0-d972ef207900
   Name:          Apollo AI
   Mission:       Build the leading autonomous software suite
   Budget Cap:    $100000.00 USD
   Hired Agents:  5

📊 Org Chart Hierarchy:

◆ Nova (CEO) (CEO) [$0.00 / $30000.00]
  ├── Iris (Product Manager) (Product Manager) [$0.00 / $10000.00]
  └── Atlas (CTO) (CTO) [$0.00 / $25000.00]
      ├── Cipher (Lead Engineer) (Lead Engineer) [$0.00 / $20000.00]
      └── Sentinel (QA Engineer) (QA Engineer) [$0.00 / $15000.00]
```

### 3. Run Heartbeat Orchestration

Trigger an agent execution tick across assigned issues:

```bash
cargo run -p paperclip-cli -- heartbeat
```

### 4. Inspect Spend & Token Analytics

```bash
cargo run -p paperclip-cli -- cost
```

```text
💰 Cost & Token Spend Summary:
   Company ID:           9730e6f0-9248-4ff4-9ef0-d972ef207900
   Total Spend:          $0.0198
   Total Tokens:         2716 (Input: 1748, Output: 968)

AGENT                        ROLE               SPEND        BUDGET CAP     % CONSUMED    
------------------------------------------------------------------------------------------
Iris (Product Manager)       Product Manager    $0.0000      $10000.00      0.0%
Cipher (Lead Engineer)       Lead Engineer      $0.0098      $20000.00      0.0%
Atlas (CTO)                  CTO                $0.0000      $25000.00      0.0%
Sentinel (QA Engineer)       QA Engineer        $0.0099      $15000.00      0.0%
Nova (CEO)                   CEO                $0.0000      $30000.00      0.0%
```

### 5. Launch the REST API Server

```bash
cargo run -p paperclip-cli -- serve --port 3100
```

---

## 🌐 REST API Endpoints

| Method | Endpoint | Description |
| :--- | :--- | :--- |
| `GET` | `/health` | Health check probe |
| `GET` | `/api/v1/info` | Global statistics (companies, agents, issues) |
| `GET` | `/api/v1/companies` | List all registered companies |
| `POST` | `/api/v1/companies` | Create a company |
| `POST` | `/api/v1/companies/seed` | Seed startup company template |
| `GET` | `/api/v1/companies/:id/org-chart` | Get hierarchical org chart & ASCII tree |
| `GET` | `/api/v1/agents` | List agents (filterable by `?company_id=...`) |
| `POST` | `/api/v1/agents` | Hire a new agent |
| `POST` | `/api/v1/agents/:id/pause` | Pause an agent |
| `POST` | `/api/v1/agents/:id/resume` | Resume an agent |
| `GET` | `/api/v1/goals` | List company goals |
| `POST` | `/api/v1/goals` | Create a goal |
| `GET` | `/api/v1/projects` | List projects |
| `POST` | `/api/v1/projects` | Create a project |
| `GET` | `/api/v1/issues` | List issues (filterable by `?project_id=...&status=...`) |
| `POST` | `/api/v1/issues` | Create an issue |
| `PATCH` | `/api/v1/issues/:id` | Move Kanban status / reassign issue |
| `POST` | `/api/v1/heartbeat/tick/:company_id` | Trigger an orchestration heartbeat tick |
| `GET` | `/api/v1/approvals` | List pending human-in-the-loop approvals |
| `POST` | `/api/v1/approvals/:id/approve` | Approve an action |
| `POST` | `/api/v1/approvals/:id/reject` | Reject an action |
| `GET` | `/api/v1/costs/summary/:company_id` | Token and financial analytics summary |
| `GET` | `/api/v1/activity` | Audit log activity feed |

---

## 🧪 Testing

Run the automated test suite across all crates:

```bash
cargo test --workspace
```

---

## 📄 License

Licensed under either of:
- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE))
- MIT License ([LICENSE-MIT](LICENSE-MIT))

at your option.
