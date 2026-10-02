use serde::{Deserialize, Serialize};
use std::{fmt, io, path::Path};

use crate::{
    app_context::AppContext,
    config,
    features::manifest::{manifest::Manifest, manifest_error::ManifestError::NotFound},
};

#[derive(Debug, thiserror::Error)]
pub enum SubProjectError {
    #[error("Validation error: {0}")]
    Validation(String),
    #[error("{0}")]
    Other(String),
    #[error(transparent)]
    Io(#[from] io::Error),
}

impl From<String> for SubProjectError {
    fn from(s: String) -> Self {
        SubProjectError::Other(s)
    }
}

impl From<&str> for SubProjectError {
    fn from(s: &str) -> Self {
        SubProjectError::Other(s.to_string())
    }
}

/// A link to another thatproject folder
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
pub struct Subproject {
    pub path: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub alias: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub note: Option<String>,
}

impl fmt::Display for Subproject {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.path)
    }
}

/// Path must exist, be a directory, exist within this project's folder and be a valid thatproject
pub fn validate_value(context: &AppContext, value: &str) -> Result<(), SubProjectError> {
    let path = Path::new(value).canonicalize()?;
    let context_dir = Path::new(&context.context_dir).canonicalize()?;

    if !path.exists() {
        return Err(SubProjectError::Validation("Path does not exist".into()));
    }

    if !path.is_dir() {
        return Err(SubProjectError::Validation(
            "Path is not a directory".into(),
        ));
    }

    if path == context_dir {
        return Err(SubProjectError::Validation(
            "Path cannot be same as this project".into(),
        ));
    }

    if !path.starts_with(context_dir) {
        return Err(SubProjectError::Validation(
            "Path was not a subdirectory of this projects".into(),
        ));
    }

    let _ = Manifest::load(&path.join(config::PROJECT_DIR)).map_err(|e| match e {
        NotFound(_) => SubProjectError::Validation(e.to_string()),
        _ => SubProjectError::Validation("Subproject's Manifest is not valid".to_string()),
    })?;

    Ok(())
}

impl Subproject {
    pub fn new(path: impl Into<String>, alias: Option<String>, note: Option<String>) -> Self {
        Self {
            path: path.into(),
            alias,
            note,
        }
    }
}
