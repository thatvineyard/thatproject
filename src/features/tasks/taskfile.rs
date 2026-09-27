use serde::{Deserialize, Serialize};
use serde;
use std::error::Error;
use std::fs;
use super::slug::slugify;
use crate::app_context::AppContext;
use crate::tools::{self, frontmatter}; 

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
  pub header: TaskFileHeader,
  pub body: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TaskFileHeader {
  pub name: String,
  pub description: String,
  pub status: TaskStatus,
}

impl TaskFile {
  pub fn create(context: &AppContext, name: String, description: String) -> std::io::Result<TaskFile> {
    let task_file = TaskFile {
      header: TaskFileHeader {
        name,
        description,
        status: TaskStatus::Draft,
      },
      body: "".to_string(),
    };

    let _ = task_file.write(context);
    Ok(task_file)
  }

  pub fn load(context: &AppContext, name: String) -> Result<Self, Box<dyn Error>> {
      let path = TaskFile::file_path(context, &name)?;
      let data = fs::read_to_string(path)?;
      let (header, body) = frontmatter::from_frontmatter_markdown::<TaskFileHeader>(&data)?;
      let result = TaskFile { header, body };
      Ok(result)
  }

  fn file_path(context: &AppContext, name: &String) -> std::io::Result<String> {
    let slug = slugify(name);
    let dir = context.get_taskfile_dir()?;
    Ok(format!("{}/{}{}", dir, slug, FILE_EXT))
  }

  pub fn write(&self, context: &AppContext) -> Result<(), Box<dyn Error>> {
    let data = tools::frontmatter::to_frontmatter_markdown(&self.header, &self.body)?;
    fs::create_dir_all(context.get_taskfile_dir()?)?;
    fs::write(&TaskFile::file_path(context, &self.header.name)?, data)?;

    Ok(())
  }
}
