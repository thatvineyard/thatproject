use crate::app_context::AppContext;

pub fn run(context: &AppContext, title: String, category: Option<String>, description: String) {
    match crate::features::tasks::create_task(context, title, category, description) {
        Ok(task) => println!("Task {} created", task.header.title),
        Err(err) => {
            eprintln!("Error: {}", err);
            std::process::exit(1);
        }
    }
}
