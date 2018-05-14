#!/usr/bin/env bash

cargo install --force --version 0.13.0 svd2rust &&
cargo install --force --version 0.7.0 rustfmt-nightly &&

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

mkdir -p src &&
svd2rust -i LPC82x.svd &&
cat lib.rs | rustfmt > src/lib.rs && rm lib.rs &&
rustfmt build.rs
