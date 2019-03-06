#!/usr/bin/env bash

cp LPC845.xml LPC845.svd

for patch in patches/*
do
	patch \
		--silent \
		--force \
		--reject-file=- \
		LPC845.svd $patch
done

rm -rf src && mkdir src &&
svd2rust -i LPC845.svd &&
form -i lib.rs -o src && rm lib.rs &&
cargo fmt &&
rustfmt build.rs
