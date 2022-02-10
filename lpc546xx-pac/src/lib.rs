//! Peripheral access API for LPC546XX microcontrollers
//! (generated using [svd2rust](https://github.com/rust-embedded/svd2rust)
//! 0.21.0)
//!
//! You can find an overview of the API here:
//! [svd2rust/#peripheral-api](https://docs.rs/svd2rust/0.21.0/svd2rust/#peripheral-api)
//!
//! For more details see the README here:
//! [stm32-rs](https://github.com/lpc-rs/lpc-pac)
//!
//! This crate supports all LPC546XX devices; for the complete list please
//! see:
//! [lpc546xx-pac](https://crates.io/crates/lpc546xx-pac)
//!
//! Due to doc build limitations, not all devices may be shown on docs.rs;
//! a representative few have been selected instead.
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![no_std]
mod generic;
pub use self::generic::*;
#[cfg(feature = "lpc54605")]
pub mod lpc54605;

#[cfg(feature = "lpc54606")]
pub mod lpc54606;

#[cfg(feature = "lpc54607")]
pub mod lpc54607;

#[cfg(feature = "lpc54608")]
pub mod lpc54608;

#[cfg(feature = "lpc54616")]
pub mod lpc54616;

#[cfg(feature = "lpc54618")]
pub mod lpc54618;

#[cfg(feature = "lpc54628")]
pub mod lpc54628;

