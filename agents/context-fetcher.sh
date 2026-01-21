#!/usr/bin/env bash
#
# agents/context-fetcher.sh
#
# Context fetcher agent - retrieves minimal relevant context for the current task.
# Follows the "minimal context" philosophy: only load what's needed.

set -e

cd "$(dirname "$0")/.."

# Colors
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
CYAN='\033[0;36m'
NC='\033[0m'

log_info() { echo -e "${GREEN}[ctx]${NC} $1"; }
log_warn() { echo -e "${YELLOW}[ctx]${NC} $1"; }
log_error() { echo -e "${RED}[ctx]${NC} $1"; }
log_agent() { echo -e "${CYAN}[agent]${NC} $1"; }

STATE_DIR=".contextium"
TASKS_FILE="$STATE_DIR/tasks.json"
CONTEXT_CACHE="$STATE_DIR/context-cache"
CONTEXT_DIR="context"

# Ensure cache directory exists
init_cache() {
    mkdir -p "$CONTEXT_CACHE"
}

# Get current task ID
get_current_task() {
    if [ -f "$TASKS_FILE" ] && command -v jq >/dev/null 2>&1; then
        jq -r '.current // empty' "$TASKS_FILE"
    fi
}

# Get task scope (files/modules the task should touch)
get_task_scope() {
    local task_id="$1"
    if [ -f "$TASKS_FILE" ] && command -v jq >/dev/null 2>&1; then
        jq -r --arg id "$task_id" '
            .tasks[] | select(.id == $id) |
            .scope // [] | join("\n")
        ' "$TASKS_FILE"
    fi
}

# Load project documentation if available
load_project_docs() {
    log_info "Loading project documentation..."

    # Check for context directory
    if [ -d "$CONTEXT_DIR" ]; then
        echo ""
        echo "=== PROJECT CONTEXT ==="

        # Load purpose if available
        if [ -f "$CONTEXT_DIR/purpose.md" ]; then
            echo "--- Purpose ---"
            head -50 "$CONTEXT_DIR/purpose.md"
            echo ""
        fi

        # Load summary if available
        if [ -f "$CONTEXT_DIR/summary.md" ]; then
            echo "--- Summary ---"
            head -30 "$CONTEXT_DIR/summary.md"
            echo ""
        fi

        echo "========================"
    fi

    # Check for Claude instructions
    if [ -f ".claude/claude.md" ]; then
        log_info "Claude instructions loaded from .claude/claude.md"
    fi
}

# Load task-specific context
load_task_context() {
    local task_id="$1"

    if [ -z "$task_id" ]; then
        log_info "No current task - loading general context only"
        return 0
    fi

    log_info "Loading context for task: $task_id"

    # Check for task-specific context file
    local task_context="$CONTEXT_CACHE/$task_id.md"
    if [ -f "$task_context" ]; then
        echo ""
        echo "=== TASK-SPECIFIC CONTEXT ==="
        cat "$task_context"
        echo "=============================="
    fi

    # Load scope files
    local scope=$(get_task_scope "$task_id")
    if [ -n "$scope" ]; then
        echo ""
        echo "=== TASK SCOPE ==="
        echo "Files/modules in scope:"
        echo "$scope" | while read -r pattern; do
            if [ -n "$pattern" ]; then
                echo "  - $pattern"
            fi
        done
        echo "==================="
    fi
}

# Check for relevant patterns in codebase
detect_patterns() {
    log_info "Detecting codebase patterns..."

    # Detect project type
    local project_type=""
    [ -f "package.json" ] && project_type="node"
    [ -f "Cargo.toml" ] && project_type="rust"
    [ -f "pyproject.toml" ] || [ -f "setup.py" ] && project_type="python"
    [ -f "go.mod" ] && project_type="go"

    if [ -n "$project_type" ]; then
        echo ""
        echo "=== PROJECT TYPE ==="
        echo "Detected: $project_type"

        case "$project_type" in
            node)
                [ -f "package.json" ] && echo "Entry: $(jq -r '.main // .module // "index.js"' package.json 2>/dev/null)"
                ;;
            rust)
                [ -f "Cargo.toml" ] && echo "Crate: $(grep '^name' Cargo.toml | head -1 | cut -d'"' -f2)"
                ;;
            python)
                [ -f "pyproject.toml" ] && echo "Project: $(grep '^name' pyproject.toml | head -1 | cut -d'"' -f2 2>/dev/null)"
                ;;
        esac
        echo "===================="
    fi
}

# Load memory from memvid if available
load_memory() {
    if command -v memvid >/dev/null 2>&1; then
        local memory_file="$STATE_DIR/memory.mv2"
        if [ -f "$memory_file" ]; then
            log_info "Memvid memory file available: $memory_file"
            # Note: Actual memvid queries would be done by Claude via the SDK
        fi
    fi
}

# Load knowledge from gibram if available
load_knowledge_graph() {
    if command -v gibram-server >/dev/null 2>&1 || curl -s http://localhost:6161/health >/dev/null 2>&1; then
        log_info "GibRAM knowledge graph available on port 6161"
        # Note: Actual gibram queries would be done by Claude via the SDK
    fi
}

# Output context summary
output_summary() {
    echo ""
    echo "=== CONTEXT SUMMARY ==="
    echo "Loaded context sources:"

    [ -d "$CONTEXT_DIR" ] && echo "  - Project documentation ($CONTEXT_DIR/)"
    [ -f ".claude/claude.md" ] && echo "  - Claude instructions (.claude/claude.md)"
    [ -f "$STATE_DIR/memory.mv2" ] && echo "  - Memvid memory"

    local task_id=$(get_current_task)
    [ -n "$task_id" ] && echo "  - Task context ($task_id)"

    echo ""
    echo "Context philosophy: Minimal by default"
    echo "Additional context injected when relevant"
    echo "========================"
}

# Main
main() {
    log_agent "Context fetcher agent starting..."

    init_cache

    local task_id=$(get_current_task)

    load_project_docs
    load_task_context "$task_id"
    detect_patterns
    load_memory
    load_knowledge_graph
    output_summary

    log_agent "Context fetch complete"
}

main "$@"
