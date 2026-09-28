---
name: thatproject-tasks
description: ''
---
# thatproject-tasks

Use the following subcommand of `thatproject` to manage a thatproject project. Always run with --agent-mode as a global flag.E.g. if the subcommand is `init` then run `thatproject --agent-mode init`

## `tasks`

**Usage:** `tasks <COMMAND>`

###### **Subcommands:**

* `add` — Adds a task to the project
* `list` — Lists all tasks



## `tasks add`

Creates a task file in the task folder defined in the manifest.

**Usage:** `tasks add [OPTIONS] --name <NAME>`

###### **Options:**

* `-n`, `--name <NAME>`
* `-d`, `--description <DESCRIPTION>`

  Default value: ``



## `tasks list`

Lists all tasks

**Usage:** `tasks list`



