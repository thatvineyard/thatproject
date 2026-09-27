use serde::{Deserialize, Serialize};
use serde;
use std::error::Error;
use crate::tools::{self, frontmatter}; 

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
  pub fn create(name: String, description: String) -> std::io::Result<TaskFile> {
    let task_file = TaskFile {
      header: TaskFileHeader {
        name,
        description,
        status: TaskStatus::Draft,
      },
      body: "".to_string(),
    };
    Ok(task_file)
  }

  pub fn from_data(data: String) -> Result<TaskFile, Box<dyn Error + 'static>> {
      let (header, body) = frontmatter::from_frontmatter_markdown::<TaskFileHeader>(&data)?;
      let result = TaskFile { header, body };
      Ok(result)
  }

  pub fn to_data(&self) -> Result<String, Box<dyn Error>> {
    Ok(tools::frontmatter::to_frontmatter_markdown(&self.header, &self.body)?)
  }

  pub fn name(&self) -> std::string::String {
    self.header.name.clone()
  }
}
