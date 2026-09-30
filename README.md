<p align="center">
  <img src="resources/logo/thatproject.svg" alt="ThatProject" width="300">
</p>

# ThatProject

A command-line tool for creating and managing a ThatProject manifest.

## Usage

Run commands from the project directory with Cargo:

```sh
cargo run -- init --name my-project --description "My project"
cargo run -- add-reference ./src --type source-code
```

Available commands:

| Commands | Description |
| ------------ | -- |
| `init` | create a manifest with a name and optional description. |
| `add-reference` | add a directory, file, glob pattern or URL to the manifest. |

## OpenCode skills

Enable the ThatProject skills by adding this to your OpenCode config:

```json
"skills": [
  "https://raw.githubusercontent.com/thatvineyard/thatproject/refs/heads/main/agents/opencode/skills"
]
```


## Build tools

To build agent skill files run:

`cargo xtask agent-skills`

To build CLI docs run:

`cargo xtask docs`
