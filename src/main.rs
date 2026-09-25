mod commands;

use clap::Parser;
use commands::Commands;

#[derive(Parser, Debug)]
#[command(name = "rust-cli", version, about = "ThatProject")]
struct Cli {
    #[command(subcommand)]
    commands: Commands,
}

fn main() {
    let cli = Cli::parse();
    cli.commands.run();
}