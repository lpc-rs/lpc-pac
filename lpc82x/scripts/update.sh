#!/usr/bin/env bash

cargo install --force --version 0.14.0 svd2rust &&
cargo install --force --version 0.6.0 form &&
cargo install --force --version 1.0.3 rustfmt-nightly &&

wget \
	-O LPC82x.svd \
	http://ds.arm.com/media/resources/db/chip/nxp/lpc824m201jdh20/LPC82x.svd &&

for patch in patches/*
do
	patch \
		--silent \
		--force \
		--reject-file=- \
		LPC82x.svd $patch
done

rm -rf src && mkdir src &&
svd2rust -i LPC82x.svd &&
form -i lib.rs -o src && rm lib.rs &&
cargo fmt &&
rustfmt build.rs
