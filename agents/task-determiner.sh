#!/usr/bin/env bash
#
# agents/task-determiner.sh
#
# Task determination agent - analyzes state and determines current task.
# Outputs task information for the main Claude session.

set -e

cd "$(dirname "$0")/.."

# Colors
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
CYAN='\033[0;36m'
NC='\033[0m'

log_info() { echo -e "${GREEN}[task]${NC} $1"; }
log_warn() { echo -e "${YELLOW}[task]${NC} $1"; }
log_error() { echo -e "${RED}[task]${NC} $1"; }
log_agent() { echo -e "${CYAN}[agent]${NC} $1"; }

STATE_DIR=".contextium"
STATE_FILE="$STATE_DIR/state.json"
TASKS_FILE="$STATE_DIR/tasks.json"
PROGRESS_FILE="claude-progress.txt"

# Check if jq is available
require_jq() {
    if ! command -v jq >/dev/null 2>&1; then
        log_error "jq required for task determination"
        exit 1
    fi
}

# Analyze git history for context
analyze_git_history() {
    if git rev-parse --is-inside-work-tree >/dev/null 2>&1; then
        log_info "Analyzing recent git history..."
        echo ""
        echo "=== RECENT COMMITS ==="
        git log --oneline -5 2>/dev/null || echo "No commits yet"
        echo ""

        # Check for in-progress work
        local uncommitted=$(git status --porcelain | wc -l)
        if [ "$uncommitted" -gt 0 ]; then
            echo "=== UNCOMMITTED CHANGES ==="
            git status --short
            echo ""
        fi
    fi
}

# Check for progress file (harness pattern)
check_progress_file() {
    if [ -f "$PROGRESS_FILE" ]; then
        log_info "Found progress file"
        echo ""
        echo "=== PROGRESS FILE ==="
        cat "$PROGRESS_FILE"
        echo ""
        echo "===================="
    fi
}

# Determine current task from tasks.json
determine_current_task() {
    if [ ! -f "$TASKS_FILE" ]; then
        log_warn "No tasks file found"
        return 0
    fi

    log_info "Analyzing task list..."

    local task_count=$(jq '.tasks | length' "$TASKS_FILE")

    if [ "$task_count" -eq 0 ]; then
        log_info "No tasks defined - ready for new work"
        return 0
    fi

    # Find first incomplete task
    local current_task=$(jq -r '
        .tasks |
        map(select(.status != "completed")) |
        first // empty |
        .id
    ' "$TASKS_FILE")

    if [ -n "$current_task" ] && [ "$current_task" != "null" ]; then
        log_info "Current task: $current_task"

        # Update state
        jq --arg task "$current_task" '.current = $task' "$TASKS_FILE" > "$TASKS_FILE.tmp" \
            && mv "$TASKS_FILE.tmp" "$TASKS_FILE"

        # Output task details
        echo ""
        echo "=== CURRENT TASK ==="
        jq -r --arg id "$current_task" '
            .tasks[] | select(.id == $id) |
            "ID: \(.id)\nDescription: \(.description)\nStatus: \(.status)\nPriority: \(.priority // "normal")"
        ' "$TASKS_FILE"
        echo "===================="
    else
        log_info "All tasks completed!"
    fi
}

# Check branch name for task hints
check_branch_task() {
    local branch=$(git branch --show-current 2>/dev/null)
    if [ -n "$branch" ]; then
        # Extract task hint from branch name patterns like:
        # feature/add-login, fix/bug-123, claude/task-xyz
        case "$branch" in
            feature/*|feat/*)
                log_info "Branch suggests feature work: ${branch#*/}"
                ;;
            fix/*|bugfix/*)
                log_info "Branch suggests bugfix: ${branch#*/}"
                ;;
            claude/*)
                log_info "Claude session branch: ${branch#*/}"
                ;;
        esac
    fi
}

# Output determination summary
output_summary() {
    echo ""
    echo "=== TASK DETERMINATION SUMMARY ==="

    if [ -f "$TASKS_FILE" ]; then
        local total=$(jq '.tasks | length' "$TASKS_FILE")
        local completed=$(jq '[.tasks[] | select(.status == "completed")] | length' "$TASKS_FILE")
        local pending=$(jq '[.tasks[] | select(.status == "pending")] | length' "$TASKS_FILE")
        local in_progress=$(jq '[.tasks[] | select(.status == "in_progress")] | length' "$TASKS_FILE")

        echo "Total tasks: $total"
        echo "Completed: $completed"
        echo "In progress: $in_progress"
        echo "Pending: $pending"
    else
        echo "No task list found"
        echo "Ready for new work or task creation"
    fi

    echo "==================================="
}

# Main
main() {
    log_agent "Task determination agent starting..."

    require_jq
    analyze_git_history
    check_progress_file
    check_branch_task
    determine_current_task
    output_summary

    log_agent "Task determination complete"
}

main "$@"
