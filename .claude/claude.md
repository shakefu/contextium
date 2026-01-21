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
