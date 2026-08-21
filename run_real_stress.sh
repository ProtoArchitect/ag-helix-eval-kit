#!/bin/bash
echo "=== PRE-TEST SENSORS ==="
sensors | grep Tctl

echo "=== GATHERING CPU UTILIZATION ==="
top -b -n 2 -d 1 | grep -A 2 "%Cpu" > top_output.txt &
TOP_PID=$!

echo "=== STARTING ZERO-COPY SWAR ON REAL 3GB GENOME ==="
./bench_real_fa > bench_results.txt

echo "=== PEAK TEST SENSORS ==="
sensors | grep Tctl

wait $TOP_PID
echo "=== CPU TOP RESULTS ==="
cat top_output.txt
echo "=== BENCHMARK RESULTS ==="
cat bench_results.txt
