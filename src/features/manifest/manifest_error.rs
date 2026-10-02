use std::{io, path::PathBuf};

#[derive(Debug, thiserror::Error)]
pub enum ManifestError {
    #[error("No project at {}", .0.display())]
    NotFound(PathBuf),
    #[error(transparent)]
    Io(#[from] io::Error),
    #[error(transparent)]
    Yaml(#[from] serde_yaml::Error),
    #[error(transparent)]
    Json(#[from] serde_json::Error),
}
