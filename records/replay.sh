#!/bin/sh

ROOT=${ROOT:-$(git rev-parse --show-toplevel)}
cd "${ROOT}/sdl"

cargo run --release --bin sdl ../roms/$1 --input "$(cat ../records/$1.txt)"
