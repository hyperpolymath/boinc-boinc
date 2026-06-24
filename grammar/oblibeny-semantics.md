<!--
SPDX-License-Identifier: CC-BY-SA-4.0
Copyright (c) Jonathan D.A. Jewell <j.d.a.jewell@open.ac.uk>
-->
# Oblibeny v0.6 Formal Semantics

## Table of Contents
1. Syntax
2. Operational Semantics
3. Type System
4. Phase System
5. Resource Semantics
6. Capability System
7. Termination Properties

---

## 1. Syntax

### Abstract Syntax

```
e ::= n                          (integer)
    | f                          (float)
    | b                          (boolean)
    | s                          (string)
    | x                          (variable)
    | (e₁ e₂ ... eₙ)            (application)
    | (defun-deploy f (x₁...xₙ) : τ e₁...eₘ)
    | (defun-compile f (x₁...xₙ) : τ e₁...eₘ)
    | (bounded-for x e₁ e₂ e₃...eₙ)
    | (with-capability cap e₁...eₙ)
    | (let ((x₁ e₁)...(xₙ eₙ)) e)
    | (if e₁ e₂ e₃)
    | (set x e)
```

### Values

```
v ::= n | f | b | s              (literals)
    | (closure (λ (x₁...xₙ) e) ρ)  (function closure)
    | (array [v₁...vₙ])          (array value)
    | (capability res budget)    (capability value)
```

---

## 2. Operational Semantics

### Evaluation Contexts

```
E ::= []
    | (E e₁ ... eₙ)
    | (v E e₁ ... eₙ)
    | (if E e₁ e₂)
    | (bounded-for x E e₁ e₂...eₙ)
    | (let ((x₁ v₁)...(xᵢ₋₁ vᵢ₋₁) (xᵢ E) ...) e)
```

### Small-Step Semantics

**Configuration**: `⟨e, ρ, σ, R⟩`
- `e`: expression
- `ρ`: environment (variable bindings)
- `σ`: store (mutable state)
- `R`: resource budget remaining

**Transition relation**: `⟨e, ρ, σ, R⟩ → ⟨e', ρ', σ', R'⟩`

#### Function Application

```
⟨((closure (λ (x₁...xₙ) e) ρ_c) v₁ ... vₙ), ρ, σ, R⟩
  → ⟨e, ρ_c[x₁↦v₁,...,xₙ↦vₙ], σ, R - cost(app)⟩
```

#### Bounded For Loop

```
⟨(bounded-for x n₁ n₂ e₁...eₘ), ρ, σ, R⟩
  → ⟨(bounded-for-aux x n₁ n₂ (n₂ - n₁) e₁...eₘ), ρ, σ, R⟩

⟨(bounded-for-aux x i end 0 e₁...eₘ), ρ, σ, R⟩
  → ⟨void, ρ, σ, R⟩

⟨(bounded-for-aux x i end (k+1) e₁...eₘ), ρ, σ, R⟩
  → ⟨e₁...eₘ; (bounded-for-aux x (i+1) end k e₁...eₘ), ρ[x↦i], σ, R - cost(iter)⟩
```

#### Let Binding

```
⟨(let ((x₁ v₁)...(xₙ vₙ)) e), ρ, σ, R⟩
  → ⟨e, ρ[x₁↦v₁,...,xₙ↦vₙ], σ, R⟩
```

#### Conditional

```
⟨(if true e₁ e₂), ρ, σ, R⟩ → ⟨e₁, ρ, σ, R⟩
⟨(if false e₁ e₂), ρ, σ, R⟩ → ⟨e₂, ρ, σ, R⟩
```

#### Variable Assignment

```
⟨(set x v), ρ, σ, R⟩ → ⟨void, ρ, σ[x↦v], R⟩
```

#### Capability Usage

```
⟨(with-capability (capability res budget) e₁...eₙ), ρ, σ, R⟩
  → ⟨e₁...eₙ, ρ[cap_active↦(res,budget)], σ, R⟩
```

