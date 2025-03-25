use clap::{Parser, Subcommand};

use crate::project_creator::{self, CreateProjectParams};

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
    },
    Build {
        #[arg(long, short)]
        release: bool,
    },
    Run {
        #[arg(long, short)]
        release: bool,
    },
}

impl Cli {
    pub fn run() {
        let args = Cli::parse();

        if let Some(command) = args.command {
            match command {
                Commands::New {
                    name,
                    no_git,
                    no_ols,
                    no_mem_tracking,
                } => {
                    project_creator::create_project(CreateProjectParams {
                        name,
                        no_ols,
                        no_git,
                        no_mem_tracking,
                    });
                }
                Commands::Build { release: _ } => println!("`build` is not supported yet"),
                Commands::Run { release: _ } => println!("`run` is not supported yet"),
            }
        }
    }
}
