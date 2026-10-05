---
name: thatproject-init
description: Initialize a ThatProject workspace
---
# thatproject-init

Use the following subcommand of `thatproject` to manage a thatproject project. Always run with --agent-mode as a global flag.E.g. if the subcommand is `init` then run `thatproject --agent-mode init`Use --subproject <subproject> (after --agent-mode for permission reasons) to peform actions on that subproject

## `init`

Creates the necessary files and directories to enable this directory as a thatproject project.

**Usage:** `init [OPTIONS] --name <NAME>`

###### **Options:**

* `-n`, `--name <NAME>`
* `-d`, `--description <DESCRIPTION>`

  Default value: ``
* `--task-dir <TASK_DIR>`

  Default value: `tasks`



