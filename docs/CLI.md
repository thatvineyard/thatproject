# Command-Line Help for `thatproject`

This document contains the help content for the `thatproject` command-line program.

**Command Overview:**

* [`thatproject`↴](#thatproject)
* [`thatproject init`↴](#thatproject-init)
* [`thatproject summary`↴](#thatproject-summary)
* [`thatproject add-reference`↴](#thatproject-add-reference)
* [`thatproject add-subproject`↴](#thatproject-add-subproject)
* [`thatproject tasks`↴](#thatproject-tasks)
* [`thatproject tasks add`↴](#thatproject-tasks-add)
* [`thatproject tasks list`↴](#thatproject-tasks-list)
* [`thatproject task`↴](#thatproject-task)
* [`thatproject task set-title`↴](#thatproject-task-set-title)
* [`thatproject task set-description`↴](#thatproject-task-set-description)
* [`thatproject task set-category`↴](#thatproject-task-set-category)
* [`thatproject task set-body`↴](#thatproject-task-set-body)
* [`thatproject task set-status`↴](#thatproject-task-set-status)
* [`thatproject task read`↴](#thatproject-task-read)
* [`thatproject task add-reference`↴](#thatproject-task-add-reference)

## `thatproject`

ThatProject

**Usage:** `thatproject [OPTIONS] <COMMAND>`

###### **Subcommands:**

* `init` — Initialize a ThatProject workspace
* `summary` — Summmarize project
* `add-reference` — Adds a reference to the project
* `add-subproject` — Adds a subproject to the project
* `tasks` — 
* `task` — 

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



## `thatproject summary`

List manifest details

**Usage:** `thatproject summary`



## `thatproject add-reference`

Add a reference (directory, file, glob pattern or URL) to the manifest. Plain paths must exist.

**Usage:** `thatproject add-reference [OPTIONS] --type <TYPE> <REFERENCE>`

###### **Arguments:**

* `<REFERENCE>`

###### **Options:**

* `--type <TYPE>`

  Possible values: `source-code`, `documentation`

* `--note <NOTE>`



## `thatproject add-subproject`

Add a subproject to the manifest. Path must be a directory within this project's directory

**Usage:** `thatproject add-subproject [OPTIONS] <PATH>`

###### **Arguments:**

* `<PATH>`

###### **Options:**

* `--alias <ALIAS>`
* `--note <NOTE>`



## `thatproject tasks`

**Usage:** `thatproject tasks <COMMAND>`

###### **Subcommands:**

* `add` — Adds a task to the project
* `list` — Lists all tasks



## `thatproject tasks add`

Creates a task file in the task folder defined in the manifest.

**Usage:** `thatproject tasks add [OPTIONS] <TITLE>`

###### **Arguments:**

* `<TITLE>`

###### **Options:**

* `--category <CATEGORY>`
* `-d`, `--description <DESCRIPTION>`

  Default value: ``



## `thatproject tasks list`

Lists all tasks

**Usage:** `thatproject tasks list`



## `thatproject task`

**Usage:** `thatproject task <KEY> <COMMAND>`

###### **Subcommands:**

* `set-title` — Sets the title of a given task
* `set-description` — Sets the description of a given task
* `set-category` — Sets the category of a given task
* `set-body` — Sets the body of a given task
* `set-status` — Sets the status of a given task
* `read` — Reads the given task
* `add-reference` — Adds a reference to a given task

###### **Arguments:**

* `<KEY>`



## `thatproject task set-title`

Sets the title in the task file header.

**Usage:** `thatproject task set-title <TITLE>`

###### **Arguments:**

* `<TITLE>`



## `thatproject task set-description`

Sets the description in the task file header.

**Usage:** `thatproject task set-description <DESCRIPTION>`

###### **Arguments:**

* `<DESCRIPTION>`



## `thatproject task set-category`

Sets the category in the task file header.

**Usage:** `thatproject task set-category <CATEGORY>`

###### **Arguments:**

* `<CATEGORY>`



## `thatproject task set-body`

Sets the body in the task file.

**Usage:** `thatproject task set-body <BODY>`

###### **Arguments:**

* `<BODY>`



## `thatproject task set-status`

Sets the status field in the task file header.

**Usage:** `thatproject task set-status <STATUS>`

###### **Arguments:**

* `<STATUS>`

  Possible values: `draft`, `ongoing`, `complete`




## `thatproject task read`

Output the contents of the file

**Usage:** `thatproject task read [OPTIONS]`

###### **Options:**

* `--json`



## `thatproject task add-reference`

Adds a reference to the task file header.

**Usage:** `thatproject task add-reference [OPTIONS] --type <TYPE> <REFERENCE>`

###### **Arguments:**

* `<REFERENCE>`

###### **Options:**

* `--type <TYPE>`

  Possible values: `source-code`, `documentation`

* `--note <NOTE>`



<hr/>

<small><i>
    This document was generated automatically by
    <a href="https://crates.io/crates/clap-markdown"><code>clap-markdown</code></a>.
</i></small>
