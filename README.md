# lpc-pac-rs

[![crates.io](https://img.shields.io/crates/v/lpc546xx-pac.svg?label=lpc81x-pac)](https://crates.io/crates/lpc81x-pac)
[![crates.io](https://img.shields.io/crates/v/lpc82x-pac.svg?label=lpc82x-pac)](https://crates.io/crates/lpc82x-pac)
[![crates.io](https://img.shields.io/crates/v/lpc845-pac.svg?label=lpc845-pac)](https://crates.io/crates/lpc845-pac)
[![crates.io](https://img.shields.io/crates/v/lpc11uxx.svg?label=lpc11uxx)](https://crates.io/crates/lpc11uxx)
[![crates.io](https://img.shields.io/crates/v/lpc546xx-pac.svg?label=lpc546xx-pac)](https://crates.io/crates/lpc546xx-pac)

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

## Generating PACs:


* Install svd2rust: `cargo install --version 0.25.0 svd2rust`
* Install form: `cargo install form --version 0.10.0`
* Install rustfmt: `rustup component add rustfmt`
* Install svdtools: `cargo install svdtools --version 0.2.6`
* Unzip bundled SVD zip files: `cd svd; ./extract.sh; cd ..`
* Generate patched SVD files: `make patch -j8`
* Generate svd2rust device crates: `make svd2rust -j8`
* Optional: Format device crates: `make form -j8`



## Generating lpc11uxx, lpc845 and lpc82x:

* Install `svd2rust`: `cargo install --version 0.24.0 svd2rust`
* Install `form`: `cargo install --version 0.8.0`
* Install `rustfmt`: `rustup component add rustfmt`
* Generate the PACs: `./generate.sh`



## Adding New Devices

* Update SVD zips in `svd/vendor` to include new SVD.
* Run `svd/extract.sh` to extract the zips into `svd` (ignored in git).
* Add new YAML file in `devices/` with the new SVD path and include any
  required SVD patches for this device, such as renaming or merging fields.
* Add device in `lpc_part_table`
* Add family/device (if necessary) in `Makefile` > `PACS`
* Add family/device (if necessary) in `scripts/makecrates.py` > 
* Re-run `scripts/makecrates.py devices/` to update the crates with the new devices.
* Run `make` to rebuild, which will make a patched SVD and then run `svd2rust`
  on it to generate the final library.

## Updating Existing Devices/Peripherals

* You'll need to run `svd/extract.sh` at least once to pull the SVDs out.
* Edit the device or peripheral YAML (see below for format).
* Run `make` to rebuild all the crates using `svd patch` and `svd2rust`.
* Test your new stuff compiles: `cd lpc546xx-pac; cargo build --features lpc54628`
