<!--
SPDX-License-Identifier: CC-BY-SA-4.0
Copyright (c) Jonathan D.A. Jewell <j.d.a.jewell@open.ac.uk>
-->
# Security Policy

## Supported Versions

| Version | Supported          | RSR Level |
| ------- | ------------------ | --------- |
| 0.6.x   | :white_check_mark: | Bronze    |
| < 0.6   | :x:                | N/A       |

## Reporting a Vulnerability

**DO NOT** open a public issue for security vulnerabilities.

### Reporting Process

1. **Email**: security@oblibeny.org (GPG key available at https://oblibeny.org/.well-known/security.txt)
2. **Response Time**: 48 hours for acknowledgment, 7 days for initial assessment
3. **Disclosure Timeline**: 90 days coordinated disclosure (negotiable)

### What to Include

- Description of the vulnerability
- Steps to reproduce
- Affected versions
- Potential impact
- Suggested mitigation (if any)

### Security Boundaries

#### Trusted Components
- Rust parser (memory-safe, type-safe)
- Elixir/OTP coordinator (fault-tolerant)
- Lean 4 proofs (formally verified)
- ArangoDB (industry-standard database)

#### Attack Surface
- Network: BOINC protocol (HTTP/HTTPS)
- Database: ArangoDB queries (parameterized, no SQL injection)
- File I/O: Configuration files (validated inputs)
- Volunteer code: Sandboxed execution (resource bounds enforced)

#### Known Limitations
- BOINC volunteers can submit malicious results (mitigated by 2/3 quorum)
- ArangoDB requires authentication (default passwords must be changed)
- Coordinator trusts database (database compromise = full compromise)

## Security Measures

### Input Validation
All user inputs are validated at multiple levels:
1. Type system (Rust/Elixir compile-time)
2. Runtime validation (bounds checking, capability enforcement)
3. Database constraints (schema validation)

### Dependency Security
- Rust: `cargo audit` in CI/CD
- Elixir: `mix audit` for hex dependencies
- Nix: Reproducible builds with content addressing

### Secrets Management
- Never commit secrets to repository
- Use environment variables for production credentials
- Rotate credentials regularly
- Use `.env` files (gitignored) for local development

### BOINC Volunteer Security
- Quorum consensus (2/3 agreement required)
- Volunteer reliability scoring
- Result validation before acceptance
- Resource limits enforced (time, memory, network)

### Formal Verification
Lean 4 proofs ensure:
1. Phase separation soundness (no unsafe code in deployment)
2. Termination guarantees (all deploy code halts)
3. Resource bounds (never exceed budgets)
4. Memory safety (no buffer overflows)

## Security Roadmap

### Current (Bronze RSR)
- [x] Memory safety (Rust ownership)
- [x] Type safety (strong static typing)
- [x] Input validation
- [x] Dependency auditing
- [x] Secrets excluded from repo

### Planned (Silver RSR)
- [ ] Penetration testing
- [ ] Fuzzing (cargo-fuzz, proptest)
- [ ] Static analysis (clippy with security lints)
- [ ] SBOM generation
- [ ] Cryptographic signing of releases

### Future (Gold RSR)
- [ ] Formal security proofs in Lean
- [ ] Hardware security module integration
- [ ] Encrypted database at rest
- [ ] Zero-knowledge proofs for volunteer privacy
- [ ] Capability-based security throughout

## Incident Response

### Process
1. **Detection**: Automated monitoring + manual reports
2. **Triage**: Assess severity (Critical/High/Medium/Low)
3. **Containment**: Isolate affected systems
4. **Remediation**: Patch and deploy fix
5. **Communication**: Notify users (within 48 hours for Critical)
6. **Post-mortem**: Document lessons learned

### Severity Levels

**Critical**: Remote code execution, data breach, complete system compromise
- Response: Immediate (< 4 hours)
- Disclosure: After patch deployed

**High**: Privilege escalation, denial of service, partial data exposure
- Response: < 24 hours
- Disclosure: 7 days after patch

**Medium**: Information disclosure, authenticated attacks
- Response: < 7 days
- Disclosure: 30 days after patch

**Low**: Minor information leaks, theoretical attacks
- Response: < 30 days
- Disclosure: 90 days after patch

## Compliance

### Standards
- RSR Framework: Bronze level minimum
- OWASP Top 10: Addressed in design
- CWE/SANS Top 25: Mitigated via Rust/Elixir safety
- RFC 9116: security.txt implemented

### Audits
- Internal: Quarterly security review
- External: Annual third-party audit (when budget allows)
- Community: Bug bounty program (planned)

## Security Contacts

- General: security@oblibeny.org
- GPG Key: See .well-known/security.txt
- PGP Fingerprint: (To be generated)

## Hall of Fame

Security researchers who responsibly disclose vulnerabilities will be credited here (with permission).

---

Last updated: 2024-11-22
RSR Compliance: Bronze (Security category)
