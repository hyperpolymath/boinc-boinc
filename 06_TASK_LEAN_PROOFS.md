<!--
SPDX-License-Identifier: MPL-2.0
Copyright (c) Jonathan D.A. Jewell <j.d.a.jewell@open.ac.uk>
-->
# Task 5: Lean 4 Formal Proofs

## Objective
Formalize Oblibeny semantics and prove the 7 key properties in Lean 4.

## Deliverables

### 1. Syntax Formalization
```lean
inductive Expr where
  | int : Int → Expr
  | bool : Bool → Expr
  | var : String → Expr
  | boundedFor : String → Expr → Expr → List Expr → Expr
  | defunDeploy : String → List String → List Expr → Expr
  | app : Expr → List Expr → Expr

inductive Phase where
  | compile
  | deploy
```

### 2. Operational Semantics
```lean
inductive Step : Expr → Expr → Prop where
  | β_app : Step (Expr.app (Expr.defunDeploy name params body) args) (subst body params args)
  | bounded_for_step : Step (Expr.boundedFor var start end body) ...
```

### 3. Type System
```lean
inductive HasType : Context → Expr → Type → Prop where
  | t_int : HasType Γ (Expr.int n) Type.int
  | t_bounded_for :
      HasType Γ start Type.int →
      HasType Γ end Type.int →
      HasType (Γ.extend var Type.int) body τ →
      HasType Γ (Expr.boundedFor var start end body) τ
```

### 4. Property Proofs

#### Property 1: Phase Separation
```lean
theorem phase_separation_sound (p : Program) :
  deploy_phase p → ¬ contains_compile_construct p := by
  sorry -- To be proven
```

#### Property 2: Termination
```lean
theorem deploy_terminates (p : Program) (input : Input) :
  deploy_phase p → ∃ n, eval_steps p input n = some value := by
  sorry -- To be proven
```

#### Properties 3-7: Similar structure

### 5. Lemmas and Tactics
- Custom tactics for proof automation
- Common lemmas library
- Proof strategies documentation

## File Structure
```
lean-proofs/
├── lakefile.lean
├── Oblibeny/
│   ├── Syntax.lean
│   ├── Semantics.lean
│   ├── TypeSystem.lean
│   ├── Properties/
│   │   ├── PhaseSeparation.lean
│   │   ├── Termination.lean
│   │   ├── ResourceBounds.lean
│   │   ├── CapabilitySound.lean
│   │   ├── ObfuscationPreserves.lean
│   │   ├── CallGraphAcyclic.lean
│   │   └── MemorySafe.lean
│   └── Proofs/
│       ├── Lemmas.lean
│       └── Tactics.lean
```
