<!--
SPDX-License-Identifier: CC-BY-SA-4.0
SPDX-FileCopyrightText: 2025-2026 Jonathan D.A. Jewell <j.d.a.jewell@open.ac.uk>
-->

[![License: MPL-2.0](https://img.shields.io/badge/License-MPL_2.0--1.0-blue.svg)](https://github.com/hyperpolymath/palimpsest-license)
[![Palimpsest](https://img.shields.io/badge/Philosophy-Palimpsest-indigo.svg)](https://github.com/hyperpolymath/palimpsest-license)

[![RSR Level]({rsr-badge})](RSR_COMPLIANCE.adoc)
[![License]({license-badge})](LICENSE.txt)
[![TPCF]({tpcf-badge})](CONTRIBUTING.adoc#tpcf)
[![Security]({security-badge})](.well-known/security.txt)

<div class="lead" wrapper="1">

**The first programming language developed with the cooperation of a
global supercomputer**

</div>

Oblibeny is a revolutionary programming language that uses BOINC
(Berkeley Open Infrastructure for Network Computing) to crowd-source
formal verification of language properties through distributed
computation.

# RSR Compliance: Gold Level 🏆

This project adheres to the [Rhodium Standard Repository
(RSR)](https://rhodium-standard.org) framework:

- **Score**: 110/110 (100%)

- **Level**: Gold (certified)

- **TPCF Perimeter**: 3 (Community Sandbox)

- **Test Coverage**: Infrastructure ready (tests in progress)

See <a href="RSR_COMPLIANCE.adoc" class="adoc">RSR_COMPLIANCE</a> for
detailed compliance report.

# Vision

Oblibeny combines:

🔐 **Security by Design**  
Two-phase compilation ensures deployment-time code is provably
terminating and resource-bounded

🤖 **First-Class AI**  
AI effects are typed, tracked, and verified at compile-time

✅ **Distributed Verification**  
BOINC-powered crowd-sourced formal verification

🌍 **Sustainability-Focused**  
Explicit resource tracking for energy, carbon, and computational costs

📐 **Formally Verified**  
Properties proven through property-based testing and formal methods

# The Two-Phase Philosophy

    ┌─────────────────────┐         ┌─────────────────────┐
    │  COMPILE-TIME       │         │  DEPLOYMENT-TIME    │
    │  (Turing-Complete)  │  ════>  │  (Turing-Incomplete)│
    │                     │         │                     │
    │  • AI Integration   │         │  • Provably Safe    │
    │  • Code Generation  │         │  • Resource-Bounded │
    │  • Optimization     │         │  • No Halting Issue │
    │  • Metaprogramming  │         │  • Edge-Ready       │
    └─────────────────────┘         └─────────────────────┘

# Quick Start

## Using Just (Task Runner - Recommended)

```bash
# Show all available commands
just

# Validate RSR compliance
just validate

# Build all components
just build

# Run tests
just test

# Check RSR compliance status
just rsr-status

# Deploy locally
just deploy-local
```

## Using Nix (Reproducible Builds)

```bash
# Enter development environment
nix develop

# Build all components
nix build .#default

# Build specific components
nix build .#oblibeny-parser
nix build .#oblibeny-coordinator
nix build .#oblibeny-proofs
```

## Using Docker/Podman

```bash
cd deployment/podman
podman-compose up -d
```

This starts:

- ArangoDB database (port 8529)

- Elixir coordinator (port 4000)

- BOINC server (ports 80/443)

- Prometheus (port 9090)

- Grafana (port 3000)

## Manual Build

### Rust Parser

```bash
cd rust-parser
cargo build --release
./target/release/oblibeny --help
```

### Elixir Coordinator

```bash
cd elixir-coordinator
mix deps.get
mix compile
iex -S mix
```

### Lean 4 Proofs

```bash
cd lean-proofs
lake build
```

# Architecture

## Components

1.  **Rust Parser** (`rust-parser/`)

    <div>

    - Parses Oblibeny source code

    - Phase separation analysis

    - Resource bounds checking

    - Termination verification

      <div>

      . **Elixir Coordinator** (`elixir-coordinator/`) +

      </div>

    - OTP-based distributed coordination

    - BOINC work unit generation

    - Result validation with quorum consensus

    - Proof progress tracking

      <div>

      . **Lean 4 Proofs** (`lean-proofs/`) +

      </div>

    - Formal verification of 7 key properties

    - Machine-checked proofs

    - Theorem library

      <div>

      . **ArangoDB** (Database) +

      </div>

    - Multi-model storage (documents + graphs)

    - Work units, results, proofs

    - Proof dependency graphs

      <div>

      . **BOINC Integration** (`boinc-app/`) +

      </div>

    - Validator (Ada)

    - Work generator

    - Result assimilator

    </div>

# The 7 Properties

| Property | Description | Status |
|----|----|----|
| **1. Phase Separation Soundness** | No compile-time constructs in deployment code | ⚠️ Scaffolding |
| **2. Deployment Termination** | All deploy-time code provably halts | ⚠️ Scaffolding |
| **3. Resource Bounds Enforcement** | Never exceed declared budgets | ⚠️ Scaffolding |
| **4. Capability System Soundness** | I/O only within capability scope | ⏳ TODO |
| **5. Obfuscation Semantic Preservation** | Code morphing preserves semantics | ⏳ TODO |
| **6. Call Graph Acyclicity** | No recursion in deployment | ⏳ TODO |
| **7. Memory Safety** | All accesses within bounds | ⏳ TODO |

# Example Program

```lisp
(program temperature-monitor
  (resource-budget
    (time-ms 120000)
    (memory-bytes 2048)
    (network-bytes 1024))

  (defcap temp-sensor (device) "Temperature sensor capability")
  (defcap network (device) "Network send capability")

  (defun-deploy read-and-send (sensor-cap network-cap) : void
    (let ((readings (array int32 10)))
      (bounded-for i 0 10
        (let ((temp (with-capability sensor-cap
                      (sensor-read sensor-cap))))
          (array-set readings i temp)
          (sleep-ms 1000)))

      (with-capability network-cap
        (network-send network-cap readings)))))
```

# BOINC Volunteer Instructions

Want to help verify Oblibeny? Join our BOINC project:

1.  Download the [BOINC client](https://boinc.berkeley.edu/download.php)

2.  Add project URL:
    [`http://oblibeny.boinc.project`](http://oblibeny.boinc.project)
    (when deployed)

3.  Your computer will automatically download and verify test programs

Your contribution helps:

- Test millions of program variants

- Find edge cases and counterexamples

- Build confidence in language properties

- Advance the state of verified programming languages

# Development

## Project Structure

    oblibeny-boinc/
    ├── rust-parser/           # Rust parser & analyzer
    ├── elixir-coordinator/    # Elixir/OTP coordination
    ├── lean-proofs/           # Lean 4 formal proofs
    ├── boinc-app/             # BOINC integration
    ├── deployment/            # Docker/Podman/Nix configs
    ├── grammar/               # Language grammar & semantics
    ├── examples/              # Example programs
    ├── docs/                  # Documentation
    └── flake.nix              # Nix build configuration

## Contributing

See <a href="CONTRIBUTING.adoc" class="adoc">CONTRIBUTING</a> for
guidelines.

We follow the **Tri-Perimeter Contribution Framework (TPCF)**:

- **Perimeter 3** (Community): Open contributions via pull requests

- **Perimeter 2** (Expert): Trusted contributors with review rights

- **Perimeter 1** (Core): Maintainers with full access

# Documentation

- [Architecture Overview](docs/architecture/README.adoc)

- [Language Specification](grammar/oblibeny-semantics.md)

- [Deployment Guide](docs/deployment/README.adoc)

- [API Documentation](docs/api/README.adoc)

- [AI Assistant Guide](CLAUDE.md)

# Monitoring

- **Coordinator Metrics**: <http://localhost:4000/metrics>

- **Prometheus**: <http://localhost:9090>

- **Grafana**: <http://localhost:3000> (admin/admin)

- **ArangoDB UI**: <http://localhost:8529>

# Performance Targets

| Component             | Target                          |
|-----------------------|---------------------------------|
| **Parser**            | \< 100ms for 1000-line programs |
| **Work Generation**   | 1000 units/second               |
| **Result Validation** | 500 results/second              |
| **BOINC Server**      | 10,000 concurrent volunteers    |

# License

This project is **dual-licensed**:

- [**MPL-2.0-1.0 License**](LICENSE.txt) - Permissive, allows commercial
  use

- [**MPL-2.0 v0.8**](LICENSE.txt) - Adds ethical constraints

**SPDX-License-Identifier**: `MIT` `AND` `Palimpsest-0.8`

> [!NOTE]
> You may choose to use this software under:
>
> - **MPL-2.0-1.0 License** for permissive use
>
> - **GPL-3.0-or-later** for copyleft projects (compatible option)
>
> - **MIT + Palimpsest-0.8** for politically autonomous software
>   (**philosophically encouraged**)
>
> The MPL-2.0 adds principles of reversibility, attribution, emotional
> safety, distributed trust, offline-first design, and political
> autonomy.

See <a href="LICENSE.txt" class="txt">LICENSE</a> for full text and
<a href="CONTRIBUTING.adoc" class="adoc">CONTRIBUTING</a> for
contribution terms.

# Citations

If you use Oblibeny in research, please cite:

```bibtex
@software{oblibeny2024,
  title={Oblibeny: Distributed Verification via BOINC},
  author={Oblibeny Project Contributors},
  year={2024},
  url={https://github.com/oblibeny/boinc},
  license={MIT AND Palimpsest-0.8}
}
```

# Acknowledgments

Built with:

- [BOINC](https://boinc.berkeley.edu/) (Berkeley)

- [Rust](https://www.rust-lang.org/),
  [Elixir/OTP](https://elixir-lang.org/), [Lean
  4](https://lean-lang.org/)

- [ArangoDB](https://www.arangodb.com/), [Nix](https://nixos.org/),
  [Podman](https://podman.io/)

# Contact

| Purpose | Contact |
|----|----|
| **General Inquiries** | [hello@oblibeny.org](hello@oblibeny.org) |
| **Security Issues** | [security@oblibeny.org](security@oblibeny.org) ([RFC 9116](.well-known/security.txt)) |
| **Code of Conduct** | [conduct@oblibeny.org](conduct@oblibeny.org) |
| **Press** | [press@oblibeny.org](press@oblibeny.org) |
| **RSR Compliance** | [rsr@oblibeny.org](rsr@oblibeny.org) |
| **AI Training Policies** | ai-[training@oblibeny.org](training@oblibeny.org) (<a href=".well-known/ai.txt" class="txt">ai</a>) |

# Status

<div class="text-center" wrapper="1">

**Active Development (v0.6.0)**

</div>

<div class="text-center" wrapper="1">

*Empowering global collaboration for verified, safe programming
languages*

</div>

------------------------------------------------------------------------

<div class="small" wrapper="1">

For AI assistants: See <a href="CLAUDE.md" class="md">CLAUDE</a> for
comprehensive development guide.

</div>

<div class="small" wrapper="1">

For humans: See <a href=".well-known/humans.txt"
class="well-known/humans txt">.well-known/humans.txt</a> for credits and
attribution.

</div>

# Architecture

See <a href="TOPOLOGY.md" class="md">TOPOLOGY</a> for a visual
architecture map and completion dashboard.
