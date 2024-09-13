#!/bin/bash
set -e 

cargo build --release

find . -maxdepth 1 -type d -name "p*" -exec sh -c '
  for dir in "$@"; do
    cargo run --bin "$(basename "$dir")" --release 2> /dev/null
  done
' sh {} +