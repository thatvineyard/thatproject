use crate::{
    app_context::AppContext,
    features::subproject::subproject::{Subproject, validate_value},
};

pub fn run(context: &mut AppContext, path: String, alias: Option<String>, note: Option<String>) {
    if let Err(err) = validate_value(context, &path) {
        eprintln!("Error: {}", err);
        std::process::exit(1);
    }

    let manifest = match context.require_manifest_mut() {
        Ok(manifest) => manifest,
        Err(error) => {
            eprintln!("Error: {}", error);
            std::process::exit(1);
        }
    };

    let subproject = Subproject::new(path.clone(), alias, note);

    if let Err(err) = manifest.add_subproject(subproject) {
        eprintln!("Error: {}", err);
        std::process::exit(1);
    }
    match manifest.store() {
        Ok(_) => println!("Subproject added: {}", path),
        Err(error) => {
            eprintln!("Error: {}", error);
            std::process::exit(1);
        }
    }
}
