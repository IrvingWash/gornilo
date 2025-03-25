use clap::{Parser, Subcommand};

use crate::project_creator;

#[derive(Parser)]
#[command(version, about)]
pub struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    New {
        #[arg(value_name = "project name")]
        name: String,
        #[arg(long)]
        no_git: Option<bool>,
        #[arg(long)]
        no_ols: Option<bool>,
    },
    Build,
    Run,
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
                } => {
                    project_creator::create_project(
                        &name,
                        no_git.unwrap_or_default(),
                        no_ols.unwrap_or_default(),
                    );
                }
                Commands::Build => println!("`build` is not supported yet"),
                Commands::Run => println!("`run` is not supported yet"),
            }
        }
    }
}
