use std::path::Path;

pub fn run(directory: String) -> Result<(), Box<dyn std::error::Error>> {
  if !Path::new(&directory).exists() {
    return Err(format!("Directory does not exist: {}", directory).into());
  }

  crate::manifest::Manifest::add_source(directory.clone())?;
  println!("Source added: {}", directory);
  Ok(())
}
