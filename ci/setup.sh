#!/bin/sh
set -ex

setup_xbuild() {
    rustup component add rust-src
    cargo install cargo-xbuild
}

setup_vitasdk() {
    git clone https://github.com/vitasdk/vdpm
    cd vdpm
    ./bootstrap-vitasdk.sh
    export VITASDK=/usr/local/vitasdk
    export PATH=$VITASDK/bin:$PATH
    # ./vdpm vitaGL
    # ./install-all.sh
}

while [ $# -gt 0 ]; do
    arg="$1"
    case $arg in
        psvita)
            setup_xbuild
            setup_vitasdk
            ;;
        sdl)
            sudo apt-get install -y -q libsdl2-dev libsdl2-ttf-dev
            ;;
        rust)
            curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- --default-toolchain none -y
            export PATH=$HOME/.cargo/bin:$PATH
            rustup toolchain install nightly --allow-downgrade --profile minimal
            ;;
        *)
            echo "Unknown arg ${arg}"
            exit 1
            ;;
    esac
    shift
done
