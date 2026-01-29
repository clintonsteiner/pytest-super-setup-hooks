// src/main.rs
//! CLI entrypoint for pytest-super-setup-hooks
//!
//! Usage:
//! - Check files: pytest-super-hooks file1.py file2.py
//! - Fix files: pytest-super-hooks --fix file1.py file2.py

use std::{env, path::Path};
use pytest_super_hooks::{check_file, fix::fix_file};

fn main() {
    let mut fix_mode = false;
    let mut errors = Vec::new();

    // Parse arguments and process each Python file
    for arg in env::args().skip(1) {
        if arg == "--fix" {
            // Enable fix mode for subsequent files
            fix_mode = true;
            continue;
        }

        if arg.ends_with(".py") {
            let path = Path::new(&arg);

            // If fix mode is enabled, attempt to fix the file
            if fix_mode {
                fix_file(path);
            }

            // Check the file (after potential fixes) and collect any errors
            errors.extend(check_file(path));
        }
    }

    // Exit with error code if any violations were found
    if !errors.is_empty() {
        println!("{}", errors.join("\n"));
        std::process::exit(1);
    }
}

