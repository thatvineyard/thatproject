use crate::app_context::AppContext;

pub fn run(context: &AppContext, name: String, category: String) {
    if let Err(err) = crate::features::tasks::set_category(context, name, category) {
        eprintln!("Error: {}", err);
        std::process::exit(1);
    }
}
