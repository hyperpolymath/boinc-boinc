<!--
SPDX-License-Identifier: MPL-2.0
Copyright (c) Jonathan D.A. Jewell <j.d.a.jewell@open.ac.uk>
-->
# Common Technical Context

This document contains shared technical specifications referenced by all task documents.

## Oblibeny Language v0.6 Specification

### Syntax Overview

**S-Expression Based** (Lisp-like):
```lisp
(defun-deploy fibonacci (n)
  (let ((a 0) (b 1) (temp 0))
    (bounded-for i 0 n
      (set temp (+ a b))
      (set a b)
      (set b temp))
    a))
```

### Phase System

#### Compile-Time Phase
**Allowed**:
- Arbitrary computation (Turing-complete)
- I/O (read files, network, etc.)
- Macros and code generation
- Recursion, unbounded loops
- Dynamic memory allocation

**Syntax**:
- `defun-compile` - Compile-time functions
- `eval-compile` - Evaluate at compile time
- `macro` - Define macros
- `include` - Include files
- Standard Lisp-like constructs

#### Deploy-Time Phase
**Restrictions**:
- Must provably terminate
- No recursion (call graph acyclic)
- Only `bounded-for` loops (static bounds)
- No syscalls (only capability-based I/O)
- Static memory bounds

**Syntax**:
- `defun-deploy` - Deploy-time functions
- `bounded-for var start end body` - Bounded iteration
- `with-capability cap body` - Capability-scoped execution
- Limited to terminable constructs

### Type System

**Static Types**:
- `int32`, `int64`, `uint32`, `uint64`
- `float32`, `float64`
- `bool`
- `string` (immutable, bounded length)
- `array[T, N]` (fixed size N, type T)
- `capability[Resource]` (linear types)

**Type Inference**:
- Bidirectional type checking
- Explicit annotations optional but recommended
- Full inference for local bindings

### Resource System

**Resource Budgets** (static analysis):
```lisp
(resource-budget
  (time-ms 1000)
  (memory-bytes 65536)
  (network-bytes 4096))
```

**Capabilities**:
```lisp
(with-capability uart-tx (bytes 100)
  (uart-send device "Hello"))
```

**Linear Capabilities**:
- Must be used exactly once
- Cannot be duplicated
- Compiler enforces via linear type system

### Termination Analysis

**Approaches**:
1. **Bounded loops**: Static upper bounds checked
2. **Call graph**: Must be acyclic (no recursion)
3. **Resource budgets**: Time/memory limits
4. **Ranking functions**: Auto-generated for complex cases

### Memory Model

**Allocation**:
- Stack-only for deploy-time code
- All sizes statically known
- No heap, no GC

**Arrays**:
- Fixed size known at compile time
- Bounds checked (proven safe)
- No dynamic resizing

### Capability Model

**I/O Abstraction**:
```lisp
(defcap uart-tx (device budgeted-bytes)
  "Write capability for UART device")

(defcap sensor-read (device max-samples)
  "Read capability for sensor")
```

**Budget Enforcement**:
- Static analysis verifies budgets not exceeded
- Runtime checks for dynamic data sizes
- Fail-stop behavior on budget violation

### Semantic Obfuscation

**Code Morphing**:
- Each deployment generates different bytecode
- Semantics proven equivalent
- Control flow randomization
- Instruction scheduling variations
- Register allocation randomization

**Preservation**:
- Operational semantics unchanged
- Observational equivalence proven
- Side effects identical
- Resource usage within bounds

### Example Programs

#### Example 1: LED Blinker
```lisp
(program led-blinker
  (resource-budget
    (time-ms 10000)
    (memory-bytes 256))

  (defun-deploy blink (led-cap times)
    (bounded-for i 0 times
      (with-capability led-cap
        (gpio-set led-cap 1)
        (sleep-ms 500)
        (gpio-set led-cap 0)
        (sleep-ms 500)))))
```

