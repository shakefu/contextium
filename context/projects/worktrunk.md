# Worktrunk

> Git worktree management for running multiple AI agents in parallel

## Overview

Worktrunk is a CLI tool that simplifies Git worktree management, making it practical to run multiple AI coding agents simultaneously on separate branches. It addresses the clunky UX of native Git worktree commands by treating worktrees like branches - addressable by name with automatic path computation.

## Purpose

The primary use case is enabling parallel AI agent development. With traditional Git workflows, running multiple Claude Code instances on the same repository creates conflicts. Worktrunk solves this by giving each agent its own isolated worktree, allowing concurrent development on different features.

## Key Features

| Feature | Description |
|---------|-------------|
| **Branch-addressed worktrees** | Reference worktrees by branch name; paths computed automatically |
| **fzf-like selector** | Interactive worktree selection with fuzzy matching |
| **Hooks system** | Execute commands on create, pre-merge, post-merge events |
| **LLM commit messages** | Auto-generate commit messages via the `llm` tool |
| **Merge workflow** | Squash, rebase, merge, and cleanup in one operation |
| **CI status & PR links** | Display pipeline information inline |
| **Claude Code integration** | Streamlined agent startup with `-x claude` flag |

## Installation

### macOS & Linux (Homebrew)
```bash
brew install worktrunk && wt config shell install
```

### Cargo (Rust)
```bash
cargo install worktrunk && wt config shell install
```

### Windows (Winget)
```bash
winget install max-sixty.worktrunk
git-wt config shell install
```

### Arch Linux
```bash
paru worktrunk-bin && wt config shell install
```

## Core Commands

### Create and switch to a worktree
```bash
wt switch -c feat          # Create worktree for branch 'feat'
wt switch -c -x claude feat # Create and start Claude Code agent
```

### List all worktrees
```bash
wt list                    # Shows status, CI info, PR links
```

### Remove worktree and branch
```bash
wt remove                  # Interactive selection
wt remove feat             # Remove specific worktree
```

### Merge with automated cleanup
```bash
wt merge                   # Squash, rebase, merge, cleanup
```

## Configuration

Worktrees follow a configurable path template. Shell integration enables directory changes within commands via `wt config shell install`.

## Integration with Contextium

Worktrunk provides the foundation for running multiple AI agents in parallel on the same codebase:

- Each agent operates in an isolated worktree with no file conflicts
- Branches can be merged back with consolidated merge operations
- Hooks can trigger on worktree creation to set up agent environments
- Progress can be tracked independently per worktree

## Technical Details

- **Language**: Rust (99.5% of codebase)
- **License**: MIT
- **Repository**: [max-sixty/worktrunk](https://github.com/max-sixty/worktrunk)
- **Documentation**: [worktrunk.dev](https://worktrunk.dev)

## Links

- [GitHub Repository](https://github.com/max-sixty/worktrunk)
- [Documentation](https://worktrunk.dev)
