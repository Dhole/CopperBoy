#!/bin/sh
set -ex

ROOT=$(git rev-parse --show-toplevel)

if [ $# -ne 1 ]; then
    echo "Usage: $0 TARGET"
    exit 1
fi

target="$1"

OPTS=${OPTS:-"--release"}
case $target in
    linux-gnu-x86_64)
        cd ${ROOT}/libretro_std
        cargo build ${OPTS} --target x86_64-unknown-linux-gnu
        ;;
    psvita)
        export VITASDK=/usr/local/vitasdk
        export PATH=$VITASDK/bin:$PATH
        cd ${ROOT}/libretro_psvita
        cargo xbuild ${OPTS} --target armv7-vita-eabihf.json
        # RUST_TARGET_PATH=$PWD xargo build --target armv7-vita-eabihf --release
        ;;
    *)
        echo "Unknown target ${target}"
        exit 1
        ;;
esac
