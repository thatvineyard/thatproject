use crate::app_context::AppContext;
use std::process::exit;

pub fn run(context: &AppContext, name: String, description: String, task_dir: String) {
    if let Some(_) = context.project_state {
        println!("Project already initialized");
        exit(0);
    }

    if let Err(err) = crate::features::initialization::create_manifest(
        &context.get_thatproject_dir(),
        name,
        description,
        task_dir,
    ) {
        eprintln!("Error: {}", err);
        std::process::exit(1);
    }

    println!("Project initialized")
}
