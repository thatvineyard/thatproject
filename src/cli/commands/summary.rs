use std::process::exit;

use crate::app_context::AppContext;

pub fn run(context: &AppContext) {
    let project = match context.require_project() {
        Ok(project) => project,
        Err(error) => {
            eprintln!("Error: {}", error);
            exit(1);
        }
    };

    match project.to_summary(context) {
        Err(error) => eprintln!("Error: {}", error),
        Ok(output) => println!("{}", output),
    };
}
