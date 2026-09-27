pub mod taskfile;
mod slug;

use crate::{app_context::AppContext, features::tasks::taskfile::{TaskFile, TaskStatus}};

pub fn create_task(context: &AppContext, name: String, description: String) -> Result<TaskFile, Box<dyn std::error::Error>> {
  Ok(taskfile::TaskFile::create(context, name, description)?)
}

pub fn set_status(context: &AppContext, name: String, status: TaskStatus) -> Result<(), Box<dyn std::error::Error>> {
  let mut task = taskfile::TaskFile::load(context, name)?;
  task.header.status = status;
  task.write(context)?;
  Ok(())
}