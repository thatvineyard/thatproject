mod commands;

use clap::Command;
use commands::Commands;
use clap::Parser;
use clap::CommandFactory;

use crate::app_context;

#[derive(Parser, Debug)]
#[command(name = "thatproject", version, about = "ThatProject")]
struct Cli {
    #[arg(short, long, global = true)]
    context_dir: Option<String>,
    #[command(subcommand)]
    commands: Commands,
}

pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    if let Some(dir) = cli.context_dir {
        crate::config::set_project_dir(dir);
    }

    let context = app_context::load()?;

    cli.commands.run(&context);

    Ok(())
}

pub fn command() -> Command {
    Cli::command()
}