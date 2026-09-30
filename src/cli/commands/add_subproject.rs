use crate::app_context::AppContext;
use crate::subproject::{Subproject, validate_value};

pub fn run(context: &AppContext, path: String, note: Option<String>) {
    if let Err(err) = validate_value(&path) {
        eprintln!("Error: {}", err);
        std::process::exit(1);
    }

    let subproject = Subproject::new(path.clone(), note);

    if let Err(err) = crate::manifest::Manifest::add_subproject(context, subproject) {
        eprintln!("Error: {}", err);
        std::process::exit(1);
    }

    println!("Subproject added: {}", path);
}
