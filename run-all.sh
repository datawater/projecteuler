#!/bin/bash
set -e

cargo build --release

find . -maxdepth 1 -type d -name "p*" | parallel '
  cargo run --bin "$(basename {})" --release 2> /dev/null
'