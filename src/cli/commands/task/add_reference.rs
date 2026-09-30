use crate::app_context::AppContext;
use crate::cli::commands::reference_type_arg::ReferenceTypeArg;
use crate::reference::ReferenceType;

pub fn run(
    context: &AppContext,
    key: String,
    reference: String,
    reference_type: ReferenceTypeArg,
    note: Option<String>,
) {
    if let Err(err) = crate::features::tasks::add_reference(
        context,
        key,
        reference,
        ReferenceType::from(reference_type),
        note,
    ) {
        eprintln!("Error: {}", err);
        std::process::exit(1);
    }
}
