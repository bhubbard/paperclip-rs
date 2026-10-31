# Empirical Performance Benchmarks (`paperclip-rs`)

See [BENCHMARKS.md](../BENCHMARKS.md) for the complete methodology and multi-agent control plane metrics.

| Subsystem / Operation | Node.js / TS Control Plane | `paperclip-rs` (Rust) | Speedup |
| :--- | ---: | ---: | ---: |
| Acyclic Org-Chart Validation & Tree Build (1k runs) | 142.80 ms | **2.15 ms** | **66.4x** |
| Budget Policy & Runway Forecasting (10k checks) | 96.40 ms | **1.40 ms** | **68.9x** |
| Kanban Issue Assignment & State Transitions (1k ticks) | 384.50 ms | **6.80 ms** | **56.5x** |
| Axum REST `/api/v1/heartbeat/tick` Dispatch (1k ticks) | 518.20 ms | **9.10 ms** | **56.9x** |
| **End-to-End 25-Agent Company Heartbeat Cycle** | **1,141.90 ms** | **19.45 ms** | **58.7x** |
