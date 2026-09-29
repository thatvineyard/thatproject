use serde::{Deserialize, Serialize};
use std::fmt;

/// The kind of content a reference points to.
#[derive(Debug, Deserialize, Serialize, Clone, Copy, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ReferenceType {
    SourceCode,
    Documentation,
}

impl fmt::Display for ReferenceType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let label = match self {
            Self::SourceCode => "source_code",
            Self::Documentation => "documentation",
        };
        f.write_str(label)
    }
}

/// A reference to external content (e.g. a path or URL) alongside its type.
/// Defined as its own type since it is used in multiple contexts
/// (e.g. project manifest references, task references).
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
pub struct Reference {
    pub value: String,
    #[serde(rename = "type")]
    pub r#type: ReferenceType,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub note: Option<String>,
}

impl fmt::Display for Reference {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl Reference {
    pub fn new(value: impl Into<String>, r#type: ReferenceType, note: Option<String>) -> Self {
        Self {
            value: value.into(),
            r#type,
            note,
        }
    }
}
