#!/bin/bash

INPUT_DIR="/data/data/com.termux/files/home/storage/github/rustc/crates/introspector/extracted_media/frames/"
OUTPUT_BASE_DIR="/data/data/com.termux/files/home/storage/github/rustc/crates/introspector/data/ascii_variants/"
GEN_ASCII_ART_PATH="/data/data/com.termux/files/home/.cargo/bin/gen-ascii-art"

WIDTH=$1
NUM_FRAMES=${2:-5} # Default to 5 frames if not specified

if [ -z "$WIDTH" ]; then
    echo "Usage: $0 <target_width> [num_frames]"
    exit 1
fi

OUTPUT_DIR="${OUTPUT_BASE_DIR}/width_${WIDTH}/"
mkdir -p "$OUTPUT_DIR"

echo "Generating ASCII art for width ${WIDTH} characters, processing ${NUM_FRAMES} frames..."

for i in $(seq -f "%05g" 1 "$NUM_FRAMES"); do
    INPUT_FILE="${INPUT_DIR}/frame_${i}.png"
    OUTPUT_FILE="${OUTPUT_DIR}/frame_${i}.txt"
    echo "Processing ${INPUT_FILE} -> ${OUTPUT_FILE}"
    "$GEN_ASCII_ART_PATH" -i "$INPUT_FILE" -w "$WIDTH" > "$OUTPUT_FILE"
done

echo "ASCII art generation complete for width ${WIDTH}."