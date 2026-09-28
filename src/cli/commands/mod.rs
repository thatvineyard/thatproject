mod add_source;
mod init;
mod task;
mod tasks;

use crate::{
    app_context::AppContext,
    cli::commands::{task::TaskCommand, tasks::TasksCommands},
};
use clap::Subcommand;

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Initialize a ThatProject workspace.
    #[command(
        long_about = "Creates the necessary files and directories to enable this directory as a thatproject project."
    )]
    Init {
        #[arg(short, long)]
        name: String,
        #[arg(short, long, default_value = "")]
        description: String,
        #[arg(long, default_value = "tasks")]
        task_dir: String,
    },

    /// Adds a source to the project
    #[command(
        long_about = "Add a source directory to the manifest so it can be included in source commands."
    )]
    AddSource {
        #[arg(short, long)]
        source: String,
    },

    // Manage the collection of tasks
    #[command(subcommand)]
    Tasks(TasksCommands),

    // Manage a specific task
    Task(TaskCommand),
}

impl Commands {
    pub fn run(self, context: &AppContext) {
        match self {
            Commands::Init {
                name,
                description,
                task_dir,
            } => init::run(context, name, description, task_dir),
            Commands::AddSource { source } => add_source::run(context, source),
            Commands::Tasks(command) => command.run(context),
            Commands::Task(command) => command.run(context),
        }
    }
}
