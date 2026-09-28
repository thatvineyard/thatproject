mod set_status;

use crate::{app_context::AppContext, cli::commands::task::set_status::TaskStatusArg};
use clap::{Args, Subcommand};

#[derive(Subcommand, Debug)]
pub enum TaskActions {
    /// Sets the status of a given task
    #[command(long_about = "Sets the status field in the task file.")]
    SetStatus {
        #[arg(ignore_case = true)]
        status: TaskStatusArg,
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
            TaskActions::SetStatus { status } => set_status::run(context, self.name, status),
        }
    }
}
