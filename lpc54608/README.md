# lpc54608-pac

This crate is a Peripheral Access Crate (or PAC) for the LPC54608 from NXP, generated using svd2rust and the svd file from the LPC54608 SDK 2.8.2 (the file has an xml extension in the SDK)

No svd modification were done.

A peripheral Access Crate contains low-level definitions.

This crate also includes a `startup.s` script to enable SRAM1, SRAM2, SRAM3 in a similar way done in their SDK example, without using the stack. Here it is executed using the  `__pre_init` call from the cortex-m-rt crate.

## Regenerating

To regernerate the crate, first make sure that svd2rust and form is installed and run:

```sh
# Generate
svd2rust -i LPC54608.svd
form -i lib.rs -o src
cargo fmt

# Clean up
rm lib.rs

# build startup.s
./assemble.sh
```