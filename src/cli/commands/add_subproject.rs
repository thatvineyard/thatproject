use crate::{
    app_context::AppContext,
    features::subproject::subproject::{SubprojectReference, validate_value},
};

pub fn run(context: &mut AppContext, path: String, alias: Option<String>, note: Option<String>) {
    if let Err(err) = validate_value(context, &path) {
        eprintln!("Error: {}", err);
        std::process::exit(1);
    }

    let project = match context.require_project_mut() {
        Ok(project) => project,
        Err(error) => {
            eprintln!("Error: {}", error);
            std::process::exit(1);
        }
    };

    let subproject = SubprojectReference::new(path.clone(), alias, note);

    if let Err(err) = project.manifest.add_subproject(subproject) {
        eprintln!("Error: {}", err);
        std::process::exit(1);
    }

    match project.manifest.store() {
        Ok(_) => println!("Subproject added: {}", path),
        Err(error) => {
            eprintln!("Error: {}", error);
            std::process::exit(1);
        }
    }
}
