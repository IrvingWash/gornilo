use std::fs::{File, create_dir_all};
use std::io::Write;
use std::path::PathBuf;
use std::process::Command;
use std::{env, fs};

mod gitignore_generator;
mod odin_code_generator;
mod ols_config_generator;

// TODO: replace with a proper config generator
const GORNILO_CONFIG_CONTENTS: &'static str = r#"[project]
name = "$project_name$"
"#;

pub struct CreateProjectParams {
    pub name: String,
    pub no_git: bool,
    pub no_ols: bool,
    pub no_mem_tracking: bool,
}

pub fn create_project(params: CreateProjectParams) {
    let CreateProjectParams {
        name,
        no_mem_tracking,
        no_git,
        no_ols,
    } = params;

    println!("Creating project \"{}\"...", name);

    let project_dir_path = create_project_dir(&name);

    if !no_git {
        init_git(&project_dir_path);
    }

    create_project_structure(&project_dir_path, &name, no_ols, no_mem_tracking);

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

    file.write_all(gitignore_generator::GITIGNORE_CONTENTS.as_bytes())
        .expect("Failed to write into .gitignore");
}

fn create_project_structure(
    project_dir_path: &PathBuf,
    project_name: &str,
    no_ols: bool,
    no_mem_tracking: bool,
) {
    // Main file
    {
        let src_path = project_dir_path.join("src");

        create_dir_all(&src_path).expect("Failed to create src directory");

        let mut main_file =
            File::create(src_path.join("main.odin")).expect("Failed to create main.odin file");

        main_file
            .write_all(if no_mem_tracking {
                odin_code_generator::BASIC_CODE.as_bytes()
            } else {
                odin_code_generator::MEM_TRACKING_CODE.as_bytes()
            })
            .expect("Failed to write main file contents");
    }

    // Gornilo config file
    {
        let mut config_file = File::create(project_dir_path.join("gornilo.toml"))
            .expect("Failed to create Gornilo config file");

        config_file
            .write_all(
                GORNILO_CONFIG_CONTENTS
                    .replace("$project_name$", project_name) // TODO: replace with proper config generator
                    .as_bytes(),
            )
            .expect("Failed to write Gornilo config contents");
    }

    // OLS & Odin Format
    if !no_ols {
        // OLS
        {
            let mut ols_file = File::create(project_dir_path.join("ols.json"))
                .expect("Failed to create OLS confing file");

            ols_file
                .write_all(ols_config_generator::OLS_CONFING_CONTENTS.as_bytes())
                .expect("Failed to write OLS config contents");
        }

        // Odin Format
        {
            let mut odinfmt_file = File::create(project_dir_path.join("odinfmt.json"))
                .expect("Failed to create Odin Format config file");

            odinfmt_file
                .write_all(ols_config_generator::ODINFMT_CONFING_CONTENTS.as_bytes())
                .expect("Failed to write Odin Format config contents");
        }
    }
}
