#![forbid(unsafe_code)]
pub mod ast;
pub mod parser;
pub mod phases;
pub mod analyzer;

pub use ast::*;
pub use parser::*;
pub use phases::*;
pub use analyzer::*;

use anyhow::Result;

/// Complete analysis of an Oblibeny program
pub struct ProgramAnalysis {
    pub exprs: Vec<Expr>,
    pub phase_check: Result<(), PhaseError>,
    pub termination_check: Result<(), TerminationError>,
    pub resource_bounds: ResourceBounds,
    pub call_graph: CallGraph,
}

impl ProgramAnalysis {
    pub fn analyze(source: &str) -> Result<Self> {
        // Parse
        let exprs = parse_file(source)?;

        // Phase separation
        let separator = PhaseSeparator::new();
        let phase_check = separator.validate_deploy_phase(&exprs);

        // Termination checking
        let term_checker = TerminationChecker::new(&exprs);
        let termination_check = term_checker.check_terminates(&exprs);

        // Resource analysis
        let resource_analyzer = ResourceAnalyzer::new();
        let mut resource_bounds = ResourceBounds::new();

        for expr in &exprs {
            if let Expr::DefunDeploy { .. } = expr {
                let bounds = resource_analyzer.analyze(expr);
                resource_bounds.add(&bounds);
            }
        }

        // Call graph
        let call_graph = CallGraph::build(&exprs);

        Ok(Self {
            exprs,
            phase_check,
            termination_check,
            resource_bounds,
            call_graph,
        })
    }

    pub fn is_valid(&self) -> bool {
        self.phase_check.is_ok() && self.termination_check.is_ok()
    }

    pub fn to_json(&self) -> Result<String> {
        Ok(serde_json::to_string_pretty(&self.exprs)?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_program() {
        let source = r#"
(defun-deploy add (a b) : int32
  (+ a b))
"#;

        let analysis = ProgramAnalysis::analyze(source).unwrap();
        assert!(analysis.is_valid());
    }

    #[test]
    fn test_bounded_loop() {
        let source = r#"
(defun-deploy sum-range (n) : int32
  (let ((total 0))
    (bounded-for i 0 n
      (set total (+ total i)))
    total))
"#;

        let analysis = ProgramAnalysis::analyze(source).unwrap();
        assert!(analysis.is_valid());
    }
}
