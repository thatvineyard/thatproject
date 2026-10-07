use serde::{Deserialize, Serialize};
use std::fmt;

/// The kind of content a reference points to.
#[derive(Debug, Deserialize, Serialize, Clone, Copy, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ReferenceType {
    SourceCode,
    Documentation,
    AgentInstruction,
}

impl fmt::Display for ReferenceType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let label = match self {
            Self::SourceCode => "source_code",
            Self::Documentation => "documentation",
            Self::AgentInstruction => "agent_instruction",
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

/// Returns true if the value is an http(s) URL.
pub fn is_url(value: &str) -> bool {
    value.starts_with("http://") || value.starts_with("https://")
}

/// Returns true if the value contains glob metacharacters.
pub fn is_glob(value: &str) -> bool {
    value.contains(['*', '?', '['])
}

/// Plain paths must exist. URLs and glob patterns are not checked.
pub fn validate_value(value: &str) -> Result<(), String> {
    if is_url(value) || is_glob(value) {
        return Ok(());
    }
    if std::path::Path::new(value).exists() {
        Ok(())
    } else {
        Err(format!("Path does not exist: {}", value))
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
