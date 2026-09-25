mod greet;

use clap::Subcommand;

#[derive(Subcommand, Debug)]
pub enum Commands {
  Greet {
    #[arg(short, long, default_value = "world")]
    name: String,
  },
}

impl Commands {
  pub fn run(self) {
    match self {
      Commands::Greet { name } => greet::run(name),
    }
  }
}