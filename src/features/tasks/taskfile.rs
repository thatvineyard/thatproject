use serde::{Deserialize, Serialize};
use std::fs;
use super::slug::slugify;
use crate::app_context::AppContext; 

const FILE_EXT: &str = ".json";

#[derive(Debug, Deserialize, Serialize)]
pub struct TaskFile {
  pub name: String,
  pub slug: String,
  pub description: String,
}

impl TaskFile {
  pub fn create(context: &AppContext, name: String, description: String) -> std::io::Result<()> {

    let slug = slugify(&name);

    let task_file = TaskFile {
      name,
      slug,
      description,
    };

    task_file.write(context)
  }

  fn file_path(&self, context: &AppContext) -> std::io::Result<String> {
    let dir = context.get_taskfile_dir()?;
    Ok(format!("{}/{}{}", dir, self.slug, FILE_EXT))
  }

  fn write(&self, context: &AppContext) -> std::io::Result<()> {
    let json = serde_json::to_string_pretty(&self).map_err(|e| {
      std::io::Error::new(std::io::ErrorKind::InvalidData, e)
    })?;
    fs::write(&self.file_path(context)?, json)
  }
}
