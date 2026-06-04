// SPDX-License-Identifier: MPL-2.0
// Copyright (c) Jonathan D.A. Jewell <j.d.a.jewell@open.ac.uk>
use super::expr::Expr;

pub trait Visitor<T> {
    fn visit_expr(&mut self, expr: &Expr) -> T;

    fn visit_int(&mut self, n: i64) -> T {
        self.default_visit()
    }

    fn visit_float(&mut self, f: f64) -> T {
        self.default_visit()
    }

    fn visit_bool(&mut self, b: bool) -> T {
        self.default_visit()
    }

    fn visit_string(&mut self, s: &str) -> T {
        self.default_visit()
    }

    fn visit_ident(&mut self, i: &str) -> T {
        self.default_visit()
    }

    fn visit_defun_deploy(&mut self, name: &str, body: &[Expr]) -> T {
        for expr in body {
            self.visit_expr(expr);
        }
        self.default_visit()
    }

    fn visit_bounded_for(&mut self, var: &str, start: &Expr, end: &Expr, body: &[Expr]) -> T {
        self.visit_expr(start);
        self.visit_expr(end);
        for expr in body {
            self.visit_expr(expr);
        }
        self.default_visit()
    }

    fn visit_function_call(&mut self, func: &Expr, args: &[Expr]) -> T {
        self.visit_expr(func);
        for arg in args {
            self.visit_expr(arg);
        }
        self.default_visit()
    }

    fn default_visit(&mut self) -> T;
}

/// Mutable visitor for AST traversal and transformation
pub trait MutVisitor {
    fn visit_expr_mut(&mut self, expr: &mut Expr);

    fn visit_defun_deploy_mut(&mut self, name: &mut String, body: &mut Vec<Expr>) {
        for expr in body {
            self.visit_expr_mut(expr);
        }
    }

    fn visit_bounded_for_mut(
        &mut self,
        var: &mut String,
        start: &mut Box<Expr>,
        end: &mut Box<Expr>,
        body: &mut Vec<Expr>,
    ) {
        self.visit_expr_mut(start);
        self.visit_expr_mut(end);
        for expr in body {
            self.visit_expr_mut(expr);
        }
    }
}

/// Collect all identifiers in an expression
pub struct IdentCollector {
    pub idents: Vec<String>,
}

impl IdentCollector {
    pub fn new() -> Self {
        Self { idents: Vec::new() }
    }

    pub fn collect(expr: &Expr) -> Vec<String> {
        let mut collector = Self::new();
        collector.visit_expr(expr);
        collector.idents
    }
}

impl Visitor<()> for IdentCollector {
    fn visit_expr(&mut self, expr: &Expr) -> () {
        match expr {
            Expr::Ident(i) => self.visit_ident(i),
            Expr::DefunDeploy { body, .. } => {
                for e in body {
                    self.visit_expr(e);
                }
            }
            Expr::BoundedFor {
                var,
                start,
                end,
                body,
            } => self.visit_bounded_for(var, start, end, body),
            Expr::FunctionCall { func, args } => self.visit_function_call(func, args),
            Expr::Let { bindings, body } => {
                for (_, expr) in bindings {
                    self.visit_expr(expr);
                }
                for expr in body {
                    self.visit_expr(expr);
                }
            }
            Expr::If {
                condition,
                then_branch,
                else_branch,
            } => {
                self.visit_expr(condition);
                self.visit_expr(then_branch);
                self.visit_expr(else_branch);
            }
            _ => {}
        }
    }

    fn visit_ident(&mut self, i: &str) -> () {
        self.idents.push(i.to_string());
    }

    fn default_visit(&mut self) -> () {}
}
