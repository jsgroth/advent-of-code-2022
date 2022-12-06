#!/usr/bin/env bash

set -euo pipefail

if [[ ! -z "${AOC_REBUILD:-}" ]]; then
    cargo clean
    cargo test
    cargo build --release
fi

for i in `seq 1 25`; do
    executable=target/release/day$i
    if [[ -f "$executable" ]]; then
        echo "---------- DAY $i ----------"
        $executable input/input$i.txt
    fi
done
