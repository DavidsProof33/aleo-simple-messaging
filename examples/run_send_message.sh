#!/usr/bin/env bash
set -euo pipefail

# examples/run_send_message.sh
# Simple example execution (local) from inside WSL.

# Go to the Leo program directory (WSL path)
SCRIPT_DIR="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)"
cd "$SCRIPT_DIR/../leo/simple_messaging"

# Example values (replace with your own)
SENDER="aleo1sender..."
RECIPIENT="aleo1recipient..."

leo execute send_message \
  "$SENDER" \
  "$RECIPIENT" \
  1field \
  10field \
  20field \
  30field
