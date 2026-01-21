# Anthropic Harness Patterns

> Techniques for long-running agents: initializer agents, progress tracking, incremental work

## Overview

Long-running AI agents face a fundamental problem: they must operate across multiple context windows, with each new session starting without memory of previous work. Anthropic's harness patterns provide solutions for maintaining continuity in complex multi-hour or multi-day projects.

## Purpose

Traditional AI coding assistants are limited to single-session interactions. Harness patterns enable:

- **Multi-session continuity**: Work persists across context windows
- **Progress tracking**: State documented for session handoffs
- **Incremental development**: Features completed one at a time
- **Reliable verification**: End-to-end testing before completion

## Core Architecture

### Two-Part Solution

#### 1. Initializer Agent (First Session)
Establishes the foundation with specialized prompting:

| Output | Purpose |
|--------|---------|
| `init.sh` | Development environment setup script |
| `claude-progress.txt` | Activity documentation |
| Initial git commit | Baseline for change tracking |
| Feature specifications | Detailed, testable requirements |

#### 2. Coding Agent (Subsequent Sessions)
Follows structured pattern each session:

1. Read progress files and git logs
2. Review feature requirements
3. Select highest-priority incomplete item
4. Implement single feature incrementally
5. Commit changes with descriptive messages
6. Update progress documentation

## Key Implementation Patterns

### Feature List Management

Maintain a JSON file with structured requirements:

```json
{
  "features": [
    {
      "id": "auth-login",
      "description": "User login with email/password",
      "status": "passing",
      "tests": ["test_login_valid", "test_login_invalid"]
    },
    {
      "id": "auth-logout",
      "description": "User logout with session cleanup",
      "status": "failing",
      "tests": ["test_logout_clears_session"]
    }
  ]
}
```

**Critical rule**: Never remove or edit tests, as this could lead to missing or buggy functionality.

### Incremental Progress Strategy

| Approach | Benefit |
|----------|---------|
| One feature per session | Reduces context exhaustion |
| Git commits per feature | Enables error recovery |
| Progress file updates | Documents state for handoffs |
| Small, focused changes | Easier verification |

### Testing Protocol

Agents must verify work end-to-end:
- Use browser automation (Puppeteer MCP)
- Mimic human user behavior
- Don't rely solely on unit tests
- Verify complete user flows

### Session Startup Routine

Every session begins with:

```bash
# 1. Verify directory
cd /path/to/project

# 2. Review git history
git log --oneline -10

# 3. Read progress file
cat claude-progress.txt

# 4. Run basic functionality tests
npm test
```

## Failure Modes Addressed

| Problem | Solution |
|---------|----------|
| Premature project completion | Comprehensive feature list prevents false declarations |
| Buggy, undocumented handoffs | Git commits + progress updates create clear state transitions |
| Incomplete feature marking | Mandatory end-to-end testing before marking complete |
| Time wasted on setup | Pre-built `init.sh` script reduces setup overhead |

## Progress File Format

```markdown
# Claude Progress

## Current Session: 2024-01-15 14:30

### Completed
- [x] User authentication (login/logout)
- [x] Dashboard layout

### In Progress
- [ ] User profile page (started, needs avatar upload)

### Blocked
- Rate limiting (waiting on Redis configuration)

### Next Steps
1. Complete avatar upload for profile page
2. Add profile settings form
3. Implement rate limiting after Redis setup
```

## Integration with Contextium

Harness patterns provide the foundational approach:

- Initializer agent pattern for project setup
- Progress tracking files for state persistence
- Incremental feature development across sessions
- Git-based state recovery and verification

## Best Practices

### Do
- Create comprehensive feature specifications upfront
- Update progress files after every significant change
- Commit frequently with descriptive messages
- Verify features with end-to-end tests
- Run basic tests before starting new work

### Don't
- Mark features complete without verification
- Remove tests that are failing
- Skip progress file updates
- Attempt too many features in one session
- Assume previous session state

## Future Directions

Anthropic suggests potential improvements:
- **Specialized agents**: Testing agents, QA agents, code cleanup agents
- **Domain generalization**: Scientific research, financial modeling
- **Multi-agent coordination**: Parallel development with merge coordination

## Technical Details

- **Source**: [Anthropic Engineering Blog](https://www.anthropic.com/engineering/effective-harnesses-for-long-running-agents)
- **Pattern type**: Software architecture / Agent design

## Links

- [Anthropic Engineering Article](https://www.anthropic.com/engineering/effective-harnesses-for-long-running-agents)
