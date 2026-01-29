// src/lib.rs
//! Checker for pytest setUp/tearDown method usage.
//!
//! This module provides validation that Python test classes correctly implement setUp/tearDown
//! methods with proper naming (camelCase) and with super() calls as the last statement.

use ruff_python_ast::Stmt;
use ruff_python_parser::parse_module;
use ruff_text_size::Ranged;
use std::{fs, path::Path};

pub mod fix;

/// Check a Python file for setUp/tearDown method violations.
///
/// Returns a list of error messages, one per violation found.
/// Returns empty vector if no violations are found.
pub fn check_file(path: &Path) -> Vec<String> {
    let Ok(src) = fs::read_to_string(path) else {
        return vec![];
    };
    let Ok(parsed) = parse_module(&src) else {
        return vec![];
    };

    let mut errors = Vec::new();

    // Recursively check all top-level statements (including classes)
    for stmt in parsed.syntax().body.iter() {
        check_stmt(stmt, path, &mut errors, &src);
    }

    errors
}

/// Recursively check a statement for setUp/tearDown violations.
///
/// - For function definitions: checks method name and super() call placement
/// - For class definitions: recursively checks all methods in the class
fn check_stmt(stmt: &Stmt, path: &Path, errors: &mut Vec<String>, src: &str) {
    match stmt {
        Stmt::FunctionDef(func_def) => {
            match func_def.name.as_str() {
                "setUp" | "tearDown" => {
                    // Check properly-cased setUp/tearDown methods
                    if func_def.body.is_empty() {
                        return;
                    }

                    // Skip validation for methods that only contain 'pass'
                    if func_def.body.len() == 1 {
                        if let Stmt::Pass(_) = &func_def.body[0] {
                            return;
                        }
                    }

                    let last = func_def.body.last().unwrap();
                    let expected = func_def.name.as_str();

                    if !is_super_call_last(last, expected) {
                        let line = src[..stmt.start().to_usize()].lines().count();
                        errors.push(format!(
                            "{}:{} super().{}() must be the last line",
                            path.display(),
                            line,
                            expected
                        ));
                    }
                }
                "setup" | "teardown" | "Setup" | "Teardown" => {
                    // Flag incorrectly-cased setUp/tearDown methods
                    let line = src[..stmt.start().to_usize()].lines().count();
                    errors.push(format!(
                        "{}:{} use correct casing: setUp / tearDown",
                        path.display(),
                        line
                    ));
                }
                _ => {
                    // Other methods are not checked
                }
            }
        }
        Stmt::ClassDef(class_def) => {
            // Recursively check all methods in the class
            for nested_stmt in &class_def.body {
                check_stmt(nested_stmt, path, errors, src);
            }
        }
        _ => {
            // Other statement types are ignored
        }
    }
}

/// Check if the last statement in a method is a super() call to the expected method.
///
/// Verifies the pattern: `super().setUp()` or `super().tearDown()`
///
/// # Arguments
/// * `stmt` - The statement to check (should be the last in a method body)
/// * `expected` - The expected method name ("setUp" or "tearDown")
///
/// # Returns
/// true if the statement matches `super().{expected}()`
fn is_super_call_last(stmt: &Stmt, expected: &str) -> bool {
    use ruff_python_ast::Expr;

    let Stmt::Expr(expr_stmt) = stmt else {
        return false;
    };
    let Expr::Call(call_expr) = &*expr_stmt.value else {
        return false;
    };
    let Expr::Attribute(attr_expr) = &*call_expr.func else {
        return false;
    };

    // Check attribute name matches what we expect (setUp or tearDown)
    if attr_expr.attr.as_str() != expected {
        return false;
    }

    // Check we're calling super().setUp() or super().tearDown()
    let Expr::Call(super_call) = &*attr_expr.value else {
        return false;
    };
    let Expr::Name(name_expr) = &*super_call.func else {
        return false;
    };

    name_expr.id.as_str() == "super"
}
