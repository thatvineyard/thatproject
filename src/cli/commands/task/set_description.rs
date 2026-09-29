use crate::app_context::AppContext;

pub fn run(context: &AppContext, key: String, description: String) {
    if let Err(err) = crate::features::tasks::set_description(context, key, description) {
        eprintln!("Error: {}", err);
        std::process::exit(1);
    }
}
