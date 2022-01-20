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
    "lpc11uxx:--nightly"
    "lpc54606"
    "lpc54608"
    "lpc82x"
    "lpc845"
    
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
    cecho "$CYAN" "Running svd2rust..."
    rm -rf src
    crate=$1
    shift
    if [[ -f "${crate}.svd.patched" ]]; then
	svd_file="${crate}.svd.patched"
    elif [[ -f "${crate}.svd" ]]; then
	svd_file="${crate}.svd"
    fi
    cecho "$CYAN" "Using svd file: ${svd_file}..."
    svd2rust "$@" -i "${svd_file}" 2> >(tee svd2rust-warnings.log >&2)
    RUST_LOG=form=warn form -i lib.rs -o src
    rm lib.rs
    cecho "$CYAN" "Formatting generated code..."
    cargo fmt
}

if (( $# == 1 )); then
    cd "$1"
    generate_pac "$1"
    exit 0
fi

for PAC in ${PACS[*]}; do
    IFS=: read -ra parts <<< "$PAC"
    crate=${parts[0]}
    cecho "$BOLDGREEN" "\nEntering $crate/"
    pushd "$crate" >/dev/null
    generate_pac "${parts[@]}"
    cecho "$BOLDGREEN" "Done"
    popd >/dev/null
done
echo ""
