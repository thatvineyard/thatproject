use crate::features::tasks::taskfile::TaskStatus;

use crate::app_context::AppContext;

#[derive(Clone, Debug, clap::ValueEnum)]
pub enum TaskStatusArg {
    Draft,
    Ongoing,
    Complete,
}

impl From<TaskStatusArg> for TaskStatus {
    fn from(status: TaskStatusArg) -> Self {
        match status {
            TaskStatusArg::Draft => Self::Draft,
            TaskStatusArg::Ongoing => Self::Ongoing,
            TaskStatusArg::Complete => Self::Complete,
        }
    }
}

pub fn run(context: &AppContext, name: String, status: TaskStatusArg) {
    if let Err(err) = crate::features::tasks::set_status(context, name, TaskStatus::from(status)) {
        eprintln!("Error: {}", err);
        std::process::exit(1);
    }
}
