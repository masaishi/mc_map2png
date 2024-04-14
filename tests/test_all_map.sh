#!/bin/bash

# Define paths
executable="./target/debug/mc_map2png"

for i in {0..36}
do
    file_name="map_$i"
    input_path="./tests/data/blaze/$file_name.dat"
    output_path="./tests/outputs/blaze/output_$file_name.png"

    echo "Running test with input file: $input_path"

    # Run the program with the test data
    $executable "$input_path" "$output_path"

    # Check if the output file was created
    if [ ! -f "$output_path" ]; then
        echo "Test failed: Output file not created for $file_name."
        exit 1
    else
        echo "Output file created successfully for $file_name."
    fi
done