#### Arithmetic

```
⟨(+ n₁ n₂), ρ, σ, R⟩ → ⟨n₁+n₂, ρ, σ, R - cost(add)⟩
⟨(- n₁ n₂), ρ, σ, R⟩ → ⟨n₁-n₂, ρ, σ, R - cost(sub)⟩
⟨(* n₁ n₂), ρ, σ, R⟩ → ⟨n₁*n₂, ρ, σ, R - cost(mul)⟩
⟨(/ n₁ n₂), ρ, σ, R⟩ → ⟨n₁/n₂, ρ, σ, R - cost(div)⟩  (if n₂ ≠ 0)
```

#### Array Operations

```
⟨(array-get (array [v₀...vₙ]) i), ρ, σ, R⟩
  → ⟨vᵢ, ρ, σ, R - cost(array-access)⟩  (if 0 ≤ i < n)

⟨(array-set (array [v₀...vₙ]) i v), ρ, σ, R⟩
  → ⟨(array [v₀...vᵢ₋₁,v,vᵢ₊₁...vₙ]), ρ, σ, R - cost(array-access)⟩  (if 0 ≤ i < n)
```

#### I/O Operations (Capability-Gated)

```
⟨(gpio-set dev val), ρ[cap_active↦(gpio,b)], σ, R⟩
  → ⟨void, ρ[cap_active↦(gpio,b-1)], σ', R - cost(gpio)⟩
  where σ' performs actual I/O, if b > 0

⟨(sensor-read dev), ρ[cap_active↦(sensor-read,b)], σ, R⟩
  → ⟨v, ρ[cap_active↦(sensor-read,b-1)], σ', R - cost(sensor)⟩
  where v is sensor reading, if b > 0
```

---

## 3. Type System

### Types

```
τ ::= int32 | int64 | uint32 | uint64
    | float32 | float64
    | bool | string
    | (array τ n)
    | (capability res)
    | (→ τ₁ ... τₙ τ)
```

### Typing Judgments

**Form**: `Γ ⊢ e : τ`

where `Γ` is a typing context mapping variables to types.

#### Rules

**T-Int**:
```
─────────────
Γ ⊢ n : int32
```

**T-Bool**:
```
─────────────
Γ ⊢ b : bool
```

**T-Var**:
```
x : τ ∈ Γ
─────────
Γ ⊢ x : τ
```

**T-App**:
```
Γ ⊢ e₀ : (→ τ₁ ... τₙ τ)    Γ ⊢ e₁ : τ₁    ...    Γ ⊢ eₙ : τₙ
───────────────────────────────────────────────────────────────
Γ ⊢ (e₀ e₁ ... eₙ) : τ
```

**T-Defun-Deploy**:
```
Γ, x₁:τ₁, ..., xₙ:τₙ ⊢ e₁ : τ'    ...    Γ, x₁:τ₁, ..., xₙ:τₙ ⊢ eₘ : τ
no_compile_constructs(e₁,...,eₘ)
────────────────────────────────────────────────────────────────────────────
Γ ⊢ (defun-deploy f (x₁:τ₁...xₙ:τₙ) : τ e₁...eₘ) : (→ τ₁...τₙ τ)
```

**T-Bounded-For**:
```
Γ ⊢ e₁ : int32    Γ ⊢ e₂ : int32    Γ, x:int32 ⊢ e₃ : τ    ...    Γ, x:int32 ⊢ eₙ : τ
───────────────────────────────────────────────────────────────────────────────────────
Γ ⊢ (bounded-for x e₁ e₂ e₃...eₙ) : void
```

**T-With-Capability**:
```
Γ ⊢ e_cap : (capability res)    Γ, cap_active:(capability res) ⊢ e₁ : τ₁    ...
────────────────────────────────────────────────────────────────────────────────────
Γ ⊢ (with-capability e_cap e₁...eₙ) : τₙ
```

