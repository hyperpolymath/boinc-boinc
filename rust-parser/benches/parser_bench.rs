// SPDX-License-Identifier: GPL-3.0-or-later
// Benchmarks for the oblibeny-parser crate.

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use oblibeny_parser::parser::parse_file;
use oblibeny_parser::ProgramAnalysis;

fn bench_parse_trivial(c: &mut Criterion) {
    let source = "(deploy-phase (defun-deploy hello () 42))";
    c.bench_function("parse_trivial", |b| {
        b.iter(|| parse_file(black_box(source)))
    });
}

fn bench_parse_nested(c: &mut Criterion) {
    let source = r#"
(deploy-phase
  (defun-deploy add (a b) (+ a b))
  (defun-deploy mul (a b) (* a b))
  (defun-deploy fma (a b c) (+ (* a b) c)))
"#;
    c.bench_function("parse_nested_functions", |b| {
        b.iter(|| parse_file(black_box(source)))
    });
}

fn bench_full_analysis(c: &mut Criterion) {
    let source = r#"
(deploy-phase
  (defun-deploy compute (x y)
    (let ((sum (+ x y))
          (diff (- x y)))
      (* sum diff))))
"#;
    c.bench_function("full_analysis", |b| {
        b.iter(|| ProgramAnalysis::analyze(black_box(source)))
    });
}

fn bench_parse_let_binding(c: &mut Criterion) {
    let source = r#"
(deploy-phase
  (defun-deploy compute () : int32
    (let ((x 10) (y 20) (z 30))
      (+ x (+ y z)))))
"#;
    c.bench_function("parse_let_binding", |b| {
        b.iter(|| parse_file(black_box(source)))
    });
}

fn bench_parse_if_expression(c: &mut Criterion) {
    let source = r#"
(deploy-phase
  (defun-deploy conditional (x) : int32
    (if (> x 10)
      (* x 2)
      (+ x 1))))
"#;
    c.bench_function("parse_if_expression", |b| {
        b.iter(|| parse_file(black_box(source)))
    });
}

fn bench_parse_bounded_for(c: &mut Criterion) {
    let source = r#"
(deploy-phase
  (defun-deploy sum-loop (n) : int32
    (let ((total 0))
      (bounded-for i 0 n
        (set total (+ total i)))
      total)))
"#;
    c.bench_function("parse_bounded_for", |b| {
        b.iter(|| parse_file(black_box(source)))
    });
}

fn bench_analysis_bounded_for(c: &mut Criterion) {
    let source = r#"
(deploy-phase
  (defun-deploy factorial (n) : int32
    (let ((result 1))
      (bounded-for i 1 n
        (set result (* result i)))
      result)))
"#;
    c.bench_function("analysis_bounded_for", |b| {
        b.iter(|| ProgramAnalysis::analyze(black_box(source)))
    });
}

criterion_group!(
    benches,
    bench_parse_trivial,
    bench_parse_nested,
    bench_full_analysis,
    bench_parse_let_binding,
    bench_parse_if_expression,
    bench_parse_bounded_for,
    bench_analysis_bounded_for
);
criterion_main!(benches);
