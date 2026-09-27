pub mod taskfile;
mod slug;
mod store;

use crate::{app_context::AppContext, features::tasks::{self, taskfile::{TaskFile, TaskStatus}}};

pub fn create_task(context: &AppContext, name: String, description: String) -> Result<TaskFile, Box<dyn std::error::Error>> {
  let task_file = taskfile::TaskFile::create(name, description)?;
  tasks::store::store(context, &task_file)?;
  Ok(task_file)
}

pub fn set_status(context: &AppContext, name: String, status: TaskStatus) -> Result<(), Box<dyn std::error::Error>> {
  let mut task_file = tasks::store::load(context, name)?;
  task_file.header.status = status;
  tasks::store::store(context, &task_file)?;
  Ok(())
}