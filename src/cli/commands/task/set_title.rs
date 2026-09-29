use crate::app_context::AppContext;

pub fn run(context: &AppContext, key: String, title: String) {
    if let Err(err) = crate::features::tasks::set_title(context, key, title) {
        eprintln!("Error: {}", err);
        std::process::exit(1);
    }
}
