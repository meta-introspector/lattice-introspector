#!/bin/bash

LOG_DIR="test_traces_primes"
mkdir -p $LOG_DIR

# Build the project once before running tests
echo "Building ultimate_blinkenlights_simulation..."
cargo build --package ultimate_blinkenlights_simulation > /dev/null 2>&1
if [ $? -ne 0 ]; then
    echo "Build failed. Exiting."
    exit 1
fi
echo "Build complete."

# Get primes from the Python script
PRIMES=$(python3 /data/data/com.termux/files/home/storage/github/rustc/crates/introspector/generate_prime_resonances.py)

EXECUTABLE="/data/data/com.termux/files/home/storage/github/rustc/crates/introspector/crates/ultimate_blinkenlights_simulation/target/debug/ultimate_blinkenlights_simulation"

for STEP_LIMIT in $PRIMES; do
    # Skip 0 and 1 if they are in the primes list, as they might not be suitable step limits
    if [ "$STEP_LIMIT" -eq 0 ] || [ "$STEP_LIMIT" -eq 1 ]; then
        continue
    fi

    LOG_FILE="$LOG_DIR/trace_run_${STEP_LIMIT}_steps.log"
    echo "Running with --step-limit=$STEP_LIMIT, logging to $LOG_FILE"
    timeout 3s "$EXECUTABLE" --step-limit=$STEP_LIMIT > "$LOG_FILE" 2>&1
    EXIT_CODE=$? # Capture the exit code of the timeout command
    
    if [ $EXIT_CODE -eq 124 ]; then
        echo "Run with --step-limit=$STEP_LIMIT timed out after 3 seconds. Stopping script."
        exit 0 # Exit the script upon timeout
    elif [ $EXIT_CODE -ne 0 ]; then
        echo "Run with --step-limit=$STEP_LIMIT failed with exit code $EXIT_CODE."
    else
        echo "Finished run with --step-limit=$STEP_LIMIT successfully."
    fi
    sleep 1 # Give a small pause between runs
done

echo "Test driver finished."