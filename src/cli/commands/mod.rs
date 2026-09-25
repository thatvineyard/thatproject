mod greet;
mod init; 

use clap::Subcommand;

#[derive(Subcommand, Debug)]
pub enum Commands {
  Greet {
    #[arg(short, long, default_value = "world")]
    name: String,
  },
  Init {
    #[arg(short, long)]
    name: String,
    #[arg(short, long, default_value = "")]
    description: String,
  },
}

impl Commands {
  pub fn run(self) {
    match self {
      Commands::Greet { name } => greet::run(name),
      Commands::Init { name, description } => init::run(name, description),
    }
  }
}