use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use std::str::FromStr;

use crate::features::manifest::manifest_error::ManifestError;
use crate::features::subproject::subproject::SubprojectReference;
use crate::reference::Reference;

const MANIFEST_FILE_NAME: &str = "manifest.json";

#[derive(Debug, Deserialize, Serialize)]
pub struct ManifestFile {
    pub name: String,
    pub description: String,
    #[serde(default)]
    pub subprojects: Vec<SubprojectReference>,
    #[serde(default)]
    pub references: Vec<Reference>,
    pub task_dir: String,
}

impl FromStr for ManifestFile {
    type Err = ManifestError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(serde_json::from_str(&s).map_err(|e| ManifestError::Json(e))?)
    }
}

impl ManifestFile {
    pub fn to_string(&self) -> Result<String, ManifestError> {
        let content = serde_json::to_string_pretty(&self).map_err(|e| ManifestError::Json(e))?;

        Ok(content)
    }

    pub fn path_in(thatproject_dir: &Path) -> PathBuf {
        thatproject_dir.join(MANIFEST_FILE_NAME)
    }
}
