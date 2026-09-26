use std::path::Path;

use crate::app_context::AppContext;

pub fn run(context: &AppContext, directory: String) {
  if !Path::new(&directory).exists() {
    eprintln!("Error: {}", format!("Directory does not exist: {}", directory));
    std::process::exit(1);
  }

  if let Err(err) = crate::manifest::Manifest::add_source(context, directory.clone()) {
    eprintln!("Error: {}", err);
    std::process::exit(1);
  }

  println!("Source added: {}", directory);
}
