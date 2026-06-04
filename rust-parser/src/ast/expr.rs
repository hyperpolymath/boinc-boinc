// SPDX-License-Identifier: MPL-2.0
// Copyright (c) Jonathan D.A. Jewell <j.d.a.jewell@open.ac.uk>
use super::types::{Parameter, ResourceType, Type};
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Expr {
    // Literals
    Int(i64),
    Float(f64),
    Bool(bool),
    String(String),
    Ident(String),

    // Deploy-time constructs
    DefunDeploy {
        name: String,
        params: Vec<Parameter>,
        return_type: Option<Type>,
        body: Vec<Expr>,
    },
    BoundedFor {
        var: String,
        start: Box<Expr>,
        end: Box<Expr>,
        body: Vec<Expr>,
    },
    WithCapability {
        capability: Box<Expr>,
        body: Vec<Expr>,
    },

    // Compile-time constructs
    DefunCompile {
        name: String,
        params: Vec<Parameter>,
        return_type: Option<Type>,
        body: Vec<Expr>,
    },
    Macro {
        name: String,
        params: Vec<Parameter>,
        body: Vec<Expr>,
    },
    EvalCompile(Box<Expr>),
    Include(String),
    For {
        var: String,
        iterable: Box<Expr>,
        body: Vec<Expr>,
    },
    While {
        condition: Box<Expr>,
        body: Vec<Expr>,
    },

    // Common constructs
    Let {
        bindings: Vec<(String, Expr)>,
        body: Vec<Expr>,
    },
    Set {
        var: String,
        value: Box<Expr>,
    },
    If {
        condition: Box<Expr>,
        then_branch: Box<Expr>,
        else_branch: Box<Expr>,
    },
    FunctionCall {
        func: Box<Expr>,
        args: Vec<Expr>,
    },

    // Array operations
    ArrayLiteral {
        elem_type: Type,
        size: usize,
    },
    ArrayGet {
        array: Box<Expr>,
        index: Box<Expr>,
    },
    ArraySet {
        array: Box<Expr>,
        index: Box<Expr>,
        value: Box<Expr>,
    },
    ArrayLength(Box<Expr>),

    // I/O operations
    GpioSet {
        device: Box<Expr>,
        value: Box<Expr>,
    },
    GpioGet(Box<Expr>),
    UartSend {
        device: Box<Expr>,
        data: Box<Expr>,
    },
    UartRecv(Box<Expr>),
    SensorRead(Box<Expr>),
    NetworkSend {
        device: Box<Expr>,
        data: Box<Expr>,
    },
    NetworkRecv(Box<Expr>),
    SleepMs(Box<Expr>),
    Timestamp,

    // Resource management
    ResourceBudget {
        specs: Vec<ResourceSpec>,
    },
    DefCap {
        name: String,
        params: Vec<Parameter>,
        description: String,
    },

    // Program structure
    Program {
        name: String,
        budget: Box<Expr>,
        forms: Vec<Expr>,
    },
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ResourceKind {
    TimeMs,
    MemoryBytes,
    NetworkBytes,
    StorageBytes,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ResourceSpec {
    pub kind: ResourceKind,
    pub amount: u64,
}

impl ResourceSpec {
    pub fn new(kind: ResourceKind, amount: u64) -> Self {
        Self { kind, amount }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Phase {
    Compile,
    Deploy,
    Mixed, // Error state
}

impl Expr {
    /// Determine the phase of an expression
    pub fn phase(&self) -> Phase {
        match self {
            // Compile-time only
            Expr::DefunCompile { .. }
            | Expr::Macro { .. }
            | Expr::EvalCompile(_)
            | Expr::Include(_)
            | Expr::For { .. }
            | Expr::While { .. } => Phase::Compile,

            // Deploy-time
            Expr::DefunDeploy { body, .. } => {
                // Check that body contains no compile-only constructs
                for expr in body {
                    if expr.phase() == Phase::Compile {
                        return Phase::Mixed; // Error!
                    }
                }
                Phase::Deploy
            }
            Expr::BoundedFor { body, .. } => {
                for expr in body {
                    if expr.phase() == Phase::Compile {
                        return Phase::Mixed;
                    }
                }
                Phase::Deploy
            }

            // Literals and identifiers are phase-neutral
            Expr::Int(_)
            | Expr::Float(_)
            | Expr::Bool(_)
            | Expr::String(_)
            | Expr::Ident(_) => Phase::Deploy, // Default to deploy

            // Recursively check compound expressions
            Expr::Let { body, .. } => {
                let phases: Vec<_> = body.iter().map(|e| e.phase()).collect();
                if phases.contains(&Phase::Compile) {
                    Phase::Compile
                } else if phases.contains(&Phase::Mixed) {
                    Phase::Mixed
                } else {
                    Phase::Deploy
                }
            }

            Expr::If {
                condition,
                then_branch,
                else_branch,
            } => {
                let phases = vec![
                    condition.phase(),
                    then_branch.phase(),
                    else_branch.phase(),
                ];
                if phases.contains(&Phase::Mixed) {
                    Phase::Mixed
                } else if phases.contains(&Phase::Compile) {
                    Phase::Compile
                } else {
                    Phase::Deploy
                }
            }

            Expr::FunctionCall { func, args } => {
                let mut phase = func.phase();
                for arg in args {
                    let arg_phase = arg.phase();
                    if arg_phase == Phase::Mixed {
                        return Phase::Mixed;
                    }
                    if arg_phase == Phase::Compile {
                        phase = Phase::Compile;
                    }
                }
                phase
            }

            // Other expressions default to deploy
            _ => Phase::Deploy,
        }
    }

    /// Check if expression is compile-only
    pub fn is_compile_only(&self) -> bool {
        matches!(
            self,
            Expr::DefunCompile { .. }
                | Expr::Macro { .. }
                | Expr::EvalCompile(_)
                | Expr::Include(_)
                | Expr::For { .. }
                | Expr::While { .. }
        )
    }

    /// Check if expression is deploy-time safe
    pub fn is_deploy_safe(&self) -> bool {
        self.phase() != Phase::Compile && self.phase() != Phase::Mixed
    }
}

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Expr::Int(n) => write!(f, "{}", n),
            Expr::Float(n) => write!(f, "{}", n),
            Expr::Bool(b) => write!(f, "{}", b),
            Expr::String(s) => write!(f, "\"{}\"", s),
            Expr::Ident(i) => write!(f, "{}", i),
            Expr::DefunDeploy {
                name,
                params,
                return_type,
                body,
            } => {
                write!(f, "(defun-deploy {} (", name)?;
                for (i, param) in params.iter().enumerate() {
                    if i > 0 {
                        write!(f, " ")?;
                    }
                    write!(f, "{}", param)?;
                }
                write!(f, ")")?;
                if let Some(ty) = return_type {
                    write!(f, " : {}", ty)?;
                }
                for expr in body {
                    write!(f, "\n  {}", expr)?;
                }
                write!(f, ")")
            }
            Expr::BoundedFor {
                var,
                start,
                end,
                body,
            } => {
                write!(f, "(bounded-for {} {} {}", var, start, end)?;
                for expr in body {
                    write!(f, "\n  {}", expr)?;
                }
                write!(f, ")")
            }
            Expr::FunctionCall { func, args } => {
                write!(f, "({}", func)?;
                for arg in args {
                    write!(f, " {}", arg)?;
                }
                write!(f, ")")
            }
            _ => write!(f, "<expr>"),
        }
    }
}
