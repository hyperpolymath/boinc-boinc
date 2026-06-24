<!--
SPDX-License-Identifier: CC-BY-SA-4.0
Copyright (c) Jonathan D.A. Jewell <j.d.a.jewell@open.ac.uk>
-->
# Oblibeny BOINC Platform - Master Orchestration

## Mission Statement

Build a distributed verification platform for Oblibeny, a novel programming language that compiles from Turing-complete (development) to Turing-incomplete (deployment) code for secure IoT/embedded systems. Uses BOINC to crowdsource millions of test cases and formal proofs of language properties.

## Architecture Overview

```
┌─────────────────────────────────────────────────────────────┐
│                     BOINC Server (VPS)                      │
│  ┌────────────────┐  ┌──────────────┐  ┌────────────────┐  │
│  │ Phoenix        │  │  Elixir/OTP  │  │   ArangoDB     │  │
│  │ Dashboard      │◄─┤  Coordinator │◄─┤   Multi-Model  │  │
│  │ (Monitoring)   │  │              │  │   Database     │  │
│  └────────────────┘  └──────┬───────┘  └────────────────┘  │
│                             │                               │
│                      ┌──────▼───────┐                       │
│                      │ BOINC Server │                       │
│                      │  Components  │                       │
│                      └──────┬───────┘                       │
└─────────────────────────────┼─────────────────────────────┘
                              │
                    ┌─────────▼─────────┐
                    │  BOINC Protocol   │
                    └─────────┬─────────┘
                              │
        ┌─────────────────────┼─────────────────────┐
        │                     │                     │
   ┌────▼────┐           ┌────▼────┐          ┌────▼────┐
   │Volunteer│           │Volunteer│          │Volunteer│
   │ Client  │           │ Client  │   ...    │ Client  │
   │  #1     │           │  #2     │          │  #N     │
   └─────────┘           └─────────┘          └─────────┘
```

## Component Breakdown

### Task 1: Rust Parser & Analyzer
**Location**: `rust-parser/`
**Purpose**: Parse Oblibeny code, build AST, separate phases, analyze resources

**Components**:
- `parser/` - pest-based parser from EBNF grammar
- `ast/` - Abstract Syntax Tree definitions
- `phases/` - Phase separation (compile-time vs deploy-time)
- `analyzer/` - Resource bounds, termination, capability analysis

**Deliverables**:
- Complete parser for Oblibeny v0.6
- Phase separator with validation
- Resource analyzer
- CLI tool for testing
- Comprehensive test suite

### Task 2: Elixir/OTP Coordinator
**Location**: `elixir-coordinator/`
**Purpose**: Distributed work coordination, result aggregation, proof orchestration

**Components**:
- `coordinator/` - Main OTP application
- `arangodb/` - Database interface layer
- `boinc/` - BOINC work generator/validator integration

**Deliverables**:
- Work unit generator
- Result validator and aggregator
- Proof dependency tracker
- Fault-tolerant distributed coordination
- ArangoDB integration

### Task 3: Nix Build System
**Location**: `deployment/nix/`
**Purpose**: Reproducible builds for all components

**Deliverables**:
- Flake for entire project
- Dev shells for each component
- Cross-compilation support
- Cache configuration

### Task 4: Nickel Configuration
**Location**: `deployment/nickel/`
**Purpose**: Type-safe configuration management

**Deliverables**:
- Server configuration schema
- Deployment configurations
- Environment-specific configs
- Validation rules

### Task 5: Lean 4 Formal Proofs
**Location**: `lean-proofs/`
**Purpose**: Machine-checked proofs of Oblibeny properties

**Target Properties**:
1. Phase separation soundness
2. Deployment termination
3. Resource bounds enforcement
4. Capability system soundness
5. Obfuscation semantic preservation
6. Call graph acyclicity
7. Memory safety

**Deliverables**:
- Formalized syntax in Lean
- Operational semantics
- Type system
- Proofs for all 7 properties (or partial progress with clear TODOs)

### Task 6: Phoenix Dashboard
**Location**: `phoenix-dashboard/`
**Purpose**: Web interface for monitoring verification progress

**Features**:
- Real-time work unit status
- Proof progress visualization
- Volunteer statistics
- Property verification status
- ArangoDB graph visualization

### BOINC Integration
**Location**: `boinc-app/`
**Purpose**: BOINC application wrapper

**Components**:
- Validator - Ada-based safety-critical validation
- Work Generator - Creates verification work units
- Assimilator - Processes validated results

### Deployment
**Location**: `deployment/`
**Purpose**: Production deployment infrastructure

**Components**:
- Podman containers for all services
- Docker Compose for development
- Nix-based builds
- Nickel configurations

## Technology Stack Rationale

### Ada (Validator)
- **Why**: Zero tolerance for bugs in validation logic
- **Use Case**: Final result validation before accepting proofs
- **Trade-off**: Slower development, but unmatched reliability

### Elixir/OTP (Coordinator)
- **Why**: Built for distributed, fault-tolerant systems
- **Use Case**: Coordinating thousands of work units across volunteers
- **Trade-off**: Smaller ecosystem, but perfect for this domain

### Rust (Parser)
- **Why**: Performance-critical, memory-safe
- **Use Case**: Parsing millions of test programs
- **Trade-off**: Steeper learning curve, but worth it for correctness

### Lean 4 (Proofs)
- **Why**: Modern, active community, good tooling
- **Use Case**: Machine-checked proofs of language properties
- **Trade-off**: Fewer resources than Coq, but more maintainable

