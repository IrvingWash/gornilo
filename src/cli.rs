use clap::{Parser, Subcommand};

use crate::{
    gornilo_config, project_builder,
    project_creator::{self, CreateProjectParams},
    test_runner,
};

#[derive(Parser)]
#[command(version, about)]
pub struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    New {
        #[arg(value_name = "PROJECT NAME")]
        name: String,
        #[arg(long)]
        no_git: bool,
        #[arg(long)]
        no_ols: bool,
        #[arg(long)]
        no_mem_tracking: bool,
        #[arg(long)]
        no_workflows: bool,
    },
    Build {
        #[arg(long, short)]
        release: bool,
        #[arg(long, short)]
        example: Option<String>,
    },
    Run {
        #[arg(long, short)]
        release: bool,
        #[arg(long, short)]
        example: Option<String>,
    },
    Clean,
    Test {
        #[arg(long, short)]
        source_path: Option<String>,
        #[arg(long, short)]
        all_packages: bool,
    },
}

impl Cli {
    #[inline]
    pub fn run() {
        let args = Cli::parse();

        if let Some(command) = args.command {
            match command {
                Commands::New {
                    name,
                    no_git,
                    no_ols,
                    no_mem_tracking,
                    no_workflows,
                } => {
                    project_creator::create_project(CreateProjectParams {
                        name,
                        no_ols,
                        no_git,
                        no_mem_tracking,
                        no_workflows,
                    });
                }
                Commands::Build { release, example } => {
                    let config = gornilo_config::parse_config();
                    project_builder::build_project(release, false, &example, &config);
                }
                Commands::Run { release, example } => {
                    let config = gornilo_config::parse_config();
                    project_builder::build_project(release, true, &example, &config);
                }
                Commands::Clean => project_builder::clean_project(),
                Commands::Test {
                    source_path,
                    all_packages,
                } => {
                    let config = gornilo_config::parse_config();
                    test_runner::run_tests(source_path, all_packages, &config);
                }
            }
        }
    }
}
