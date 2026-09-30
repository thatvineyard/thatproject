---
name: thatproject-add-reference
description: Adds a reference to the project
---
# thatproject-add-reference

Use the following subcommand of `thatproject` to manage a thatproject project. Always run with --agent-mode as a global flag.E.g. if the subcommand is `init` then run `thatproject --agent-mode init`

## `add-reference`

Add a reference (directory, file, glob pattern or URL) to the manifest. Plain paths must exist.

**Usage:** `add-reference [OPTIONS] --type <TYPE> <REFERENCE>`

###### **Arguments:**

* `<REFERENCE>`

###### **Options:**

* `--type <TYPE>`

  Possible values: `source-code`, `documentation`

* `--note <NOTE>`



