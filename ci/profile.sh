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
cd "${ROOT}/arduboy"
export RUSTFLAGS="-g --emit asm"
cargo bench --no-run --target x86_64-unknown-linux-gnu --bench benchmark --no-default-features --features "$features"

cd "${ROOT}"
TARGET="x86_64-unknown-linux-gnu"
BENCH="target/${TARGET}/release/deps/$(ls -tr "target/${TARGET}/release/deps/" | tail -n 1)"

TEST_ID="emulator.run.castle_boy.780/60"
# TEST_ID="emulator.run.mystic_balloon.1020/60"

# valgrind --tool=callgrind \
#          --dump-instr=yes \
#          --collect-jumps=yes \
#          --simulate-cache=yes \
#          $BENCH --bench --profile-time 10 "$TEST_ID"

# flamegraph --freq 1000 $BENCH --bench --profile-time 10 "$TEST_ID"
