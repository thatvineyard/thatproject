use crate::app_context::AppContext;
use crate::cli::commands::reference_type_arg::ReferenceTypeArg;
use crate::reference::{Reference, ReferenceType, validate_value};

pub fn run(
    context: &mut AppContext,
    value: String,
    reference_type: ReferenceTypeArg,
    note: Option<String>,
) {
    if let Err(err) = validate_value(&value) {
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

    let reference = Reference::new(value.clone(), ReferenceType::from(reference_type), note);

    if let Err(err) = project.manifest.add_reference(reference) {
        eprintln!("Error: {}", err);
        std::process::exit(1);
    }

    match project.manifest.store() {
        Ok(_) => println!("Reference added: {}", value),
        Err(error) => {
            eprintln!("Error: {}", error);
            std::process::exit(1);
        }
    }
}
