#!/bin/bash

# Arg 1: Number of iterations (Default: 10)
MAX_ITERATIONS=${1:-10}
# Arg 2: PRD filename (Default: PRD.md)
PRD_FILE=${2:-PRD.md}

PROGRESS_FILE="progress.txt"
TEMP_LOG=$(mktemp) # Create a temporary file to capture output for checking

# Validation
if [[ ! -f "$PRD_FILE" ]]; then
    echo "❌ Error: File '$PRD_FILE' not found."
    exit 1
fi

# Ensure cleanup of temp file on exit
trap "rm -f $TEMP_LOG" EXIT

echo "🚀 Starting Ralph Loop (Streaming Mode)"
echo "Target: $PRD_FILE"
echo "Iterations: $MAX_ITERATIONS"

for ((i=1; i<=MAX_ITERATIONS; i++)); do
    echo ""
    echo "=================================================="
    echo "🔄 Iteration $i / $MAX_ITERATIONS"
    echo "=================================================="

    # Run claude and pipe output to tee. 
    # This streams to stdout immediately AND writes to the temp file.
    claude --dangerously-skip-permissions -p "
    GOAL: Autonomous Task Completion
    
    CONTEXT FILES:
    - @$PRD_FILE (The master task list)
    - @$PROGRESS_FILE (The persistent memory/log)

    INSTRUCTIONS:
    1. Read @$PRD_FILE. Find the *first* incomplete task (unchecked box [ ]).
    2. IF ALL TASKS ARE COMPLETE: 
       - Output the exact text: <RALPH_COMPLETE>
       - Do nothing else.
    3. IF TASKS REMAIN:
       - Implement *only* that one task.
       - Run tests/checks to verify it works.
       - Update @$PROGRESS_FILE with a summary of what you did, what worked, and any errors encountered.
       - Update @$PRD_FILE by marking the task as complete ([x]).
       - Commit your changes to git with a descriptive message.
    
    IMPORTANT:
    - Start a FRESH session for this task.
    - Do NOT attempt multiple tasks.
    " | tee "$TEMP_LOG"

    # Check the temp file for the completion signal
    if grep -q "<RALPH_COMPLETE>" "$TEMP_LOG"; then
        echo ""
        echo "✅ All tasks in $PRD_FILE are complete!"
        exit 0
    fi
    
    # Optional: slight pause to let file I/O settle
    sleep 2
done

echo ""
echo "⚠️ Max iterations reached. Check $PRD_FILE for remaining tasks."
