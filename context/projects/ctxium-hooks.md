# ctxium Hook Explorer

> Rust CLI for capturing and inspecting Claude Code hook payloads

## Purpose

The `ctxium` CLI provides tools for exploring what data Claude Code sends to hooks. This enables understanding hook payloads before building the full Contextium server infrastructure.

## Architecture

```
┌─────────────────┐      HTTP POST       ┌─────────────────┐
│  Claude Code    │  ──────────────────▶ │  ctxium         │
│  Hook Config    │      :6160/hook      │  hookserver     │
└─────────────────┘                      └─────────────────┘
        │                                        │
        │ stdin                                  │
        ▼                                        ▼
┌─────────────────┐                      ┌─────────────────┐
│  ctxium hook    │                      │  stdout +       │
│  <HookName>     │                      │  ~/.contextium/ │
└─────────────────┘                      │  hooks.log      │
                                         └─────────────────┘
```

## Building

```bash
cd ctxium
cargo build --release

# Binary will be at: ctxium/target/release/ctxium
```

## Commands

### `ctxium hookserver`

Starts an HTTP server that receives hook data and displays it.

```bash
# Start with defaults (port 6160, log to ~/.contextium/hooks.log)
ctxium hookserver

# Custom port and log file
ctxium hookserver --port 8080 --log-file /tmp/hooks.log
```

**Output:** Pretty-prints each hook to stdout with colored formatting, and appends JSON to the log file.

### `ctxium hook <name>`

Sends hook data to the hookserver. Reads from stdin, POSTs to server.

```bash
# Manual test
echo '{"tool":"Bash","command":"ls"}' | ctxium hook PreToolUse
```

**Behavior:**

- Reads all of stdin
- Parses as JSON (falls back to string if invalid JSON)
- POSTs to `http://127.0.0.1:6160/hook`
- **Fails silently** (exit 0) if server unreachable - this is intentional so hooks don't block Claude Code

## Manual Testing

### Step 1: Start the hookserver

In one terminal:

```bash
cd /path/to/contextium/ctxium
cargo run -- hookserver
```

You'll see:

```
[ctxium] Hookserver listening on 127.0.0.1:6160
[ctxium] Log file: ~/.contextium/hooks.log
[ctxium] Waiting for hook events...
```

### Step 2: Send test hooks

In another terminal:

```bash
# Test with JSON payload
echo '{"tool":"Bash","command":"ls -la"}' | cargo run -- hook PreToolUse

# Test with different hook names
echo '{"session":"abc123"}' | cargo run -- hook SessionStart
echo '{"result":"success"}' | cargo run -- hook PostToolUse
```

### Step 3: View results

The hookserver terminal will show:

```
────────────────────────────────────────────────────────────
[HOOK] PreToolUse @ 2026-01-21T20:55:28.866944693+00:00
────────────────────────────────────────────────────────────
{
  "command": "ls -la",
  "tool": "Bash"
}
```

### Step 4: Check log file

```bash
cat ~/.contextium/hooks.log
```

Each line is a JSON object:

```json
{"hook_name":"PreToolUse","timestamp":"2026-01-21T20:55:28Z","stdin_data":{"tool":"Bash"}}
```

## Wiring to Claude Code

To capture real hook data from Claude Code sessions, configure hooks in `.claude/settings.json`:

```json
{
  "hooks": {
    "PreToolUse": [
      {
        "type": "command",
        "command": "/path/to/ctxium hook PreToolUse"
      }
    ],
    "PostToolUse": [
      {
        "type": "command",
        "command": "/path/to/ctxium hook PostToolUse"
      }
    ],
    "SessionStart": [
      {
        "type": "command",
        "command": "/path/to/ctxium hook SessionStart"
      }
    ],
    "SessionEnd": [
      {
        "type": "command",
        "command": "/path/to/ctxium hook SessionEnd"
      }
    ]
  }
}
```

**Note:** Make sure `ctxium hookserver` is running before starting a Claude Code session, or the hooks will silently fail (by design).

## Technical Details

| Component | Technology |
| -- | -- |
| CLI parsing | clap 4 |
| HTTP server | axum 0.7 |
| HTTP client | reqwest 0.12 |
| Serialization | serde + serde_json |
| Async runtime | tokio |
| Colored output | colored 2 |

