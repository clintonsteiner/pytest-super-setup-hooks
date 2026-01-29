// src/fix.rs
//! Automatic fixing of setUp/tearDown method violations.
//!
//! This module provides functionality to automatically fix:
//! - Incorrect method naming (setup -> setUp, teardown -> tearDown)
//! - Missing super() calls (adds them as the last statement)

use ruff_python_ast::Stmt;
use ruff_python_parser::parse_module;
use ruff_text_size::Ranged;
use std::{fs, path::Path};

/// Attempt to fix setUp/tearDown violations in a Python file.
///
/// # Arguments
/// * `path` - Path to the Python file to fix
///
/// # Returns
/// true if the file was modified, false otherwise
///
/// # Modifications
/// - Renames incorrectly-cased methods (setup -> setUp, teardown -> tearDown)
/// - Adds super().setUp()/super().tearDown() as the last statement if missing
/// - Preserves all other code and formatting
pub fn fix_file(path: &Path) -> bool {
    let Ok(src) = fs::read_to_string(path) else {
        return false;
    };
    let Ok(parsed) = parse_module(&src) else {
        return false;
    };

    // Convert source to lines for manipulation
    let mut lines: Vec<String> = src.lines().map(|s| s.to_string()).collect();
    let mut modified = false;

    // Recursively fix all top-level statements (including classes)
    for stmt in parsed.syntax().body.iter() {
        if fix_stmt(stmt, &src, &mut lines) {
            modified = true;
        }
    }

    // Write back the fixed content if any modifications were made
    if modified {
        fs::write(path, lines.join("\n")).ok();
    }

    modified
}

/// Recursively fix a statement for setUp/tearDown violations.
///
/// - For function definitions: fixes method name and adds/corrects super() calls
/// - For class definitions: recursively fixes all methods in the class
fn fix_stmt(stmt: &Stmt, src: &str, lines: &mut Vec<String>) -> bool {
    match stmt {
        Stmt::FunctionDef(func_def) => {
            let (correct_call, correct_name) = match func_def.name.as_str() {
                "setUp" => ("super().setUp()", None),
                "tearDown" => ("super().tearDown()", None),
                "setup" => ("super().setUp()", Some("setUp")),
                "teardown" => ("super().tearDown()", Some("tearDown")),
                _ => return false,
            };

            if func_def.body.is_empty() {
                return false;
            }

            // Don't add super() to methods that only have pass
            if func_def.body.len() == 1 {
                if let Stmt::Pass(_) = &func_def.body[0] {
                    return false;
                }
            }

            let start_line = src[..stmt.start().to_usize()].lines().count() - 1;
            let end_line = src[..func_def.body.last().unwrap().end().to_usize()]
                .lines()
                .count()
                - 1;

            if start_line >= lines.len() || end_line >= lines.len() {
                return false;
            }

            let mut modified = false;

            // Fix the method name if needed
            if let Some(new_name) = correct_name {
                let old_name = func_def.name.as_str();
                if let Some(line) = lines.get_mut(start_line) {
                    let old_def = format!("def {}(", old_name);
                    if line.contains(&old_def) {
                        *line = line.replace(&old_def, &format!("def {}(", new_name));
                        modified = true;
                    }
                }
            }

            // Remove super() calls from anywhere in the method body (they'll be added at the end)
            let mut body_lines: Vec<String> = lines[start_line + 1..=end_line]
                .iter()
                .filter(|l| !l.contains("super().setUp()") && !l.contains("super().tearDown()"))
                .map(|s| s.to_string())
                .collect();

            // Check if super() was on the last line
            let last_line = &lines[end_line];
            let had_super_at_end =
                last_line.contains("super().setUp()") || last_line.contains("super().tearDown()");

            // Add super() call if it wasn't already there or if it wasn't at the end
            if body_lines.len() < (end_line - start_line) || !had_super_at_end {
                // Get proper indentation from body
                let body_indent = if !body_lines.is_empty() {
                    body_lines[0]
                        .chars()
                        .take_while(|c| c.is_whitespace())
                        .collect::<String>()
                } else {
                    // Fallback: add 4 spaces
                    lines[start_line]
                        .chars()
                        .take_while(|c| c.is_whitespace())
                        .collect::<String>()
                        + "    "
                };

                body_lines.push(format!("{}{}", body_indent, correct_call));
                modified = true;
            }

            if modified {
                lines.splice(start_line + 1..=end_line, body_lines);
            }

            modified
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
