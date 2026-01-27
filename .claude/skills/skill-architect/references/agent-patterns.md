# Agent Patterns

## Effective Patterns

### Fan-Out Research
Spawn multiple agents to search different domains, collect results.

```
┌─ Agent: search domain A ─┐
│                          │
├─ Agent: search domain B ─┼─▶ Collect ─▶ Synthesize
│                          │
└─ Agent: search domain C ─┘
```

**Use when:** Need information from multiple independent sources.

### Parallel Validation
Same validation logic applied to multiple inputs.

```
Input list ─┬─▶ Agent: validate item 1
            ├─▶ Agent: validate item 2
            └─▶ Agent: validate item 3
```

**Use when:** Processing multiple files, entries, or configurations.

### Explore-Then-Act
Agent explores options, returns summary, main flow decides.

```
Agent: find options ─▶ Options summary ─▶ User/main decides ─▶ Act
```

**Use when:** Decision requires exploration but shouldn't pollute main context.

### Staged Pipeline
Each stage is an agent, results flow forward.

```
Agent: extract ─▶ Agent: transform ─▶ Agent: validate ─▶ Done
```

**Use when:** Clear processing stages with well-defined interfaces.

## Anti-Patterns

### Over-Delegation
**Problem:** Spawning agents for trivial operations.
**Fix:** Only delegate work that benefits from isolation or parallelism.

### Context Starvation
**Problem:** Agent lacks necessary context to complete task.
**Fix:** Include all required context in agent prompt, or use agents with "access to current context."

### Serial Bottleneck
**Problem:** Spawning agents sequentially when they could run parallel.
**Fix:** Identify independent operations, spawn in single message block.

### Result Bloat
**Problem:** Agent returns too much data, negating context savings.
**Fix:** Specify concise return format, ask for summaries not raw data.

## Agent Type Selection

| Subagent Type | Best For |
|---------------|----------|
| `Explore` | Codebase navigation, file finding, quick searches |
| `general-purpose` | Complex multi-step tasks, research with web search |
| `Bash` | Git operations, command execution, system tasks |
| `Plan` | Architecture decisions, implementation planning |
