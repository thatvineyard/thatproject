<p align="center">
  <img src="resources/logo/thatproject.svg" alt="ThatProject" width="300">
</p>

# ThatProject

A command-line tool for creating and managing a ThatProject manifest.

## Usage

Run commands from the project directory with Cargo:

```sh
cargo run -- init --name my-project --description "My project"
cargo run -- add-source --source ./src
```

Available commands:

| Commands | Description |
| ------------ | -- |
| `init` | create a manifest with a name and optional description. |
| `add-source` | add an existing directory to the manifest. |

## OpenCode skills

Enable the ThatProject skills by adding this to your OpenCode config:

```json
"skills": [
  "https://raw.githubusercontent.com/thatvineyard/thatproject/refs/heads/main/agents/opencode/skills"
]
```
