pub mod taskfile;
mod slug;

use crate::{app_context::AppContext, features::tasks::taskfile::TaskStatus};

pub fn create_task(context: &AppContext, name: String, description: String) -> Result<(), Box<dyn std::error::Error>> {
  taskfile::TaskFile::create(context, name, description)?;
  Ok(())
}

pub fn set_status(context: &AppContext, name: String, status: TaskStatus) -> Result<(), Box<dyn std::error::Error>> {
  let mut task = taskfile::TaskFile::load(context, name)?;
  task.status = status;
  task.write(context)?;
  Ok(())
}