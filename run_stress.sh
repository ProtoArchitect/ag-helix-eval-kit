#!/bin/bash
echo "=== PRE-TEST SENSORS ==="
sensors | grep Tctl

echo "=== STARTING STRESS TEST IN BACKGROUND ==="
./stress_qtdu > stress_results.txt &
PID=$!

echo "=== GATHERING CPU UTILIZATION ==="
top -b -n 2 -d 10 | grep -A 2 "%Cpu"

echo "=== PEAK TEST SENSORS ==="
sensors | grep Tctl

wait $PID
echo "=== BENCHMARK RESULTS ==="
cat stress_results.txt
