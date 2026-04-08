# Proof Requirements — Oblibeny Language

## Current State (Updated 2026-04-04)
- **Property 2: Deployment Termination**: Formal model established in `lean-proofs/Oblibeny/Properties/Termination.lean`. 
- **Stochastic Meta-Prover**: Prototype Julia buddy (`EchidnaBuddy.jl`) implemented with Simulated Annealing and A2ML stabilization.
- **Rust Parser Integration**: `TerminationChecker` in `rust-parser/src/analyzer/termination.rs` implements acyclic call graph and bounded-loop checks.

## What needs proving
- [ ] **Property 1: Phase Separation Soundness**: Prove that compile-time forms (macros, eval-compile) never leak into deployment code.
- [ ] **Property 2: Deployment Termination (Deep)**: Extend `hasRankingFunction` to handle variable bounds and complex arithmetic.
- [ ] **Property 3: Resource Bounds Enforcement**: Prove that the resource-budget is strictly respected by the deployment runtime.
- [ ] **Property 4: Capability System Soundness**: Prove that I/O operations are impossible without a valid capability witness.
- [ ] **Property 7: Memory Safety**: Prove that array accesses are always within bounds.

## 48-Hour Sprint: Termination Stress Test
- **Goal**: Achieve 100% verified termination for the standard example suite.
- **Mechanism**: Use `EchidnaBuddy.jl` to find proof paths for non-trivial loop bounds.
- **Status**: IN PROGRESS
