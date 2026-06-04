// SPDX-License-Identifier: MPL-2.0
// Copyright (c) Jonathan D.A. Jewell <j.d.a.jewell@open.ac.uk>
use super::expr::Expr;

pub struct PrettyPrinter {
    indent: usize,
}

impl PrettyPrinter {
    pub fn new() -> Self {
        Self { indent: 0 }
    }

    pub fn print(expr: &Expr) -> String {
        let mut printer = Self::new();
        printer.print_expr(expr)
    }

    fn print_expr(&mut self, expr: &Expr) -> String {
        match expr {
            Expr::Int(n) => format!("{}", n),
            Expr::Float(f) => format!("{}", f),
            Expr::Bool(b) => format!("{}", b),
            Expr::String(s) => format!("\"{}\"", s),
            Expr::Ident(i) => i.clone(),

            Expr::DefunDeploy {
                name,
                params,
                return_type,
                body,
            } => {
                let mut result = format!("(defun-deploy {} (", name);
                for (i, param) in params.iter().enumerate() {
                    if i > 0 {
                        result.push_str(" ");
                    }
                    result.push_str(&format!("{}", param));
                }
                result.push(')');

                if let Some(ty) = return_type {
                    result.push_str(&format!(" : {}", ty));
                }

                self.indent += 2;
                for expr in body {
                    result.push('\n');
                    result.push_str(&self.indent_str());
                    result.push_str(&self.print_expr(expr));
                }
                self.indent -= 2;

                result.push(')');
                result
            }

            Expr::BoundedFor {
                var,
                start,
                end,
                body,
            } => {
                let mut result = format!(
                    "(bounded-for {} {} {}",
                    var,
                    self.print_expr(start),
                    self.print_expr(end)
                );

                self.indent += 2;
                for expr in body {
                    result.push('\n');
                    result.push_str(&self.indent_str());
                    result.push_str(&self.print_expr(expr));
                }
                self.indent -= 2;

                result.push(')');
                result
            }

            Expr::Let { bindings, body } => {
                let mut result = String::from("(let (");

                for (i, (name, expr)) in bindings.iter().enumerate() {
                    if i > 0 {
                        result.push(' ');
                    }
                    result.push_str(&format!("({} {})", name, self.print_expr(expr)));
                }

                result.push(')');

                self.indent += 2;
                for expr in body {
                    result.push('\n');
                    result.push_str(&self.indent_str());
                    result.push_str(&self.print_expr(expr));
                }
                self.indent -= 2;

                result.push(')');
                result
            }

            Expr::If {
                condition,
                then_branch,
                else_branch,
            } => format!(
                "(if {} {} {})",
                self.print_expr(condition),
                self.print_expr(then_branch),
                self.print_expr(else_branch)
            ),

            Expr::FunctionCall { func, args } => {
                let mut result = format!("({}", self.print_expr(func));
                for arg in args {
                    result.push(' ');
                    result.push_str(&self.print_expr(arg));
                }
                result.push(')');
                result
            }

            Expr::Set { var, value } => {
                format!("(set {} {})", var, self.print_expr(value))
            }

            Expr::ArrayGet { array, index } => {
                format!(
                    "(array-get {} {})",
                    self.print_expr(array),
                    self.print_expr(index)
                )
            }

            Expr::ArraySet {
                array,
                index,
                value,
            } => {
                format!(
                    "(array-set {} {} {})",
                    self.print_expr(array),
                    self.print_expr(index),
                    self.print_expr(value)
                )
            }

            Expr::SleepMs(ms) => {
                format!("(sleep-ms {})", self.print_expr(ms))
            }

            Expr::Program { name, budget, forms } => {
                let mut result = format!("(program {}\n", name);
                result.push_str(&format!("  {}\n", self.print_expr(budget)));

                self.indent += 2;
                for form in forms {
                    result.push('\n');
                    result.push_str(&self.indent_str());
                    result.push_str(&self.print_expr(form));
                }
                self.indent -= 2;

                result.push(')');
                result
            }

            _ => format!("<{:?}>", expr),
        }
    }

    fn indent_str(&self) -> String {
        " ".repeat(self.indent)
    }
}

impl Default for PrettyPrinter {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::types::Parameter;

    #[test]
    fn test_pretty_print_simple() {
        let expr = Expr::Int(42);
        assert_eq!(PrettyPrinter::print(&expr), "42");
    }

    #[test]
    fn test_pretty_print_function_call() {
        let expr = Expr::FunctionCall {
            func: Box::new(Expr::Ident("+".to_string())),
            args: vec![Expr::Int(1), Expr::Int(2)],
        };
        assert_eq!(PrettyPrinter::print(&expr), "(+ 1 2)");
    }
}
