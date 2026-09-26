# Command-Line Help for `thatproject`

This document contains the help content for the `thatproject` command-line program.

**Command Overview:**

* [`thatproject`↴](#thatproject)
* [`thatproject greet`↴](#thatproject-greet)
* [`thatproject init`↴](#thatproject-init)
* [`thatproject add-source`↴](#thatproject-add-source)
* [`thatproject task-add`↴](#thatproject-task-add)
* [`thatproject task-set-status`↴](#thatproject-task-set-status)

## `thatproject`

ThatProject

**Usage:** `thatproject [OPTIONS] <COMMAND>`

###### **Subcommands:**

* `greet` — 
* `init` — 
* `add-source` — 
* `task-add` — 
* `task-set-status` — 

###### **Options:**

* `-c`, `--context-dir <CONTEXT_DIR>`



## `thatproject greet`

**Usage:** `thatproject greet [OPTIONS]`

###### **Options:**

* `-n`, `--name <NAME>`

  Default value: `world`



## `thatproject init`

**Usage:** `thatproject init [OPTIONS] --name <NAME>`

###### **Options:**

* `-n`, `--name <NAME>`
* `-d`, `--description <DESCRIPTION>`

  Default value: ``
* `--task-dir <TASK_DIR>`

  Default value: `tasks`



## `thatproject add-source`

**Usage:** `thatproject add-source --source <SOURCE>`

###### **Options:**

* `-s`, `--source <SOURCE>`



## `thatproject task-add`

**Usage:** `thatproject task-add [OPTIONS] --name <NAME>`

###### **Options:**

* `-n`, `--name <NAME>`
* `-d`, `--description <DESCRIPTION>`

  Default value: ``



## `thatproject task-set-status`

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
