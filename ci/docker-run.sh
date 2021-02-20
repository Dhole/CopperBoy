#!/bin/sh
set -ex

ROOT=${ROOT:-$(git rev-parse --show-toplevel)}

docker run --mount type=bind,source=${ROOT},destination=/workspace copperboy-builder sh -c \
    "cd /workspace/ci/ && \
     ./setup.sh rust && \
     ./test.sh unit \"std\""
# docker run -it --mount type=bind,source=${ROOT},destination=/workspace copperboy-builder /bin/bash
