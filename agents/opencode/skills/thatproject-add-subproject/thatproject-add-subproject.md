---
name: thatproject-add-subproject
description: Adds a subproject to the project
---
# thatproject-add-subproject

Use the following subcommand of `thatproject` to manage a thatproject project. Always run with --agent-mode as a global flag.E.g. if the subcommand is `init` then run `thatproject --agent-mode init`Use --subproject <subproject> (after --agent-mode for permission reasons) to peform actions on that subproject

## `add-subproject`

Add a subproject to the manifest. Path must be a directory within this project's directory

**Usage:** `add-subproject [OPTIONS] <PATH>`

###### **Arguments:**

* `<PATH>`

###### **Options:**

* `--alias <ALIAS>`
* `--note <NOTE>`



