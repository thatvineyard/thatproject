use std::path::Path;

use crate::features::manifest::manifest::Manifest;

pub fn create_manifest(
    thatproject_dir: &Path,
    name: String,
    description: String,
    task_dir: String,
) -> Result<Manifest, Box<dyn std::error::Error>> {
    let manifest = Manifest::create(thatproject_dir, name.clone(), description, task_dir)?;
    manifest.store()?;

    Ok(manifest)
}
