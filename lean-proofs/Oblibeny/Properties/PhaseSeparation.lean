/-
  Property 1: Phase Separation Soundness

  Theorem: No compile-time construct appears in deploy-time code.
-/

import Oblibeny.Syntax
import Oblibeny.Semantics

namespace Oblibeny.Properties

open Expr

/-- Phase separation property: deploy-time code contains no compile-only constructs -/
def phaseSeparationSound (prog : Program) : Prop :=
  ∀ def ∈ prog.defs,
    match def with
    | defunDeploy _ _ body => ¬ (body.any (·.containsCompileOnly))
    | _ => True

/-- Helper: Check if a list of expressions contains compile-only constructs -/
def allDeploySafe (exprs : List Expr) : Bool :=
  !exprs.any (·.containsCompileOnly)

theorem phaseSeparation_preservedByBoundedFor
    (x : String) (start end_ : Expr) (body : List Expr) :
    allDeploySafe [start, end_] →
    allDeploySafe body →
    ¬(Expr.boundedFor x start end_ body).containsCompileOnly := by
  intro h_bounds h_body h_contains
  simp [Expr.containsCompileOnly] at h_contains
  -- h_contains : start.containsCompileOnly = true ∨ end_.containsCompileOnly = true
  --              ∨ body.any containsCompileOnly = true
  -- h_bounds : allDeploySafe [start, end_] means neither start nor end_ containsCompileOnly
  -- h_body : allDeploySafe body means no body element containsCompileOnly
  unfold allDeploySafe at h_bounds h_body
  simp [List.any] at h_bounds h_body
  -- h_contains gives us a disjunction; each case contradicts the hypotheses
  rcases h_contains with h_start | h_end | h_body_bad
  · simp [h_start] at h_bounds
  · simp [h_end] at h_bounds
  · simp [h_body_bad] at h_body

theorem phaseSeparation_validProgram (prog : Program) :
    (∀ def ∈ prog.defs, match def with
      | defunDeploy _ _ body => allDeploySafe body
      | _ => true) →
    phaseSeparationSound prog := by
  intro h
  unfold phaseSeparationSound
  intro def h_mem
  cases def with
  | defunDeploy name params body =>
    have h_safe := h def h_mem
    simp at h_safe
    intro h_contains
    exact Bool.noConfusion (Eq.trans h_safe.symm (Bool.of_decide h_contains))
  | _ => trivial

/-- Main theorem: Phase separation is decidable -/
theorem phaseSeparation_decidable (prog : Program) :
    Decidable (phaseSeparationSound prog) := by
  unfold phaseSeparationSound
  -- phaseSeparationSound quantifies over list membership and uses
  -- containsCompileOnly (which returns Bool), making it decidable
  -- in principle. However, the nested pattern match on Expr constructors
  -- and the universal quantifier over List.any make this require
  -- decidable equality and instance resolution infrastructure.
  -- We note that the property is checked by allDeploySafe (a Bool function),
  -- so decidability follows from the boolean reflection.
  sorry  -- GENUINE: requires DecidableEq Expr instance + List.any decidability wiring

end Oblibeny.Properties
