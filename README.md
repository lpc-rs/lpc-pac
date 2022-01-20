# lpc-pac-rs

[![crates.io](https://img.shields.io/crates/v/lpc11uxx.svg?label=lpc11uxx)](https://crates.io/crates/lpc11uxx)
[![crates.io](https://img.shields.io/crates/v/lpc546xx-pac.svg?label=lpc546xx-pac)](https://crates.io/crates/lpc546xx-pac)
[![crates.io](https://img.shields.io/crates/v/lpc82x-pac.svg?label=lpc82x-pac)](https://crates.io/crates/lpc82x-pac)
[![crates.io](https://img.shields.io/crates/v/lpc845-pac.svg?label=lpc845-pac)](https://crates.io/crates/lpc845-pac)
 
This repository provides Rust device support crates for some LPC microcontrollers, providing an API to that device's peripherals using [svd2rust] and the SVD files from NXP. These crates are commonly known as peripheral access crates or "PACs".

[svd2rust]: https://github.com/rust-embedded/svd2rust

Not every register of each device will have been tested on hardware, and so errors or omissions may remain. Please report any bugs you find, or open a PR!

## Using Device Crates In Your Own Project

In your own project's `Cargo.toml`:

```toml
[dependencies.lpc546xx-pac]
version = "x.y.z"
features = ["lpc54608", "rt"] 
```

The `rt` feature is optional but helpful. see [svd2rust](https://docs.rs/svd2rust/latest/svd2rust/#the-rt-feature) for details.

Then, in your code:

```rust
use lpc54606_pac as pac;

let mut peripherals = pac::Peripherals::take().unwrap();
```

Refer to `svd2rust` [documentation](https://docs.rs/svd2rust) for further usage.

Replace `lpc54606_pac` with your own device; see the individual crate READMEs for the complete list of supported devices.

## Generating Device Crates / Building Locally

* Install `svd2rust`: `cargo install --version 0.21.0 svd2rust`
* Install `form`: `cargo install --version 0.8.0`
* Install `rustfmt`: `rustup component add rustfmt`
* Generate the PACs: `./generate.sh`

## Generating super pacs

## Adding New Devices

* Create a new directory.
* Copy the extracted SVD in this directory.
* Patch it if need be.
* Create the `Cargo.toml` of your PAC.
* Create the `README.md` of your PAC.
* Add your pac in the list in `generate.sh`.
* Generate the PACs: `./generate.sh`
* Test that yout new pac compiles using `cargo build`

