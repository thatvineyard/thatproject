use std::process::exit;

use crate::{app_context::AppContext, features::tasks};

pub fn run(context: &AppContext, name: String, json: bool) {
    let task = tasks::read(context, name).unwrap_or_else(|error| {
        eprintln!("Error when reading taskfile: {}", error);
        exit(1);
    });

    let result = if json || context.agent_mode {
        task.to_json()
    } else {
        task.to_data()
    };

    let output = result.unwrap_or_else(|error| {
        eprintln!("Error when formatting taskfile: {}", error);
        exit(1);
    });

    println!("{}", output);
}
