# AutoCoder

> Autonomous AI coding agent using Claude Agent SDK for multi-session development

## Overview

AutoCoder is an autonomous AI-powered coding agent built on the Claude Agent SDK. It enables building complete applications through multiple sessions using a two-agent pattern, with a React-based monitoring interface for real-time progress tracking.

## Purpose

Building complete applications with AI requires multiple sessions due to context window limitations. AutoCoder solves this by:

- **Two-agent architecture**: Initializer + Coding agent pattern
- **Feature tracking**: SQLite-based progress across sessions
- **Automatic resume**: Continues work between interruptions
- **Real-time monitoring**: React UI with WebSocket streaming

## Key Features

| Feature | Description |
|---------|-------------|
| **Initializer Agent** | Generates specs, test cases, project structure, seeds feature database |
| **Coding Agent** | Implements features sequentially, marks completed work |
| **Long-running sessions** | Auto-resumes with 3-second delays |
| **Feature tracking** | SQLite + SQLAlchemy ORM for state management |
| **Real-time UI** | React interface with WebSocket streaming |
| **Security** | Command allowlist, filesystem sandboxing, validation hooks |

## Architecture

### Two-Agent Pattern

1. **Initializer Agent** (first session):
   - Processes application specifications
   - Generates comprehensive test cases
   - Establishes project structure
   - Seeds SQLite feature database

2. **Coding Agent** (subsequent sessions):
   - Reads feature database for current state
   - Implements highest-priority incomplete features
   - Marks features complete after verification
   - Commits changes with descriptive messages

## Installation

### Prerequisites
```bash
# Install Claude Code CLI

# macOS/Linux
curl -fsSL https://claude.ai/install.sh | bash

# Windows
irm https://claude.ai/install.ps1 | iex
```

### Authentication
```bash
claude login  # For Pro/Max subscription
# Or configure ANTHROPIC_API_KEY
```

## Usage

### Web UI (Recommended)
```bash
# Windows
start_ui.bat

# macOS/Linux
./start_ui.sh
```
Access at `http://localhost:5173`

### CLI Mode
```bash
# Windows
start.bat

# macOS/Linux
./start.sh
```

### Create Specification
Use `/create-spec` for AI-assisted specification generation within the UI.

## Generated Project Structure

```
generations/project_name/
├── features.db          # SQLite feature database
├── prompts/             # Application specs and session prompts
├── init.sh              # Environment setup script
└── src/                 # Application source code
```

## Security Model

Defense-in-depth approach:

| Layer | Protection |
|-------|------------|
| OS-level | Bash sandboxing |
| Filesystem | Restricted to project directories |
| Commands | Allowlist (npm, node, git, file inspection) |
| Hooks | Block dangerous commands |

## Configuration

Optional `.env` variables:

```bash
PROGRESS_N8N_WEBHOOK_URL=https://webhook-endpoint
ANTHROPIC_BASE_URL=https://api.z.ai/api/anthropic  # Alternative providers
API_TIMEOUT_MS=3000000
```

## Web UI Stack

- React 18 + TypeScript
- TanStack Query for data fetching
- Tailwind CSS v4
- Radix UI components
- WebSocket for real-time updates

## Timing Expectations

| Phase | Duration |
|-------|----------|
| First session | Several minutes (test generation) |
| Feature iteration | 5-15 minutes per feature |
| Complete application | Multiple hours across sessions |

## Integration with Contextium

AutoCoder provides the multi-session agent framework:

- Initializer agent pattern for project setup
- Feature database for progress tracking
- Incremental work across context windows
- Real-time monitoring and debugging

## Technical Details

- **Language**: Python (backend), TypeScript/React (frontend)
- **License**: GNU AGPL v3.0
- **Repository**: [leonvanzyl/autocoder](https://github.com/leonvanzyl/autocoder)

## Links

- [GitHub Repository](https://github.com/leonvanzyl/autocoder)
