#!/bin/bash
# Examples for using Yee-Haw with test data

# Make sure target directories exist
mkdir -p output
mkdir -p output_with_dupes

echo "=== Example 1: Basic usage - organize photos ==="
cargo run -- -s ./test_data/photos -d ./output -q "*.jpg" --dry

echo -e "\n=== Example 2: Organize photos with duplicate tracking enabled ==="
cargo run -- -s ./test_data/photos -d ./output_with_dupes -q "*.jpg" --track-duplicates --dry

echo -e "\n=== Example 3: Organize all files with lowercase renaming ==="
cargo run -- -s ./test_data -d ./output -q "*" --rename-style lowercase --dry

echo -e "\n=== Example 4: Run with custom grouping style ==="
cargo run -- -s ./test_data -d ./output -q "*" --group-style incremental --dry

echo -e "\n=== Example 5: Actually move files (remove --dry to execute) ==="
cargo run -- -s ./test_data -d ./output -q "*.pdf" --track-duplicates --rename-style combined --dry

echo -e "\n=== You can uncomment the line below to actually move files ==="
cargo run -- -s ./test_data -d ./output -q "*.pdf" --track-duplicates --rename-style combined

echo -e "\nAll examples complete!" 