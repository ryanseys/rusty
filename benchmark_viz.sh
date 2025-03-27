#!/bin/bash

# Exit on error
set -e

echo "Starting Fibonacci performance simulation..."

# Make visualization directory if it doesn't exist
mkdir -p visualization

# Run the simulation
ruby simulate.rb

# Check if Python 3 is available
if command -v python3 &> /dev/null; then
    PYTHON_CMD="python3"
elif command -v python &> /dev/null; then
    PYTHON_CMD="python"
else
    echo "Error: Python is not installed"
    exit 1
fi

echo "Simulation complete. Starting web server..."
echo "Please open http://localhost:8000/visualization/ in your web browser"
echo "Press Ctrl+C to stop the server"

# Start a simple HTTP server
cd "$(dirname "$0")"
$PYTHON_CMD -m http.server 8000