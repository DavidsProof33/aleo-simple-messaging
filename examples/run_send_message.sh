#!/bin/bash
# Example: Simple Leo execution for the simple_messaging demo

# Navigate to the Leo program directory
cd "$(dirname "$0")/../leo/simple_messaging"

# Example execution (with test values)
leo execute simple_messaging.aleo send_message \
  aleo1sender... \
  aleo1recipient... \
  1field \
  10field \
  20field \
  30field
