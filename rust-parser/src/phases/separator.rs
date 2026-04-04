use crate::ast::{Expr, Phase};
use std::collections::HashSet;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum PhaseError {
    #[error("Compile-time construct found in deploy-time code: {0}")]
    CompileInDeploy(String),

    #[error("Mixed phase in expression (compile and deploy mixed)")]
    MixedPhase,

    #[error("Recursion detected in deploy-time code")]
    RecursionInDeploy,
}

pub struct PhaseSeparator {
    compile_only_constructs: HashSet<String>,
}

impl PhaseSeparator {
    pub fn new() -> Self {
        let mut compile_only_constructs = HashSet::new();
        compile_only_constructs.insert("defun-compile".to_string());
        compile_only_constructs.insert("macro".to_string());
        compile_only_constructs.insert("eval-compile".to_string());
        compile_only_constructs.insert("include".to_string());
        compile_only_constructs.insert("for".to_string());
        compile_only_constructs.insert("while".to_string());

        Self {
            compile_only_constructs,
        }
    }

    /// Analyze an expression and determine its phase
    pub fn analyze(&self, expr: &Expr) -> Result<Phase, PhaseError> {
        match expr {
            Expr::DefunDeploy { body, name, .. } => {
                // Ensure no compile-time constructs in body
                for e in body {
                    if self.is_compile_only(e) {
                        return Err(PhaseError::CompileInDeploy(format!(
                            "in function {}",
                            name
                        )));
                    }
                    // Recursively check
                    self.analyze(e)?;
                }
                Ok(Phase::Deploy)
            }

            Expr::DefunCompile { .. }
            | Expr::Macro { .. }
            | Expr::EvalCompile(_)
            | Expr::Include(_)
            | Expr::For { .. }
            | Expr::While { .. } => Ok(Phase::Compile),

            Expr::BoundedFor { body, .. } => {
                // Ensure body is deploy-safe
                for e in body {
                    if self.is_compile_only(e) {
                        return Err(PhaseError::CompileInDeploy(
                            "in bounded-for loop".to_string(),
                        ));
                    }
                    self.analyze(e)?;
                }
                Ok(Phase::Deploy)
            }

            Expr::WithCapability { body, .. } => {
                for e in body {
                    if self.is_compile_only(e) {
                        return Err(PhaseError::CompileInDeploy(
                            "in with-capability block".to_string(),
                        ));
                    }
                    self.analyze(e)?;
                }
                Ok(Phase::Deploy)
            }

            Expr::Let { bindings, body } => {
                for (_, expr) in bindings {
                    self.analyze(expr)?;
                }
                for expr in body {
                    self.analyze(expr)?;
                }
                // Infer phase from body
                let phases: Vec<_> = body.iter().map(|e| e.phase()).collect();
                if phases.contains(&Phase::Compile) {
                    Ok(Phase::Compile)
                } else {
                    Ok(Phase::Deploy)
                }
            }

            Expr::If {
                condition,
                then_branch,
                else_branch,
            } => {
                self.analyze(condition)?;
                self.analyze(then_branch)?;
                self.analyze(else_branch)?;
                Ok(Phase::Deploy)
            }

            Expr::FunctionCall { func, args } => {
                self.analyze(func)?;
                for arg in args {
                    self.analyze(arg)?;
                }
                Ok(Phase::Deploy)
            }

            // Literals are phase-neutral (can be used in both)
            Expr::Int(_)
            | Expr::Float(_)
            | Expr::Bool(_)
            | Expr::String(_)
            | Expr::Ident(_) => Ok(Phase::Deploy),

            // Other constructs default to deploy
            _ => Ok(Phase::Deploy),
        }
    }

    /// Check if an expression is compile-only
    fn is_compile_only(&self, expr: &Expr) -> bool {
        expr.is_compile_only()
    }

    /// Extract all deploy-time functions from a program
    pub fn extract_deploy_functions<'a>(&self, exprs: &'a [Expr]) -> Vec<&'a Expr> {
        exprs
            .iter()
            .filter(|e| matches!(e, Expr::DefunDeploy { .. }))
            .collect()
    }

    /// Extract all compile-time functions from a program
    pub fn extract_compile_functions<'a>(&self, exprs: &'a [Expr]) -> Vec<&'a Expr> {
        exprs
            .iter()
            .filter(|e| matches!(e, Expr::DefunCompile { .. } | Expr::Macro { .. }))
            .collect()
    }

    /// Validate that all deploy functions are phase-correct
    pub fn validate_deploy_phase(&self, exprs: &[Expr]) -> Result<(), PhaseError> {
        for expr in exprs {
            if let Expr::DefunDeploy { .. } = expr {
                self.analyze(expr)?;
            }
        }
        Ok(())
    }
}

impl Default for PhaseSeparator {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::types::Parameter;

    #[test]
    fn test_deploy_function_valid() {
        let separator = PhaseSeparator::new();

        let expr = Expr::DefunDeploy {
            name: "test".to_string(),
            params: vec![],
            return_type: None,
            body: vec![
                Expr::Int(42),
                Expr::BoundedFor {
                    var: "i".to_string(),
                    start: Box::new(Expr::Int(0)),
                    end: Box::new(Expr::Int(10)),
                    body: vec![Expr::Int(1)],
                },
            ],
        };

        assert!(separator.analyze(&expr).is_ok());
    }

    #[test]
    fn test_deploy_function_invalid() {
        let separator = PhaseSeparator::new();

        let expr = Expr::DefunDeploy {
            name: "test".to_string(),
            params: vec![],
            return_type: None,
            body: vec![
                Expr::Int(42),
                Expr::While {
                    condition: Box::new(Expr::Bool(true)),
                    body: vec![Expr::Int(1)],
                },
            ],
        };

        assert!(separator.analyze(&expr).is_err());
    }
}
