mod commands;

use clap::Command;
use clap::CommandFactory;
use clap::Parser;
use commands::Commands;

use crate::app_context;

#[derive(Parser, Debug)]
#[command(name = "thatproject", version, about = "ThatProject")]
struct Cli {
    #[arg(short, long, global = true)]
    context_dir: Option<String>,
    #[arg(short, long, global = true)]
    agent_mode: bool,
    #[command(subcommand)]
    commands: Commands,
}

pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    if let Some(dir) = cli.context_dir {
        crate::config::set_project_dir(dir);
    }

    let context = app_context::load(cli.agent_mode)?;

    cli.commands.run(&context);

    Ok(())
}

pub fn command() -> Command {
    Cli::command()
}
