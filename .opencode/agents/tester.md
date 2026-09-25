---
description: Run Rust tests and verify test results
mode: subagent
permissions:
  - action: shell
    resource: "cargo test*"
    effect: allow
  - action: shell
    resource: "wsl -d NixOS direnv exec . cargo test*"
    effect: allow
---

# Test Agent

Runs Rust tests for this project.

## Commands

**Run all tests**
```bash
cargo test
```

**Run tests with output**
```bash
cargo test -- --nocapture
```

**Run a single test by name**
```bash
cargo test test_name
```

**Run single-threaded**
```bash
cargo test -- --test-threads=1
```

## Structure

Tests live in `#[cfg(test)]` blocks in source files, or as integration tests in the `tests/` directory.