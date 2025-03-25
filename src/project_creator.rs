use std::fs::{File, create_dir_all};
use std::io::Write;
use std::path::PathBuf;
use std::process::Command;
use std::{env, fs};

const GITIGNORE_ENTRIES: &'static str = "build
.DS_Store
";

const MAIN_FILE_CONTENT: &'static str = r#"package main

import "core:fmt"

main :: proc() {
    fmt.println("Hellope!")
}
"#;

pub fn create_project(name: &str, no_git: bool) {
    println!("Creating project \"{}\"...", name);

    let project_dir_path = create_project_dir(name);

    if !no_git {
        init_git(&project_dir_path);
    }

    create_project_structure(&project_dir_path);

    println!("Finished");
}

fn create_project_dir(name: &str) -> PathBuf {
    let project_dir_path = env::current_dir()
        .expect("Failed to get the current directory path")
        .join(name);

    fs::create_dir_all(&project_dir_path).expect("Failed to create project directory");

    project_dir_path
}

fn init_git(project_dir_path: &PathBuf) {
    Command::new("git")
        .arg("init")
        .arg(project_dir_path)
        .output()
        .expect("Failed to initialize a git repository");

    let gitignore_path = project_dir_path.join(".gitignore");

    let mut file = File::create(gitignore_path).expect("Failed to create .gitignore");

    file.write_all(GITIGNORE_ENTRIES.as_bytes())
        .expect("Failed to write into .gitignore");
}

fn create_project_structure(project_dir_path: &PathBuf) {
    let src_path = project_dir_path.join("src");

    create_dir_all(&src_path).expect("Failed to create src directory");

    let mut main_file =
        File::create(src_path.join("main.odin")).expect("Failed to create main.odin file");

    main_file
        .write_all(MAIN_FILE_CONTENT.as_bytes())
        .expect("Failed to write main file contents");
}
