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

            let data = if context.agent_mode {
                match serde_json::to_string(manifest) {
                    Ok(data) => data,
                    Err(err) => {
                        eprintln!("Error: {}", err);
                        exit(1);
                    }
                }
            } else {
                match serde_yaml::to_string(manifest) {
                    Ok(data) => data,
                    Err(err) => {
                        eprintln!("Error: {}", err);
                        exit(1);
                    }
                }
            };

            println!("{}", data)
        }
    }
}
