// SPDX-License-Identifier: MPL-2.0
// Copyright (c) Jonathan D.A. Jewell <j.d.a.jewell@open.ac.uk>
use crate::ast::{Expr, ResourceKind, ResourceSpec};
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ResourceBounds {
    pub time_ms: u64,
    pub memory_bytes: u64,
    pub network_bytes: u64,
    pub storage_bytes: u64,
}

impl ResourceBounds {
    pub fn new() -> Self {
        Self {
            time_ms: 0,
            memory_bytes: 0,
            network_bytes: 0,
            storage_bytes: 0,
        }
    }

    pub fn add(&mut self, other: &ResourceBounds) {
        self.time_ms += other.time_ms;
        self.memory_bytes += other.memory_bytes;
        self.network_bytes += other.network_bytes;
        self.storage_bytes += other.storage_bytes;
    }

    pub fn max(&mut self, other: &ResourceBounds) {
        self.time_ms = self.time_ms.max(other.time_ms);
        self.memory_bytes = self.memory_bytes.max(other.memory_bytes);
        self.network_bytes = self.network_bytes.max(other.network_bytes);
        self.storage_bytes = self.storage_bytes.max(other.storage_bytes);
    }

    pub fn multiply(&mut self, factor: u64) {
        self.time_ms *= factor;
        self.memory_bytes *= factor;
        self.network_bytes *= factor;
        self.storage_bytes *= factor;
    }

    pub fn fits_within(&self, budget: &ResourceBounds) -> bool {
        self.time_ms <= budget.time_ms
            && self.memory_bytes <= budget.memory_bytes
            && self.network_bytes <= budget.network_bytes
            && self.storage_bytes <= budget.storage_bytes
    }
}

impl Default for ResourceBounds {
    fn default() -> Self {
        Self::new()
    }
}

pub struct ResourceAnalyzer {
    costs: HashMap<String, u64>,
}

impl ResourceAnalyzer {
    pub fn new() -> Self {
        let mut costs = HashMap::new();

        // Operation costs (arbitrary units)
        costs.insert("add".to_string(), 1);
        costs.insert("sub".to_string(), 1);
        costs.insert("mul".to_string(), 2);
        costs.insert("div".to_string(), 10);
        costs.insert("mod".to_string(), 10);
        costs.insert("array-access".to_string(), 1);
        costs.insert("gpio".to_string(), 100);
        costs.insert("uart".to_string(), 200);
        costs.insert("sensor".to_string(), 500);
        costs.insert("network".to_string(), 1000);
        costs.insert("sleep".to_string(), 0); // Time, not compute

        Self { costs }
    }

