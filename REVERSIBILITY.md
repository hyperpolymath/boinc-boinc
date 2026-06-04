<!--
SPDX-License-Identifier: MPL-2.0
Copyright (c) Jonathan D.A. Jewell <j.d.a.jewell@open.ac.uk>
-->
# Reversibility

This document explains how all operations in the Oblibeny BOINC Platform are reversible, in accordance with the MPL-2.0 v0.8 and RSR (Rhodium Standard Repository) principles.

## Core Principle

**Every action can be undone.** This eliminates fear of experimentation and enables learning from mistakes without permanent consequences.

## Git-Based Reversibility

### Code Changes

All code changes are tracked in git, making them inherently reversible:

```bash
# Undo last commit (keep changes)
git reset --soft HEAD~1

# Undo last commit (discard changes)
git reset --hard HEAD~1

# Revert a specific commit (creates new commit)
git revert <commit-hash>

# Restore a deleted file
git checkout HEAD~1 -- <file-path>

# Restore entire repository to previous state
git checkout <commit-hash>
```

**Timeframe**: Commits are kept indefinitely in git history.

### Branch Protection

Protected branches (main, release/*) prevent accidental overwrites:
- Require pull request reviews (2 approvals minimum)
- Require status checks to pass
- Require signed commits
- No force-push allowed
- No deletion allowed

### Tag Protection

Release tags are immutable:
- Once created, never deleted
- Signed with PGP keys
- Verifiable with `git tag -v <tag-name>`

## Database Reversibility

### Soft Deletes

All database deletions are "soft" (logical):

```elixir
# Instead of:
Repo.delete(record)

# We use:
Repo.update(changeset(%{deleted_at: DateTime.utc_now()}))
```

**Recovery**: Set `deleted_at` to `null` to restore.

**Timeframe**: Soft-deleted records kept for 90 days, then hard-deleted.

### Change Tracking

Every database modification is logged:

```elixir
# Schema includes:
schema "users" do
  field :name, :string
  field :email, :string
  
  # Audit fields
  field :created_at, :utc_datetime
  field :updated_at, :utc_datetime
  field :created_by, :id
  field :updated_by, :id
  field :deleted_at, :utc_datetime
  field :deleted_by, :id
end
```

**Recovery**: Review audit log, restore previous version.

### Event Sourcing

Critical operations use event sourcing:
- Work unit creation
- Result validation
- Credit assignment
- Governance votes

**Recovery**: Replay events up to desired point in time.

## Infrastructure Reversibility

### Nix-Based Deployment

All infrastructure is defined as code:

```nix
# flake.nix defines entire environment
{
  inputs.nixpkgs.url = "github:NixOS/nixpkgs/nixos-23.11";
  
  outputs = { self, nixpkgs }: {
    # Exact versions, reproducible
  };
}
```

**Recovery**: Rollback to previous flake.lock:

```bash
git checkout HEAD~1 -- flake.lock
nix flake update
```

### Podman Container Rollback

Container images are versioned:

```bash
# Tag current as backup
podman tag oblibeny-coordinator:latest oblibeny-coordinator:backup

# Rollback to previous version
podman-compose down
podman-compose pull --tag v0.5.0
podman-compose up -d
```

**Timeframe**: Images kept for 6 months.

### Database Backups

Automated backups every 6 hours:

```bash
# Restore from backup
pg_restore -d oblibeny_prod backup_2024-11-23_12-00-00.dump
```

**Retention**: 7 days hourly, 30 days daily, 12 months weekly.

## Configuration Reversibility

### Nickel Contracts

Configuration changes are type-checked:

```nickel
{
  database | {
    host | String,
    port | Port,
    ..
  }
}
```

**Recovery**: Invalid configs rejected at deploy time, no manual rollback needed.

### Salt States

Configuration management via SaltStack (temporary, migrating to Nickel):

```bash
# Rollback to previous state
salt '*' state.apply previous_state
```

## User Action Reversibility

### Work Unit Reassignment

If work unit marked invalid:

```elixir
# Reassign to different volunteers
WorkQueue.reassign(work_unit_id, reason: "original results invalid")
```

**Timeframe**: 48 hours to challenge validation.

### Credit Adjustment

If credit incorrectly assigned:

```elixir
# Adjust credit with audit trail
Credit.adjust(user_id, -amount, reason: "validation error corrected")
```

**Timeframe**: 30 days to dispute credit.

### Account Deletion

User account deletion is reversible:

1. Request deletion → Account marked `deleted_at`
2. 30-day grace period → Account hidden but data preserved
3. After 30 days → Hard delete (data exported first)

**Recovery**: Contact conduct@oblibeny.org within 30 days.

## Governance Reversibility

### Decision Reversal

Any governance decision can be reversed:

```yaml
# RFC Process
1. Original decision: "Adopt technology X"
2. New RFC: "Reverse adoption of technology X"
3. Community discussion (14 days minimum)
4. Vote (3/4 Perimeter 1 required)
5. Reversal executed
```

**Timeframe**: No time limit on reversing decisions.

### Maintainer Demotion/Removal

If maintainer removed:

- Automatic archive of their permissions
- Can be reinstated via unanimous Perimeter 1 vote
- All contributions remain attributed

**Timeframe**: No time limit on reinstatement (pending current team consensus).

## Security Incident Reversibility

### Compromised Credentials

If credentials leaked:

1. Rotate immediately (automated)
2. Audit all actions with compromised credentials
3. Revert malicious changes
4. Issue post-mortem
5. Implement prevention

**Timeframe**: Immediate (< 1 hour).

### Malicious Commits

If malicious code merged:

1. Revert commit
2. Force-push to protected branch (requires 2 maintainers)
3. Notify community
4. Security audit
5. Implement safeguards

**Timeframe**: Immediate (< 4 hours).

### Compromised Releases

If release compromised:

1. Revoke PGP signature
2. Delete release artifacts
3. Create new patched release
4. Issue CVE
5. Notify all users

**Timeframe**: < 24 hours.

## Data Export (Maximum Reversibility)

Users can export all their data:

```bash
# Via API
curl -H "Authorization: Bearer <token>" \
  https://api.oblibeny.org/v1/user/export \
  > my-data.json

# Via web interface
Settings → Privacy → Export All Data
```

**Formats**: JSON, CSV, SQL dump

**Includes**:
- User profile
- Contributions (git history)
- Work units and results
- Credit and statistics
- Preferences and settings

## Exceptions (Non-Reversible Operations)

Some operations are inherently non-reversible:

### 1. Cryptographic Signatures

Once a commit is signed, the signature cannot be "unsigned" (but commit can be reverted).

**Mitigation**: Revoke PGP key if compromised.

### 2. Public Disclosures

Once information is publicly disclosed (e.g., security vulnerability), it cannot be "undisclosed".

**Mitigation**: Careful review before disclosure, embargo periods.

### 3. Hard Deletes (After Retention Period)

After retention period (90 days for soft-deleted data), hard deletion is permanent.

**Mitigation**: Clear warnings before deletion, export data first.

### 4. Sent Emails

Emails cannot be unsent (but recipients can be asked to disregard).

**Mitigation**: Email preview, confirmation for bulk sends.

### 5. External Dependencies

Changes to external dependencies (npm, cargo, etc.) cannot be controlled by us.

**Mitigation**: Pin exact versions, vendor critical dependencies.

## Robot Vacuum Cleaner (RVC)

Automated "tidying" preserves reversibility:

```bash
# RVC runs daily via cron
just rvc-tidy

# What it does:
- Formats code (git tracks changes)
- Updates dependencies (creates new commit)
- Regenerates documentation (versioned)
- Runs security scans (reports logged)
```

**All RVC actions are committed to git and thus reversible.**

## Testing Reversibility

### Automated Tests

```bash
# Test reversibility of common operations
just test-reversibility

# Example tests:
- Create user → Delete user → Restore user
- Merge PR → Revert PR → Re-merge PR
- Deploy release → Rollback release → Redeploy
- Add config → Remove config → Restore config
```

### Manual Checklist

- [ ] Can I undo this commit?
- [ ] Can I restore this deleted record?
- [ ] Can I rollback this deployment?
- [ ] Can I revert this configuration change?
- [ ] Can I export my data before deletion?

**If any answer is "no", redesign the feature.**

## Philosophical Foundation

> "The palimpsest is a manuscript page that has been scraped off and used again. The underlying text can often still be read through the newer text."

Like a palimpsest, our system preserves layers of history. Nothing is truly erased, only overlaid. This enables:

1. **Learning**: Mistakes are learning opportunities, not catastrophes
2. **Trust**: Users trust a system that won't trap them
3. **Experimentation**: Innovation requires safety to fail
4. **Accountability**: History shows who did what and why
5. **Resilience**: Systems that can rewind are more robust

## Contact

**Questions**: reversibility@oblibeny.org

**Report non-reversible operation**: governance@oblibeny.org

**Request data recovery**: support@oblibeny.org

## See Also

- [LICENSE.txt](LICENSE.txt) - MPL-2.0 v0.8
- [GOVERNANCE.adoc](GOVERNANCE.adoc) - Governance reversibility
- [SECURITY.md](SECURITY.md) - Security incident reversibility
- [CONTRIBUTING.adoc](CONTRIBUTING.adoc) - Contribution process

---

**SPDX-License-Identifier**: CC0-1.0 (this document is public domain)

**Last Updated**: 2024-11-23

**Version**: 1.0
