pub mod taskfile;
mod slug;

use crate::app_context::AppContext;

pub fn create_task(context: &AppContext, name: String, description: String) -> Result<(), Box<dyn std::error::Error>> {
  taskfile::TaskFile::create(context, name, description)?;
  Ok(())
}