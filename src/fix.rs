// src/fix.rs
//! Automatic fixing of setUp/tearDown method violations.
//!
//! This module provides functionality to automatically fix:
//! - Incorrect method naming (setup -> setUp, teardown -> tearDown)
//! - Missing super() calls (adds them as the last statement)

use std::{fs, path::Path};
use ruff_python_parser::parse_module;
use ruff_python_ast::Stmt;
use ruff_text_size::Ranged;

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
    let Ok(src) = fs::read_to_string(path) else { return false };
    let Ok(parsed) = parse_module(&src) else {
        return false
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
            let end_line = src[..func_def.body.last().unwrap().end().to_usize()].lines().count() - 1;

            if start_line >= lines.len() || end_line >= lines.len() {
                return false;
            }

            let mut modified = false;

            // First, fix the method name if needed
            if let Some(new_name) = correct_name {
                let old_name = func_def.name.as_str();
                if let Some(line) = lines.get_mut(start_line) {
                    if line.contains(&format!("def {}(", old_name)) {
                        *line = line.replace(&format!("def {}(", old_name), &format!("def {}(", new_name));
                        modified = true;
                    }
                }
            }

            // Then, add the super call if it's missing
            let mut filtered: Vec<String> = lines[start_line..=end_line]
                .iter()
                .filter(|l| !l.contains("super().setUp()") && !l.contains("super().tearDown()"))
                .map(|s| s.to_string())
                .collect();

            // Only add super call if it's not already there
            let last_in_filtered = filtered.last().map(|l| l.trim()).unwrap_or("");
            let has_super = last_in_filtered.contains("super().setUp()") || last_in_filtered.contains("super().tearDown()");

            if !has_super {
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

                filtered.push(format!("{}{}", body_indent, correct_call));
                modified = true;
            }

            if modified {
                lines.splice(start_line..=end_line, filtered);
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

