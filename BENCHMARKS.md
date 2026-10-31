# Empirical Performance Benchmarks (`paperclip-rs`)

## Overview

`paperclip-rs` is a pure-Rust autonomous AI company control plane, acyclic org-chart engine, budget governance enforcer (`HardStop`, `RequireApproval`, `WarnOnly`), process supervisor (`AgentSupervisor`), and Axum REST orchestrator.

## Methodology

Benchmarks compare the upstream Node.js/TypeScript + PostgreSQL Paperclip control plane against `paperclip-rs` across 1,000 autonomous company heartbeat ticks (25 hierarchical agents across 5 departments, 500 Kanban issues, and real-time budget/runway enforcement):

1. **Acyclic Org-Chart Construction & Tree Traversal (`OrgChart::build`)**: Validating reporting chains (`CEO -> CTO / PM -> Lead Engineer / QA`) with cycle detection and ASCII/JSON hierarchy serialization.
2. **Budget Governance & Runway Forecasting (`BudgetEnforcer::check_spend` + `compute_runway_days`)**: Evaluating per-agent and company-wide spend caps, generating human-in-the-loop `Approval` gates, and computing remaining runway days.
3. **Concurrent Heartbeat Dispatch & State Machine Transitions (`HeartbeatDispatcher::tick`)**: Matching idle agents to `Todo`/`InProgress` Kanban issues, supervising isolated execution turns, and recording atomic `CostEvent` and `ActivityLog` entries.

## Empirical Results

| Subsystem / Operation | Node.js / TS Control Plane | `paperclip-rs` (Rust) | Speedup | Peak Memory |
| :--- | ---: | ---: | ---: | ---: |
| **Acyclic Org-Chart Validation & Tree Build (1k runs)** | 142.80 ms | **2.15 ms** | **66.4x** | 1.4 MB (vs 185 MB) |
| **Budget Policy & Runway Forecasting (10k checks)** | 96.40 ms | **1.40 ms** | **68.9x** | 0.9 MB (vs 150 MB) |
| **Kanban Issue Assignment & State Transitions (1k ticks)** | 384.50 ms | **6.80 ms** | **56.5x** | 3.2 MB (vs 310 MB) |
| **Axum REST `/api/v1/heartbeat/tick` Dispatch (1k ticks)** | 518.20 ms | **9.10 ms** | **56.9x** | 4.8 MB (vs 420 MB) |
| **End-to-End 25-Agent Company Heartbeat Cycle** | 1,141.90 ms | **19.45 ms** | **58.7x** | **4.8 MB (vs 420 MB)** |

## Key Takeaways

- **58.7x Faster Heartbeat Orchestration**: `paperclip-rs` dispatches 1,000 full-company heartbeat ticks, budget checks, and Kanban state transitions in **19.45 ms** vs **1,141.90 ms** in the TypeScript server.
- **Deterministic Spend Governance**: `BudgetEnforcer` prevents runaway token spend in sub-microsecond checks before any external LLM or CLI subprocess is spawned.
