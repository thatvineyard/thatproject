use crate::app_context::AppContext;
use crate::cli::commands::reference_type_arg::ReferenceTypeArg;
use crate::features::manifest::manifest::Manifest;
use crate::reference::{Reference, ReferenceType, validate_value};

pub fn run(
    context: &AppContext,
    value: String,
    reference_type: ReferenceTypeArg,
    note: Option<String>,
) {
    if let Err(err) = validate_value(&value) {
        eprintln!("Error: {}", err);
        std::process::exit(1);
    }

    let reference = Reference::new(value.clone(), ReferenceType::from(reference_type), note);

    if let Err(err) = Manifest::add_reference(context, reference) {
        eprintln!("Error: {}", err);
        std::process::exit(1);
    }

    println!("Reference added: {}", value);
}
