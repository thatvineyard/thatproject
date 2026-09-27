use crate::app_context::{AppContext, ProjectState};
use std::process::exit;

pub fn run(context: &AppContext, name: String, description: String, task_dir: String) {
  if let ProjectState::Valid(_) = context.project {
     println!("Project already initialized");
     exit(0); 
  }

  if let Err(err) = crate::features::initialization::create_manifest(name, description, task_dir) {
    eprintln!("Error: {}", err);
    std::process::exit(1);
  }

  println!("Project initialized")
}