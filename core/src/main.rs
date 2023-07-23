#[macro_use]
extern crate log;

mod create;
mod build;
mod deploy;
mod info;
mod git;
mod utils;
mod models;

use clap::{Parser, Subcommand};
use create::create_project;
use build::build_project;
use deploy::deploy_project;

use crate::{models::Context, git::{pull_repo, push_repo}};


#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Create,
    Build,
}

fn main() {
    env_logger::init();

    let cli = Cli::parse();
    
    match &cli.command {
        Commands::Create => {
            let context = Context::from_stdin();
            info!("Creating cli with context {:?}", context);
            create_project(&context);
            push_repo("", "");
            build_project(&context);
            deploy_project(&context);
        }
        Commands::Build => {
            let context = Context::from_stdin();
            info!("Building with context {:?}", context);
            pull_repo("", "");
            build_project(&context);
            deploy_project(&context);
        }
    }
}
