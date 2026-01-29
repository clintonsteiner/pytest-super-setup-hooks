// src/lib.rs
use std::{fs, path::Path};
use ruff_python_parser::parse_module;
use ruff_python_ast::Stmt;
use ruff_text_size::Ranged;

pub mod fix;

pub fn check_file(path: &Path) -> Vec<String> {
    let Ok(src) = fs::read_to_string(path) else { return vec![] };
    let Ok(parsed) = parse_module(&src) else {
        return vec![]
    };

    let mut errors = Vec::new();

    for stmt in parsed.syntax().body.iter() {
        check_stmt(stmt, path, &mut errors, &src);
    }

    errors
}

fn check_stmt(stmt: &Stmt, path: &Path, errors: &mut Vec<String>, src: &str) {
    match stmt {
        Stmt::FunctionDef(func_def) => {
            match func_def.name.as_str() {
                "setUp" | "tearDown" => {
                    if func_def.body.is_empty() {
                        return;
                    }
                    let last = func_def.body.last().unwrap();
                    let expected = func_def.name.as_str();

                    if !is_super_call_last(last, expected) {
                        let line = src[..stmt.start().to_usize()]
                            .lines()
                            .count();
                        errors.push(format!(
                            "{}:{} super().{}() must be the last line",
                            path.display(),
                            line,
                            expected
                        ));
                    }
                }
                "setup" | "teardown" | "Setup" | "Teardown" => {
                    let line = src[..stmt.start().to_usize()]
                        .lines()
                        .count();
                    errors.push(format!(
                        "{}:{} use correct casing: setUp / tearDown",
                        path.display(),
                        line
                    ));
                }
                _ => {
                    // Don't check nested functions - only check at class level
                }
            }
        }
        Stmt::ClassDef(class_def) => {
            // Recursively check class body for methods
            for nested_stmt in &class_def.body {
                check_stmt(nested_stmt, path, errors, src);
            }
        }
        _ => {}
    }
}

fn is_super_call_last(stmt: &Stmt, expected: &str) -> bool {
    use ruff_python_ast::Expr;

    if let Stmt::Expr(expr_stmt) = stmt {
        if let Expr::Call(call_expr) = &*expr_stmt.value {
            if let Expr::Attribute(attr_expr) = &*call_expr.func {
                if attr_expr.attr.as_str() == expected {
                    if let Expr::Call(super_call) = &*attr_expr.value {
                        if let Expr::Name(name_expr) = &*super_call.func {
                            return name_expr.id.as_str() == "super";
                        }
                    }
                }
            }
        }
    }
    false
}

