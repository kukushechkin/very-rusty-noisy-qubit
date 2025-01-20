#!/bin/sh
set -e

# Get the directory of this script in a POSIX-compatible way
DIR=$(dirname "$0")
cd "$DIR"
DIR=$(pwd)

# Compile the test
cc "$DIR/test.c" \
   -o "$DIR/ctest" \
   -I"$DIR/../very-noisy-qubit-api/include" \
   -L"$DIR/../target/debug" \
   -lvery_rusty_noisy_qubit \
   -Wl,-rpath,"$DIR/../target/debug"

# Run the test
"$DIR/ctest"