**T-Let**:
```
Γ ⊢ e₁ : τ₁    ...    Γ ⊢ eₙ : τₙ    Γ, x₁:τ₁, ..., xₙ:τₙ ⊢ e : τ
───────────────────────────────────────────────────────────────────
Γ ⊢ (let ((x₁ e₁)...(xₙ eₙ)) e) : τ
```

**T-If**:
```
Γ ⊢ e₁ : bool    Γ ⊢ e₂ : τ    Γ ⊢ e₃ : τ
──────────────────────────────────────────
Γ ⊢ (if e₁ e₂ e₃) : τ
```

**T-Array-Get**:
```
Γ ⊢ e₁ : (array τ n)    Γ ⊢ e₂ : int32
──────────────────────────────────────────
Γ ⊢ (array-get e₁ e₂) : τ
```

---

## 4. Phase System

### Phase Assignment

Every expression is assigned a phase: `Compile` or `Deploy`.

**Definition**: `phase(e) = Compile | Deploy`

**Rules**:

1. `phase(defun-compile ...) = Compile`
2. `phase(defun-deploy ...) = Deploy`
3. `phase(macro ...) = Compile`
4. `phase(eval-compile ...) = Compile`
5. `phase(for ...) = Compile` (unbounded loop)
6. `phase(while ...) = Compile` (unbounded loop)
7. `phase(bounded-for ...) = Deploy`
8. For other expressions, phase is inferred from context

### Phase Separation Property

**Invariant**: In any `defun-deploy` function body, all subexpressions must be compatible with `Deploy` phase.

**Formally**:
```
∀ e ∈ body(defun-deploy),
  phase(e) ≠ Compile
```

**Compile-only constructs**:
- `defun-compile`
- `macro`
- `eval-compile`
- `for` (unbounded)
- `while` (unbounded)
- File I/O
- Network I/O at compile time
- Dynamic code generation

---

## 5. Resource Semantics

### Resource Budget

Each program declares a resource budget:

```
(resource-budget
  (time-ms T)
  (memory-bytes M)
  (network-bytes N)
  (storage-bytes S))
```

### Resource Tracking

**Cost Function**: `cost(operation) → ℕ`

Examples:
- `cost(add) = 1` (arbitrary unit)
- `cost(mul) = 2`
- `cost(div) = 10`
- `cost(array-access) = 1`
- `cost(gpio) = 100`
- `cost(sensor) = 500`

### Resource Consumption Rules

```
⟨e, ρ, σ, R⟩ → ⟨e', ρ', σ', R'⟩
where R' = R - cost(operation in e → e')
```

**Resource exhaustion**:
```
⟨e, ρ, σ, 0⟩ → error("Resource budget exceeded")
```

### Static Resource Analysis

For deploy-time code, we compute worst-case resource usage:

```
WCET(n) = 1  (for literals)
WCET(x) = 1  (for variables)
WCET((+ e₁ e₂)) = WCET(e₁) + WCET(e₂) + cost(add)
WCET((bounded-for x e₁ e₂ e₃...eₙ)) =
  (eval_const(e₂) - eval_const(e₁)) * (WCET(e₃) + ... + WCET(eₙ))
WCET((if e₁ e₂ e₃)) = WCET(e₁) + max(WCET(e₂), WCET(e₃))
```

**Property**: For any deployment program `P` with budget `B`:
```
∀ input I, resources_used(eval(P, I)) ≤ WCET(P) ≤ B
```

---

## 6. Capability System

### Capability Values

A capability is a linear resource granting permission for I/O:

```
(capability resource budget)
```

Examples:
- `(capability gpio-tx 100)` - Can perform 100 GPIO writes
- `(capability sensor-read 10)` - Can read sensor 10 times
- `(capability network-send 4096)` - Can send 4096 bytes

### Linear Type Rules

Capabilities have linear types - they must be used exactly once:

**L-Capability**:
```
Γ ⊢ e_cap : (capability res)    used_exactly_once(e_cap)
──────────────────────────────────────────────────────────
Γ ⊢ (with-capability e_cap e₁...eₙ) : τ
```

