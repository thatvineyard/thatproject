use crate::app_context::AppContext;

pub fn run(context: &AppContext, key: String, body: String) {
    if let Err(err) = crate::features::tasks::set_body(context, key, body) {
        eprintln!("Error: {}", err);
        std::process::exit(1);
    }
}
