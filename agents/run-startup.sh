#!/usr/bin/env bash
#
# agents/run-startup.sh
#
# Main startup orchestrator - runs all initialization agents in sequence.
# Called by Claude Code SessionStart hook.

set -e

cd "$(dirname "$0")/.."

AGENTS_DIR="$(dirname "$0")"
LOG_DIR="/tmp/contextium"

# Colors
CYAN='\033[0;36m'
GREEN='\033[0;32m'
NC='\033[0m'

log_orchestrator() { echo -e "${CYAN}[orchestrator]${NC} $1"; }

# Setup logging
setup_logging() {
    mkdir -p "$LOG_DIR"
}

# Run agent and capture output
run_agent() {
    local agent="$1"
    local script="$AGENTS_DIR/$agent.sh"
    local log_file="$LOG_DIR/$agent.log"

    if [ -x "$script" ]; then
        log_orchestrator "Running $agent agent..."
        "$script" 2>&1 | tee "$log_file"
        echo ""
    else
        log_orchestrator "Agent not found: $agent"
    fi
}

# Main orchestration
main() {
    echo -e "${CYAN}╔════════════════════════════════════════╗${NC}"
    echo -e "${CYAN}║     Contextium Session Startup         ║${NC}"
    echo -e "${CYAN}╚════════════════════════════════════════╝${NC}"
    echo ""

    setup_logging

    # Phase 1: Initialize environment
    run_agent "initializer"

    # Phase 2: Determine current task
    run_agent "task-determiner"

    # Phase 3: Fetch relevant context
    run_agent "context-fetcher"

    echo -e "${GREEN}╔════════════════════════════════════════╗${NC}"
    echo -e "${GREEN}║     Session Ready                      ║${NC}"
    echo -e "${GREEN}╚════════════════════════════════════════╝${NC}"
    echo ""
    echo "Logs available in: $LOG_DIR/"
}

main "$@"
