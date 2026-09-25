use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Debug, Deserialize, Serialize)]
pub struct Manifest {
  pub name: String,
  pub description: String,
  pub sources: Vec<String>,
}

impl Manifest {
  pub fn load() -> std::io::Result<Self> {
    let path = crate::config::get_manifest_path();
    let content = fs::read_to_string(&path)?;
    serde_json::from_str(&content).map_err(|e| {
      std::io::Error::new(std::io::ErrorKind::InvalidData, e)
    })
  }

  pub fn create(name: String, description: String) -> std::io::Result<()> {
    let manifest = Manifest {
      name,
      description,
      sources: Vec::new(),
    };
    let path = crate::config::get_manifest_path();
    let json = serde_json::to_string_pretty(&manifest).map_err(|e| {
      std::io::Error::new(std::io::ErrorKind::InvalidData, e)
    })?;
    fs::write(&path, json)
  }

  pub fn add_source(source: String) -> std::io::Result<()> {
    let path = crate::config::get_manifest_path();
    let content = fs::read_to_string(&path)?;
    let mut manifest: Manifest = serde_json::from_str(&content).map_err(|e| {
      std::io::Error::new(std::io::ErrorKind::InvalidData, e)
    })?;

    if !manifest.sources.contains(&source) {
      manifest.sources.push(source);
    }

    let json = serde_json::to_string_pretty(&manifest).map_err(|e| {
      std::io::Error::new(std::io::ErrorKind::InvalidData, e)
    })?;
    fs::write(&path, json)
  }
}
