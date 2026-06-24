<!--
SPDX-License-Identifier: CC-BY-SA-4.0
Copyright (c) Jonathan D.A. Jewell <j.d.a.jewell@open.ac.uk>
-->
# Maintainers

This document lists the current maintainers of the Oblibeny BOINC Platform.

## TPCF Perimeter Assignment

**Current Project Status**: Perimeter 3 (Community Sandbox)
- Anyone can contribute via fork + pull request
- No special permissions required
- All contributions welcome

## Core Maintainers (Perimeter 1)

### Project Lead
- **Name**: (To be assigned)
- **Role**: Overall project direction, final decisions on architecture
- **GPG Key**: (To be added)
- **Availability**: (To be specified)

### Component Maintainers

#### Rust Parser
- **Maintainer**: (Open position)
- **Responsibilities**: Parser, AST, phase separation, resource analysis
- **Contact**: (To be added)

#### Elixir/OTP Coordinator
- **Maintainer**: (Open position)
- **Responsibilities**: Work generation, result validation, proof tracking
- **Contact**: (To be added)

#### Lean 4 Formal Proofs
- **Maintainer**: (Open position)
- **Responsibilities**: Formal verification, theorem proving
- **Contact**: (To be added)

#### Deployment & Infrastructure
- **Maintainer**: (Open position)
- **Responsibilities**: Nix, Docker, CI/CD, production deployment
- **Contact**: (To be added)

## Trusted Contributors (Perimeter 2)

*List will be populated as contributors are promoted from Perimeter 3*

Criteria for promotion:
- 10+ merged contributions
- 6+ months of consistent participation
- Demonstrated understanding of project architecture
- Adherence to Code of Conduct
- Community trust vote (2/3 majority of Perimeter 1)

## Becoming a Maintainer

### Path to Perimeter 1 (Core Maintainer)

1. **Start in Perimeter 3** (Community Sandbox)
   - Fork repository
   - Submit pull requests
   - Participate in discussions

2. **Progress to Perimeter 2** (Trusted Contributor)
   - Demonstrate reliability (10+ merged PRs)
   - Show deep understanding of component
   - Help review others' contributions
   - Be nominated by Perimeter 1 maintainer

3. **Advance to Perimeter 1** (Core Maintainer)
   - 6+ months in Perimeter 2
   - Consistent high-quality contributions
   - Community building activities
   - Invitation by existing Perimeter 1 maintainers
   - GPG key setup for signed commits

### Responsibilities of Maintainers

#### All Maintainers
- Uphold Code of Conduct
- Review pull requests within 7 days
- Participate in architectural decisions
- Mentor new contributors
- Maintain emotional safety in community

#### Perimeter 1 (Core Maintainers) Additionally
- Sign releases with GPG
- Have write access to `main` branch
- Make final decisions in their component area
- Participate in security incident response
- Quarterly security reviews

#### Perimeter 2 (Trusted Contributors) Additionally
- Can approve pull requests
- Write access to feature branches
- Participate in design discussions
- Help with triage and community support

## Decision Making

### Consensus Model
- **Small decisions**: Component maintainer decides
- **Medium decisions**: Affected maintainers reach consensus
- **Large decisions**: All Perimeter 1 maintainers vote (2/3 majority)
- **Deadlock**: Project lead has tie-breaking vote

### What Requires Consensus?
- Adding new dependencies
- Changing build system
- Modifying license
- Adding new perimeter rules
- Promoting contributors between perimeters

### Emergency Decisions
- Security issues: Any Perimeter 1 can act, notify others within 24h
- Downtime: On-call rotation handles immediately
- Code of Conduct violations: Any maintainer can take action

## Stepping Down

Maintainers can step down at any time:
1. Notify other maintainers
2. Transfer responsibilities (2 week transition period)
3. Remain in emeritus status (can return)
4. Update MAINTAINERS.md

## Inactive Maintainers

If a maintainer is unresponsive for:
- **30 days**: Gentle ping to check status
- **60 days**: Redistribute review load to other maintainers
- **90 days**: Move to inactive status, remove write access
- Can return anytime by notifying team

## Contact

- **General**: maintainers@oblibeny.org
- **Security**: security@oblibeny.org
- **Code of Conduct**: conduct@oblibeny.org

## Acknowledgments

This maintainer structure is inspired by:
- Rust project governance
- Kubernetes SIG structure
- Apache Software Foundation
- Tri-Perimeter Contribution Framework (TPCF)

---

**Last Updated**: 2024-11-22
**RSR Compliance**: Bronze (Community category)
**TPCF Perimeter**: 3 (Community Sandbox)
