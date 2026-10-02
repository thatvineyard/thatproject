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
    // Access the project in the given folder
    #[arg(short, long, global = true)]
    context_dir: Option<PathBuf>,
    #[arg(short, long, global = true)]
    agent_mode: bool,
    #[command(subcommand)]
    commands: Commands,
}

pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    let mut context = app_context::load(cli.context_dir, cli.agent_mode)?;

    cli.commands.run(&mut context);

    Ok(())
}

pub fn command() -> Command {
    Cli::command()
}
