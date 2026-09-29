---
name: thatproject-task
description: ''
---
# thatproject-task

Use the following subcommand of `thatproject` to manage a thatproject project. Always run with --agent-mode as a global flag.E.g. if the subcommand is `init` then run `thatproject --agent-mode init`

## `task`

**Usage:** `task <KEY> <COMMAND>`

###### **Subcommands:**

* `set-title` — Sets the title of a given task
* `set-description` — Sets the description of a given task
* `set-category` — Sets the category of a given task
* `set-body` — Sets the body of a given task
* `set-status` — Sets the status of a given task
* `read` — Reads the given task

###### **Arguments:**

* `<KEY>`



## `task set-title`

Sets the title in the task file header.

**Usage:** `task set-title <TITLE>`

###### **Arguments:**

* `<TITLE>`



## `task set-description`

Sets the description in the task file header.

**Usage:** `task set-description <DESCRIPTION>`

###### **Arguments:**

* `<DESCRIPTION>`



## `task set-category`

Sets the category in the task file header.

**Usage:** `task set-category <CATEGORY>`

###### **Arguments:**

* `<CATEGORY>`



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



