use serde::{Deserialize, Serialize};
use std::fs;

use crate::app_context::AppContext;
use crate::reference::Reference;

#[derive(Debug, Deserialize, Serialize)]
pub struct Manifest {
    pub name: String,
    pub description: String,
    #[serde(default)]
    pub references: Vec<Reference>,
    pub task_dir: String,
}

impl Manifest {
    pub fn load() -> std::io::Result<Self> {
        let path = crate::config::get_manifest_path();
        let content = fs::read_to_string(&path)?;
        serde_json::from_str(&content)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))
    }

    pub fn create(name: String, description: String, task_dir: String) -> std::io::Result<()> {
        let manifest = Manifest {
            name,
            description,
            references: Vec::new(),
            task_dir,
        };
        let path = crate::config::get_manifest_path();
        let json = serde_json::to_string_pretty(&manifest)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;
        fs::create_dir_all(crate::config::get_thatproject_dir())?;
        fs::write(&path, json)
    }

    pub fn add_reference(_context: &AppContext, reference: Reference) -> std::io::Result<()> {
        let path = crate::config::get_manifest_path();
        let content = fs::read_to_string(&path)?;
        let mut manifest: Manifest = serde_json::from_str(&content)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;

        if !manifest.references.contains(&reference) {
            manifest.references.push(reference);
        }

        let json = serde_json::to_string_pretty(&manifest)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;
        fs::write(&path, json)
    }

    pub fn get_taskfile_dir(&self) -> String {
        format!("{}/{}", crate::config::get_thatproject_dir(), self.task_dir)
    }
}
