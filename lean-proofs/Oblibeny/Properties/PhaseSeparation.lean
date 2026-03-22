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

/-- Boolean checker for phase separation soundness -/
def phaseSeparationSoundBool (prog : Program) : Bool :=
  prog.defs.all fun def_ =>
    match def_ with
    | defunDeploy _ _ body => !body.any (·.containsCompileOnly)
    | _ => true

/-- The boolean checker is sound: if it returns true, the property holds -/
theorem phaseSeparationSoundBool_sound (prog : Program) :
    phaseSeparationSoundBool prog = true → phaseSeparationSound prog := by
  intro h_bool
  unfold phaseSeparationSound
  unfold phaseSeparationSoundBool at h_bool
  intro def_ h_mem
  cases def_ with
  | defunDeploy name params body =>
    have h_all := List.all_eq_true.mp h_bool def_ h_mem
    simp at h_all
    intro h_contains
    simp [h_contains] at h_all
  | _ => trivial

/-- The boolean checker is complete: if the property holds, it returns true -/
theorem phaseSeparationSoundBool_complete (prog : Program) :
    phaseSeparationSound prog → phaseSeparationSoundBool prog = true := by
  intro h_sound
  unfold phaseSeparationSoundBool
  apply List.all_eq_true.mpr
  intro def_ h_mem
  have h_def := h_sound def_ h_mem
  cases def_ with
  | defunDeploy name params body =>
    simp
    simp at h_def
    exact h_def
  | _ => simp

/-- Main theorem: Phase separation is decidable -/
theorem phaseSeparation_decidable (prog : Program) :
    Decidable (phaseSeparationSound prog) :=
  -- Decidability follows from boolean reflection: phaseSeparationSoundBool
  -- is a total Boolean function that is equivalent to the Prop.
  if h : phaseSeparationSoundBool prog then
    isTrue (phaseSeparationSoundBool_sound prog h)
  else
    isFalse (fun h_sound =>
      h (phaseSeparationSoundBool_complete prog h_sound))

end Oblibeny.Properties
