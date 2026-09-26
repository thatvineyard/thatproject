pub mod taskfile;
mod slug;

pub fn create_task(name: String, description: String) -> Result<(), Box<dyn std::error::Error>> {
  taskfile::TaskFile::create(name, description)?;
  Ok(())
}