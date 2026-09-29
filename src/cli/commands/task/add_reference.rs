use crate::app_context::AppContext;
use crate::reference::ReferenceType;

#[derive(Clone, Debug, clap::ValueEnum)]
pub enum ReferenceTypeArg {
    SourceCode,
    Documentation,
}

impl From<ReferenceTypeArg> for ReferenceType {
    fn from(reference_type: ReferenceTypeArg) -> Self {
        match reference_type {
            ReferenceTypeArg::SourceCode => Self::SourceCode,
            ReferenceTypeArg::Documentation => Self::Documentation,
        }
    }
}

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
