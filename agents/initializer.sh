#!/usr/bin/env bash
#
# agents/initializer.sh
#
# Session initialization agent - verifies environment and prepares state files.
# Runs as part of SessionStart hook.

set -e

cd "$(dirname "$0")/.."

# Colors
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
CYAN='\033[0;36m'
NC='\033[0m'

log_info() { echo -e "${GREEN}[init]${NC} $1"; }
log_warn() { echo -e "${YELLOW}[init]${NC} $1"; }
log_error() { echo -e "${RED}[init]${NC} $1"; }
log_agent() { echo -e "${CYAN}[agent]${NC} $1"; }

STATE_DIR=".contextium"
STATE_FILE="$STATE_DIR/state.json"
TASKS_FILE="$STATE_DIR/tasks.json"

# Ensure state directory exists
init_state_dir() {
    mkdir -p "$STATE_DIR/context-cache"
    mkdir -p "$STATE_DIR/sessions"
}

# Check for required tools
check_tools() {
    local missing=()

    # Core tools (warnings only)
    command -v git >/dev/null 2>&1 || missing+=("git")
    command -v jq >/dev/null 2>&1 || missing+=("jq")

    if [ ${#missing[@]} -gt 0 ]; then
        log_warn "Missing recommended tools: ${missing[*]}"
    fi

    # Optional Contextium tools
    local optional=()
    command -v wt >/dev/null 2>&1 || optional+=("worktrunk")
    command -v memvid >/dev/null 2>&1 || optional+=("memvid")
    command -v gibram-server >/dev/null 2>&1 || optional+=("gibram")

    if [ ${#optional[@]} -gt 0 ]; then
        log_info "Optional tools not installed: ${optional[*]}"
        log_info "Run script/bootstrap to install"
    fi
}

# Initialize or load session state
init_state() {
    if [ ! -f "$STATE_FILE" ]; then
        log_info "Creating initial state file..."
        cat > "$STATE_FILE" << EOF
{
  "version": "1.0.0",
  "session": {
    "id": "$(date +%Y%m%d-%H%M%S)-$$",
    "started": "$(date -Iseconds)",
    "repo": "$(basename "$(git rev-parse --show-toplevel 2>/dev/null || pwd)")",
    "branch": "$(git branch --show-current 2>/dev/null || echo 'unknown')"
  },
  "status": "initialized",
  "current_task": null,
  "completed_tasks": []
}
EOF
    else
        log_info "Loading existing state..."
        # Update session info
        if command -v jq >/dev/null 2>&1; then
            local tmp=$(mktemp)
            jq --arg id "$(date +%Y%m%d-%H%M%S)-$$" \
               --arg started "$(date -Iseconds)" \
               --arg branch "$(git branch --show-current 2>/dev/null || echo 'unknown')" \
               '.session.id = $id | .session.started = $started | .session.branch = $branch | .status = "initialized"' \
               "$STATE_FILE" > "$tmp" && mv "$tmp" "$STATE_FILE"
        fi
    fi
}

# Initialize tasks file if needed
init_tasks() {
    if [ ! -f "$TASKS_FILE" ]; then
        log_info "Creating initial tasks file..."
        cat > "$TASKS_FILE" << EOF
{
  "version": "1.0.0",
  "tasks": [],
  "current": null,
  "history": []
}
EOF
    fi
}

# Check git status
check_git_status() {
    if git rev-parse --is-inside-work-tree >/dev/null 2>&1; then
        local branch=$(git branch --show-current)
        local status=$(git status --porcelain | wc -l)
        log_info "Git branch: $branch"
        if [ "$status" -gt 0 ]; then
            log_warn "Uncommitted changes: $status files"
        fi
    fi
}

# Output summary for Claude
output_summary() {
    log_agent "Session initialized"
    echo ""
    echo "=== SESSION SUMMARY ==="
    if [ -f "$STATE_FILE" ]; then
        if command -v jq >/dev/null 2>&1; then
            echo "Session ID: $(jq -r '.session.id' "$STATE_FILE")"
            echo "Repository: $(jq -r '.session.repo' "$STATE_FILE")"
            echo "Branch: $(jq -r '.session.branch' "$STATE_FILE")"
            echo "Status: $(jq -r '.status' "$STATE_FILE")"

            if [ -f "$TASKS_FILE" ]; then
                local task_count=$(jq '.tasks | length' "$TASKS_FILE")
                local current=$(jq -r '.current // "none"' "$TASKS_FILE")
                echo "Tasks: $task_count total, current: $current"
            fi
        else
            # Fallback without jq
            echo "Repository: $(basename "$(git rev-parse --show-toplevel 2>/dev/null || pwd)")"
            echo "Branch: $(git branch --show-current 2>/dev/null || echo 'unknown')"
            echo "Status: initialized"
            echo "(Install jq for detailed state info)"
        fi
    fi
    echo "======================="
}

# Main
main() {
    log_agent "Initializer agent starting..."

    init_state_dir
    check_tools
    init_state
    init_tasks
    check_git_status
    output_summary

    log_agent "Initialization complete"
}

main "$@"
