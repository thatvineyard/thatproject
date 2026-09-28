use crate::app_context::AppContext;

pub fn run(context: &AppContext, name: String, body: String) {
    if let Err(err) = crate::features::tasks::set_body(context, name, body) {
        eprintln!("Error: {}", err);
        std::process::exit(1);
    }
}
