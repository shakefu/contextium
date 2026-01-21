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

## Known Hook Types

Based on Claude Code documentation:

| Hook | When Triggered |
| -- | -- |
| `SessionStart` | When a Claude Code session begins |
| `SessionEnd` | When a Claude Code session ends |
| `PreToolUse` | Before Claude executes a tool |
| `PostToolUse` | After Claude executes a tool |
| `UserPromptSubmit` | When user submits a prompt |

## Next Steps

Once hook payloads are understood:

1. Design proper data models based on actual payload structure
2. Build the full Contextium server with persistent storage
3. Implement context injection based on hook types
4. Add filtering/routing logic for different hook types
