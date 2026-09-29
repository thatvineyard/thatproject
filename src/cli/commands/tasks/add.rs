use crate::app_context::AppContext;

pub fn run(context: &AppContext, name: String, category: Option<String>, description: String) {
    match crate::features::tasks::create_task(context, name, category, description) {
        Ok(task) => println!("Task {} created", task.header.name),
        Err(err) => {
            eprintln!("Error: {}", err);
            std::process::exit(1);
        }
    }
}
