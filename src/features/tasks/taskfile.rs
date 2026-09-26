use serde::{Deserialize, Serialize};
use std::fs;
use super::slug::slugify;

#[derive(Debug, Deserialize, Serialize)]
pub struct TaskFile {
  pub name: String,
  pub slug: String,
  pub description: String,
}

impl TaskFile {
  pub fn create(name: String, description: String) -> std::io::Result<()> {

    let slug = slugify(&name);

    let task_file = TaskFile {
      name,
      slug,
      description,
    };
    let dir = crate::config::get_taskfile_dir();
    let path = format!("{}/{}.task", dir, task_file.slug);
    let json = serde_json::to_string_pretty(&task_file).map_err(|e| {
      std::io::Error::new(std::io::ErrorKind::InvalidData, e)
    })?;
    fs::write(&path, json)
  }
}
