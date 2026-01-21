# Claude Code Instructions

## Tool Restrictions

- **Do NOT use the `AskUserQuestion` tool.** It does not work on Claude Code iOS. Instead, ask questions directly in your response text when clarification is needed.

## Session Startup

At the start of each session, run the bootstrap script in the background to ensure all Contextium tools are installed:

```bash
# Run bootstrap in background (non-blocking)
nohup ./script/bootstrap > /tmp/contextium-bootstrap.log 2>&1 &
```

This installs/updates the following tools silently:
- **worktrunk** - Git worktree management for parallel agents
- **memvid** - Portable memory system for AI agents
- **gibram** - In-memory knowledge graph for RAG
- **adversarial-spec** - Multi-model specification refinement (litellm)

The bootstrap runs in parallel and won't block your session. Check `/tmp/contextium-bootstrap.log` if you need to verify installation status.

## Scripts to Rule Them All

This project follows the [Scripts to Rule Them All](https://github.blog/engineering/engineering-principles/scripts-to-rule-them-all/) pattern:

| Script | Purpose |
|--------|---------|
| `script/bootstrap` | Install/update all dependencies |
| `script/setup` | First-time project setup (runs bootstrap) |
| `script/update` | Update after git pull |
| `script/install/<tool>` | Install individual tools |

### Available Tool Installers

- `script/install/worktrunk` - Git worktree management
- `script/install/memvid` - Portable AI memory
- `script/install/gibram` - Knowledge graph for RAG
- `script/install/adversarial-spec` - Multi-model debate

## Sub-Agents

Contextium uses sub-agents for session initialization. These run automatically via the SessionStart hook.

### Agent Pipeline

```
SessionStart
    │
    ├─▶ script/bootstrap (background)
    │
    └─▶ agents/run-startup.sh
            │
            ├─▶ initializer     → Verify environment, init state files
            ├─▶ task-determiner → Analyze state, determine current task
            └─▶ context-fetcher → Load minimal relevant context
```

### Available Agents

| Agent | Script | Purpose |
|-------|--------|---------|
| Initializer | `agents/initializer.sh` | Environment setup, state initialization |
| Task Determiner | `agents/task-determiner.sh` | Analyze progress, select current task |
| Context Fetcher | `agents/context-fetcher.sh` | Load minimal task-relevant context |

### Manual Invocation

```bash
# Run full startup sequence
./agents/run-startup.sh

# Run individual agents
./agents/initializer.sh
./agents/task-determiner.sh
./agents/context-fetcher.sh
```

### State Files

| File | Purpose |
|------|---------|
| `.contextium/state.json` | Session state and metadata |
| `.contextium/tasks.json` | Task list and progress tracking |
| `.contextium/context-cache/` | Cached context for tasks |

### Logs

Startup logs are written to:
- `/tmp/contextium-bootstrap.log` - Tool installation
- `/tmp/contextium-startup.log` - Agent startup output
- `/tmp/contextium/*.log` - Individual agent logs
