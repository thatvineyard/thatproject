---
name: thatproject-task
description: ''
---
# thatproject-task

Use the following subcommand of `thatproject` to manage a thatproject project. Always run with --agent-mode as a global flag.E.g. if the subcommand is `init` then run `thatproject --agent-mode init`

## `task`

**Usage:** `task <NAME> <COMMAND>`

###### **Subcommands:**

* `set-body` — Sets the body of a given task
* `set-status` — Sets the status of a given task
* `read` — Reads the given task

###### **Arguments:**

* `<NAME>`



## `task set-body`

Sets the body in the task file.

**Usage:** `task set-body <BODY>`

###### **Arguments:**

* `<BODY>`



## `task set-status`

Sets the status field in the task file header.

**Usage:** `task set-status <STATUS>`

###### **Arguments:**

* `<STATUS>`

  Possible values: `draft`, `ongoing`, `complete`




## `task read`

Output the contents of the file

**Usage:** `task read [OPTIONS]`

###### **Options:**

* `--json`



