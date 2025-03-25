use clap::{Parser, Subcommand};

use crate::{
    gornilo_config, project_buider,
    project_creator::{self, CreateProjectParams},
};

#[derive(Parser)]
#[command(version, about)]
pub struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,

    #[arg(long, short)]
    verbose: bool,
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
                Commands::Build {
                    release,
                    example: _,
                } => {
                    let config = gornilo_config::parse_config();
                    project_buider::build_project(release, false, config);
                }
                Commands::Run {
                    release,
                    example: _,
                } => {
                    let config = gornilo_config::parse_config();
                    project_buider::build_project(release, true, config);
                }
                Commands::Clean => project_buider::clean_project(),
            }
        }
    }
}