### Budget Enforcement

Within a `with-capability` block, each I/O operation decrements the budget:

```
budget_remaining = initial_budget - Σ(operations_performed)
```

**Property**: All I/O operations must occur within capability scope with sufficient budget.

```
∀ I/O operation Op in program P,
  ∃ capability C in scope(Op),
    budget(C) ≥ cost(Op)
```

---

## 7. Termination Properties

### Bounded Loops

All deploy-time loops must be `bounded-for` with statically known bounds.

**Syntax**:
```
(bounded-for x start end body...)
```

**Constraint**:
```
start, end must be compile-time constants or
statically analyzable expressions
```

**Termination**: A `bounded-for` loop always terminates in exactly `(end - start)` iterations.

### Call Graph Acyclicity

The call graph of all deploy-time functions must be acyclic (no recursion).

**Definition**:
```
call_graph(P) = (V, E)
where V = {all deploy functions in P}
      E = {(f, g) | f calls g}
```

**Property**:
```
call_graph(P) is a DAG (Directed Acyclic Graph)
```

**Algorithm**: Topological sort succeeds iff graph is acyclic.

### Termination Guarantee

**Theorem**: All deploy-time programs terminate.

**Proof Sketch**:
1. All loops are bounded → each loop terminates
2. Call graph is acyclic → functions terminate by induction on topological order
3. Resource budget is finite → program terminates when budget exhausted (if not earlier)

**Formally**:
```
∀ deploy program P, ∀ input I,
  ∃ n ∈ ℕ, ⟨P, ∅, ∅, budget(P)⟩ →ⁿ ⟨v, ρ, σ, R⟩
  where v is a value (normal termination)
     or R = 0 (resource exhaustion termination)
```

---

## 8. Semantic Obfuscation

### Code Morphing

Each deployment generates a semantically equivalent but structurally different program.

**Transformations**:
1. **Control flow randomization**: Reorder independent statements
2. **Instruction scheduling**: Vary instruction order
3. **Register allocation**: Use different register assignments
4. **Constant folding variations**: Precompute different subexpressions

### Preservation Property

**Definition**: Observational equivalence

```
P₁ ≈ P₂  iff  ∀ I, observable(eval(P₁, I)) = observable(eval(P₂, I))
```

where `observable` includes:
- Return value
- I/O effects
- Resource consumption (within bounds)

**Theorem**: For original program `P` and morphed variant `P'`:
```
morph(P) = P'  →  P ≈ P'
```

---

## 9. Memory Safety

### Array Bounds

All array accesses must be within bounds.

**Static Analysis**:
```
For (array-get arr i):
  if can_prove(0 ≤ i < length(arr)) then
    ✓ statically safe
  else
    insert runtime check
```

**Runtime Check**:
```
⟨(array-get (array [v₀...vₙ]) i), ρ, σ, R⟩
  → error("Array index out of bounds")  if i < 0 or i ≥ n
  → ⟨vᵢ, ρ, σ, R⟩                        otherwise
```

### Memory Layout

All memory is stack-allocated with static sizes:

```
memory_layout(P) = {
  stack: [
    (var₁, size₁, alignment₁),
    (var₂, size₂, alignment₂),
    ...
  ]
}
```

**Property**: Total memory usage is statically bounded.

```
total_memory(P) = Σ size_i ≤ memory_budget(P)
```

---

## 10. Formal Guarantees Summary

1. **Type Safety**: Well-typed programs don't get stuck
2. **Phase Separation**: No compile-time constructs in deploy code
3. **Termination**: All deploy programs halt
4. **Resource Bounds**: Resource usage never exceeds budget
5. **Capability Soundness**: All I/O gated by capabilities
6. **Memory Safety**: No buffer overflows or out-of-bounds access
7. **Semantic Preservation**: Code morphing preserves semantics

---

*End of Formal Semantics*