### ArangoDB (Database)
- **Why**: Multi-model (documents + graphs)
- **Use Case**: Proof dependencies are naturally graph-shaped
- **Trade-off**: Less common than Postgres, but better fit

### Nix (Builds)
- **Why**: Truly reproducible builds
- **Use Case**: Ensuring volunteers run identical code
- **Trade-off**: Steep learning curve, but essential for security

### Nickel (Config)
- **Why**: Type-safe configuration
- **Use Case**: Preventing deployment mistakes
- **Trade-off**: Young project, but solves real pain points

## Data Flow

```
1. Elixir Coordinator generates test program specification
   ↓
2. Rust Parser generates program variants (semantic obfuscation)
   ↓
3. Work units created with program + verification tasks
   ↓
4. BOINC distributes to volunteers
   ↓
5. Volunteers run verification tasks (bounded execution)
   ↓
6. Results returned to BOINC server
   ↓
7. Ada Validator checks results for correctness
   ↓
8. Elixir Coordinator aggregates results
   ↓
9. Lean 4 proofs updated with new evidence
   ↓
10. Phoenix Dashboard shows progress
```

## ArangoDB Schema

### Collections

1. **programs** - Test programs
2. **work_units** - Individual verification tasks
3. **results** - Volunteer-submitted results
4. **proofs** - Formal proofs in progress
5. **properties** - The 7 properties being verified
6. **volunteers** - BOINC volunteer tracking
7. **proof_steps** - Individual proof steps
8. **counterexamples** - Failed test cases
9. **statistics** - Aggregated metrics

### Graphs

1. **proof_dependencies** - Which proofs depend on which lemmas
2. **program_variants** - Semantic obfuscation relationships
3. **property_coverage** - Which programs test which properties

## BOINC Deployment Model

**Self-Hosted Server**:
- VPS running BOINC server software
- Public URL for volunteers to attach
- No permission needed from external parties

**Volunteer Client**:
- Standard BOINC client (users already have installed)
- Points to your server URL
- Downloads work units, uploads results

**Work Unit Design**:
- Small programs (< 10KB)
- Bounded execution time (< 5 minutes)
- Checkpointing for longer tasks
- Redundancy: 3 volunteers per work unit
- Quorum validation: 2/3 agreement required

## Security Model

### Capability System
- All I/O mediated through capabilities
- Static budgets (bytes, time, memory)
- No ambient authority

### Deployment Constraints
- No unbounded loops (all `bounded-for`)
- No recursion (call graph must be acyclic)
- No syscalls (only capability-based I/O)
- Static memory bounds

### Semantic Obfuscation
- Each deployment morphs code structure
- Semantics preserved (proven in Lean)
- Makes reverse engineering harder

## Development Workflow

```bash
# Enter development shell
nix develop

# Build all components
nix build .#all

# Run tests
nix build .#tests

# Deploy to staging
./scripts/deploy/staging.sh

# Deploy to production
./scripts/deploy/production.sh
```

## Success Metrics

1. **Coverage**: 10M+ test cases generated
2. **Proof Progress**: All 7 properties proven or concrete counterexamples found
3. **Performance**: < 100ms parse time for typical programs
4. **Reliability**: 99.9% uptime for BOINC server
5. **Volunteer Engagement**: 100+ active volunteers

## Current Status

- **Sprint 1 Complete**: Ada validator, ArangoDB schema, Podman infrastructure
- **Sprint 2 In Progress**: Building all 6 tasks autonomously
- **Next**: Integration testing, production deployment

## File Organization

```
oblibeny-boinc/
├── 00_MASTER_ORCHESTRATION.md (this file)
├── 01_COMMON_CONTEXT.md
├── 02_TASK_RUST_PARSER.md
├── 03_TASK_ELIXIR_COORDINATOR.md
├── 04_TASK_NIX_BUILD.md
├── 05_TASK_NICKEL_CONFIG.md
├── 06_TASK_LEAN_PROOFS.md
├── 07_TASK_PHOENIX_DASHBOARD.md
├── HANDOVER_TO_NEW_CLAUDE.md
├── rust-parser/
├── elixir-coordinator/
├── lean-proofs/
├── phoenix-dashboard/
├── boinc-app/
├── deployment/
├── grammar/
├── examples/
├── docs/
├── scripts/
└── tests/
```

## Key Decision Log

1. **Self-hosted BOINC**: Avoids bureaucracy, full control
2. **Ada for validation**: Safety-critical code deserves safety-critical language
3. **Elixir for coordination**: Perfect fit for distributed fault-tolerance
4. **Rust for parser**: Performance + correctness
5. **Lean 4 over Coq**: Modern tooling, better for newcomers
6. **ArangoDB over Neo4j+Postgres**: Single DB with both models
7. **Nix over Docker**: True reproducibility
8. **Nickel for config**: Type safety prevents deployment errors

## Open Questions / TODOs

- [ ] Determine optimal work unit size (balance overhead vs granularity)
- [ ] Choose proof strategy for property #5 (semantic preservation)
- [ ] Decide on volunteer credit/badging system
- [ ] Plan for handling malicious volunteers (Byzantine fault tolerance)
- [ ] Design cache warming strategy for Nix builds
- [ ] Determine ArangoDB sharding strategy for scale

## References

- BOINC Documentation: https://boinc.berkeley.edu/
- Lean 4 Documentation: https://lean-lang.org/
- ArangoDB Manual: https://www.arangodb.com/docs/
- Oblibeny Language Spec: See `grammar/` directory
