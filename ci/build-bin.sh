#!/bin/sh
set -ex

ROOT=${ROOT:-$(git rev-parse --show-toplevel)}
export PATH=$HOME/.cargo/bin:$PATH

if [ $# -ne 2 ]; then
    echo "Usage: $0 BIN TARGET"
    exit 1
fi

bin="$1"
target="$2"

OPTS="--release"
case $bin in
    sdl)
        cd ${ROOT}/sdl
        case $target in
            linux-gnu-x86_64)
                cargo build ${OPTS} --target x86_64-unknown-linux-gnu --bin sdl
                ;;
            *)
                echo "Unknown target ${target}"
                exit 1
                ;;
        esac
        ;;
    *)
        echo "Unknown bin ${bin}"
        ;;
esac
