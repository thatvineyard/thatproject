use crate::app_context::{AppContext, ProjectState};
use std::process::exit;

pub fn run(context: &AppContext) {
    match &context.project {
        ProjectState::Unset => {
            println!("Project is not initialized");
            exit(0);
        }
        ProjectState::Valid(manifest) => {
            let manifest = &manifest.manifest;

            match manifest.to_string(context.agent_mode) {
                Err(error) => eprintln!("Error: {}", error),
                Ok(data) => println!("{}", data),
            }
        }
    }
}
