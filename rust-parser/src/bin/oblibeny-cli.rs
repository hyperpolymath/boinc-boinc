// SPDX-License-Identifier: MPL-2.0
// Copyright (c) Jonathan D.A. Jewell <j.d.a.jewell@open.ac.uk>
use clap::{Parser, Subcommand};
use oblibeny_parser::*;
use std::fs;
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "oblibeny")]
#[command(about = "Oblibeny language parser and analyzer", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Parse an Oblibeny source file
    Parse {
        /// Input file path
        #[arg(short, long)]
        input: PathBuf,

        /// Output AST as JSON
        #[arg(short, long)]
        json: bool,

        /// Pretty print
        #[arg(short, long)]
        pretty: bool,
    },

    /// Analyze an Oblibeny program
    Analyze {
        /// Input file path
        #[arg(short, long)]
        input: PathBuf,

        /// Verbose output
        #[arg(short, long)]
        verbose: bool,
    },

    /// Check phase separation
    CheckPhases {
        /// Input file path
        #[arg(short, long)]
        input: PathBuf,
    },

    /// Check termination properties
    CheckTermination {
        /// Input file path
        #[arg(short, long)]
        input: PathBuf,
    },

    /// Analyze resource usage
    Resources {
        /// Input file path
        #[arg(short, long)]
        input: PathBuf,
    },

    /// Generate call graph
    CallGraph {
        /// Input file path
        #[arg(short, long)]
        input: PathBuf,

        /// Output format (text or dot)
        #[arg(short, long, default_value = "text")]
        format: String,
    },
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Parse {
            input,
            json,
            pretty,
        } => {
            let source = fs::read_to_string(&input)?;
            let exprs = parse_file(&source)?;

            if json {
                println!("{}", serde_json::to_string_pretty(&exprs)?);
            } else if pretty {
                for expr in &exprs {
                    println!("{}", PrettyPrinter::print(expr));
                    println!();
                }
            } else {
                for expr in &exprs {
                    println!("{:?}", expr);
                }
            }
        }

        Commands::Analyze { input, verbose } => {
            let source = fs::read_to_string(&input)?;
            let analysis = ProgramAnalysis::analyze(&source)?;

            println!("=== Oblibeny Program Analysis ===\n");

            println!("Phase Check: {}",
                if analysis.phase_check.is_ok() {
                    "✓ PASS"
                } else {
                    "✗ FAIL"
                }
            );

            if let Err(e) = &analysis.phase_check {
                println!("  Error: {}", e);
            }

            println!("\nTermination Check: {}",
                if analysis.termination_check.is_ok() {
                    "✓ PASS"
                } else {
                    "✗ FAIL"
                }
            );

            if let Err(e) = &analysis.termination_check {
                println!("  Error: {}", e);
            }

            println!("\nResource Bounds (WCET):");
            println!("  Time: {} ms", analysis.resource_bounds.time_ms);
            println!("  Memory: {} bytes", analysis.resource_bounds.memory_bytes);
            println!("  Network: {} bytes", analysis.resource_bounds.network_bytes);
            println!("  Storage: {} bytes", analysis.resource_bounds.storage_bytes);

            println!("\nCall Graph:");
            println!("  Functions: {}", analysis.call_graph.function_count());
            println!("  Cyclic: {}", if analysis.call_graph.has_cycles() { "Yes" } else { "No" });

            if let Some(order) = analysis.call_graph.topological_order() {
                println!("  Topological order: {}", order.join(" -> "));
            }

            println!("\nOverall: {}",
                if analysis.is_valid() {
                    "✓ VALID DEPLOYMENT CODE"
                } else {
                    "✗ INVALID FOR DEPLOYMENT"
                }
            );

            if verbose {
                println!("\n=== Parsed Expressions ===\n");
                for (i, expr) in analysis.exprs.iter().enumerate() {
                    println!("{}. {}", i + 1, PrettyPrinter::print(expr));
                }
            }
        }

        Commands::CheckPhases { input } => {
            let source = fs::read_to_string(&input)?;
            let exprs = parse_file(&source)?;
            let separator = PhaseSeparator::new();

            match separator.validate_deploy_phase(&exprs) {
                Ok(()) => println!("✓ Phase separation: PASS"),
                Err(e) => {
                    println!("✗ Phase separation: FAIL");
                    println!("  {}", e);
                    std::process::exit(1);
                }
            }
        }

        Commands::CheckTermination { input } => {
            let source = fs::read_to_string(&input)?;
            let exprs = parse_file(&source)?;
            let checker = TerminationChecker::new(&exprs);

            match checker.check_terminates(&exprs) {
                Ok(()) => println!("✓ Termination: GUARANTEED"),
                Err(e) => {
                    println!("✗ Termination: CANNOT PROVE");
                    println!("  {}", e);
                    std::process::exit(1);
                }
            }
        }

        Commands::Resources { input } => {
            let source = fs::read_to_string(&input)?;
            let exprs = parse_file(&source)?;
            let analyzer = ResourceAnalyzer::new();

            println!("=== Resource Analysis ===\n");

            for expr in &exprs {
                if let Expr::DefunDeploy { name, .. } = expr {
                    let bounds = analyzer.analyze(expr);
                    println!("Function: {}", name);
                    println!("  Time: {} ms", bounds.time_ms);
                    println!("  Memory: {} bytes", bounds.memory_bytes);
                    println!("  Network: {} bytes", bounds.network_bytes);
                    println!();
                }
            }

            if let Some(budget) = ResourceAnalyzer::extract_budget(&exprs) {
                println!("Program Budget:");
                println!("  Time: {} ms", budget.time_ms);
                println!("  Memory: {} bytes", budget.memory_bytes);
                println!("  Network: {} bytes", budget.network_bytes);
            }
        }

        Commands::CallGraph { input, format } => {
            let source = fs::read_to_string(&input)?;
            let exprs = parse_file(&source)?;
            let cg = CallGraph::build(&exprs);

            match format.as_str() {
                "text" => {
                    println!("Call Graph:");
                    println!("  Functions: {}", cg.function_count());
                    println!("  Cyclic: {}", if cg.has_cycles() { "Yes" } else { "No" });

                    if let Some(order) = cg.topological_order() {
                        println!("\nTopological Order:");
                        for (i, func) in order.iter().enumerate() {
                            println!("  {}. {}", i + 1, func);
                        }
                    } else {
                        println!("\nCannot compute topological order (graph is cyclic)");
                    }
                }
                "dot" => {
                    println!("digraph CallGraph {{");
                    // Would need to extract edges from the graph
                    // Placeholder for now
                    println!("  // TODO: Generate DOT format");
                    println!("}}");
                }
                _ => {
                    eprintln!("Unknown format: {}", format);
                    std::process::exit(1);
                }
            }
        }
    }

    Ok(())
}
