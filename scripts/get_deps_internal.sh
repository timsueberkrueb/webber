#!/usr/bin/env bash

if [[ ! -d "lib/$ARCH_TRIPLET/bin" ]]; then
    apt-get update
    cd build/
    apt-get download tar:$ARCH
    dpkg -x tar_*_$ARCH.deb tar_pkg
    mkdir -p ../lib/$ARCH_TRIPLET/bin
    cp tar_pkg/bin/tar ../lib/$ARCH_TRIPLET/bin
    apt-get download gzip:$ARCH
    dpkg -x gzip_*_$ARCH.deb gzip_pkg
    cp gzip_pkg/bin/gzip ../lib/$ARCH_TRIPLET/bin
fi
