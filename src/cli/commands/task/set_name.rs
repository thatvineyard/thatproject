use crate::app_context::AppContext;

pub fn run(context: &AppContext, name: String, new_name: String) {
    if let Err(err) = crate::features::tasks::set_name(context, name, new_name) {
        eprintln!("Error: {}", err);
        std::process::exit(1);
    }
}
