#!/bin/sh
set -ex

ROOT=${ROOT:-$(git rev-parse --show-toplevel)}
export PATH=$HOME/.cargo/bin:$PATH

if [ $# -lt 1 ]; then
    echo "Usage: $0 TESTNAME [FEATURES]"
    exit 1
fi

testname="$1"
features="$2"

case $testname in
    build)
        cd ${ROOT}/arduboy
        cargo build --target x86_64-unknown-linux-gnu --no-default-features --features "${features}"
        ;;
    unit)
        cd ${ROOT}/arduboy
        cargo test --target x86_64-unknown-linux-gnu --no-default-features --features "${features}" $3
        ;;
    vector)
        cd ${ROOT}/test-framework
        tar -xf vectors.tar.xz

        cd ${ROOT}/arduboy
        cargo test --target x86_64-unknown-linux-gnu --features test_vectors test_vectors $3

        cd ${ROOT}/test-framework
        rm -r vectors
        ;;
    *)
        echo "Unknown testname ${target}"
        exit 1
        ;;
esac
