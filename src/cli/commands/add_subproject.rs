use std::path::PathBuf;

use crate::{
    app_context::AppContext,
    features::subproject::subproject::{SubprojectReference, validate_value},
    tools::path::relativize,
};

pub fn run(context: &mut AppContext, path: PathBuf, alias: Option<String>, note: Option<String>) {
    if let Err(err) = validate_value(context, &path) {
        eprintln!("Error: {}", err);
        std::process::exit(1);
    }

    let path = match relativize(&context.root_dir(), &path) {
        Ok(path) => path,
        Err(error) => {
            eprintln!("Error: {}", error);
            std::process::exit(1);
        }
    };

    let project = match context.require_project_mut() {
        Ok(project) => project,
        Err(error) => {
            eprintln!("Error: {}", error);
            std::process::exit(1);
        }
    };

    let subproject = SubprojectReference::new(path.to_path_buf(), alias, note);

    if let Err(err) = project.manifest.add_subproject(subproject) {
        eprintln!("Error: {}", err);
        std::process::exit(1);
    }

    match project.manifest.store() {
        Ok(_) => println!("Subproject added: {}", path.display()),
        Err(error) => {
            eprintln!("Error: {}", error);
            std::process::exit(1);
        }
    }
}
