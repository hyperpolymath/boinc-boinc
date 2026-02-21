<!-- SPDX-License-Identifier: PMPL-1.0-or-later -->
<!-- TOPOLOGY.md — Project architecture map and completion dashboard -->
<!-- Last updated: 2026-02-19 -->

# Oblibeny BOINC Platform — Project Topology

## System Architecture

```
                        ┌─────────────────────────────────────────┐
                        │              BOINC VOLUNTEERS           │
                        │        (Distributed Verification)       │
                        └───────────────────┬─────────────────────┘
                                            │
                                            ▼
                        ┌─────────────────────────────────────────┐
                        │           COORDINATOR LAYER             │
                        │  ┌───────────┐  ┌───────────────────┐  │
                        │  │ Elixir/OTP│  │  BOINC App        │  │
                        │  │ Coordinator│ │  (Ada Validator)  │  │
                        │  └─────┬─────┘  └────────┬──────────┘  │
                        └────────│─────────────────│──────────────┘
                                 │                 │
                                 ▼                 ▼
                        ┌─────────────────────────────────────────┐
                        │           LANGUAGE COMPILER             │
                        │  ┌───────────┐  ┌───────────────────┐  │
                        │  │ Rust      │  │  Lean 4           │  │
                        │  │ Parser    │  │  Proofs           │  │
                        │  └─────┬─────┘  └────────┬──────────┘  │
                        └────────│─────────────────│──────────────┘
                                 │                 │
                                 ▼                 ▼
                        ┌─────────────────────────────────────────┐
                        │             DATA LAYER                  │
                        │  ┌───────────┐  ┌───────────────────┐  │
                        │  │ ArangoDB  │  │ Prometheus /      │  │
                        │  │ (Graph)   │  │ Grafana           │  │
                        │  └───────────┘  └───────────────────┘  │
                        └─────────────────────────────────────────┘

                        ┌─────────────────────────────────────────┐
                        │          REPO INFRASTRUCTURE            │
                        │  Nix / flake.nix    .machine_readable/  │
                        │  Justfile           Tri-Perimeter CF    │
                        └─────────────────────────────────────────┘
```

## Completion Dashboard

```
COMPONENT                          STATUS              NOTES
─────────────────────────────────  ──────────────────  ─────────────────────────────────
LANGUAGE CORE
  Rust Parser                       ██████████ 100%    Termination checking stable
  Lean 4 Proofs                     ████░░░░░░  40%    7 properties scaffolding
  Grammar & Semantics               ████████░░  80%    Lisp-style syntax defined

DISTRIBUTED SYSTEM
  Elixir Coordinator                ██████████ 100%    OTP supervision tree stable
  BOINC Integration                 ██████████ 100%    Work unit generator active
  Ada Validator                     ████████░░  80%    Quorum consensus refined

INFRASTRUCTURE
  Nix Development Env               ██████████ 100%    Reproducible builds verified
  ArangoDB (Multi-model)            ██████████ 100%    Proof dependency graphs active
  Podman/Docker Compose             ██████████ 100%    Full stack deployment stable

REPO INFRASTRUCTURE
  Justfile                          ██████████ 100%    RSR validation tasks
  .machine_readable/                ██████████ 100%    STATE.a2ml tracking
  Tri-Perimeter CF                  ██████████ 100%    Governance standard verified

─────────────────────────────────────────────────────────────────────────────
OVERALL:                            ████████░░  ~80%   v0.6.0 Active Development
```

## Key Dependencies

```
Rust Parser ───► Elixir Coordinator ───► BOINC Work ───► Proofs
     │                 │                   │              │
     └─────────────────┴────────┬──────────┴──────────────┘
                                ▼
                           ArangoDB (Graph)
```

## Update Protocol

This file is maintained by both humans and AI agents. When updating:

1. **After completing a component**: Change its bar and percentage
2. **After adding a component**: Add a new row in the appropriate section
3. **After architectural changes**: Update the ASCII diagram
4. **Date**: Update the `Last updated` comment at the top of this file

Progress bars use: `█` (filled) and `░` (empty), 10 characters wide.
Percentages: 0%, 10%, 20%, ... 100% (in 10% increments).