**Default port:** 6160 (one below gibram's 6161)

**Log format:** NDJSON (newline-delimited JSON), one object per hook event

## Claude Code Hooks Reference

Complete reference for all Claude Code hook types.

### All Hook Types

| Hook | When Triggered | Matcher |
| -- | -- | -- |
| `SessionStart` | Session begins or resumes | N/A |
| `SessionEnd` | Session terminates | N/A |
| `PreToolUse` | Before Claude executes a tool | Tool name pattern |
| `PostToolUse` | After tool completes | Tool name pattern |
| `UserPromptSubmit` | When user submits a prompt | N/A |
| `PermissionRequest` | When Claude requests permission for a tool (v2.0.45+) | N/A |
| `Stop` | When Claude finishes responding | N/A |
| `SubagentStop` | When a subagent finishes (v1.0.41+) | N/A |
| `PreCompact` | Before context compaction | N/A |

### Hook Configuration Structure

```json
{
  "hooks": {
    "EventName": [
      {
        "matcher": "ToolPattern",
        "hooks": [
          {
            "type": "command",
            "command": "your-command-here"
          }
        ]
      }
    ]
  }
}
```

**Matcher patterns** (for PreToolUse/PostToolUse only):

- Empty string `""` or omitted: matches nothing specific
- `"Write"`: matches only the Write tool (case-sensitive)
- `"*"`: matches all tools

### Hook Execution Types

| Type | Description |
| -- | -- |
| `command` | Runs a bash command |
| `prompt` | LLM-based evaluation |

### Exit Codes and Communication

Hooks communicate via exit codes, stdout, and stderr:

| Exit Code | Meaning |
| -- | -- |
| 0 | Success - stdout shown to user in transcript mode |
| 2 | Blocking error - stderr fed back to Claude to process |
| Other | Non-blocking error - stderr shown to user, execution continues |

### Structured JSON Output

Hooks can return structured JSON for more control:

```json
{
  "decision": "approve|block|allow|deny",
  "reason": "Explanation shown to Claude",
  "continue": true,
  "updatedInput": { }
}
```

| Field | Purpose |
| -- | -- |
| `decision` | approve, block, allow, or deny |
| `reason` | Explanation shown to Claude |
| `continue` | For Stop hooks - force continuation |
| `updatedInput` | Modify tool parameters before execution |

### Environment Variables

| Variable | Description |
| -- | -- |
| `CLAUDE_PROJECT_DIR` | Absolute path to project root |
| `CLAUDE_CODE_REMOTE` | "true" if web environment, unset for local CLI |

### Execution Behavior

- **Timeout**: 60 seconds by default, configurable per command
- **Parallelism**: All matching hooks run in parallel
- **Deduplication**: Multiple identical hook commands are deduplicated

### Full Configuration Example

```json
{
  "hooks": {
    "SessionStart": [
      {
        "matcher": "",
        "hooks": [
          { "type": "command", "command": "ctxium hook SessionStart" }
        ]
      }
    ],
    "PreToolUse": [
      {
        "matcher": "*",
        "hooks": [
          { "type": "command", "command": "ctxium hook PreToolUse" }
        ]
      }
    ],
    "PostToolUse": [
      {
        "matcher": "*",
        "hooks": [
          { "type": "command", "command": "ctxium hook PostToolUse" }
        ]
      }
    ],
    "UserPromptSubmit": [
      {
        "matcher": "",
        "hooks": [
          { "type": "command", "command": "ctxium hook UserPromptSubmit" }
        ]
      }
    ],
    "PermissionRequest": [
      {
        "matcher": "",
        "hooks": [
          { "type": "command", "command": "ctxium hook PermissionRequest" }
        ]
      }
    ],
    "Stop": [
      {
        "matcher": "",
        "hooks": [
          { "type": "command", "command": "ctxium hook Stop" }
        ]
      }
    ],
    "SubagentStop": [
      {
        "matcher": "",
        "hooks": [
          { "type": "command", "command": "ctxium hook SubagentStop" }
        ]
      }
    ],
    "PreCompact": [
      {
        "matcher": "",
        "hooks": [
          { "type": "command", "command": "ctxium hook PreCompact" }
        ]
      }
    ],
    "SessionEnd": [
      {
        "matcher": "",
        "hooks": [
          { "type": "command", "command": "ctxium hook SessionEnd" }
        ]
      }
    ]
  }
}
```

### Official Documentation

- [Hooks reference - Claude Code Docs](https://docs.claude.com/en/docs/claude-code/hooks)
- [How to configure hooks](https://claude.com/blog/how-to-configure-hooks)

## Next Steps

Once hook payloads are understood:

1. Design proper data models based on actual payload structure
2. Build the full Contextium server with persistent storage
3. Implement context injection based on hook types
4. Add filtering/routing logic for different hook types
