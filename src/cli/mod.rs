mod commands;

use commands::Commands;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(name = "rust-cli", version, about = "ThatProject")]
struct Cli {
    #[command(subcommand)]
    commands: Commands,
}

pub fn run() {
    let cli = Cli::parse();
    cli.commands.run();
}
