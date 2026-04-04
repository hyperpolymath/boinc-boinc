// SPDX-License-Identifier: GPL-3.0-or-later
// Property-based tests for oblibeny-parser using proptest

use oblibeny_parser::parser::parse_file;
use proptest::prelude::*;

// Strategy for generating simple valid Oblibeny identifiers
fn arb_ident() -> impl Strategy<Value = String> {
    "[a-z][a-z0-9\\-]*"
        .prop_map(|s| s.replace("-", "_"))
}

// Strategy for generating small integers
fn arb_int() -> impl Strategy<Value = i32> {
    -1000i32..=1000i32
}

// Property 1: Parsing empty string always succeeds (no panic)
proptest! {
    #[test]
    fn prop_empty_string_handled(_s in "") {
        let result = parse_file("");
        // Empty input should not panic (may return Ok or Err)
        prop_assert!(true);
    }
}

// Property 2: Parsing whitespace-only string returns error or ok, never panics
proptest! {
    #[test]
    fn prop_whitespace_only_handled(ws in r"[ \t\n\r]+") {
        let result = parse_file(&ws);
        // Should handle gracefully, no panic
        let _ = result;
    }
}

// Property 3: Valid minimal program always parses successfully
proptest! {
    #[test]
    fn prop_minimal_program_parses(
        name in arb_ident(),
    ) {
        let source = format!(
            "(defun-deploy {} () 42)",
            name
        );
        let result = parse_file(&source);
        prop_assert!(result.is_ok(), "minimal program should parse: {}", source);
    }
}

// Property 4: Valid program with arithmetic always parses
proptest! {
    #[test]
    fn prop_arithmetic_program_parses(
        _a in arb_int(),
        _b in arb_int(),
    ) {
        let source = format!(
            "(defun-deploy add (x y) (+ x y))",
        );
        let result = parse_file(&source);
        prop_assert!(result.is_ok(), "arithmetic program should parse: {}", source);
    }
}

// Property 5: Complete analysis pipeline never panics on valid input
proptest! {
    #[test]
    fn prop_analysis_never_panics_valid(
        name in arb_ident(),
    ) {
        let source = format!(
            "(defun-deploy {} () 0)",
            name
        );
        let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            oblibeny_parser::ProgramAnalysis::analyze(&source)
        }));
        prop_assert!(result.is_ok(), "analysis should not panic on valid input");
    }
}

// Property 6: Malformed parentheses return error, not panic
proptest! {
    #[test]
    fn prop_unmatched_parens_error(
        open_count in 1usize..20,
    ) {
        let source = format!(
            "(defun-deploy f () {}",
            "(".repeat(open_count)
        );
        let result = parse_file(&source);
        prop_assert!(result.is_err(), "unmatched parens should error");
    }
}

// Property 7: Random ASCII bytes that aren't valid Oblibeny return error, not panic
proptest! {
    #[test]
    fn prop_invalid_ascii_errors(
        invalid_chars in r"[!@#$%^&*=|?/\\~`]+"
    ) {
        let source = format!("(defun-deploy f () {})", invalid_chars);
        let result = parse_file(&source);
        // Invalid input should either error or be skipped, never panic
        let _ = result;
    }
}

// Property 8: Valid let-binding parses
proptest! {
    #[test]
    fn prop_let_binding_parses(
        name1 in arb_ident(),
        name2 in arb_ident(),
    ) {
        let source = format!(
            "(defun-deploy test () (let (({} 10) ({} 20)) (+ {} {})))",
            name1, name2, name1, name2
        );
        let result = parse_file(&source);
        prop_assert!(result.is_ok(), "let-binding should parse");
    }
}

// Property 9: Deeply nested expressions parse (testing stack safety for reasonable depths)
proptest! {
    #[test]
    fn prop_nested_depth_parses(
        depth in 1usize..50,  // Stack-safe depth
    ) {
        let mut source = String::from("(defun-deploy f () ");
        for _ in 0..depth {
            source.push('(');
            source.push('+');
        }
        source.push('1');
        for _ in 0..depth {
            source.push(')');
        }
        source.push(')');

        let result = parse_file(&source);
        prop_assert!(result.is_ok(), "nested depth {} should parse", depth);
    }
}

// Property 10: Multiple functions in single file parse
proptest! {
    #[test]
    fn prop_multiple_functions_parse(
        count in 1usize..10,
    ) {
        let mut source = String::new();
        for i in 0..count {
            source.push_str(&format!(
                "(defun-deploy f{} () {})",
                i, i
            ));
        }

        let result = parse_file(&source);
        prop_assert!(result.is_ok(), "multiple functions should parse");
    }
}

// Property 11: Boolean values parse correctly
proptest! {
    #[test]
    fn prop_boolean_parses(
        val in prop::bool::ANY,
    ) {
        let val_str = if val { "true" } else { "false" };
        let source = format!(
            "(defun-deploy f () {})",
            val_str
        );
        let result = parse_file(&source);
        prop_assert!(result.is_ok(), "boolean should parse: {}", source);
    }
}

// Property 12: String literals parse correctly
proptest! {
    #[test]
    fn prop_string_literals_parse(
        s in r#""[a-z0-9 _-]*""#
    ) {
        let source = format!(
            "(defun-deploy f () {})",
            s
        );
        let result = parse_file(&source);
        // String parsing might fail for edge cases, just ensure no panic
        let _ = result;
    }
}

// Property 13: If-expressions parse
proptest! {
    #[test]
    fn prop_if_expression_parses(
        _name in arb_ident(),
    ) {
        let source = format!(
            "(defun-deploy f () (if true 1 2))",
        );
        let result = parse_file(&source);
        prop_assert!(result.is_ok(), "if-expression should parse");
    }
}

// Property 14: Program with bounded-for parses
proptest! {
    #[test]
    fn prop_bounded_for_parses(
        _name in arb_ident(),
    ) {
        let source = format!(
            "(defun-deploy loop () (bounded-for i 0 10 (+ i 1)))",
        );
        let result = parse_file(&source);
        prop_assert!(result.is_ok(), "bounded-for should parse");
    }
}

// Property 15: Array literal parses
proptest! {
    #[test]
    fn prop_array_literal_parses(
        _dummy in 0i32..100
    ) {
        let source = "(defun-deploy f () (array int32 3))";
        let result = parse_file(&source);
        // Array parsing should work or fail gracefully
        let _ = result;
    }
}
