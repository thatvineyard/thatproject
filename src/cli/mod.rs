mod commands;

use std::path::PathBuf;

use clap::Command;
use clap::CommandFactory;
use clap::Parser;
use commands::Commands;

use crate::app_context;

#[derive(Parser, Debug)]
#[command(name = "thatproject", version, about = "ThatProject")]
struct Cli {
    #[arg(
        long,
        global = true,
        help = "Select a directory as context",
        long_help = "Select a directory as context for all actions to be run on. Note: this must be a thatproject-enabled directory, not the .thatproject folder"
    )]
    context_dir: Option<PathBuf>,

    #[arg(
        long,
        global = true,
        help = "Select a subproject as context",
        long_help = "Select a subproject as context for all actions to be run on."
    )]
    subproject: Option<String>,

    #[arg(
        short,
        long,
        global = true,
        help = "Activate to output data in a agent friendly format."
    )]
    agent_mode: bool,

    #[command(subcommand)]
    commands: Commands,
}

pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    let mut context = app_context::load(cli.context_dir, cli.subproject, cli.agent_mode)?;

    cli.commands.run(&mut context);

    Ok(())
}

pub fn command() -> Command {
    Cli::command()
}
