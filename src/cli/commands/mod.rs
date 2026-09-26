mod greet;
mod init; 
mod add_source;
mod task;

use clap::Subcommand;
use crate::app_context::AppContext;
use crate::cli::commands::task::set_status::TaskStatusArg;

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
  TaskAdd {
    #[arg(short, long)]
    name: String,
    #[arg(short, long, default_value = "")]
    description: String,
  },
  TaskSetStatus {
    #[arg(short, long)]
    name: String,
    #[arg(short, long, value_enum, ignore_case = true)]
    status: TaskStatusArg,
  },
}

impl Commands {
  pub fn run(self, context: &AppContext) {
    match self {
      Commands::Greet { name } => greet::run(context, name),
      Commands::Init { name, description, task_dir } => init::run(context, name, description, task_dir),
      Commands::AddSource { source } => add_source::run(context, source),
      Commands::TaskAdd { name, description } => task::add::run(context, name, description),
      Commands::TaskSetStatus { name, status } => task::set_status::run(context, name, status),
    }
  }
}