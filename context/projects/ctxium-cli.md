# ctxium CLI

Rust-based CLI tool for exploring Claude Code hook payloads.

## Purpose

Before building the full Contextium server, we need to understand exactly what data Claude Code sends through its hooks. This tool captures and displays hook payloads for analysis.

## Architecture

```
┌─────────────────┐     HTTP POST      ┌─────────────────┐
│  Claude Code    │   :6160/hook       │  ctxium         │
│  Hook Config    │──────────────────▶ │  hookserver     │
│                 │                    │                 │
│  calls:         │                    │  - pretty-print │
│  ctxium hook X  │                    │  - log to file  │
└─────────────────┘                    └─────────────────┘
        │                                      │
        │ stdin: JSON payload                  │
        ▼                                      ▼
┌─────────────────┐                    ~/.contextium/hooks.log
│  ctxium hook    │
│  <HookName>     │
│                 │
│  - reads stdin  │
│  - POST to srv  │
│  - silent fail  │
└─────────────────┘
```

## Building

```bash
cd /home/user/contextium/ctxium
cargo build --release

# Binary location
./target/release/ctxium
```

## Commands

### hookserver

Starts HTTP server to receive and display hook events.

```bash
ctxium hookserver [OPTIONS]

Options:
  -p, --port <PORT>      Port to listen on [default: 6160]
  -l, --log-file <PATH>  Log file path [default: ~/.contextium/hooks.log]
```

Output:
- Pretty-prints each hook to stdout with colors
- Appends JSON to log file (one line per hook)

### hook

Sends hook data to the hookserver. Called by Claude Code hooks.

```bash
ctxium hook <HOOK_NAME>

# Reads JSON from stdin, sends to http://127.0.0.1:6160/hook
# Exits 0 even if server unreachable (silent failure)
```

## Manual Testing

### Terminal 1: Start the server

```bash
cd /home/user/contextium/ctxium
cargo run -- hookserver
```

Expected output:
```
[ctxium] Hookserver listening on 127.0.0.1:6160
[ctxium] Log file: ~/.contextium/hooks.log
[ctxium] Waiting for hook events...
```

### Terminal 2: Send test hooks

```bash
# Simulate PreToolUse hook
echo '{"tool":"Bash","command":"ls -la"}' | ./target/debug/ctxium hook PreToolUse

# Simulate SessionStart hook
echo '{"session_id":"test123"}' | ./target/debug/ctxium hook SessionStart

# Simulate PostToolUse hook
echo '{"tool":"Read","result":"file contents..."}' | ./target/debug/ctxium hook PostToolUse
```

### Alternative: Test with curl

```bash
curl -X POST http://127.0.0.1:6160/hook \
  -H "Content-Type: application/json" \
  -d '{"hook_name":"TestHook","timestamp":"2024-01-01T00:00:00Z","stdin_data":{"test":"data"}}'
```

## Wiring to Claude Code Hooks

To capture real hook data from Claude Code, add to `.claude/settings.json`:

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

## Log File Format

Each line in `~/.contextium/hooks.log` is a JSON object:

```json
{"hook_name":"PreToolUse","timestamp":"2026-01-21T20:55:28.866944693+00:00","stdin_data":{"command":"ls -la","tool":"Bash"}}
```

Use `jq` to analyze:

```bash
# Pretty-print all hooks
cat ~/.contextium/hooks.log | jq .

# Filter by hook name
cat ~/.contextium/hooks.log | jq 'select(.hook_name == "PreToolUse")'

# Extract just stdin_data
cat ~/.contextium/hooks.log | jq '.stdin_data'
```

## Source Files

| File | Purpose |
|------|---------|
| `ctxium/Cargo.toml` | Dependencies: clap, axum, reqwest, serde, colored, chrono |
| `ctxium/src/main.rs` | CLI entry point with subcommand routing |
| `ctxium/src/hook.rs` | Hook command: stdin → HTTP POST, silent failure |
| `ctxium/src/hookserver.rs` | Server: receive, pretty-print, log |

## Next Steps

Once we understand the hook payload structure:

1. Design the full Contextium server API based on actual hook data
2. Implement context injection in PreToolUse hooks
3. Implement state persistence in SessionEnd hooks
4. Add task tracking based on tool usage patterns
