use std::path::Path;

pub fn run(directory: String) {
  if !Path::new(&directory).exists() {
    eprintln!("Error: {}", format!("Directory does not exist: {}", directory));
    std::process::exit(1);
  }

  if let Err(err) = crate::manifest::Manifest::add_source(directory.clone()) {
    eprintln!("Error: {}", err);
    std::process::exit(1);
  }

  println!("Source added: {}", directory);
}
