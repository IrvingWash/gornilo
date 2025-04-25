use std::{collections::HashMap, env, fs, process::Command};

use crate::gornilo_config::VetFlagsConfig;

#[inline]
pub fn is_in_project_dir() -> bool {
    let current_dir = env::current_dir().expect("Failed to get the current directory path");

    fs::exists(current_dir.join("gornilo.toml"))
        .expect("Failed to check if the command was called from the project root")
}

#[inline]
pub fn add_vet_flags(command: &mut Command, vet_flags: &VetFlagsConfig) {
    if vet_flags.warnings_as_errors {
        command.arg("-warnings-as-errors");
    }
    if vet_flags.unused_variables {
        command.arg("-vet-unused-variables");
    }
    if vet_flags.unused_imports {
        command.arg("-vet-unused-imports");
    }
    if vet_flags.tabs {
        command.arg("-vet-tabs");
    }
    if vet_flags.style {
        command.arg("-vet-style");
    }
    if vet_flags.semicolon {
        command.arg("-vet-semicolon");
    }
    if vet_flags.cast {
        command.arg("-vet-cast");
    }
}

#[inline]
pub fn add_collections(command: &mut Command, collections: &HashMap<String, String>) {
    for (name, path) in collections {
        command.arg(format!("-collection:{name}={path}"));
    }
}
