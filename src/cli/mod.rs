mod commands;

use commands::Commands;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(name = "rust-cli", version, about = "ThatProject")]
struct Cli {
    #[arg(short, long, global = true)]
    context_dir: Option<String>,
    #[command(subcommand)]
    commands: Commands,
}

pub fn run() {
    let cli = Cli::parse();

    if let Some(dir) = cli.context_dir {
        crate::config::set_project_dir(dir);
    }

    cli.commands.run();
}
