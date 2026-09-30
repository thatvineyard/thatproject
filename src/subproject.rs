use serde::{Deserialize, Serialize};
use std::{error::Error, fmt, path::Path};

use crate::config;

/// A link to another thatproject folder
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
pub struct Subproject {
    pub path: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub note: Option<String>,
}

impl fmt::Display for Subproject {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.path)
    }
}

/// Path must exist, be a directory, exist within this project's folder and be a valid thatproject
pub fn validate_value(value: &str) -> Result<(), Box<dyn Error>> {
    let path = Path::new(value);

    if !path.exists() {
        return Err("Path does not exist".into());
    }

    if !path.is_dir() {
        return Err("Path is not a directory".into());
    }

    let path = path.canonicalize()?;
    let context_dir = Path::new(config::get_context_dir().as_str()).canonicalize()?;

    if path == context_dir {
        return Err("Path cannot be same as this project".into());
    }

    if !path.starts_with(context_dir) {
        return Err("Path was not a subdirectory of this projects".into());
    }

    Ok(())
}

impl Subproject {
    pub fn new(path: impl Into<String>, note: Option<String>) -> Self {
        Self {
            path: path.into(),
            note,
        }
    }
}
