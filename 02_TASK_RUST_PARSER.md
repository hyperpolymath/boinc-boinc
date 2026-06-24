<!--
SPDX-License-Identifier: CC-BY-SA-4.0
Copyright (c) Jonathan D.A. Jewell <j.d.a.jewell@open.ac.uk>
-->
# Task 1: Rust Parser & Analyzer

## Objective
Build a complete parser and analyzer for Oblibeny that can:
1. Parse Oblibeny source code into AST
2. Separate compile-time and deploy-time phases
3. Analyze resource usage and termination
4. Generate test vectors for BOINC work units

## Deliverables

### 1. Parser (`src/parser/`)
- pest-based PEG parser from EBNF grammar
- Complete coverage of Oblibeny v0.6 syntax
- Error reporting with line/column information
- Parser combinators for reusability

### 2. AST (`src/ast/`)
- Typed AST representation
- Pattern matching support
- Visitor pattern for traversal
- Pretty-printing for debugging

### 3. Phase Separator (`src/phases/`)
- Identify compile-time vs deploy-time code
- Validate phase boundaries
- Extract deploy-time subset for verification

### 4. Analyzer (`src/analyzer/`)
- **Termination**: Bounded loop checking, call graph acyclicity
- **Resources**: WCET, memory bounds, capability budgets
- **Safety**: Memory access bounds, type checking

### 5. CLI Tool
- Parse and validate Oblibeny programs
- Generate reports (AST, phase info, resources)
- Batch processing for BOINC integration

## Implementation Plan

### Step 1: pest Grammar
Convert EBNF to pest syntax:

```pest
program = { SOI ~ form* ~ EOI }
form = { list | atom | string | number }
list = { "(" ~ form* ~ ")" }

// Deploy-time constructs
defun_deploy = { "(" ~ "defun-deploy" ~ ident ~ param_list ~ form* ~ ")" }
bounded_for = { "(" ~ "bounded-for" ~ ident ~ expr ~ expr ~ form* ~ ")" }

// Compile-time constructs
defun_compile = { "(" ~ "defun-compile" ~ ident ~ param_list ~ form* ~ ")" }
macro_def = { "(" ~ "macro" ~ ident ~ param_list ~ form* ~ ")" }
```

### Step 2: AST Design
```rust
pub enum Expr {
    Int(i64),
    Float(f64),
    Bool(bool),
    String(String),
    Ident(String),
    List(Vec<Expr>),

    // Deploy-time
    DefunDeploy { name: String, params: Vec<Param>, body: Vec<Expr> },
    BoundedFor { var: String, start: Box<Expr>, end: Box<Expr>, body: Vec<Expr> },
    WithCapability { cap: Box<Expr>, body: Vec<Expr> },

    // Compile-time
    DefunCompile { name: String, params: Vec<Param>, body: Vec<Expr> },
    Macro { name: String, params: Vec<Param>, body: Vec<Expr> },
    EvalCompile(Box<Expr>),
}

pub enum Phase {
    Compile,
    Deploy,
    Mixed, // Error state
}
```

### Step 3: Phase Separation Algorithm
```rust
pub struct PhaseSeparator {
    compile_only_constructs: HashSet<&'static str>,
    deploy_only_constructs: HashSet<&'static str>,
}

impl PhaseSeparator {
    pub fn analyze(&self, expr: &Expr) -> Result<Phase, PhaseError> {
        match expr {
            Expr::DefunDeploy { body, .. } => {
                // Ensure no compile-time constructs in body
                for e in body {
                    if self.is_compile_only(e) {
                        return Err(PhaseError::CompileInDeploy);
                    }
                }
                Ok(Phase::Deploy)
            }
            Expr::DefunCompile { .. } => Ok(Phase::Compile),
            _ => self.infer_phase(expr),
        }
    }
}
```

### Step 4: Resource Analyzer
```rust
pub struct ResourceAnalyzer {
    call_graph: CallGraph,
}

impl ResourceAnalyzer {
    pub fn analyze(&self, expr: &Expr) -> ResourceBounds {
        ResourceBounds {
            time_ms: self.compute_wcet(expr),
            memory_bytes: self.compute_max_memory(expr),
            network_bytes: self.compute_network_usage(expr),
        }
    }

    fn compute_wcet(&self, expr: &Expr) -> u64 {
        match expr {
            Expr::BoundedFor { start, end, body, .. } => {
                let iterations = self.eval_const(end) - self.eval_const(start);
                let body_time = body.iter().map(|e| self.compute_wcet(e)).sum();
                iterations * body_time
            }
            _ => self.instruction_cost(expr),
        }
    }
}
```

### Step 5: Termination Checker
```rust
pub struct TerminationChecker {
    call_graph: CallGraph,
}

impl TerminationChecker {
    pub fn check_deploy_terminates(&self, program: &Program) -> Result<(), TerminationError> {
        // Check 1: Call graph is acyclic
        if self.call_graph.has_cycles() {
            return Err(TerminationError::Recursion);
        }

        // Check 2: All loops are bounded
        for func in program.deploy_functions() {
            self.check_bounded_loops(func)?;
        }

        Ok(())
    }
}
```

## Testing Strategy

### Unit Tests
- Parse all example programs
- Verify AST structure
- Test phase separation
- Resource analysis correctness

### Property Tests
- Generate random valid programs
- Parse → pretty-print → parse roundtrip
- Resource bounds never negative

### Benchmarks
- Parse 1000-line programs in < 100ms
- Analysis overhead < 50ms

## File Structure
```
rust-parser/
├── Cargo.toml
├── src/
│   ├── lib.rs
│   ├── ast/
│   │   ├── mod.rs
│   │   ├── expr.rs
│   │   ├── visitor.rs
│   │   └── pretty_print.rs
│   ├── parser/
│   │   ├── mod.rs
│   │   ├── grammar.pest
│   │   └── parser.rs
│   ├── phases/
│   │   ├── mod.rs
│   │   └── separator.rs
│   ├── analyzer/
│   │   ├── mod.rs
│   │   ├── resources.rs
│   │   ├── termination.rs
│   │   ├── call_graph.rs
│   │   └── bounds.rs
│   └── bin/
│       └── oblibeny-cli.rs
└── tests/
    ├── parser_tests.rs
    ├── phase_tests.rs
    └── analyzer_tests.rs
```

## Dependencies
```toml
[dependencies]
pest = "2.7"
pest_derive = "2.7"
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
anyhow = "1.0"
thiserror = "1.0"
clap = { version = "4.5", features = ["derive"] }

[dev-dependencies]
proptest = "1.4"
criterion = "0.5"
```

## Next Steps
1. Implement pest grammar
2. Build AST types
3. Write parser
4. Implement phase separator
5. Build resource analyzer
6. Add CLI
7. Write comprehensive tests
