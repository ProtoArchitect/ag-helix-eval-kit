# AG-Helix Open Evaluation Framework

This repository provides an open-source benchmarking harness for independently verifying the memory and performance claims of the AG-Helix topological processor.

**Our goal is transparency and independent falsification.** We invite researchers and computational biologists to clone this repository, inspect the memory-tracking code, and run these tests against their own genomic data to identify edge cases and failure modes.

## How it works
This kit includes:
1. **Open Memory-Tracking Harness (`src/memory_telemetry.rs`):** A transparent Rust implementation of a custom `GlobalAlloc` that intercepts and logs every byte of heap memory requested by the OS.
2. **Topological Stress Tests (`src/biological_stress.rs`):** Scripts to measure how the AG384 Euler invariants respond to induced frameshifts and noise.
3. **Pre-compiled Container Generator (`bin/qtdu_bench`):** A utility to convert your FASTA files into the `.qtdu` topological format for testing. (The core encoding algorithms are pending publication).

## Running the Benchmark

```bash
# 1. Clone the repository
git clone https://github.com/ProtoArchitect/ag-helix-eval-kit.git
cd ag-helix-eval-kit

# 2. Encode your clinical FASTA dataset
./bin/qtdu_bench my_dataset.fa my_dataset.qtdu 1

# 3. Run the Open-Source Zero-Copy Verifier
# (Observe the memory allocations reported by the transparent harness)
cargo run --release --bin memory_telemetry -- my_dataset.qtdu
```
