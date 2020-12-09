#!/usr/bin/env bash

set -euxo pipefail

# cflags taken from cc 1.0.22

crate=lpc54608-pac

# remove existing blobs because otherwise this will append object files to the old blobs
rm -f bin/*.a

arm-none-eabi-as -march=armv7e-m startup.s -o bin/$crate.o
ar crs bin/thumbv7em-none-eabi.a bin/$crate.o
ar crs bin/thumbv7em-none-eabihf.a bin/$crate.o

rm bin/$crate.o