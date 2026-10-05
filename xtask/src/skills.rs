use std::fs;

use thatproject::tools::frontmatter::to_frontmatter_markdown;

const SKILLS_DIR: &'static str = "agents/opencode/skills";
const SKILLS_INDEX_FILENAME: &'static str = "index.json";
const SKILLS_INTO: &'static str = concat!(
    "Use the following subcommand of `thatproject` to manage a thatproject project. ",
    "Always run with --agent-mode as a global flag.",
    "E.g. if the subcommand is `init` then run `thatproject --agent-mode init`",
    "Use --subproject <subproject> (after --agent-mode for permission reasons) to peform actions on that subproject",
);

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

#[derive(serde::Serialize)]
struct SkillHeader {
    name: String,
    description: String,
}

pub fn generate_agent_skills() -> std::result::Result<(), Box<dyn std::error::Error>> {
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
        let hash = String::from_utf8_lossy(&git_rev.stdout).trim().to_owned();

        let markdown_options = clap_markdown::MarkdownOptions::new()
            .title(skill_name.clone())
            .show_footer(false)
            .show_table_of_contents(false)
            .show_aliases(false);

        let markdown = clap_markdown::help_markdown_command_custom(subcommand, &markdown_options);

        let intro_to_be_replaced = format!(
            "This document contains the help content for the `{command_name}` command-line program."
        );

        let markdown = markdown.replacen(&intro_to_be_replaced, SKILLS_INTO, 1);

        let content = to_frontmatter_markdown(
            &SkillHeader {
                name: skill_name.clone(),
                description,
            },
            &markdown,
        )?;

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
