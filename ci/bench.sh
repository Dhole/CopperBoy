#!/bin/sh
set -ex

# Example: $0 "std"

ROOT=${ROOT:-$(git rev-parse --show-toplevel)}
export PATH=$HOME/.cargo/bin:$PATH

if [ $# -lt 1 ]; then
    echo "Usage: $0 FEATURES"
    exit 1
fi

features="$1"
cd ${ROOT}/arduboy
cargo bench --target x86_64-unknown-linux-gnu --bench benchmark --no-default-features --features "$features"
