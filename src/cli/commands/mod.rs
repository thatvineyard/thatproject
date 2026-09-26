mod greet;
mod init; 
mod add_source;
mod add_task;

use clap::Subcommand;
use crate::app_context::AppContext;

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
    #[arg(long, default_value = "tasks")]
    task_dir: String,
  },
  AddSource {
    #[arg(short, long)]
    source: String,
  },
  AddTask {
    #[arg(short, long)]
    name: String,
    #[arg(short, long, default_value = "")]
    description: String,
  },
}

impl Commands {
  pub fn run(self, context: &AppContext) {
    match self {
      Commands::Greet { name } => greet::run(context, name),
      Commands::Init { name, description, task_dir } => init::run(context, name, description, task_dir),
      Commands::AddSource { source } => add_source::run(context, source),
      Commands::AddTask { name, description } => add_task::run(context, name, description),
    }
  }
}