pub mod taskfile;
mod slug;
mod store;

use std::error::Error;

use crate::{app_context::AppContext, features::tasks::{self, taskfile::{TaskFile, TaskStatus}}};

pub fn create_task(context: &AppContext, name: String, description: String) -> Result<TaskFile, Box<dyn Error>> {
  let task_file = taskfile::TaskFile::create(name, description)?;
  tasks::store::store(context, &task_file)?;
  Ok(task_file)
}

pub fn set_status(context: &AppContext, name: String, status: TaskStatus) -> Result<(), Box<dyn Error>> {
  let mut task_file = tasks::store::load(context, name)?;
  task_file.header.status = status;
  tasks::store::store(context, &task_file)?;
  Ok(())
}

pub fn list(context: &AppContext) -> Result<(), Box<dyn Error>> {
  let tasks = tasks::store::list(context)?;

  if tasks.is_empty() {
    println!("No tasks found");
    return Ok(())
  }

  for task in tasks {
    println!("{}", task.to_one_liner());
  };

  Ok(())
}