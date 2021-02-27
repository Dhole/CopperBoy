#!/bin/sh
set -ex

docker image build --build-arg USER_UID=${USER_UID:-1004} -t copperboy-builder .
