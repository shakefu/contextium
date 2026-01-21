# Contextium Sub-Agents

Sub-agents for Claude Code session management following the [harness patterns](https://www.anthropic.com/engineering/effective-harnesses-for-long-running-agents) for long-running agents.

## Agent Types

| Agent | Purpose | When Invoked |
|-------|---------|--------------|
| `initializer` | Session setup, environment check, tool verification | SessionStart hook |
| `task-determiner` | Analyze state files, determine current task | After initialization |
| `context-fetcher` | Retrieve minimal relevant context for task | Before task execution |

## Architecture

```
SessionStart Hook
       │
       ▼
┌─────────────────┐
│   Initializer   │──▶ Verify tools, check state files
└────────┬────────┘
         │
         ▼
┌─────────────────┐
│ Task Determiner │──▶ Read progress, select next task
└────────┬────────┘
         │
         ▼
┌─────────────────┐
│ Context Fetcher │──▶ Load minimal context for task
└────────┬────────┘
         │
         ▼
    Main Session
```

## Usage

Agents are invoked automatically via Claude Code hooks. Manual invocation:

```bash
# Run full initialization sequence
./agents/run-startup.sh

# Run individual agents
./agents/initializer.sh
./agents/task-determiner.sh
./agents/context-fetcher.sh
```

## State Files

| File | Purpose |
|------|---------|
| `.contextium/state.json` | Current session state |
| `.contextium/tasks.json` | Task list and progress |
| `.contextium/context-cache/` | Cached context snippets |
