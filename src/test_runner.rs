use std::{
    env, fs,
    path::PathBuf,
    process::{self, Command},
};

use crate::{gornilo_config::GorniloConfig, helpers};

#[inline]
pub fn run_tests(source_path: Option<String>, all_packages: bool, config: &GorniloConfig) {
    if !helpers::is_in_project_dir() {
        eprintln!("The test command should be called from the project's root");
        process::exit(1);
    }

    let (source_dir, build_dir) = make_source_and_build_dirs(source_path, config);

    fs::create_dir_all(&build_dir).expect("Failed to create build directory");

    let mut odin_command = Command::new("odin");

    odin_command.arg("test").arg(source_dir);

    if all_packages {
        odin_command.arg("--all-packages");
    }

    odin_command.arg(format!("-out:{}", build_dir.join("test").to_str().unwrap()));

    helpers::add_vet_flags(&mut odin_command, &config.vet_flags);
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

fn make_source_and_build_dirs(
    source_path: Option<String>,
    config: &GorniloConfig,
) -> (PathBuf, PathBuf) {
    let current_dir = env::current_dir().expect("Failed to get the project directory");

    let source_path = match source_path {
        Some(source_path) => source_path,
        None => {
            if config.testing.source_path.is_none() {
                eprintln!(
                    "Package path is not provide nor through through the argument nor in the config"
                );
                process::exit(1);
            };
            config.testing.source_path.clone().unwrap()
        }
    };

    let source_dir = current_dir.join(source_path);

    let build_dir = current_dir.join("build").join("tests");

    (source_dir, build_dir)
}
