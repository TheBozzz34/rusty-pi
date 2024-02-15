#!/bin/bash

# Define colors for output
GREEN='\033[0;32m'
RED='\033[0;31m'
NC='\033[0m' # No Color

# Function to print error messages
print_error() {
    echo -e "${RED}Error: $1${NC}" >&2
}

# Function to remove existing builds
remove_existing_builds() {
    if [[ -d "target" ]]; then
        echo -e "Removing existing builds..."
        rm -rf target
        rm -rf kernel7.img
    fi
}


# Main script starts here
echo -e "Building!"

# Remove existing builds
remove_existing_builds

# Parse input options for debug or release build
if [[ "$1" == "debug" ]]; then
    echo "Building debug version..."
    cargo rustc -- -C link-arg=--script=./linker.ld || { print_error "Build failed"; exit 1; }
elif [[ "$1" == "release" ]]; then
    echo "Building release version..."
    cargo rustc --release -- -C link-arg=--script=./linker.ld || { print_error "Build failed"; exit 1; }
else
    print_error "Invalid option: Specify 'debug' or 'release'"
    exit 1
fi


# Copying to kernel7.img
echo "Copying to kernel7.img"
arm-none-eabi-objcopy -O binary target/armv7a-none-eabi/"$1"/rusty-pi ./kernel7.img || { print_error "Copying failed"; exit 1; }

echo -e "${GREEN}Build successful.${NC}"

