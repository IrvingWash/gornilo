use std::{
    env, fs,
    path::PathBuf,
    process::{self, Command},
};

use crate::{gornilo_config::GorniloConfig, helpers};

#[inline]
pub fn build_project(release: bool, run: bool, example: &Option<String>, config: &GorniloConfig) {
    if !helpers::is_in_project_dir() {
        eprintln!("The build command should be called from the project's root");
        process::exit(1);
    }

    let (souce_dir, build_dir) = make_souce_and_build_dirs(example, release);

    fs::create_dir_all(&build_dir).expect("Failed to create build directory");

    let mut odin_command = Command::new("odin");

    odin_command
        .arg(if run { "run" } else { "build" })
        .arg(souce_dir);

    helpers::add_vet_flags(&mut odin_command, &config.vet_flags);

    if !release {
        odin_command.arg("-debug");
    } else {
        odin_command.arg("-no-bounds-check").arg("-o:speed");
    }

    let build_file = build_dir.join(config.project.name.clone());

    odin_command.arg(format!("-out:{}", build_file.to_str().unwrap()));

    helpers::add_collections(&mut odin_command, &config.collections);

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

fn make_souce_and_build_dirs(example: &Option<String>, release: bool) -> (PathBuf, PathBuf) {
    let project_dir = env::current_dir().expect("Failed to get the project directory");

    match example {
        None => (
            project_dir.join("src"),
            if release {
                project_dir.join("build").join("release")
            } else {
                project_dir.join("build").join("debug")
            },
        ),
        Some(example) => (
            project_dir.join("examples").join(example).join("src"),
            if release {
                project_dir
                    .join("build")
                    .join("examples")
                    .join(example)
                    .join("release")
            } else {
                project_dir
                    .join("build")
                    .join("examples")
                    .join(example)
                    .join("debug")
            },
        ),
    }
}
