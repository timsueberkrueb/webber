#!/usr/bin/env bash

set -e

docker run -v $(pwd):/build \
    -w /build \
    -e ARCH \
    -e ARCH_TRIPLET \
    clickable/ubuntu-sdk:16.04-$ARCH \
    ./scripts/get_deps_internal.sh
