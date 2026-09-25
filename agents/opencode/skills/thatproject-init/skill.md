---
description: Initialize a new project with name and description
type: skill
---

# thatproject-init

Initialize a new project using the `init` command.

## Usage

```bash
thatproject init --name <NAME> [--description <DESCRIPTION>]
```

## Arguments

- `--name` (required): The name of the project being initialized
- `--description` (optional, default: empty string): A description for the project

## Functionality

Creates a the necessary project files under .thatproject

## Example

```bash
thatproject init --name my-project --description "A sample project"
```

This initializes a new project named "my-project" with the given description.
