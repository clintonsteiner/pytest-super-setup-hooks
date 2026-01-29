// src/main.rs
use std::{env, path::Path};
use pytest_super_hooks::{check_file, fix::fix_file};

fn main() {
    let mut fix = false;
    let mut errors = Vec::new();

    for arg in env::args().skip(1) {
        if arg == "--fix" {
            fix = true;
            continue;
        }
        if arg.ends_with(".py") {
            let path = Path::new(&arg);
            if fix {
                fix_file(path);
            }
            errors.extend(check_file(path));
        }
    }

    if !errors.is_empty() {
        println!("{}", errors.join("\n"));
        std::process::exit(1);
    }
}

