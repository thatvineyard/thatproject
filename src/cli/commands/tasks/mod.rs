mod add;
mod list;

use crate::app_context::AppContext;
use clap::Subcommand;

#[derive(Subcommand, Debug)]
pub enum TasksCommands {
    /// Adds a task to the project
    #[command(long_about = "Creates a task file in the task folder defined in the manifest.")]
    Add {
        #[arg()]
        name: String,
        #[arg(long)]
        category: Option<String>,
        #[arg(short, long, default_value = "")]
        description: String,
    },

    /// Lists all tasks
    #[command()]
    List {},
}

impl TasksCommands {
    pub fn run(self, context: &AppContext) {
        match self {
            TasksCommands::Add {
                name,
                category,
                description,
            } => add::run(context, name, category, description),
            TasksCommands::List {} => list::run(context),
        }
    }
}
