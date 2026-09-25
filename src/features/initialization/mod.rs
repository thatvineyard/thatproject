use crate::manifest::Manifest;

pub fn create_manifest(name: String, description: String) -> Result<Manifest, Box<dyn std::error::Error>> {
  Manifest::create(name.clone(), description)?;
  Ok(Manifest::load()?)
}
