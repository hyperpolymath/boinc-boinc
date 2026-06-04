// SPDX-License-Identifier: MPL-2.0
// Copyright (c) Jonathan D.A. Jewell <j.d.a.jewell@open.ac.uk>
use crate::ast::Expr;
use crate::analyzer::call_graph::CallGraph;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum TerminationError {
    #[error("Recursion detected in deploy-time code")]
    Recursion,

    #[error("Unbounded loop found in deploy-time code: {0}")]
    UnboundedLoop(String),

    #[error("Cannot prove loop bounds are finite: {0}")]
    UnknownBounds(String),

    #[error("Infinite resource budget (deployment must have finite resources)")]
    InfiniteResources,
}

pub struct TerminationChecker {
    call_graph: CallGraph,
}

impl TerminationChecker {
    pub fn new(exprs: &[Expr]) -> Self {
        let call_graph = CallGraph::build(exprs);
        Self { call_graph }
    }

    /// Check that all deploy-time code provably terminates
    pub fn check_terminates(&self, exprs: &[Expr]) -> Result<(), TerminationError> {
        // Check 1: Call graph must be acyclic (no recursion)
        if self.call_graph.has_cycles() {
            return Err(TerminationError::Recursion);
        }

        // Check 2: All loops must be bounded
        for expr in exprs {
            self.check_loops(expr)?;
        }

        Ok(())
    }

    /// Recursively check that all loops are bounded
    fn check_loops(&self, expr: &Expr) -> Result<(), TerminationError> {
        match expr {
            // While loops are unbounded - not allowed in deploy
            Expr::While { .. } => Err(TerminationError::UnboundedLoop(
                "while loop".to_string(),
            )),

            // For loops are unbounded - not allowed in deploy
            Expr::For { .. } => Err(TerminationError::UnboundedLoop(
                "for loop".to_string(),
            )),

            // Bounded-for is OK if bounds are static
            Expr::BoundedFor {
                var,
                start,
                end,
                body,
            } => {
                // Check that bounds are computable
                if !self.are_bounds_finite(start, end) {
                    return Err(TerminationError::UnknownBounds(format!(
                        "bounded-for {}",
                        var
                    )));
                }

                // Recursively check body
                for e in body {
                    self.check_loops(e)?;
                }

                Ok(())
            }

            // Recursively check compound expressions
            Expr::DefunDeploy { body, .. } => {
                for e in body {
                    self.check_loops(e)?;
                }
                Ok(())
            }

            Expr::Let { bindings, body } => {
                for (_, e) in bindings {
                    self.check_loops(e)?;
                }
                for e in body {
                    self.check_loops(e)?;
                }
                Ok(())
            }

            Expr::If {
                condition,
                then_branch,
                else_branch,
            } => {
                self.check_loops(condition)?;
                self.check_loops(then_branch)?;
                self.check_loops(else_branch)?;
                Ok(())
            }

            Expr::WithCapability { body, .. } => {
                for e in body {
                    self.check_loops(e)?;
                }
                Ok(())
            }

            Expr::FunctionCall { func, args } => {
                self.check_loops(func)?;
                for arg in args {
                    self.check_loops(arg)?;
                }
                Ok(())
            }

            _ => Ok(()),
        }
    }

    /// Check if loop bounds are finite and computable
    fn are_bounds_finite(&self, start: &Expr, end: &Expr) -> bool {
        // For now, accept only integer literals or simple expressions
        // A more sophisticated analysis would handle variables, etc.
        matches!(start, Expr::Int(_)) && matches!(end, Expr::Int(_))
    }

    /// Get the topological order of functions (for verification)
    pub fn function_order(&self) -> Option<Vec<String>> {
        self.call_graph.topological_order()
    }

    /// Compute a termination ranking function for a loop
    /// Returns the maximum iterations or None if unbounded
    pub fn loop_ranking_function(&self, start: &Expr, end: &Expr) -> Option<u64> {
        match (start, end) {
            (Expr::Int(s), Expr::Int(e)) => {
                if e >= s {
                    Some((e - s) as u64)
                } else {
                    Some(0)
                }
            }
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bounded_for_valid() {
        let expr = Expr::BoundedFor {
            var: "i".to_string(),
            start: Box::new(Expr::Int(0)),
            end: Box::new(Expr::Int(10)),
            body: vec![Expr::Int(1)],
        };

        let exprs = vec![expr];
        let checker = TerminationChecker::new(&exprs);
        assert!(checker.check_terminates(&exprs).is_ok());
    }

    #[test]
    fn test_while_invalid() {
        let expr = Expr::While {
            condition: Box::new(Expr::Bool(true)),
            body: vec![Expr::Int(1)],
        };

        let exprs = vec![expr];
        let checker = TerminationChecker::new(&exprs);
        assert!(checker.check_terminates(&exprs).is_err());
    }

    #[test]
    fn test_recursion_detection() {
        let exprs = vec![
            Expr::DefunDeploy {
                name: "foo".to_string(),
                params: vec![],
                return_type: None,
                body: vec![Expr::FunctionCall {
                    func: Box::new(Expr::Ident("foo".to_string())),
                    args: vec![],
                }],
            },
        ];

        let checker = TerminationChecker::new(&exprs);
        assert!(checker.check_terminates(&exprs).is_err());
    }
}