#### Example 2: Temperature Monitor
```lisp
(program temp-monitor
  (resource-budget
    (time-ms 60000)
    (memory-bytes 1024)
    (network-bytes 512))

  (defun-deploy monitor (sensor-cap network-cap)
    (let ((readings (array int32 120)))
      (bounded-for i 0 120
        (let ((temp (with-capability sensor-cap
                      (sensor-read sensor-cap))))
          (array-set readings i temp)
          (sleep-ms 500)))
      (with-capability network-cap
        (network-send network-cap readings)))))
```

#### Example 3: Encryption
```lisp
(program simple-encrypt
  (resource-budget
    (time-ms 1000)
    (memory-bytes 2048))

  (defun-deploy xor-encrypt (data key)
    (let ((result (array uint8 256))
          (key-len (array-length key)))
      (bounded-for i 0 (array-length data)
        (let ((key-byte (array-get key (mod i key-len))))
          (array-set result i
            (bitwise-xor (array-get data i) key-byte))))
      result)))
```

## The 7 Properties to Verify

### 1. Phase Separation Soundness
**Statement**: No compile-time construct appears in deploy-time code.

**Formalization**:
```
∀ program P, ∀ expression E ∈ deploy-phase(P),
  compile-only-construct(E) → False
```

**Verification Strategy**:
- Static AST analysis
- Type system enforcement
- Formal proof in Lean

### 2. Deployment Termination
**Statement**: All deploy-time code provably terminates.

**Formalization**:
```
∀ program P, ∀ input I,
  ∃ n ∈ ℕ, eval(deploy-phase(P), I, n) = value
```

**Verification Strategy**:
- Bounded loop analysis
- Call graph acyclicity
- Ranking functions
- Resource budget limits

### 3. Resource Bounds Enforcement
**Statement**: Resource usage never exceeds declared budgets.

**Formalization**:
```
∀ program P with budget B, ∀ execution trace T,
  resources(T) ≤ B
```

**Verification Strategy**:
- Static resource analysis
- WCET (Worst-Case Execution Time) analysis
- Memory usage analysis
- Formal proof of bounds

### 4. Capability System Soundness
**Statement**: I/O operations only succeed within capability scope and budget.

**Formalization**:
```
∀ I/O operation Op, ∀ execution E,
  success(Op, E) → ∃ capability C ∈ scope(E), allows(C, Op)
```

**Verification Strategy**:
- Linear type system
- Effect tracking
- Budget consumption analysis

### 5. Obfuscation Semantic Preservation
**Statement**: Code morphing preserves program semantics.

**Formalization**:
```
∀ program P, ∀ morphed variant P',
  ∀ input I, eval(P, I) ≈ eval(P', I)
```
(where ≈ is observational equivalence)

**Verification Strategy**:
- Translation validation
- Bisimulation proofs
- Property-based testing

### 6. Call Graph Acyclicity
**Statement**: No recursion in deploy-time code.

**Formalization**:
```
∀ program P, call-graph(deploy-phase(P)) is acyclic
```

**Verification Strategy**:
- Static call graph construction
- Cycle detection
- Topological sort

### 7. Memory Safety
**Statement**: All memory accesses within bounds.

**Formalization**:
```
∀ array access A[i] in execution E,
  0 ≤ i < length(A)
```

**Verification Strategy**:
- Bounds check insertion
- Static array bounds analysis
- Proof of access safety

## BOINC Work Unit Design

### Work Unit Structure
```json
{
  "unit_id": "uuid",
  "type": "property_test",
  "property_id": 1-7,
  "program": "base64-encoded-oblibeny-source",
  "test_vector": {
    "inputs": [...],
    "expected_properties": [...]
  },
  "timeout_seconds": 300,
  "memory_limit_mb": 512
}
```

### Result Structure
```json
{
  "unit_id": "uuid",
  "volunteer_id": "uuid",
  "status": "success|failure|timeout",
  "properties_verified": [1, 2, 3],
  "counterexample": null | {...},
  "execution_time_ms": 1234,
  "memory_used_bytes": 45678,
  "hash": "sha256-of-results"
}
```

