use std::{env, error::Error, fs};

const SKILLS_DIR: &'static str = "agents/opencode/skills";
const SKILLS_INDEX_FILENAME: &'static str = "index.json";
const DOCS_CLI_PATH: &'static str = "docs/CLI.md";

fn main() -> Result<(), Box<dyn Error>> {
    match env::args().nth(1).as_deref() {
        Some("docs") => generate_docs()?,
        Some("agent-skills") => generate_agent_skills()?,
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

#[derive(serde::Serialize)]
struct SkillsIndex {
    skills: Vec<SkillEntry>,
}

#[derive(serde::Serialize)]
struct SkillEntry {
    name: String,
    version: String,
    files: Vec<String>,
}

fn generate_agent_skills() -> std::io::Result<()> {
    let root_command = thatproject::cli::command();
    fs::create_dir_all(SKILLS_DIR)?;

    let mut skills: Vec<SkillEntry> = Vec::new();

    for subcommand in root_command.get_subcommands() {
        let command_name = subcommand.get_name();
        let skill_name = format!("thatproject-{command_name}");
        let description = subcommand
            .get_about()
            .map(ToString::to_string)
            .unwrap_or_default();
        let git_rev = std::process::Command::new("git")
          .args(["rev-parse", "HEAD"])
          .output()?;
        let hash = String::from_utf8_lossy(&git_rev.stdout)
            .trim()
            .to_owned();

        let markdown_options = clap_markdown::MarkdownOptions::new()
            .title(skill_name.clone())
            .show_footer(false)
            .show_table_of_contents(false)
            .show_aliases(false);

        let markdown = clap_markdown::help_markdown_command_custom(subcommand, &markdown_options);

        let intro_to_be_replaced = format!(
            "This document contains the help content for the `{command_name}` command-line program."
        );
        let intro = format!("Use the following subcommand of `thatproject` to manage a thatproject project. E.g. if the subcommand is `init` then run `thatproject init`");
        let markdown = markdown.replacen(&intro_to_be_replaced, &intro, 1);

        let frontmatter = format!(
            r#"---
name: {skill_name}
description: {description}
---"#
        );

        let content = format!(
            r#"{frontmatter}
          
{markdown}
          "#
        );

        let dir = format!("{}/{}", SKILLS_DIR, skill_name);
        let file_name = format!("{}.md", skill_name);
        let path = format!("{}/{}", dir, file_name);
        fs::create_dir_all(dir)?;
        fs::write(path, content)?;

        skills.push(SkillEntry {
            name: skill_name,
            version: hash,
            files: vec![file_name],
        });
    }

    let json = serde_json::to_string_pretty(&SkillsIndex { skills })?;
    let index_path = format!("{}/{}", SKILLS_DIR, SKILLS_INDEX_FILENAME);
    fs::write(index_path, json)?;

    Ok(())
}
