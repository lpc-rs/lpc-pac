#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - I/O configuration for port PIO0"]
    pub pio0__: [PIO0__; 4],
    #[doc = "0x10 - I/O configuration for open-drain pin PIO0_4"]
    pub pio0_4: PIO0_4,
    #[doc = "0x14 - I/O configuration for open-drain pin PIO0_5"]
    pub pio0_5: PIO0_5,
    #[doc = "0x18 - I/O configuration for port PIO0"]
    pub pio0_6: PIO0_,
    #[doc = "0x1c - I/O configuration for port PIO0"]
    pub pio0_7: PIO0_,
    #[doc = "0x20 - I/O configuration for port PIO0"]
    pub pio0_8: PIO0_,
    #[doc = "0x24 - I/O configuration for port PIO0"]
    pub pio0_9: PIO0_,
    #[doc = "0x28 - I/O configuration for port PIO0"]
    pub pio0_10: PIO0_,
    #[doc = "0x2c - I/O configuration for port PIO0"]
    pub pio0_11: PIO0_,
    #[doc = "0x30 - I/O configuration for port PIO0"]
    pub pio0_12: PIO0_,
    #[doc = "0x34 - I/O configuration for port PIO0"]
    pub pio0_13: PIO0_,
    #[doc = "0x38 - I/O configuration for port PIO0"]
    pub pio0_14: PIO0_,
    #[doc = "0x3c - I/O configuration for port PIO0"]
    pub pio0_15: PIO0_,
    #[doc = "0x40 - I/O configuration for port PIO0"]
    pub pio0_16: PIO0_,
    #[doc = "0x44 - I/O configuration for port PIO0"]
    pub pio0_17: PIO0_,
    #[doc = "0x48 - I/O configuration for port PIO0"]
    pub pio0_18: PIO0_,
    #[doc = "0x4c - I/O configuration for port PIO0"]
    pub pio0_19: PIO0_,
    #[doc = "0x50 - I/O configuration for port PIO0"]
    pub pio0_20: PIO0_,
    #[doc = "0x54 - I/O configuration for port PIO0"]
    pub pio0_21: PIO0_,
    #[doc = "0x58 - I/O configuration for port PIO0"]
    pub pio0_22: PIO0_,
    #[doc = "0x5c - I/O configuration for port PIO0"]
    pub pio0_23: PIO0_,
    #[doc = "0x60 - I/O configuration for port PIO1"]
    pub pio1_: [PIO1_; 32],
    _reserved22: [u8; 16usize],
    #[doc = "0xf0 - I/O configuration for port PIO2"]
    pub pio2__: [PIO2__; 2],
    _reserved23: [u8; 4usize],
    #[doc = "0xfc - I/O configuration for port PIO2"]
    pub pio2_2: PIO2_,
    #[doc = "0x100 - I/O configuration for port PIO2"]
    pub pio2_3: PIO2_,
    #[doc = "0x104 - I/O configuration for port PIO2"]
    pub pio2_4: PIO2_,
    #[doc = "0x108 - I/O configuration for port PIO2"]
    pub pio2_5: PIO2_,
    #[doc = "0x10c - I/O configuration for port PIO2"]
    pub pio2_6: PIO2_,
    #[doc = "0x110 - I/O configuration for port PIO2"]
    pub pio2_7: PIO2_,
    #[doc = "0x114 - I/O configuration for port PIO2"]
    pub pio2_8: PIO2_,
    #[doc = "0x118 - I/O configuration for port PIO2"]
    pub pio2_9: PIO2_,
    #[doc = "0x11c - I/O configuration for port PIO2"]
    pub pio2_10: PIO2_,
    #[doc = "0x120 - I/O configuration for port PIO2"]
    pub pio2_11: PIO2_,
    #[doc = "0x124 - I/O configuration for port PIO2"]
    pub pio2_12: PIO2_,
    #[doc = "0x128 - I/O configuration for port PIO2"]
    pub pio2_13: PIO2_,
    #[doc = "0x12c - I/O configuration for port PIO2"]
    pub pio2_14: PIO2_,
    #[doc = "0x130 - I/O configuration for port PIO2"]
    pub pio2_15: PIO2_,
    #[doc = "0x134 - I/O configuration for port PIO2"]
    pub pio2_16: PIO2_,
    #[doc = "0x138 - I/O configuration for port PIO2"]
    pub pio2_17: PIO2_,
    #[doc = "0x13c - I/O configuration for port PIO2"]
    pub pio2_18: PIO2_,
    #[doc = "0x140 - I/O configuration for port PIO2"]
    pub pio2_19: PIO2_,
    #[doc = "0x144 - I/O configuration for port PIO2"]
    pub pio2_20: PIO2_,
    #[doc = "0x148 - I/O configuration for port PIO2"]
    pub pio2_21: PIO2_,
    #[doc = "0x14c - I/O configuration for port PIO2"]
    pub pio2_22: PIO2_,
    #[doc = "0x150 - I/O configuration for port PIO2"]
    pub pio2_23: PIO2_,
}
#[doc = "I/O configuration for port PIO0"]
pub struct PIO0__ {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I/O configuration for port PIO0"]
pub mod pio0__;
#[doc = "I/O configuration for open-drain pin PIO0_4"]
pub struct PIO0_4 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I/O configuration for open-drain pin PIO0_4"]
pub mod pio0_4;
#[doc = "I/O configuration for open-drain pin PIO0_5"]
pub struct PIO0_5 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I/O configuration for open-drain pin PIO0_5"]
pub mod pio0_5;
#[doc = "I/O configuration for port PIO0"]
pub struct PIO0_ {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I/O configuration for port PIO0"]
pub mod pio0_;
#[doc = "I/O configuration for port PIO1"]
pub struct PIO1_ {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I/O configuration for port PIO1"]
pub mod pio1_;
#[doc = "I/O configuration for port PIO2"]
pub struct PIO2__ {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I/O configuration for port PIO2"]
pub mod pio2__;
#[doc = "I/O configuration for port PIO2"]
pub struct PIO2_ {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I/O configuration for port PIO2"]
pub mod pio2_;
