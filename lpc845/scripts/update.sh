#!/usr/bin/env bash

cargo install --version 0.14.0 svd2rust &&
cargo install --version 0.6.0 form

rm -rf src && mkdir src &&
svd2rust -i LPC845.svd &&
form -i lib.rs -o src && rm lib.rs &&
cargo fmt &&
rustfmt build.rs
