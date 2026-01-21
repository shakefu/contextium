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

## Multi-Agent Coordination

The server acts as a **scheduler and conflict detector** for parallel agents working in the same repo.

### How It Works

1. **Task Analysis** - When a task is created, server analyzes:
   - Which files/modules will likely be touched
   - Dependencies between tasks
   - Potential for merge conflicts

2. **Conflict Detection** - Server maintains awareness of:
   - Which agents are currently working
   - What files each agent is modifying
   - Which branches exist and their divergence

3. **Smart Scheduling** - Server decides:
   - Tasks that can safely run in parallel (different modules, no overlap)
   - Tasks that must be sequential (same files, high conflict risk)
   - Tasks that need coordination (shared interfaces, API changes)

### Coordination Strategies

| Scenario | Strategy |
|----------|----------|
| Non-overlapping modules | Run in parallel, merge freely |
| Same file, different sections | Run in parallel with caution flag |
| Same file, same sections | Sequential execution |
| Interface changes | Notify dependent tasks, coordinate merge order |
| Shared test files | Queue or partition test ownership |

### Agent Awareness

Each agent knows:
- Its assigned task scope (files it "owns" for this task)
- Other active agents and their scopes
- Whether to proceed, wait, or flag potential conflicts

```
$ ctxium status
Agent: claude-session-abc123
Task: implement-user-auth
Scope: src/auth/*, tests/auth/*
Parallel agents: 2
  - claude-session-def456: refactor-logging (src/utils/logger.ts)
  - claude-session-ghi789: add-dashboard (src/pages/dashboard/*)
Conflicts: none
```

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
2. **Parallel agents** - Multiple agents work simultaneously with isolated context
3. **Conflict-aware scheduling** - Server prevents merge conflicts before they happen
4. **No PR friction** - State persists externally, sessions just pick up
5. **Focused context** - Only load what's needed, accuracy stays high
6. **Cross-repo aware** - Unified context layer spans multiple projects
7. **Learned behaviors** - Server remembers and reinforces patterns

## Future Considerations

- Context summarization for long-running projects
- Integration with existing project management tools (Jira, Linear, GitHub Issues)
- Automatic merge conflict resolution suggestions
- Learning optimal task decomposition from historical data
