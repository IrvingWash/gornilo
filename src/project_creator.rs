use std::fs::{File, create_dir_all};
use std::io::Write;
use std::path::PathBuf;
use std::process::Command;
use std::{env, fs};

const GITIGNORE_ENTRIES: &'static str = r#"build
.DS_Store
"#;

const MAIN_FILE_CONTENTS: &'static str = r#"package main

import "core:fmt"

main :: proc() {
    fmt.println("Hellope!")
}

"#;

const GORNILO_CONFIG_CONTENTS: &'static str = r#"[test]"#;

const OLS_CONFING_CONTENTS: &'static str = r#"{
	"$schema": "https://raw.githubusercontent.com/DanielGavin/ols/master/misc/ols.schema.json",
	"enable_format": true,
	"enable_semantic_tokens": true,
	"enable_document_symbols": true,
	"enable_hover": true,
	"enable_snippets": true
}
"#;

const ODINFMT_CONFING_CONTENTS: &'static str = r#"{
	"$schema": "https://raw.githubusercontent.com/DanielGavin/ols/master/misc/odinfmt.schema.json",
	"character_width": 80,
	"newline_limit": 1,
	"tabs": true,
	"tabs_width": 4
	"sort_imports": false,
}
"#;

pub fn create_project(name: &str, no_git: bool, no_ols: bool) {
    println!("Creating project \"{}\"...", name);

    let project_dir_path = create_project_dir(name);

    if !no_git {
        init_git(&project_dir_path);
    }

    create_project_structure(&project_dir_path, no_ols);

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

fn create_project_structure(project_dir_path: &PathBuf, no_ols: bool) {
    // Main file
    {
        let src_path = project_dir_path.join("src");

        create_dir_all(&src_path).expect("Failed to create src directory");

        let mut main_file =
            File::create(src_path.join("main.odin")).expect("Failed to create main.odin file");

        main_file
            .write_all(MAIN_FILE_CONTENTS.as_bytes())
            .expect("Failed to write main file contents");
    }

    // Gornilo config file
    {
        let mut config_file = File::create(project_dir_path.join("gornilo.toml"))
            .expect("Failed to create Gornilo config file");

        config_file
            .write_all(GORNILO_CONFIG_CONTENTS.as_bytes())
            .expect("Failed to write Gornilo config contents");
    }

    // OLS & Odin Format
    if !no_ols {
        // OLS
        {
            let mut ols_file = File::create(project_dir_path.join("ols.json"))
                .expect("Failed to create OLS confing file");

            ols_file
                .write_all(OLS_CONFING_CONTENTS.as_bytes())
                .expect("Failed to write OLS config contents");
        }

        // Odin Format
        {
            let mut odinfmt_file = File::create(project_dir_path.join("odinfmt.json"))
                .expect("Failed to create Odin Format config file");

            odinfmt_file
                .write_all(ODINFMT_CONFING_CONTENTS.as_bytes())
                .expect("Failed to write Odin Format config contents");
        }
    }
}
