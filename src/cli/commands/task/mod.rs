mod read;
mod set_body;
mod set_category;
mod set_description;
mod set_name;
mod set_status;

use crate::{app_context::AppContext, cli::commands::task::set_status::TaskStatusArg};
use clap::{Args, Subcommand};

#[derive(Subcommand, Debug)]
pub enum TaskActions {
    /// Sets the name of a given task
    #[command(long_about = "Sets the name in the task file header.")]
    SetName {
        #[arg()]
        name: String,
    },
    /// Sets the description of a given task
    #[command(long_about = "Sets the description in the task file header.")]
    SetDescription {
        #[arg()]
        description: String,
    },
    /// Sets the category of a given task
    #[command(long_about = "Sets the category in the task file header.")]
    SetCategory {
        #[arg()]
        category: String,
    },
    /// Sets the body of a given task
    #[command(long_about = "Sets the body in the task file.")]
    SetBody {
        #[arg()]
        body: String,
    },
    /// Sets the status of a given task
    #[command(long_about = "Sets the status field in the task file header.")]
    SetStatus {
        #[arg(ignore_case = true)]
        status: TaskStatusArg,
    },
    /// Reads the given task
    #[command(long_about = "Output the contents of the file")]
    Read {
        // Output in json. (Always enabled if --agent-mode is true)
        #[arg(long)]
        json: bool,
    },
}

#[derive(Args, Debug)]
pub struct TaskCommand {
    // Task name
    #[arg(value_name = "NAME")]
    name: String,
    #[command(subcommand)]
    action: TaskActions,
}

impl TaskCommand {
    pub fn run(self, context: &AppContext) {
        match self.action {
            TaskActions::SetName { name } => set_name::run(context, self.name, name),
            TaskActions::SetDescription { description } => {
                set_description::run(context, self.name, description)
            }
            TaskActions::SetCategory { category } => {
                set_category::run(context, self.name, category)
            }
            TaskActions::SetBody { body } => set_body::run(context, self.name, body),
            TaskActions::SetStatus { status } => set_status::run(context, self.name, status),
            TaskActions::Read { json } => read::run(context, self.name, json),
        }
    }
}
