mod add_reference;
mod read;
mod set_body;
mod set_category;
mod set_description;
mod set_status;
mod set_title;

use crate::cli::commands::reference_type_arg::ReferenceTypeArg;
use crate::{app_context::AppContext, cli::commands::task::set_status::TaskStatusArg};
use clap::{Args, Subcommand};

#[derive(Subcommand, Debug)]
pub enum TaskActions {
    /// Sets the title of a given task
    #[command(long_about = "Sets the title in the task file header.")]
    SetTitle {
        #[arg()]
        title: String,
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
    /// Adds a reference to a given task
    #[command(long_about = "Adds a reference to the task file header.")]
    AddReference {
        #[arg()]
        reference: String,
        #[arg(long, ignore_case = true)]
        r#type: ReferenceTypeArg,
        #[arg(long)]
        note: Option<String>,
    },
}

#[derive(Args, Debug)]
pub struct TaskCommand {
    // Task key (abcd:###)
    #[arg(value_name = "KEY")]
    key: String,
    #[command(subcommand)]
    action: TaskActions,
}

impl TaskCommand {
    pub fn run(self, context: &AppContext) {
        match self.action {
            TaskActions::SetTitle { title } => set_title::run(context, self.key, title),
            TaskActions::SetDescription { description } => {
                set_description::run(context, self.key, description)
            }
            TaskActions::SetCategory { category } => set_category::run(context, self.key, category),
            TaskActions::SetBody { body } => set_body::run(context, self.key, body),
            TaskActions::SetStatus { status } => set_status::run(context, self.key, status),
            TaskActions::Read { json } => read::run(context, self.key, json),
            TaskActions::AddReference {
                reference,
                r#type,
                note,
            } => add_reference::run(context, self.key, reference, r#type, note),
        }
    }
}
