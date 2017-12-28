# lpc11uxx-rs

Rust register mappings for the NXP LPC11Uxx family of ARM Cortex-M0+
microcontrollers. The code is generated automatically from the [SVD
file](http://ds.arm.com/media/resources/db/chip/nxp/lpc11u24fbd48_301/LPC11Uxx.svd)
using [svd2rust](https://crates.io/crates/svd2rust).

## Creating The Bindings

The SVD file contained some duplicate definitions and some other errors which
needed to be fixed.

```Bash
svd2rust -i LPC11Uxx.svd|rustfmt > src/lib.rs
```
