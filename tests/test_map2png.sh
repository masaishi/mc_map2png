#!/bin/bash

# Define paths
executable="./target/debug/mc_map2png"
file_name="map_test_0"
input_path="./tests/data/$file_name.dat"
output_path="./tests/outputs/output_$file_name.png"

echo "Running test with input file: $input_path"

# Run the program with the test data
$executable "$input_path" "$output_path"

# Check if the output file was created
if [ ! -f "$output_path" ]; then
    echo "Test failed: Output file not created."
    exit 1
else
    echo "Output file created successfully."
fi


