// src/fix.rs
use std::{fs, path::Path};
use ruff_python_parser::parse_module;
use ruff_python_ast::Stmt;
use ruff_text_size::Ranged;

pub fn fix_file(path: &Path) -> bool {
    let Ok(src) = fs::read_to_string(path) else { return false };
    let Ok(parsed) = parse_module(&src) else {
        return false
    };

    let mut lines: Vec<String> = src.lines().map(|s| s.to_string()).collect();
    let mut modified = false;

    for stmt in parsed.syntax().body.iter() {
        if fix_stmt(stmt, &src, &mut lines) {
            modified = true;
        }
    }

    if modified {
        fs::write(path, lines.join("\n")).ok();
    }

    modified
}

fn fix_stmt(stmt: &Stmt, src: &str, lines: &mut Vec<String>) -> bool {
    match stmt {
        Stmt::FunctionDef(func_def) => {
            let correct = match func_def.name.as_str() {
                "setUp" => "super().setUp()",
                "tearDown" => "super().tearDown()",
                "setup" => "super().setUp()",
                "teardown" => "super().tearDown()",
                _ => return false,
            };

            if func_def.body.is_empty() {
                return false;
            }

            let start_line = src[..stmt.start().to_usize()].lines().count() - 1;
            let end_line = src[..func_def.body.last().unwrap().end().to_usize()].lines().count() - 1;

            if start_line < lines.len() && end_line < lines.len() {
                let mut filtered: Vec<String> = lines[start_line..=end_line]
                    .iter()
                    .filter(|l| !l.contains("super().setUp()") && !l.contains("super().tearDown()"))
                    .map(|s| s.to_string())
                    .collect();

                // Get indent from the first line of the function body
                let body_indent = if start_line + 1 < lines.len() {
                    lines[start_line + 1]
                        .chars()
                        .take_while(|c: &char| c.is_whitespace())
                        .collect::<String>()
                } else {
                    // Fallback: add 4 spaces to the def line indent
                    let def_indent = lines[start_line]
                        .chars()
                        .take_while(|c: &char| c.is_whitespace())
                        .collect::<String>();
                    format!("{}    ", def_indent)
                };

                filtered.push(format!("{}{}", body_indent, correct));
                lines.splice(start_line..=end_line, filtered);
                return true;
            }
            false
        }
        Stmt::ClassDef(class_def) => {
            let mut modified = false;
            for nested_stmt in &class_def.body {
                if fix_stmt(nested_stmt, src, lines) {
                    modified = true;
                }
            }
            modified
        }
        _ => false,
    }
}

