use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::{env, fs};

use crate::gornilo_config::GorniloConfig;

mod github_workflow_generator;
mod gitignore_generator;
mod odin_code_generator;
mod ols_config_generator;

pub struct CreateProjectParams {
    pub name: String,
    pub no_git: bool,
    pub no_ols: bool,
    pub no_mem_tracking: bool,
    pub no_workflows: bool,
}

#[inline]
pub fn create_project(params: CreateProjectParams) {
    let CreateProjectParams {
        name,
        no_mem_tracking,
        no_git,
        no_ols,
        no_workflows,
    } = params;

    println!("Creating project \"{}\"...", name);

    let project_dir_path = create_project_dir(&name);

    if !no_git {
        init_git(&project_dir_path);
    }

    create_project_structure(
        &project_dir_path,
        &name,
        no_ols,
        no_mem_tracking,
        no_workflows,
    );

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
    project_dir_path: &Path,
    project_name: &str,
    no_ols: bool,
    no_mem_tracking: bool,
    no_workflows: bool,
) {
    // Main file
    {
        let src_path = project_dir_path.join("src");

        fs::create_dir_all(&src_path).expect("Failed to create src directory");

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
                toml::to_string_pretty(&GorniloConfig::default(project_name))
                    .unwrap()
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

    // Github Workflows
    if !no_workflows {
        let workflows_path = project_dir_path.join(".github/workflows");

        fs::create_dir_all(&workflows_path).expect("Failed to create Github Workflows dir");

        let mut odin_workflow_path = File::create(workflows_path.join("odin.yml"))
            .expect("Failed to create Odin workflow file");

        odin_workflow_path
            .write_all(github_workflow_generator::GITHUB_WORKFLOW_CONTENTS.as_bytes())
            .expect("Failed to write Odin Workflow contents");
    }
}
