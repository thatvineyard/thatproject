use crate::app_context::AppContext;

pub fn run(context: &AppContext) {
    if let Err(err) = crate::features::tasks::list(context) {
        eprintln!("Error: {}", err);
        std::process::exit(1);
    }
}
