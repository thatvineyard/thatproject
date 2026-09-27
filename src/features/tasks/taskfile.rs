use serde::{Deserialize, Serialize};
use serde;
use std::fs;
use super::slug::slugify;
use crate::app_context::AppContext; 

const FILE_EXT: &str = ".json";

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum TaskStatus {
  Draft,
  Ongoing,
  Complete,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TaskFile {
  pub name: String,
  pub description: String,
  pub status: TaskStatus,
}

impl TaskFile {
  pub fn create(context: &AppContext, name: String, description: String) -> std::io::Result<TaskFile> {
    let task_file = TaskFile {
      name,
      description,
      status: TaskStatus::Draft,
    };

    let _ = task_file.write(context);
    Ok(task_file)
  }

  pub fn load(context: &AppContext, name: String) -> std::io::Result<Self> {
      let path = TaskFile::file_path(context, &name)?;
      let json = fs::read_to_string(path)?;
      serde_json::from_str(&json).map_err(|e| {
          std::io::Error::new(std::io::ErrorKind::InvalidData, e)
      })
  }

  fn file_path(context: &AppContext, name: &String) -> std::io::Result<String> {
    let slug = slugify(name);
    let dir = context.get_taskfile_dir()?;
    Ok(format!("{}/{}{}", dir, slug, FILE_EXT))
  }

  pub fn write(&self, context: &AppContext) -> std::io::Result<()> {
    let json = serde_json::to_string_pretty(&self).map_err(|e| {
      std::io::Error::new(std::io::ErrorKind::InvalidData, e)
    })?;
    fs::create_dir_all(context.get_taskfile_dir()?)?;
    fs::write(&TaskFile::file_path(context, &self.name)?, json)
  }
}
