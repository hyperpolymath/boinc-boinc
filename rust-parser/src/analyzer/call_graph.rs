// SPDX-License-Identifier: MPL-2.0
// Copyright (c) Jonathan D.A. Jewell <j.d.a.jewell@open.ac.uk>
use crate::ast::Expr;
use petgraph::graph::{DiGraph, NodeIndex};
use petgraph::algo::is_cyclic_directed;
use std::collections::HashMap;

pub struct CallGraph {
    graph: DiGraph<String, ()>,
    node_map: HashMap<String, NodeIndex>,
}

impl CallGraph {
    pub fn new() -> Self {
        Self {
            graph: DiGraph::new(),
            node_map: HashMap::new(),
        }
    }

    /// Build call graph from deploy functions
    pub fn build(exprs: &[Expr]) -> Self {
        let mut cg = Self::new();

        // First pass: add all function nodes
        for expr in exprs {
            if let Expr::DefunDeploy { name, .. } = expr {
                cg.add_function(name.clone());
            }
        }

        // Second pass: add edges for function calls
        for expr in exprs {
            if let Expr::DefunDeploy { name, body, .. } = expr {
                let calls = Self::extract_function_calls(body);
                for called in calls {
                    cg.add_call(name.clone(), called);
                }
            }
        }

        cg
    }

    fn add_function(&mut self, name: String) {
        if !self.node_map.contains_key(&name) {
            let idx = self.graph.add_node(name.clone());
            self.node_map.insert(name, idx);
        }
    }

    fn add_call(&mut self, caller: String, callee: String) {
        self.add_function(callee.clone());

        if let (Some(&from), Some(&to)) = (self.node_map.get(&caller), self.node_map.get(&callee))
        {
            self.graph.add_edge(from, to, ());
        }
    }

    fn extract_function_calls(exprs: &[Expr]) -> Vec<String> {
        let mut calls = Vec::new();

        for expr in exprs {
            Self::collect_calls(expr, &mut calls);
        }

        calls
    }

    fn collect_calls(expr: &Expr, calls: &mut Vec<String>) {
        match expr {
            Expr::FunctionCall { func, args } => {
                if let Expr::Ident(name) = func.as_ref() {
                    calls.push(name.clone());
                }
                for arg in args {
                    Self::collect_calls(arg, calls);
                }
            }
            Expr::BoundedFor { body, .. } => {
                for e in body {
                    Self::collect_calls(e, calls);
                }
            }
            Expr::Let { bindings, body } => {
                for (_, e) in bindings {
                    Self::collect_calls(e, calls);
                }
                for e in body {
                    Self::collect_calls(e, calls);
                }
            }
            Expr::If {
                condition,
                then_branch,
                else_branch,
            } => {
                Self::collect_calls(condition, calls);
                Self::collect_calls(then_branch, calls);
                Self::collect_calls(else_branch, calls);
            }
            Expr::WithCapability { body, .. } => {
                for e in body {
                    Self::collect_calls(e, calls);
                }
            }
            _ => {}
        }
    }

    /// Check if the call graph has cycles (recursion)
    pub fn has_cycles(&self) -> bool {
        is_cyclic_directed(&self.graph)
    }

    /// Get topological order of functions (None if cyclic)
    pub fn topological_order(&self) -> Option<Vec<String>> {
        if self.has_cycles() {
            return None;
        }

        petgraph::algo::toposort(&self.graph, None)
            .ok()
            .map(|order| {
                order
                    .into_iter()
                    .map(|idx| self.graph[idx].clone())
                    .collect()
            })
    }

    /// Get the number of functions
    pub fn function_count(&self) -> usize {
        self.graph.node_count()
    }
}

impl Default for CallGraph {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::types::Parameter;

    #[test]
    fn test_acyclic_call_graph() {
        let exprs = vec![
            Expr::DefunDeploy {
                name: "main".to_string(),
                params: vec![],
                return_type: None,
                body: vec![Expr::FunctionCall {
                    func: Box::new(Expr::Ident("helper".to_string())),
                    args: vec![],
                }],
            },
            Expr::DefunDeploy {
                name: "helper".to_string(),
                params: vec![],
                return_type: None,
                body: vec![Expr::Int(42)],
            },
        ];

        let cg = CallGraph::build(&exprs);
        assert!(!cg.has_cycles());
        assert_eq!(cg.function_count(), 2);
    }

    #[test]
    fn test_cyclic_call_graph() {
        let exprs = vec![
            Expr::DefunDeploy {
                name: "foo".to_string(),
                params: vec![],
                return_type: None,
                body: vec![Expr::FunctionCall {
                    func: Box::new(Expr::Ident("bar".to_string())),
                    args: vec![],
                }],
            },
            Expr::DefunDeploy {
                name: "bar".to_string(),
                params: vec![],
                return_type: None,
                body: vec![Expr::FunctionCall {
                    func: Box::new(Expr::Ident("foo".to_string())),
                    args: vec![],
                }],
            },
        ];

        let cg = CallGraph::build(&exprs);
        assert!(cg.has_cycles());
    }
}