    /// Analyze resource usage of an expression (WCET)
    pub fn analyze(&self, expr: &Expr) -> ResourceBounds {
        match expr {
            // Literals: minimal cost
            Expr::Int(_) | Expr::Float(_) | Expr::Bool(_) | Expr::String(_) | Expr::Ident(_) => {
                let mut bounds = ResourceBounds::new();
                bounds.time_ms = 1;
                bounds
            }

            // Bounded for loop: multiply body cost by iterations
            Expr::BoundedFor {
                var: _,
                start,
                end,
                body,
            } => {
                let iterations = self.eval_const_diff(start, end).unwrap_or(100);

                let mut body_bounds = ResourceBounds::new();
                for expr in body {
                    let expr_bounds = self.analyze(expr);
                    body_bounds.add(&expr_bounds);
                }

                body_bounds.multiply(iterations);
                body_bounds
            }

            // Let binding: sum of bindings + body
            Expr::Let { bindings, body } => {
                let mut bounds = ResourceBounds::new();

                for (_, expr) in bindings {
                    let expr_bounds = self.analyze(expr);
                    bounds.add(&expr_bounds);
                }

                for expr in body {
                    let expr_bounds = self.analyze(expr);
                    bounds.add(&expr_bounds);
                }

                bounds
            }

            // If: condition + max of branches
            Expr::If {
                condition,
                then_branch,
                else_branch,
            } => {
                let mut bounds = self.analyze(condition);
                let then_bounds = self.analyze(then_branch);
                let else_bounds = self.analyze(else_branch);

                bounds.max(&then_bounds);
                bounds.max(&else_bounds);
                bounds
            }

            // Function call: analyze arguments + call cost
            Expr::FunctionCall { func, args } => {
                let mut bounds = ResourceBounds::new();
                bounds.time_ms = 10; // Base call overhead

                for arg in args {
                    let arg_bounds = self.analyze(arg);
                    bounds.add(&arg_bounds);
                }

                // If we know the function, add its cost
                // For now, use heuristic
                if let Expr::Ident(name) = func.as_ref() {
                    if let Some(cost) = self.costs.get(name) {
                        bounds.time_ms += cost;
                    }
                }

                bounds
            }

            // I/O operations
            Expr::GpioSet { .. } | Expr::GpioGet(_) => {
                let mut bounds = ResourceBounds::new();
                bounds.time_ms = *self.costs.get("gpio").unwrap_or(&100);
                bounds
            }

            Expr::SensorRead(_) => {
                let mut bounds = ResourceBounds::new();
                bounds.time_ms = *self.costs.get("sensor").unwrap_or(&500);
                bounds
            }

            Expr::NetworkSend { data, .. } => {
                let mut bounds = ResourceBounds::new();
                bounds.time_ms = *self.costs.get("network").unwrap_or(&1000);
                // Estimate network bytes (would need better analysis)
                bounds.network_bytes = 256;
                bounds
            }

            Expr::SleepMs(ms_expr) => {
                let mut bounds = ResourceBounds::new();
                if let Expr::Int(ms) = ms_expr.as_ref() {
                    bounds.time_ms = *ms as u64;
                } else {
                    bounds.time_ms = 1000; // Conservative estimate
                }
                bounds
            }

            // Array operations
            Expr::ArrayGet { array, index } => {
                let mut bounds = self.analyze(array);
                bounds.add(&self.analyze(index));
                bounds.time_ms += self.costs.get("array-access").unwrap_or(&1);
                bounds
            }

            Expr::ArraySet {
                array,
                index,
                value,
            } => {
                let mut bounds = self.analyze(array);
                bounds.add(&self.analyze(index));
                bounds.add(&self.analyze(value));
                bounds.time_ms += self.costs.get("array-access").unwrap_or(&1);
                bounds
            }

            Expr::ArrayLiteral { elem_type, size } => {
                let mut bounds = ResourceBounds::new();
                // Memory for array
                bounds.memory_bytes = (*size as u64) * 8; // Assume 8 bytes per element
                bounds
            }

            // Capability: analyze body
            Expr::WithCapability { body, .. } => {
                let mut bounds = ResourceBounds::new();
                for expr in body {
                    bounds.add(&self.analyze(expr));
                }
                bounds
            }

            // DefunDeploy: analyze body
            Expr::DefunDeploy { body, .. } => {
                let mut bounds = ResourceBounds::new();
                for expr in body {
                    bounds.add(&self.analyze(expr));
                }
                bounds
            }

            _ => ResourceBounds::new(),
        }
    }

    /// Try to evaluate constant integer difference (for loop bounds)
    fn eval_const_diff(&self, start: &Expr, end: &Expr) -> Option<u64> {
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

    /// Extract resource budget from program
    pub fn extract_budget(exprs: &[Expr]) -> Option<ResourceBounds> {
        for expr in exprs {
            if let Expr::Program { budget, .. } = expr {
                if let Expr::ResourceBudget { specs } = budget.as_ref() {
                    return Some(Self::specs_to_bounds(specs));
                }
            } else if let Expr::ResourceBudget { specs } = expr {
                return Some(Self::specs_to_bounds(specs));
            }
        }
        None
    }

    fn specs_to_bounds(specs: &[ResourceSpec]) -> ResourceBounds {
        let mut bounds = ResourceBounds::new();

        for spec in specs {
            match spec.kind {
                ResourceKind::TimeMs => bounds.time_ms = spec.amount,
                ResourceKind::MemoryBytes => bounds.memory_bytes = spec.amount,
                ResourceKind::NetworkBytes => bounds.network_bytes = spec.amount,
                ResourceKind::StorageBytes => bounds.storage_bytes = spec.amount,
            }
        }

        bounds
    }
}

impl Default for ResourceAnalyzer {
    fn default() -> Self {
        Self::new()
    }
}