### Validation Logic (Ada)
```ada
procedure Validate_Result(
  Unit : Work_Unit;
  Result : Work_Result;
  Is_Valid : out Boolean
) is
begin
  -- Check timeout
  if Result.Execution_Time_Ms > Unit.Timeout_Seconds * 1000 then
    Is_Valid := False;
    return;
  end if;

  -- Check memory
  if Result.Memory_Used_Bytes > Unit.Memory_Limit_Mb * 1048576 then
    Is_Valid := False;
    return;
  end if;

  -- Verify hash
  if not Verify_Hash(Result) then
    Is_Valid := False;
    return;
  end if;

  -- Check property verification results
  Is_Valid := Validate_Property_Results(Unit, Result);
end Validate_Result;
```

## ArangoDB Schema Details

### Document Collections

#### `programs`
```json
{
  "_key": "program-uuid",
  "source": "oblibeny source code",
  "phase": "compile|deploy",
  "resource_budget": {
    "time_ms": 1000,
    "memory_bytes": 65536,
    "network_bytes": 0
  },
  "properties_target": [1, 2, 3, 4, 5, 6, 7],
  "created_at": "ISO8601 timestamp",
  "status": "pending|testing|verified|counterexample"
}
```

#### `work_units`
```json
{
  "_key": "unit-uuid",
  "program_id": "program-uuid",
  "property_id": 1-7,
  "status": "pending|assigned|completed|validated",
  "redundancy": 3,
  "results": ["result-uuid-1", "result-uuid-2"],
  "created_at": "timestamp",
  "assigned_to": ["volunteer-uuid-1", "volunteer-uuid-2"]
}
```

#### `results`
```json
{
  "_key": "result-uuid",
  "work_unit_id": "unit-uuid",
  "volunteer_id": "volunteer-uuid",
  "status": "success|failure|timeout|error",
  "properties_verified": [1, 2],
  "counterexample": null,
  "execution_metrics": {
    "time_ms": 123,
    "memory_bytes": 4096
  },
  "validation_status": "pending|valid|invalid",
  "submitted_at": "timestamp"
}
```

#### `proofs`
```json
{
  "_key": "proof-uuid",
  "property_id": 1-7,
  "status": "in_progress|complete|failed",
  "lean_code": "Lean 4 source",
  "dependencies": ["lemma-uuid-1", "lemma-uuid-2"],
  "coverage": {
    "test_cases": 123456,
    "programs_tested": 5678
  },
  "last_updated": "timestamp"
}
```

#### `properties`
```json
{
  "_key": "1-7",
  "name": "Phase Separation Soundness",
  "description": "...",
  "formalization": "Lean 4 statement",
  "verification_strategy": "...",
  "status": "unproven|partial|proven|counterexample",
  "proof_id": "proof-uuid"
}
```

#### `volunteers`
```json
{
  "_key": "volunteer-uuid",
  "boinc_id": "external-id",
  "total_work_units": 12345,
  "valid_results": 12000,
  "invalid_results": 45,
  "reliability_score": 0.996,
  "joined_at": "timestamp",
  "last_active": "timestamp"
}
```

### Graph Collections

#### `proof_dependencies` (edge collection)
```json
{
  "_from": "proofs/proof-uuid",
  "_to": "proofs/lemma-uuid",
  "dependency_type": "requires|uses|extends"
}
```

#### `program_variants` (edge collection)
```json
{
  "_from": "programs/original-uuid",
  "_to": "programs/morphed-uuid",
  "morphing_strategy": "control_flow|instruction_scheduling",
  "semantic_equivalence": "proven|assumed|testing"
}
```

#### `property_coverage` (edge collection)
```json
{
  "_from": "programs/program-uuid",
  "_to": "properties/property-id",
  "coverage_status": "tests|verifies|violates",
  "evidence": ["result-uuid-1", "result-uuid-2"]
}
```

### AQL Queries (Examples)

**Find unreliable volunteers**:
```aql
FOR v IN volunteers
  FILTER v.reliability_score < 0.9
  SORT v.reliability_score ASC
  RETURN {
    id: v._key,
    score: v.reliability_score,
    valid: v.valid_results,
    invalid: v.invalid_results
  }
```

**Get proof dependency tree**:
```aql
FOR v, e, p IN 1..10 OUTBOUND 'proofs/main-theorem'
  GRAPH 'proof_dependencies'
  RETURN p
```

