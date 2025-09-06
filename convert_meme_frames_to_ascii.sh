#!/bin/bash

INPUT_DIR="/data/data/com.termux/files/home/storage/github/rustc/crates/introspector/extracted_media/frames/"
OUTPUT_DIR="/data/data/com.termux/files/home/storage/github/rustc/crates/introspector/extracted_media/ascii_frames_original_meme/"
MONOCHORA_PATH="/data/data/com.termux/files/home/.cargo/bin/monochora"

mkdir -p "$OUTPUT_DIR"

echo "Converting original meme frames to ASCII art..."

for i in $(seq -f "%05g" 1 624); do
    INPUT_FILE="${INPUT_DIR}/frame_${i}.png"
    OUTPUT_FILE="${OUTPUT_DIR}/frame_${i}.txt"
    echo "Processing ${INPUT_FILE} -> ${OUTPUT_FILE}"
    "$MONOCHORA_PATH" -i "$INPUT_FILE" -o "$OUTPUT_FILE"
done

echo "ASCII art conversion complete."