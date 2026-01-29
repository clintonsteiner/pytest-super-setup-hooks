// src/main.rs
//! CLI entrypoint for pytest-super-setup-hooks
//!
//! Usage:
//! - Check files: pytest-super-hooks file1.py file2.py
//! - Fix files: pytest-super-hooks --fix file1.py file2.py

use pytest_super_hooks::{check_file, fix::fix_file};
use std::{env, path::Path};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: {} [--fix] <file1.py> [file2.py] ...", args[0]);
        std::process::exit(1);
    }

    let mut fix_mode = false;
    let mut errors = Vec::new();

    // Parse arguments and process each Python file
    for arg in &args[1..] {
        if arg == "--fix" {
            fix_mode = true;
            continue;
        }

        if arg.ends_with(".py") {
            let path = Path::new(arg);

            // Fix the file if requested
            if fix_mode {
                let _ = fix_file(path);
            }

            // Check and collect any errors
            errors.extend(check_file(path));
        }
    }

    // Exit with error code if any violations were found
    if !errors.is_empty() {
        for error in &errors {
            eprintln!("{}", error);
        }
        std::process::exit(1);
    }
}
