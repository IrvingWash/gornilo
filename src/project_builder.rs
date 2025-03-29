use std::{
    env, fs,
    process::{self, Command},
};

use crate::{
    gornilo_config::{GorniloConfig, VetFlagsConfig},
    helpers,
};

#[inline]
pub fn build_project(release: bool, run: bool, config: GorniloConfig) {
    if !helpers::is_in_project_dir() {
        eprintln!("The build command should be called from the project's root");
        process::exit(1);
    }

    let project_dir = env::current_dir().expect("Failed to get the project directory");

    let souce_dir = project_dir.join("src");
    let build_dir = project_dir.join("build");

    fs::create_dir_all(&build_dir).expect("Failed to create build directory");

    let mut odin_command = Command::new("odin");

    odin_command
        .arg(if run { "run" } else { "build" })
        .arg(souce_dir);

    add_vet_flags(&mut odin_command, &config.vet_flags);

    if !release {
        odin_command.arg("-debug");
    } else {
        odin_command.arg("-no-bounds-check").arg("-o:speed");
    }

    let build_file = build_dir.join(config.project.name);

    odin_command.arg(format!("-out:{}", build_file.to_str().unwrap()));

    println!("{:?}", odin_command);

    let output = odin_command
        .output()
        .expect("Failed to execute Odin command");

    if output.status.success() {
        print!(
            "{}",
            String::from_utf8(output.stdout).expect("Failed to get stdout of Odin build command")
        );
    } else {
        eprint!(
            "{}",
            String::from_utf8(output.stderr).expect("Failed to get stderr of Odin build command")
        );
    }
}

#[inline]
pub fn clean_project() {
    if !helpers::is_in_project_dir() {
        eprintln!("The clean command should be called from the project's root");
        process::exit(1);
    }

    let build_dir = env::current_dir()
        .expect("Failed to get the project directory")
        .join("build");

    fs::remove_dir_all(build_dir).expect("Failed to remove build directory");
}

fn add_vet_flags(command: &mut Command, vet_flags: &VetFlagsConfig) {
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
