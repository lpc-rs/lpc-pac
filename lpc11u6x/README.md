# lpc11u6x-rs

Rust register mappings for the NXP LPC11U6x family of ARM Cortex-M0+
microcontrollers. The code is generated automatically from the [SVD
file](https://community.nxp.com/servlet/JiveServlet/downloadBody/334624-102-1-275615/LPC11U6x+SVD+file.zip)
using [svd2rust](https://crates.io/crates/svd2rust) 0.15.

LPC11U6x/LPC11E6x user manual: [http://www.mouser.com/ds/2/302/UM10732-315822.pdf](http://www.mouser.com/ds/2/302/UM10732-315822.pdf)

This crate is part of the [lpc-pac](https://github.com/lpc-rs/lpc-pac/)
project.

This crate requires nightly Rust to compile since it uses `untagged_unions` to
access registers which map to the same address.
