use crate::app_context::AppContext;

pub fn run(context: &AppContext, key: String, category: String) {
    if let Err(err) = crate::features::tasks::set_category(context, key, category) {
        eprintln!("Error: {}", err);
        std::process::exit(1);
    }
}
