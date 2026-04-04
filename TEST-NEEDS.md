# TEST-NEEDS.md — boinc-boinc

## CRG Grade: C — ACHIEVED 2026-04-04

<!-- SPDX-License-Identifier: PMPL-1.0-or-later -->

## Test Coverage Summary

| Category | Count | Location | Status |
|----------|-------|----------|--------|
| Unit tests | 11 | `rust-parser/src/` inline | ✓ PASS |
| Smoke tests | 5 | `rust-parser/tests/integration_tests.rs` | ✓ PASS |
| E2E tests | 2 | `rust-parser/tests/integration_tests.rs` | ✓ PASS |
| Reflexive/Contract/Aspect | 18 | `rust-parser/tests/integration_tests.rs` | ✓ PASS |
| Property-based tests | 30 | `rust-parser/tests/property_tests.rs` | ✓ PASS |
| Benchmarks | 7 fns | `rust-parser/benches/parser_bench.rs` | ✓ BASELINE |

**Total**: 66+ tests, all passing

## CRG C Checklist

- [x] **Unit tests**: 11 inline tests in `rust-parser/src/`
- [x] **Smoke tests**: 5 smoke tests (trivial function, arithmetic, loop, if-expr, let-binding)
- [x] **Build tests**: `cargo build` succeeds
- [x] **P2P (property-based)**: 30 proptest invariants in `property_tests.rs`
- [x] **E2E tests**: Full analysis pipeline, multiple-function analysis
- [x] **Reflexive tests**: Parser round-trips, output idempotency
- [x] **Contract tests**: API boundary invariants
- [x] **Aspect tests**: Security (malformed input), performance, error handling
- [x] **Benchmarks**: 7 Criterion benchmark functions in `parser_bench.rs`

## Test Files

```
rust-parser/
├── src/            # 11 unit tests (inline #[test])
├── tests/
│   ├── integration_tests.rs   # 25 tests: smoke + E2E + reflexive + contract + aspect
│   └── property_tests.rs      # 30 property-based invariants (proptest)
└── benches/
    └── parser_bench.rs        # 7 Criterion benchmarks

tests/
└── fuzz/           # Fuzz placeholder (future: libfuzzer harness)
```
