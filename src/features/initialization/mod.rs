use std::fs;
use std::path::Path;
use manifest::Manifest; 

pub mod manifest;

pub fn create_manifest(name: String, description: String) -> Result<Manifest, Box<dyn std::error::Error>> {
  let manifest = Manifest {
    name,
    description,
    sources: Vec::new(),
  };

  let project_dir = ".thatproject";
  if !Path::new(project_dir).exists() {
    fs::create_dir(project_dir)?;
  }

  let manifest_path = format!("{}/manifest.json", project_dir);
  let json = serde_json::to_string_pretty(&manifest)?;
  fs::write(&manifest_path, json)?;

  Ok(manifest)
}