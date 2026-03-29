/-
  Property 1: Phase Separation Soundness

  Theorem: No compile-time construct appears in deploy-time code.

  SPDX-License-Identifier: PMPL-1.0-or-later
-/

import Oblibeny.Syntax
import Oblibeny.Semantics

namespace Oblibeny.Properties

open Expr

/-- Phase separation property: deploy-time code contains no compile-only constructs -/
def phaseSeparationSound (prog : Program) : Prop :=
  ∀ def_ ∈ prog.defs,
    match def_ with
    | defunDeploy _ _ body => ¬ (body.any (·.containsCompileOnly))
    | _ => True

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
  intro d h_mem
  cases d with
  | defunDeploy name params body =>
    have h_all := List.all_eq_true.mp h_bool (defunDeploy name params body) h_mem
    simp at h_all
    intro h_any
    have h_any' := List.any_eq_true.mp h_any
    obtain ⟨e, h_mem_e, h_co⟩ := h_any'
    have := h_all e h_mem_e
    simp [h_co] at this
  | _ => trivial

/-- The boolean checker is complete: if the property holds, it returns true -/
theorem phaseSeparationSoundBool_complete (prog : Program) :
    phaseSeparationSound prog → phaseSeparationSoundBool prog = true := by
  intro h_sound
  unfold phaseSeparationSoundBool
  apply List.all_eq_true.mpr
  intro d h_mem
  have h_def := h_sound d h_mem
  cases d with
  | defunDeploy name params body =>
    simp
    simp at h_def
    exact h_def
  | _ => simp

/-- Phase separation is decidable via boolean reflection -/
instance phaseSeparation_decidable (prog : Program) :
    Decidable (phaseSeparationSound prog) :=
  if h : phaseSeparationSoundBool prog then
    isTrue (phaseSeparationSoundBool_sound prog h)
  else
    isFalse (fun h_sound =>
      h (phaseSeparationSoundBool_complete prog h_sound))

end Oblibeny.Properties
