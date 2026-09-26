use serde::{Deserialize, Serialize};
use std::fs;
use super::slug::slugify;

const FILE_EXT: &str = ".json";

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

    task_file.write()
  }

  fn file_path(&self) -> String {
    let dir = crate::config::get_taskfile_dir();
    format!("{}/{}{}", dir, self.slug, FILE_EXT)
  }

  fn write(&self) -> std::io::Result<()> {
    let json = serde_json::to_string_pretty(&self).map_err(|e| {
      std::io::Error::new(std::io::ErrorKind::InvalidData, e)
    })?;
    fs::write(&self.file_path(), json)
  }
}
