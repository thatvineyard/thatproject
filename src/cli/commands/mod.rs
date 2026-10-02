mod add_reference;
mod add_subproject;
mod init;
mod reference_type_arg;
mod summary;
mod task;
mod tasks;

use crate::{
    app_context::AppContext,
    cli::commands::{
        reference_type_arg::ReferenceTypeArg, task::TaskCommand, tasks::TasksCommands,
    },
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

    /// Summmarize project
    #[command(long_about = "List manifest details")]
    Summary,

    /// Adds a reference to the project
    #[command(
        long_about = "Add a reference (directory, file, glob pattern or URL) to the manifest. Plain paths must exist."
    )]
    AddReference {
        #[arg()]
        reference: String,
        #[arg(long, ignore_case = true)]
        r#type: ReferenceTypeArg,
        #[arg(long)]
        note: Option<String>,
    },

    /// Adds a subproject to the project
    #[command(
        long_about = "Add a subproject to the manifest. Path must be a directory within this project's directory"
    )]
    AddSubproject {
        #[arg()]
        path: String,
        #[arg(long)]
        note: Option<String>,
    },

    // Manage the collection of tasks
    #[command(subcommand)]
    Tasks(TasksCommands),

    // Manage a specific task
    Task(TaskCommand),
}

impl Commands {
    pub fn run(self, context: &mut AppContext) {
        match self {
            Commands::Init {
                name,
                description,
                task_dir,
            } => init::run(context, name, description, task_dir),
            Commands::Summary => summary::run(context),
            Commands::AddReference {
                reference,
                r#type,
                note,
            } => add_reference::run(context, reference, r#type, note),
            Commands::AddSubproject { path, note } => add_subproject::run(context, path, note),
            Commands::Tasks(command) => command.run(context),
            Commands::Task(command) => command.run(context),
        }
    }
}
