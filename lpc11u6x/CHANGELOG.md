# Change Log

All notable changes to this project will be documented in this file.
This project adheres to [Semantic Versioning](http://semver.org/).

## Unreleased

 * Generate using the `--nightly` flag to allow access to overlapping registers
   (#13)
 * Rename `INACTIVE` field in pull-up/down mode enums (and associated methods)
   to `FLOATING` (#9)

## 0.3.0

 * Use svd2rust 0.14.0 to generate code
 * Clean up SVD, rename a lot of registers (#6)

## 0.2.0

 * Fix reset value of the SYSAHBCLKCTRL register 
 * Use svd2rust 0.13.1 to generate code

## 0.1.0

 * Initial release
