use crate::reference::ReferenceType;

#[derive(Clone, Debug, clap::ValueEnum)]
pub enum ReferenceTypeArg {
    SourceCode,
    Documentation,
    AgentInstruction,
}

impl From<ReferenceTypeArg> for ReferenceType {
    fn from(reference_type: ReferenceTypeArg) -> Self {
        match reference_type {
            ReferenceTypeArg::SourceCode => Self::SourceCode,
            ReferenceTypeArg::Documentation => Self::Documentation,
            ReferenceTypeArg::AgentInstruction => Self::AgentInstruction,
        }
    }
}
