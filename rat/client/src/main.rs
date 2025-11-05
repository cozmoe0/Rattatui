use clap::{Parser, Subcommand};

mod api;
mod cli;
mod config;
mod error;

pub use error::Error;

use crate::config::Config;

#[derive(Parser)]
#[command(name = env!("CARGO_PKG_NAME"))]
#[command(version = env!("CARGO_PKG_VERSION"))]
#[command(about = env!("CARGO_PKG_DESCRIPTION"), long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// List all agents
    Agents,
    /// Generates a new identity keypair
    Identity,
    /// Execute a command
    Exec {
        /// The agent id to execute the command on
        #[arg(short, long)]
        agent: String,
        
        /// The command to execute, with its arguments
        command: String,
    },
}

fn main() -> Result<(), anyhow::Error> {
    let cli = Cli::parse();
    
    let api_client = api::Client::new(config::SERVER_URL.to_string());

    match cli.command {
        Commands::Agents => {
            cli::agents::run(&api_client)?;
        }
        Commands::Identity => {
            cli::identity::run();
        }
        Commands::Exec { agent, command } => {
            let conf = Config::load()?;
            cli::exec::run(&api_client, &agent, &command, conf)?;
        }
    }

    Ok(())
}
