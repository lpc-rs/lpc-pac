#!/usr/bin/env bash

# Bash script to (re)generate Platform Access Crates (PACs) with svd2rust.
# For PAC to be generated, the following items are required:
#
# - It needs to be registered below in the `PACS` array.
# - The directory needs to contain an SVD file with the name matching
#   the crate name.

set -euo pipefail


### Configuration

PACS=(
    "lpc54605"
    "lpc54606"
    "lpc54607"
    "lpc54608"
    "lpc54616"
    "lpc54618"
    "lpc54628"
)

# everything is relative to the generate script
cd "$(dirname "$0")"


### Helper functions

RED="\e[31m"
CYAN="\e[36m"
BOLDGREEN="\e[1;32m"
RESET="\e[0m"

cecho() {
    # Colorized echo
    echo -e "${1}${2}${RESET}"
}

fail() {
    cecho "$RED" "Error: $2"
    exit "$1"
}

command_exists() {
    command -v "$1" >/dev/null 2>&1
}

require_command() {
    # Params:
    # - 1: Command name
    # - 2: Version string (optional)
    if ! command_exists "$1"; then
        fail 1 "Command $1 not found."
    fi
    set +u  # Optional parameter
    if [ -n "$2" ]; then
        out="$("$1" --version)"; read -ra parts <<< "$out"; version=${parts[1]}
        if [ "$version" != "$2" ]; then
            fail 1 "Command $1 version $2 was expected, but $version is installed"
        fi
    fi
    set -u
}

### Main

require_command svd2rust 0.21.0
require_command cargo-fmt
require_command form  0.8.0

generate_pac() {
    crate=$1
    shopt -s nullglob
    cecho "$CYAN" "patching svd..."
    cp "${crate}.svd" "${crate}.svd.patched"
    for patch in ./patches/common/*.patch; do
        [ -f "$patch" ] || continue # ensure that there actually is a file
        cecho "$CYAN" "Applying patch ${patch} to ${crate}"
        patch -u "${crate}.svd.patched" -i $patch
    done

    for patch in ./patches/${crate}/*.patch; do
        [ -f "$patch" ] || continue # ensure that there actually is a file
        cecho "$CYAN" "Applying patch ${patch} to ${crate}"
        patch -u "${crate}.svd.patched" -i $patch
    done
    # remove .orig, don't fail if it does not exist because it was not patched
    rm -f "${crate}.svd.patched.orig"

    rm -rf $crate
    mkdir -p $crate
    shift
    if [[ -f "${crate}.svd.patched" ]]; then
	svd_file="${crate}.svd.patched"
    elif [[ -f "${crate}.svd" ]]; then
	svd_file="${crate}.svd"
    fi
    cecho "$CYAN" "Running svd2rust using svd file: ${svd_file}..."
    svd2rust "$@" -m -g --strict -i "${svd_file}" -o ${crate} 2> >(tee svd2rust-warnings.log >&2)
    pushd $crate/ >/dev/null
    rustfmt ./*.rs
    rm build.rs
    mv -f generic.rs ../
    cecho "$CYAN" "Formatting generated code..."
    RUST_LOG=form=warn form -i mod.rs -o form-output
    rm mod.rs
    mv form-output/* .
    mv lib.rs mod.rs
    rmdir form-output
    rustfmt mod.rs
    popd >/dev/null
}

check_devices() {

    for PAC in ${PACS[*]}; do
        cecho "$CYAN" "checking device $PAC"
        cargo check --features rt,$PAC
    done
}
if (( $# == 1 )); then
    cd "src/"
    generate_pac "$1"
    exit 0
fi

for PAC in ${PACS[*]}; do
    IFS=: read -ra parts <<< "$PAC"
    crate=${parts[0]}
    cecho "$BOLDGREEN" "\nEntering src/"
    pushd src/ >/dev/null
    generate_pac "${parts[@]}"
    cecho "$BOLDGREEN" "Done"
    popd >/dev/null
done
check_devices
echo ""