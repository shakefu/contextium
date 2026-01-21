# Contextium Purpose

> External, selective, multi-repo context management for AI coding agents

## Problems We're Solving

| # | Problem | Current State |
|---|---------|---------------|
| 1 | **Context pollution** | Progress files, feature JSONs committed to project repos |
| 2 | **Single-agent bottleneck** | Shared context files cause conflicts between agents |
| 3 | **PR friction** | Must PR between every session just to persist agent state |
| 4 | **Context bloat** | Larger context windows = worse LLM accuracy |
| 5 | **Cross-repo isolation** | No unified context when working across multiple repos |

## Architecture

### Three Components

```
┌─────────────────┐     ┌─────────────────┐     ┌─────────────────┐
│  Claude Code    │────▶│   ctxium CLI    │────▶│ Contextium      │
│  + Hooks        │◀────│                 │◀────│ Server          │
└─────────────────┘     └─────────────────┘     └─────────────────┘
                                                       │
                                                       ▼
                                                ┌─────────────────┐
                                                │ Memory Layer    │
                                                │ (memvid/gibram) │
                                                └─────────────────┘
```

### 1. ctxium CLI

The command-line interface that bridges Claude Code and the server:

- Downloaded/installed automatically via SessionStartup hook
- Registers plans and creates subtasks with attached context
- Marks tasks as done or in-progress at session end
- Provides simplified access to wrapped tools/skills/prompts

### 2. Contextium Server

A hosted, persistent backend that:

- Determines what should be done in the current session
- Provides **minimal, task-relevant context** (not everything)
- Runs external tools (litellm for adversarial-spec, etc.)
- Wraps complex prompts/tools/skills behind simple CLI commands
- Analyzes tasks and plans to attach relevant context from memory

### 3. Claude Code Hooks

The integration glue that makes it automatic:

| Hook | Purpose |
|------|---------|
| `SessionStartup` | Install ctxium CLI, fetch current task + minimal context |
| `PreToolUse` | Inject opportunistic reminders (e.g., "use X for linting" before commit) |
| `UserPromptSubmit` | Add task-specific context per prompt |
| `SessionEnd` | Persist task status back to server via CLI |

## Workflow

### Planning Phase

1. Claude receives a feature request
2. Claude uses `ctxium plan create` to register the plan with the server
3. Claude uses `ctxium task create` to break plan into subtasks with context
4. Server analyzes and enriches tasks with relevant memory (memvid/gibram)

### Execution Phase

1. **Session starts** → Hook installs ctxium, fetches current task + minimal context
2. Claude works on the task (doesn't need full history, just what's relevant)
3. **Pre-tool hooks** inject reminders opportunistically
4. **Session ends** → Claude uses `ctxium task update` to mark progress
5. Next fresh session picks up seamlessly

### Context Injection Philosophy

**Minimal by default.** Claude doesn't need to know:
- Commit procedures when it's just starting up
- Full project history for a focused bugfix
- All feature specs when working on one task

Context is injected **when relevant**:
- Commit guidelines → before Claude tries to commit
- Related code patterns → when Claude is implementing similar features
- Test requirements → when Claude is writing tests

## External Tool Integration

The server can run or wrap external tools:

| Tool | Server Role |
|------|-------------|
| **adversarial-spec** | Run litellm, expose via `ctxium spec refine` |
| **memvid** | Semantic memory retrieval for context enrichment |
| **gibram** | Session-scoped memory for learned behaviors |
| **worktrunk** | Could coordinate multi-agent worktree management |

## Key Benefits

1. **Clean repos** - No context files polluting project code
2. **Parallel agents** - Each agent/branch gets isolated context
3. **No PR friction** - State persists externally, sessions just pick up
4. **Focused context** - Only load what's needed, accuracy stays high
5. **Cross-repo aware** - Unified context layer spans multiple projects
6. **Learned behaviors** - Server remembers and reinforces patterns

## Future Considerations

- Multi-agent coordination (who's working on what)
- Context summarization for long-running projects
- Conflict detection when agents touch same files
- Integration with existing project management tools
