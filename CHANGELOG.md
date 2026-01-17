# Changelog

All notable changes to the Oblibeny BOINC Platform will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Planned
- Phoenix LiveView dashboard
- Nickel configuration system
- Ada BOINC validator
- Complete Lean 4 proofs (properties 4-7)
- Comprehensive test suites (unit + integration)
- Performance benchmarks
- First BOINC deployment

## [0.6.0] - 2024-11-22

### Added - Infrastructure Foundation

#### Rust Parser & Analyzer
- pest-based PEG parser for Oblibeny v0.6 syntax
- Complete AST with typed expressions and pretty printing
- Phase separation validator (compile vs deploy time)
- Termination checker with bounded loop analysis
- Call graph builder with cycle detection
- Resource bounds analyzer (WCET computation)
- CLI tool with 6 subcommands (parse, analyze, check-phases, etc.)
- 15 Rust modules (~2,000 LOC)

#### Elixir/OTP Coordinator
- Fault-tolerant OTP supervision tree
- Work unit generator for all 7 verification properties
- Byzantine fault-tolerant result validator (2/3 quorum consensus)
- Volunteer reliability scoring system
- Proof progress tracker with coverage analysis
- ArangoDB integration with connection pooling
- Telemetry and monitoring infrastructure
- 10 Elixir modules (~1,500 LOC)

#### Lean 4 Formal Proofs
- Complete syntax formalization in Lean 4
- Small-step operational semantics
- Type system formalization
- Proof scaffolding for properties 1-3:
  - Phase separation soundness
  - Deployment termination
  - Resource bounds enforcement
- Infrastructure for properties 4-7
- 7 Lean modules (~800 LOC)

#### Deployment Infrastructure
- Nix flake with reproducible builds for all components
- Docker Compose with 5 services:
  - ArangoDB (multi-model database)
  - Elixir coordinator
  - BOINC server
  - Prometheus (metrics)
  - Grafana (dashboards)
- ArangoDB schema: 9 collections + 3 graphs
- Production deployment scripts (Bash)
- Development environment setup

#### Documentation
- Master orchestration document (architecture overview)
- 6 detailed task handover documents (Tasks 1-7)
- Oblibeny grammar in EBNF (586 lines)
- Formal semantics specification (678 lines)
- README with quick start guide
- Contributing guidelines with TPCF
- Handover document for new developers
- ~10,000 lines of documentation

#### Examples & Configuration
- 3 complete Oblibeny example programs:
  - LED blinker with SOS pattern
  - Temperature monitoring system
  - Multi-round XOR encryption
- GitLab CI/CD pipeline configuration
- Nix development shells
- Environment-specific configs

### RSR Compliance Added
- LICENSE.txt (dual MIT + Palimpsest v0.8)
- SECURITY.md (vulnerability reporting, security measures)
- CODE_OF_CONDUCT.md (Contributor Covenant + CCCP)
- MAINTAINERS.md (TPCF perimeter structure)
- CHANGELOG.md (this file)
- .well-known/security.txt (RFC 9116 compliance)
- .well-known/ai.txt (AI training policies)
- .well-known/humans.txt (attribution)
- justfile (build automation with 20+ recipes)
- RSR self-verification script

### Technical Decisions
- Rust for parser: Performance + memory safety + pest ecosystem
- Elixir/OTP for coordinator: Built for distributed fault tolerance
- Lean 4 over Coq: Modern tooling, active community
- ArangoDB over Neo4j+Postgres: Single multi-model database
- Nix for builds: Reproducible across platforms
- Self-hosted BOINC: Full control, no external dependencies

### Known Limitations
- Test coverage: 0% (infrastructure ready, tests not written)
- Lean proofs: Scaffolding only, `sorry` placeholders
- Properties 4-7: Not yet formalized in Lean
- Phoenix dashboard: Not implemented
- Nickel config: Not implemented
- Ada validator: Not implemented

### Security
- Memory safety via Rust ownership model
- Type safety via strong static typing (Rust/Elixir/Lean)
- Input validation at multiple levels
- Dependency auditing in CI/CD (cargo audit, mix audit)
- Byzantine fault tolerance (2/3 quorum for results)
- Resource bounds enforcement (prevents DoS)

### Performance Targets Established
- Parser: < 100ms for 1000-line programs
- Work generation: 1000 units/second
- Result validation: 500 results/second
- BOINC server: 10,000 concurrent volunteers

## [0.5.0] - 2024-11 (Pre-release Planning)

### Added
- Initial project conception
- Grammar design discussions
- BOINC feasibility analysis
- Property identification (the 7 key properties)

---

## Version Numbering

We use Semantic Versioning (SemVer):
- **MAJOR**: Incompatible API changes, grammar breaking changes
- **MINOR**: New features, backward-compatible additions
- **PATCH**: Bug fixes, documentation updates

## Release Process

1. Update CHANGELOG.md with all changes
2. Update version in all Cargo.toml, mix.exs, etc.
3. Run full test suite (when implemented)
4. Tag release: `git tag v0.6.0`
5. Push tag: `git push --tags`
6. CI/CD builds and publishes artifacts
7. Announce on mailing list / forum

## Links

- [Unreleased]: https://gitlab.com/oblibeny/boinc/compare/v0.6.0...HEAD
- [0.6.0]: https://gitlab.com/oblibeny/boinc/releases/v0.6.0

---

**RSR Compliance**: Bronze (Documentation category)
**Maintained by**: See MAINTAINERS.md
