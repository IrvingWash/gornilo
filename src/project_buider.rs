use std::{
    env, fs,
    process::{self, Command},
};

use crate::helpers;

#[inline]
pub fn build_project(release: bool, run: bool) {
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

    if !release {
        odin_command.arg("-debug");
    } else {
        odin_command.arg("-no-bounds-check").arg("-o:speed");
    }

    let build_file = build_dir.join("exe.out");

    odin_command.arg(format!("-out:{}", build_file.to_str().unwrap()));

    println!("{:?}", odin_command);

    odin_command.output().unwrap();
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
