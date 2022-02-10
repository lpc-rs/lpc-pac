"""
makecrates.py
Copyright 2017-2019 Adam Greig
Licensed under the MIT and Apache 2.0 licenses.
Autogenerate the crate Cargo.toml, build.rs, README.md and src/lib.rs files
based on available YAML files for each STM32 family.
Usage: python3 scripts/makecrates.py devices/
"""

import os
import glob
import os.path
import argparse
import re
import yaml

VERSION = "0.14.0"
SVD2RUST_VERSION = "0.21.0"

CRATE_RENAMING = {
    "lpc546": {"cratename": "lpc546xx-pac", "familyname": "lpc546xx"},
}


def main(yes, families):
    for crate in CRATE_RENAMING.keys():
        with open(os.path.join(crate, "Cargo.toml"), "r") as f:
            data = f.read()
            data = data.replace(
                f'name = "{crate}"', f'name = "{CRATE_RENAMING[crate]["cratename"]}"')
            data = data.replace(
                f'description = "Device support crates for {crate.upper()} devices"', f'description = "Device support crates for {CRATE_RENAMING[crate]["familyname"].upper()} devices"')
        with open(os.path.join(crate, "Cargo.toml"), "w") as f:
            f.write(data)

        with open(os.path.join(crate, "src", "lib.rs"), "r") as f:
            data = f.read()
            data = data.replace(
                f'{crate.upper()}', f'{CRATE_RENAMING[crate]["familyname"].upper()}')
            data = data.replace(
                f'//! [{crate}](https://crates.io/crates/{crate})', f'//! [{CRATE_RENAMING[crate]["cratename"]}](https://crates.io/crates/{CRATE_RENAMING[crate]["cratename"]})')
        with open(os.path.join(crate, "src", "lib.rs"), "w") as f:
            f.write(data)

        with open(os.path.join(crate, "README.md"), "r") as f:
            data = f.read()
            data = data.replace(
                f'{crate.upper()}', f'{CRATE_RENAMING[crate]["familyname"].upper()}')
            data = data.replace(
                f'[documentation]: https://docs.rs/{crate}/latest/{crate}/', f'[documentation]: https://docs.rs/{CRATE_RENAMING[crate]["cratename"]}/latest/{CRATE_RENAMING[crate]["cratename"]}/')
            data = data.replace(
                f'[dependencies.{crate}]', f'[dependencies.{CRATE_RENAMING[crate]["cratename"]}]')
            data = data.replace(
                f'use {crate}::', f'use {CRATE_RENAMING[crate]["cratename"]}::')
        with open(os.path.join(crate, "README.md"), "w") as f:
            f.write(data)

        os.rename(crate, CRATE_RENAMING[crate]["cratename"])


if __name__ == "__main__":
    parser = argparse.ArgumentParser()
    parser.add_argument("-y",
                        help="Assume 'yes' to prompt",
                        action="store_true")
    parser.add_argument('--families',
                        help="Families of components to generate crates for",
                        nargs='+',
                        required=False,
                        metavar='FAMILY',
                        default=[],
                        type=str)
    args = parser.parse_args()
    main(args.y, args.families)
