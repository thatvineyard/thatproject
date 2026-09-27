# Command-Line Help for `thatproject`

This document contains the help content for the `thatproject` command-line program.

**Command Overview:**

* [`thatproject`↴](#thatproject)
* [`thatproject init`↴](#thatproject-init)
* [`thatproject add-source`↴](#thatproject-add-source)
* [`thatproject task-add`↴](#thatproject-task-add)
* [`thatproject task-set-status`↴](#thatproject-task-set-status)

## `thatproject`

ThatProject

**Usage:** `thatproject [OPTIONS] <COMMAND>`

###### **Subcommands:**

* `init` — Initialize a ThatProject workspace
* `add-source` — Adds a source to the project
* `task-add` — Adds a task to the project
* `task-set-status` — Sets the status of a given task

###### **Options:**

* `-c`, `--context-dir <CONTEXT_DIR>`
* `-a`, `--agent-mode`



## `thatproject init`

Creates the necessary files and directories to enable this directory as a thatproject project.

**Usage:** `thatproject init [OPTIONS] --name <NAME>`

###### **Options:**

* `-n`, `--name <NAME>`
* `-d`, `--description <DESCRIPTION>`

  Default value: ``
* `--task-dir <TASK_DIR>`

  Default value: `tasks`



## `thatproject add-source`

Add a source directory to the manifest so it can be included in source commands.

**Usage:** `thatproject add-source --source <SOURCE>`

###### **Options:**

* `-s`, `--source <SOURCE>`



## `thatproject task-add`

Creates a task file in the task folder defined in the manifest.

**Usage:** `thatproject task-add [OPTIONS] --name <NAME>`

###### **Options:**

* `-n`, `--name <NAME>`
* `-d`, `--description <DESCRIPTION>`

  Default value: ``



## `thatproject task-set-status`

Sets the status field in the task file.

**Usage:** `thatproject task-set-status --name <NAME> --status <STATUS>`

###### **Options:**

* `-n`, `--name <NAME>`
* `-s`, `--status <STATUS>`

  Possible values: `draft`, `ongoing`, `complete`




<hr/>

<small><i>
    This document was generated automatically by
    <a href="https://crates.io/crates/clap-markdown"><code>clap-markdown</code></a>.
</i></small>
