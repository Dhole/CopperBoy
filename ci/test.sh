#!/bin/sh
set -ex

ROOT=${ROOT:-$(git rev-parse --show-toplevel)}
export PATH=$HOME/.cargo/bin:$PATH

if [ $# -lt 1 ]; then
    echo "Usage: $0 CMD [FEATURES]"
    exit 1
fi

cmd="$1"
features="$2"
arg="$3"

case $cmd in
    build)
        cd ${ROOT}/arduboy
        cargo build --target x86_64-unknown-linux-gnu --no-default-features --features "${features}"
        ;;
    unit)
        cd ${ROOT}/arduboy
        cargo test --target x86_64-unknown-linux-gnu --no-default-features --features "${features}" $arg
        ;;
    vector)
        cd ${ROOT}/test-framework
        tar -xf vectors.tar.xz

        cd ${ROOT}/arduboy
        cargo test --target x86_64-unknown-linux-gnu --features test_vectors test_vectors $arg

        cd ${ROOT}/test-framework
        rm -r vectors
        ;;
    *)
        echo "Unknown cmd ${cmd}"
        exit 1
        ;;
esac
