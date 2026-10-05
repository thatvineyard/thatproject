use crate::{
    app_context::AppContext,
    features::tasks::{ListFilters, taskfile::TaskStatus},
};

pub fn run(context: &AppContext, status_filter: Option<TaskStatus>) {
    let filters = ListFilters {
        status: status_filter,
    };
    if let Err(err) = crate::features::tasks::list(context, filters) {
        eprintln!("Error: {}", err);
        std::process::exit(1);
    }
}
