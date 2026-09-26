use crate::manifest::Manifest;

pub fn create_manifest(name: String, description: String, task_dir: String) -> Result<(), Box<dyn std::error::Error>> {
  Manifest::create(name.clone(), description, task_dir)?;
  Manifest::load()?;
  
  Ok(())
}
