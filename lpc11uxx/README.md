# lpc11uxx-rs

Rust register mappings for the NXP LPC11Uxx family of ARM Cortex-M0+
microcontrollers. The code is generated automatically from the [SVD
file](http://ds.arm.com/media/resources/db/chip/nxp/lpc11u24fbd48_301/LPC11Uxx.svd)
using [svd2rust](https://crates.io/crates/svd2rust) 0.14.

LPC11U3x/2x/1x user manual: [https://gab.wallawalla.edu/~curt.nelson/engr355/datasheets/lpc11u24/user%20manual.pdf](https://gab.wallawalla.edu/~curt.nelson/engr355/datasheets/lpc11u24/user%20manual.pdf)

This crate is part of the [lpc-pac](https://github.com/lpc-rs/lpc-pac/)
project.

This crate requires nightly Rust to compile since it uses `untagged_unions` to
access registers which map to the same address.
