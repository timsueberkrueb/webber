#!/bin/bash

if [[ ! -d "lib/arm-linux-gnueabihf/bin" ]]; then
    apt-get update
    cd build/
    apt-get download tar:armhf
    dpkg -x tar_*_armhf.deb tar_pkg
    mkdir -p ../lib/arm-linux-gnueabihf/bin
    cp tar_pkg/bin/tar ../lib/arm-linux-gnueabihf/bin
    apt-get download gzip:armhf
    dpkg -x gzip_*_armhf.deb gzip_pkg
    cp gzip_pkg/bin/gzip ../lib/arm-linux-gnueabihf/bin
fi
