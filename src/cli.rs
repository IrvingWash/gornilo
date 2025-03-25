use clap::{Parser, Subcommand};

use crate::{
    project_buider,
    project_creator::{self, CreateProjectParams},
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
    },
    Run {
        #[arg(long, short)]
        release: bool,
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
                Commands::Build { release } => project_buider::build_project(release),
                Commands::Run { release: _ } => todo!(),
                Commands::Clean => project_buider::clean_project(),
            }
        }
    }
}
