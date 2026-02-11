# Design

This document contains design decisions for the application.

## Features

1. Tasks have a title
2. Can prioritise tasks
3. Tasks can have a status
4. Tasks can be grouped/tagged
5. Tasks should be stored in a plain text format
6. Can have separate workspaces of tasks
7. Tasks can have a formatted description
8. Tasks are ordered

## Task File format

Each task resides in its own file.

A directory containing task files is considered a workspace.

A task file is stored in TOML

```toml
title = "Task title"
status = "pending"
priority = "high"
tags = ["tag 1", "tag 2"]
description = """
Multi-line string
"""
```

The name of the task file is in the format `<index>.toml`.
Where the `<index>` is the order in the task appears in the workspace.

## Functions required

List workspaces:

- Provide zero or more workspaces as a command line argument
- Configure zero or more workspaces in a configuration file
  - Configuration file read from the current directory
  - Configuration file read from user's home directory

A workspace should have a `workspace.toml` configuration file for
workspace specific configuration.