**Find programs testing specific property**:
```aql
FOR p IN programs
  FOR prop IN OUTBOUND p property_coverage
    FILTER prop._key == '1'
    RETURN p
```

## Development Environment

### Nix Flake Structure
```nix
{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, rust-overlay, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system:
      # ... package definitions
    );
}
```

### Nickel Configuration Schema
```nickel
{
  Server = {
    host | String,
    port | Number,
    database | {
      url | String,
      name | String,
      auth | {
        username | String,
        password | String,
      },
    },
    boinc | {
      project_name | String,
      project_url | String,
      work_units | {
        redundancy | Number,
        timeout_seconds | Number,
      },
    },
  },
}
```

## Testing Strategy

### Unit Tests
- **Rust Parser**: Parse all example programs, verify AST structure
- **Elixir Coordinator**: Test work unit generation, result aggregation
- **Lean Proofs**: Each lemma has test cases

### Integration Tests
- End-to-end work unit flow
- Database consistency checks
- BOINC integration

### Property-Based Testing
- Generate random valid Oblibeny programs
- Verify parser roundtrip
- Test semantic preservation

### Benchmark Suite
- Parser performance (target: <100ms for typical programs)
- Work unit generation rate
- Result validation throughput

## Deployment Architecture

```
┌─────────────────────────────────────────────────────────┐
│ VPS (Ubuntu/NixOS)                                      │
│ ┌─────────────────────────────────────────────────────┐ │
│ │ Podman                                              │ │
│ │ ┌─────────────┐ ┌──────────────┐ ┌──────────────┐ │ │
│ │ │ Phoenix     │ │ Elixir/OTP   │ │ ArangoDB     │ │ │
│ │ │ (Port 4000) │ │ (Internal)   │ │ (Port 8529)  │ │ │
│ │ └─────────────┘ └──────────────┘ └──────────────┘ │ │
│ │ ┌─────────────────────────────────────────────────┐ │ │
│ │ │ BOINC Server (Ports 80/443)                     │ │ │
│ │ └─────────────────────────────────────────────────┘ │ │
│ └─────────────────────────────────────────────────────┘ │
└─────────────────────────────────────────────────────────┘
```

## Security Considerations

### Code Signing
- All work units signed by server
- Volunteers verify signatures
- Prevents tampering

### Result Validation
- Multiple redundancy (3x default)
- Quorum consensus (2/3)
- Statistical outlier detection
- Blacklist unreliable volunteers

### Network Security
- HTTPS for all communication
- TLS 1.3 minimum
- Certificate pinning

### Database Security
- Authentication required
- Role-based access control
- Audit logging
- Encrypted backups

## Performance Targets

- **Parser**: < 100ms for 1000-line programs
- **Work Generation**: 1000 units/second
- **Result Validation**: 500 results/second
- **Database Queries**: < 50ms for complex queries
- **Dashboard Updates**: Real-time (< 1s latency)
- **BOINC Server**: 10,000 concurrent volunteers

## Monitoring & Observability

### Metrics (Prometheus)
- Work units generated/second
- Results validated/second
- Property verification progress
- Volunteer count/reliability
- Database performance

### Logging (Structured)
- All components use JSON logging
- Centralized via syslog
- Log levels: DEBUG, INFO, WARN, ERROR

### Alerts
- BOINC server down
- Database connection loss
- Validator crashes
- Abnormal result patterns
- Proof assistant errors

## Git Workflow

- **Main Branch**: `main` (protected)
- **Feature Branches**: `feature/<name>`
- **Release Branches**: `release/v<version>`
- **Hotfix Branches**: `hotfix/<issue>`

**Commit Message Format**:
```
<type>(<scope>): <subject>

<body>

<footer>
```

Types: `feat`, `fix`, `docs`, `style`, `refactor`, `test`, `chore`

## License

To be determined (likely GPL or AGPL for copyleft + network provision).

## Contributors

- Initial development: Autonomous Claude session (2024)
- Project conception: [User]

---

**Note**: This is a living document. Update as the project evolves.
