mod skills;

use std::{env, error::Error, fs};

const DOCS_CLI_PATH: &'static str = "docs/CLI.md";

fn main() -> Result<(), Box<dyn Error>> {
    match env::args().nth(1).as_deref() {
        Some("docs") => generate_docs()?,
        Some("agent-skills") => skills::generate_agent_skills()?,
        _ => return Err("usage: cargo xtask docs".into()),
    }
    Ok(())
}

fn generate_docs() -> std::io::Result<()> {
    let command = thatproject::cli::command();
    let markdown = clap_markdown::help_markdown_command(&command);
    fs::create_dir_all("docs")?;
    fs::write(DOCS_CLI_PATH, markdown)
}
