use crate::app_context::AppContext;

pub fn run(context: &AppContext, name: String, description: String) {
  if let Err(err) = crate::features::tasks::create_task(context, name, description) {
    eprintln!("Error: {}", err);
    std::process::exit(1);
  }
}