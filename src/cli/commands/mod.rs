mod init; 
mod add_source;
mod task;

use clap::Subcommand;
use crate::app_context::AppContext;
use crate::cli::commands::task::set_status::TaskStatusArg;

#[derive(Subcommand, Debug)]
pub enum Commands {
  /// Initialize a ThatProject workspace.
  #[command(long_about = "Creates the necessary files and directories to enable this directory as a thatproject project.")]
  Init {
    #[arg(short, long)]
    name: String,
    #[arg(short, long, default_value = "")]
    description: String,
    #[arg(long, default_value = "tasks")]
    task_dir: String,
  },
  
  /// Adds a source to the project
  #[command(long_about = "Add a source directory to the manifest so it can be included in source commands.")]
  AddSource {
    #[arg(short, long)]
    source: String,
  },

  /// Adds a task to the project
  #[command(long_about = "Creates a task file in the task folder defined in the manifest.")]
  TaskAdd {
    #[arg(short, long)]
    name: String,
    #[arg(short, long, default_value = "")]
    description: String,
  },

  /// Sets the status of a given task
  #[command(long_about = "Sets the status field in the task file.")]
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
      Commands::Init { name, description, task_dir } => init::run(context, name, description, task_dir),
      Commands::AddSource { source } => add_source::run(context, source),
      Commands::TaskAdd { name, description } => task::add::run(context, name, description),
      Commands::TaskSetStatus { name, status } => task::set_status::run(context, name, status),
    }
  }
}