use crate::app_context::AppContext;

pub fn run(context: &AppContext, name: String, description: String) {
    match crate::features::tasks::create_task(context, name, description) {
        Ok(task) => println!("Task {} created", task.name),
        Err(err) => {
            eprintln!("Error: {}", err);
            std::process::exit(1);
        }
    }
}
