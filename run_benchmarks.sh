#!/bin/bash

# Number of runs for each configuration
RUNS=10

# Clear previous results
echo "Clearing previous results..."
rm -f results/results.csv

echo "Running benchmarks with $RUNS runs each..."
echo ""

# Sequential benchmarks
echo "=== Sequential (seq) ==="
for i in $(seq 1 $RUNS); do
    echo "  Run $i/$RUNS - kernel 3x3..."
    ./target/release/median-filter -i image.jpg -o output_seq_3.jpg -n 0.1 -m seq -k 3 > /dev/null
    echo "  Run $i/$RUNS - kernel 5x5..."
    ./target/release/median-filter -i image.jpg -o output_seq_5.jpg -n 0.1 -m seq -k 5 > /dev/null
done

# Parallel benchmarks
echo "=== Parallel (par) ==="
for i in $(seq 1 $RUNS); do
    echo "  Run $i/$RUNS - kernel 3x3..."
    ./target/release/median-filter -i image.jpg -o output_par_3.jpg -n 0.1 -m par -k 3 > /dev/null
    echo "  Run $i/$RUNS - kernel 5x5..."
    ./target/release/median-filter -i image.jpg -o output_par_5.jpg -n 0.1 -m par -k 5 > /dev/null
done

# GPU benchmarks
echo "=== GPU (gpu) ==="
for i in $(seq 1 $RUNS); do
    echo "  Run $i/$RUNS - kernel 3x3..."
    ./target/release/median-filter -i image.jpg -o output_gpu_3.jpg -n 0.1 -m gpu -k 3 > /dev/null
    echo "  Run $i/$RUNS - kernel 5x5..."
    ./target/release/median-filter -i image.jpg -o output_gpu_5.jpg -n 0.1 -m gpu -k 5 > /dev/null
done

# Distributed benchmarks with different process counts
for np in 2 4 8; do
    echo "=== Distributed (dist) with $np processes ==="
    for i in $(seq 1 $RUNS); do
        echo "  Run $i/$RUNS - kernel 3x3..."
        mpirun -np $np ./target/release/median-filter -i image.jpg -o output_dist_${np}_3.jpg -n 0.1 -m dist -k 3 > /dev/null
        echo "  Run $i/$RUNS - kernel 5x5..."
        mpirun -np $np ./target/release/median-filter -i image.jpg -o output_dist_${np}_5.jpg -n 0.1 -m dist -k 5 > /dev/null
    done
done

echo ""
echo "All benchmarks completed!"
echo "Results saved to: results/results.csv"
