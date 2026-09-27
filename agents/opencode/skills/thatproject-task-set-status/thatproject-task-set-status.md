---
name: thatproject-task-set-status
description: Sets the status of a given task
---
# thatproject-task-set-status

Use the following subcommand of `thatproject` to manage a thatproject project. Always run with --agent-mode as a global flag.E.g. if the subcommand is `init` then run `thatproject --agent-mode init`

## `task-set-status`

Sets the status field in the task file.

**Usage:** `task-set-status --name <NAME> --status <STATUS>`

###### **Options:**

* `-n`, `--name <NAME>`
* `-s`, `--status <STATUS>`

  Possible values: `draft`, `ongoing`, `complete`




