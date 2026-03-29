/-
  Oblibeny Syntax Formalization in Lean 4

  This file defines the abstract syntax of Oblibeny programs.

  SPDX-License-Identifier: PMPL-1.0-or-later
-/

namespace Oblibeny

/-- Types in Oblibeny -/
inductive Ty : Type where
  | int32 : Ty
  | int64 : Ty
  | uint32 : Ty
  | uint64 : Ty
  | bool : Ty
  | string : Ty
  | void : Ty
  | array : Ty → Nat → Ty
  | capability : ResourceType → Ty
  | fn : List Ty → Ty → Ty
  deriving Repr

/-- Resource types for capabilities -/
inductive ResourceType where
  | uart_tx
  | uart_rx
  | gpio
  | sensor_read
  | network_send
  deriving Repr, DecidableEq

/-- Phase of execution -/
inductive Phase where
  | compile : Phase
  | deploy : Phase
  deriving Repr, DecidableEq

/-- Expressions in Oblibeny -/
inductive Expr where
  | int : Int → Expr
  | bool : Bool → Expr
  | var : String → Expr
  | boundedFor : String → Expr → Expr → List Expr → Expr
  | defunDeploy : String → List String → List Expr → Expr
  | defunCompile : String → List String → List Expr → Expr
  | app : Expr → List Expr → Expr
  | let_ : String → Expr → Expr → Expr
  | if_ : Expr → Expr → Expr → Expr
  | set : String → Expr → Expr
  | withCapability : Expr → List Expr → Expr
  deriving Repr

/-- Check if an expression is compile-only -/
def Expr.isCompileOnly : Expr → Bool
  | defunCompile _ _ _ => true
  | _ => false

/-- Extract phase from expression -/
def Expr.phase : Expr → Phase
  | defunDeploy _ _ _ => Phase.deploy
  | defunCompile _ _ _ => Phase.compile
  | boundedFor _ _ _ _ => Phase.deploy
  | _ => Phase.deploy

/-- Check if any expression in a list contains a compile-only construct -/
def anyCompileOnly : List Expr → Bool
  | [] => false
  | e :: es =>
    match e with
    | Expr.defunCompile _ _ _ => true
    | _ => anyCompileOnly es

/-- Check if expression contains any compile-only construct (top-level only) -/
def Expr.containsCompileOnly : Expr → Bool
  | Expr.defunCompile _ _ _ => true
  | Expr.defunDeploy _ _ body => anyCompileOnly body
  | Expr.let_ _ e1 e2 => e1.containsCompileOnly || e2.containsCompileOnly
  | Expr.if_ cond t e => cond.containsCompileOnly || t.containsCompileOnly || e.containsCompileOnly
  | _ => false

/-- Resource budget specification -/
structure ResourceBudget where
  time_ms : Nat
  memory_bytes : Nat
  network_bytes : Nat
  deriving Repr, DecidableEq

/-- Program is a list of top-level definitions -/
structure Program where
  defs : List Expr
  budget : ResourceBudget
  deriving Repr

end Oblibeny
