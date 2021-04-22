#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Byte pin registers for all port 0 and 1 GPIO pins"]
    pub b0_0: crate::Reg<b0_0::B0_0_SPEC>,
    #[doc = "0x01 - Byte pin registers for all port 0 and 1 GPIO pins"]
    pub b0_1: crate::Reg<b0_1::B0_1_SPEC>,
    #[doc = "0x02 - Byte pin registers for all port 0 and 1 GPIO pins"]
    pub b0_2: crate::Reg<b0_2::B0_2_SPEC>,
    #[doc = "0x03 - Byte pin registers for all port 0 and 1 GPIO pins"]
    pub b0_3: crate::Reg<b0_3::B0_3_SPEC>,
    #[doc = "0x04 - Byte pin registers for all port 0 and 1 GPIO pins"]
    pub b0_4: crate::Reg<b0_4::B0_4_SPEC>,
    #[doc = "0x05 - Byte pin registers for all port 0 and 1 GPIO pins"]
    pub b0_5: crate::Reg<b0_5::B0_5_SPEC>,
    #[doc = "0x06 - Byte pin registers for all port 0 and 1 GPIO pins"]
    pub b0_6: crate::Reg<b0_6::B0_6_SPEC>,
    #[doc = "0x07 - Byte pin registers for all port 0 and 1 GPIO pins"]
    pub b0_7: crate::Reg<b0_7::B0_7_SPEC>,
    #[doc = "0x08 - Byte pin registers for all port 0 and 1 GPIO pins"]
    pub b0_8: crate::Reg<b0_8::B0_8_SPEC>,
    #[doc = "0x09 - Byte pin registers for all port 0 and 1 GPIO pins"]
    pub b0_9: crate::Reg<b0_9::B0_9_SPEC>,
    #[doc = "0x0a - Byte pin registers for all port 0 and 1 GPIO pins"]
    pub b0_10: crate::Reg<b0_10::B0_10_SPEC>,
    #[doc = "0x0b - Byte pin registers for all port 0 and 1 GPIO pins"]
    pub b0_11: crate::Reg<b0_11::B0_11_SPEC>,
    #[doc = "0x0c - Byte pin registers for all port 0 and 1 GPIO pins"]
    pub b0_12: crate::Reg<b0_12::B0_12_SPEC>,
    #[doc = "0x0d - Byte pin registers for all port 0 and 1 GPIO pins"]
    pub b0_13: crate::Reg<b0_13::B0_13_SPEC>,
    #[doc = "0x0e - Byte pin registers for all port 0 and 1 GPIO pins"]
    pub b0_14: crate::Reg<b0_14::B0_14_SPEC>,
    #[doc = "0x0f - Byte pin registers for all port 0 and 1 GPIO pins"]
    pub b0_15: crate::Reg<b0_15::B0_15_SPEC>,
    #[doc = "0x10 - Byte pin registers for all port 0 and 1 GPIO pins"]
    pub b0_16: crate::Reg<b0_16::B0_16_SPEC>,
    #[doc = "0x11 - Byte pin registers for all port 0 and 1 GPIO pins"]
    pub b0_17: crate::Reg<b0_17::B0_17_SPEC>,
    #[doc = "0x12 - Byte pin registers for all port 0 and 1 GPIO pins"]
    pub b0_18: crate::Reg<b0_18::B0_18_SPEC>,
    #[doc = "0x13 - Byte pin registers for all port 0 and 1 GPIO pins"]
    pub b0_19: crate::Reg<b0_19::B0_19_SPEC>,
    #[doc = "0x14 - Byte pin registers for all port 0 and 1 GPIO pins"]
    pub b0_20: crate::Reg<b0_20::B0_20_SPEC>,
    #[doc = "0x15 - Byte pin registers for all port 0 and 1 GPIO pins"]
    pub b0_21: crate::Reg<b0_21::B0_21_SPEC>,
    #[doc = "0x16 - Byte pin registers for all port 0 and 1 GPIO pins"]
    pub b0_22: crate::Reg<b0_22::B0_22_SPEC>,
    #[doc = "0x17 - Byte pin registers for all port 0 and 1 GPIO pins"]
    pub b0_23: crate::Reg<b0_23::B0_23_SPEC>,
    #[doc = "0x18 - Byte pin registers for all port 0 and 1 GPIO pins"]
    pub b0_24: crate::Reg<b0_24::B0_24_SPEC>,
    #[doc = "0x19 - Byte pin registers for all port 0 and 1 GPIO pins"]
    pub b0_25: crate::Reg<b0_25::B0_25_SPEC>,
    #[doc = "0x1a - Byte pin registers for all port 0 and 1 GPIO pins"]
    pub b0_26: crate::Reg<b0_26::B0_26_SPEC>,
    #[doc = "0x1b - Byte pin registers for all port 0 and 1 GPIO pins"]
    pub b0_27: crate::Reg<b0_27::B0_27_SPEC>,
    #[doc = "0x1c - Byte pin registers for all port 0 and 1 GPIO pins"]
    pub b0_28: crate::Reg<b0_28::B0_28_SPEC>,
    _reserved29: [u8; 4067usize],
    #[doc = "0x1000 - Word pin registers for all port 0 and 1 GPIO pins"]
    pub w0_0: crate::Reg<w0_0::W0_0_SPEC>,
    #[doc = "0x1004 - Word pin registers for all port 0 and 1 GPIO pins"]
    pub w0_1: crate::Reg<w0_1::W0_1_SPEC>,
    #[doc = "0x1008 - Word pin registers for all port 0 and 1 GPIO pins"]
    pub w0_2: crate::Reg<w0_2::W0_2_SPEC>,
    #[doc = "0x100c - Word pin registers for all port 0 and 1 GPIO pins"]
    pub w0_3: crate::Reg<w0_3::W0_3_SPEC>,
    #[doc = "0x1010 - Word pin registers for all port 0 and 1 GPIO pins"]
    pub w0_4: crate::Reg<w0_4::W0_4_SPEC>,
    #[doc = "0x1014 - Word pin registers for all port 0 and 1 GPIO pins"]
    pub w0_5: crate::Reg<w0_5::W0_5_SPEC>,
    #[doc = "0x1018 - Word pin registers for all port 0 and 1 GPIO pins"]
    pub w0_6: crate::Reg<w0_6::W0_6_SPEC>,
    #[doc = "0x101c - Word pin registers for all port 0 and 1 GPIO pins"]
    pub w0_7: crate::Reg<w0_7::W0_7_SPEC>,
    #[doc = "0x1020 - Word pin registers for all port 0 and 1 GPIO pins"]
    pub w0_8: crate::Reg<w0_8::W0_8_SPEC>,
    #[doc = "0x1024 - Word pin registers for all port 0 and 1 GPIO pins"]
    pub w0_9: crate::Reg<w0_9::W0_9_SPEC>,
    #[doc = "0x1028 - Word pin registers for all port 0 and 1 GPIO pins"]
    pub w0_10: crate::Reg<w0_10::W0_10_SPEC>,
    #[doc = "0x102c - Word pin registers for all port 0 and 1 GPIO pins"]
    pub w0_11: crate::Reg<w0_11::W0_11_SPEC>,
    #[doc = "0x1030 - Word pin registers for all port 0 and 1 GPIO pins"]
    pub w0_12: crate::Reg<w0_12::W0_12_SPEC>,
    #[doc = "0x1034 - Word pin registers for all port 0 and 1 GPIO pins"]
    pub w0_13: crate::Reg<w0_13::W0_13_SPEC>,
    #[doc = "0x1038 - Word pin registers for all port 0 and 1 GPIO pins"]
    pub w0_14: crate::Reg<w0_14::W0_14_SPEC>,
    #[doc = "0x103c - Word pin registers for all port 0 and 1 GPIO pins"]
    pub w0_15: crate::Reg<w0_15::W0_15_SPEC>,
    #[doc = "0x1040 - Word pin registers for all port 0 and 1 GPIO pins"]
    pub w0_16: crate::Reg<w0_16::W0_16_SPEC>,
    #[doc = "0x1044 - Word pin registers for all port 0 and 1 GPIO pins"]
    pub w0_17: crate::Reg<w0_17::W0_17_SPEC>,
    #[doc = "0x1048 - Word pin registers for all port 0 and 1 GPIO pins"]
    pub w0_18: crate::Reg<w0_18::W0_18_SPEC>,
    #[doc = "0x104c - Word pin registers for all port 0 and 1 GPIO pins"]
    pub w0_19: crate::Reg<w0_19::W0_19_SPEC>,
    #[doc = "0x1050 - Word pin registers for all port 0 and 1 GPIO pins"]
    pub w0_20: crate::Reg<w0_20::W0_20_SPEC>,
    #[doc = "0x1054 - Word pin registers for all port 0 and 1 GPIO pins"]
    pub w0_21: crate::Reg<w0_21::W0_21_SPEC>,
    #[doc = "0x1058 - Word pin registers for all port 0 and 1 GPIO pins"]
    pub w0_22: crate::Reg<w0_22::W0_22_SPEC>,
    #[doc = "0x105c - Word pin registers for all port 0 and 1 GPIO pins"]
    pub w0_23: crate::Reg<w0_23::W0_23_SPEC>,
    #[doc = "0x1060 - Word pin registers for all port 0 and 1 GPIO pins"]
    pub w0_24: crate::Reg<w0_24::W0_24_SPEC>,
    #[doc = "0x1064 - Word pin registers for all port 0 and 1 GPIO pins"]
    pub w0_25: crate::Reg<w0_25::W0_25_SPEC>,
    #[doc = "0x1068 - Word pin registers for all port 0 and 1 GPIO pins"]
    pub w0_26: crate::Reg<w0_26::W0_26_SPEC>,
    #[doc = "0x106c - Word pin registers for all port 0 and 1 GPIO pins"]
    pub w0_27: crate::Reg<w0_27::W0_27_SPEC>,
    #[doc = "0x1070 - Word pin registers for all port 0 and 1 GPIO pins"]
    pub w0_28: crate::Reg<w0_28::W0_28_SPEC>,
    _reserved58: [u8; 3980usize],
    #[doc = "0x2000 - Direction registers"]
    pub dir0: crate::Reg<dir0::DIR0_SPEC>,
    _reserved59: [u8; 124usize],
    #[doc = "0x2080 - Mask register"]
    pub mask0: crate::Reg<mask0::MASK0_SPEC>,
    _reserved60: [u8; 124usize],
    #[doc = "0x2100 - Port pin register"]
    pub pin0: crate::Reg<pin0::PIN0_SPEC>,
    _reserved61: [u8; 124usize],
    #[doc = "0x2180 - Masked port register"]
    pub mpin0: crate::Reg<mpin0::MPIN0_SPEC>,
    _reserved62: [u8; 124usize],
    #[doc = "0x2200 - Write: Set register for port Read: output bits for port"]
    pub set0: crate::Reg<set0::SET0_SPEC>,
    _reserved63: [u8; 124usize],
    #[doc = "0x2280 - Clear port"]
    pub clr0: crate::Reg<clr0::CLR0_SPEC>,
    _reserved64: [u8; 124usize],
    #[doc = "0x2300 - Toggle port"]
    pub not0: crate::Reg<not0::NOT0_SPEC>,
    _reserved65: [u8; 124usize],
    #[doc = "0x2380 - Set pin direction bits for port"]
    pub dirset0: crate::Reg<dirset0::DIRSET0_SPEC>,
    _reserved66: [u8; 124usize],
    #[doc = "0x2400 - Clear pin direction bits for port"]
    pub dirclr0: crate::Reg<dirclr0::DIRCLR0_SPEC>,
    _reserved67: [u8; 124usize],
    #[doc = "0x2480 - Toggle pin direction bits for port"]
    pub dirnot0: crate::Reg<dirnot0::DIRNOT0_SPEC>,
}
#[doc = "B0_0 register accessor: an alias for `Reg<B0_0_SPEC>`"]
pub type B0_0 = crate::Reg<b0_0::B0_0_SPEC>;
#[doc = "Byte pin registers for all port 0 and 1 GPIO pins"]
pub mod b0_0;
#[doc = "B0_1 register accessor: an alias for `Reg<B0_1_SPEC>`"]
pub type B0_1 = crate::Reg<b0_1::B0_1_SPEC>;
#[doc = "Byte pin registers for all port 0 and 1 GPIO pins"]
pub mod b0_1;
#[doc = "B0_2 register accessor: an alias for `Reg<B0_2_SPEC>`"]
pub type B0_2 = crate::Reg<b0_2::B0_2_SPEC>;
#[doc = "Byte pin registers for all port 0 and 1 GPIO pins"]
pub mod b0_2;
#[doc = "B0_3 register accessor: an alias for `Reg<B0_3_SPEC>`"]
pub type B0_3 = crate::Reg<b0_3::B0_3_SPEC>;
#[doc = "Byte pin registers for all port 0 and 1 GPIO pins"]
pub mod b0_3;
#[doc = "B0_4 register accessor: an alias for `Reg<B0_4_SPEC>`"]
pub type B0_4 = crate::Reg<b0_4::B0_4_SPEC>;
#[doc = "Byte pin registers for all port 0 and 1 GPIO pins"]
pub mod b0_4;
#[doc = "B0_5 register accessor: an alias for `Reg<B0_5_SPEC>`"]
pub type B0_5 = crate::Reg<b0_5::B0_5_SPEC>;
#[doc = "Byte pin registers for all port 0 and 1 GPIO pins"]
pub mod b0_5;
#[doc = "B0_6 register accessor: an alias for `Reg<B0_6_SPEC>`"]
pub type B0_6 = crate::Reg<b0_6::B0_6_SPEC>;
#[doc = "Byte pin registers for all port 0 and 1 GPIO pins"]
pub mod b0_6;
#[doc = "B0_7 register accessor: an alias for `Reg<B0_7_SPEC>`"]
pub type B0_7 = crate::Reg<b0_7::B0_7_SPEC>;
#[doc = "Byte pin registers for all port 0 and 1 GPIO pins"]
pub mod b0_7;
#[doc = "B0_8 register accessor: an alias for `Reg<B0_8_SPEC>`"]
pub type B0_8 = crate::Reg<b0_8::B0_8_SPEC>;
#[doc = "Byte pin registers for all port 0 and 1 GPIO pins"]
pub mod b0_8;
#[doc = "B0_9 register accessor: an alias for `Reg<B0_9_SPEC>`"]
pub type B0_9 = crate::Reg<b0_9::B0_9_SPEC>;
#[doc = "Byte pin registers for all port 0 and 1 GPIO pins"]
pub mod b0_9;
#[doc = "B0_10 register accessor: an alias for `Reg<B0_10_SPEC>`"]
pub type B0_10 = crate::Reg<b0_10::B0_10_SPEC>;
#[doc = "Byte pin registers for all port 0 and 1 GPIO pins"]
pub mod b0_10;
#[doc = "B0_11 register accessor: an alias for `Reg<B0_11_SPEC>`"]
pub type B0_11 = crate::Reg<b0_11::B0_11_SPEC>;
#[doc = "Byte pin registers for all port 0 and 1 GPIO pins"]
pub mod b0_11;
#[doc = "B0_12 register accessor: an alias for `Reg<B0_12_SPEC>`"]
pub type B0_12 = crate::Reg<b0_12::B0_12_SPEC>;
#[doc = "Byte pin registers for all port 0 and 1 GPIO pins"]
pub mod b0_12;
#[doc = "B0_13 register accessor: an alias for `Reg<B0_13_SPEC>`"]
pub type B0_13 = crate::Reg<b0_13::B0_13_SPEC>;
#[doc = "Byte pin registers for all port 0 and 1 GPIO pins"]
pub mod b0_13;
#[doc = "B0_14 register accessor: an alias for `Reg<B0_14_SPEC>`"]
pub type B0_14 = crate::Reg<b0_14::B0_14_SPEC>;
#[doc = "Byte pin registers for all port 0 and 1 GPIO pins"]
pub mod b0_14;
#[doc = "B0_15 register accessor: an alias for `Reg<B0_15_SPEC>`"]
pub type B0_15 = crate::Reg<b0_15::B0_15_SPEC>;
#[doc = "Byte pin registers for all port 0 and 1 GPIO pins"]
pub mod b0_15;
#[doc = "B0_16 register accessor: an alias for `Reg<B0_16_SPEC>`"]
pub type B0_16 = crate::Reg<b0_16::B0_16_SPEC>;
#[doc = "Byte pin registers for all port 0 and 1 GPIO pins"]
pub mod b0_16;
#[doc = "B0_17 register accessor: an alias for `Reg<B0_17_SPEC>`"]
pub type B0_17 = crate::Reg<b0_17::B0_17_SPEC>;
#[doc = "Byte pin registers for all port 0 and 1 GPIO pins"]
pub mod b0_17;
#[doc = "B0_18 register accessor: an alias for `Reg<B0_18_SPEC>`"]
pub type B0_18 = crate::Reg<b0_18::B0_18_SPEC>;
#[doc = "Byte pin registers for all port 0 and 1 GPIO pins"]
pub mod b0_18;
#[doc = "B0_19 register accessor: an alias for `Reg<B0_19_SPEC>`"]
pub type B0_19 = crate::Reg<b0_19::B0_19_SPEC>;
#[doc = "Byte pin registers for all port 0 and 1 GPIO pins"]
pub mod b0_19;
#[doc = "B0_20 register accessor: an alias for `Reg<B0_20_SPEC>`"]
pub type B0_20 = crate::Reg<b0_20::B0_20_SPEC>;
#[doc = "Byte pin registers for all port 0 and 1 GPIO pins"]
pub mod b0_20;
#[doc = "B0_21 register accessor: an alias for `Reg<B0_21_SPEC>`"]
pub type B0_21 = crate::Reg<b0_21::B0_21_SPEC>;
#[doc = "Byte pin registers for all port 0 and 1 GPIO pins"]
pub mod b0_21;
#[doc = "B0_22 register accessor: an alias for `Reg<B0_22_SPEC>`"]
pub type B0_22 = crate::Reg<b0_22::B0_22_SPEC>;
#[doc = "Byte pin registers for all port 0 and 1 GPIO pins"]
pub mod b0_22;
#[doc = "B0_23 register accessor: an alias for `Reg<B0_23_SPEC>`"]
pub type B0_23 = crate::Reg<b0_23::B0_23_SPEC>;
#[doc = "Byte pin registers for all port 0 and 1 GPIO pins"]
pub mod b0_23;
#[doc = "B0_24 register accessor: an alias for `Reg<B0_24_SPEC>`"]
pub type B0_24 = crate::Reg<b0_24::B0_24_SPEC>;
#[doc = "Byte pin registers for all port 0 and 1 GPIO pins"]
pub mod b0_24;
#[doc = "B0_25 register accessor: an alias for `Reg<B0_25_SPEC>`"]
pub type B0_25 = crate::Reg<b0_25::B0_25_SPEC>;
#[doc = "Byte pin registers for all port 0 and 1 GPIO pins"]
pub mod b0_25;
#[doc = "B0_26 register accessor: an alias for `Reg<B0_26_SPEC>`"]
pub type B0_26 = crate::Reg<b0_26::B0_26_SPEC>;
#[doc = "Byte pin registers for all port 0 and 1 GPIO pins"]
pub mod b0_26;
#[doc = "B0_27 register accessor: an alias for `Reg<B0_27_SPEC>`"]
pub type B0_27 = crate::Reg<b0_27::B0_27_SPEC>;
#[doc = "Byte pin registers for all port 0 and 1 GPIO pins"]
pub mod b0_27;
#[doc = "B0_28 register accessor: an alias for `Reg<B0_28_SPEC>`"]
pub type B0_28 = crate::Reg<b0_28::B0_28_SPEC>;
#[doc = "Byte pin registers for all port 0 and 1 GPIO pins"]
pub mod b0_28;
#[doc = "W0_0 register accessor: an alias for `Reg<W0_0_SPEC>`"]
pub type W0_0 = crate::Reg<w0_0::W0_0_SPEC>;
#[doc = "Word pin registers for all port 0 and 1 GPIO pins"]
pub mod w0_0;
#[doc = "W0_1 register accessor: an alias for `Reg<W0_1_SPEC>`"]
pub type W0_1 = crate::Reg<w0_1::W0_1_SPEC>;
#[doc = "Word pin registers for all port 0 and 1 GPIO pins"]
pub mod w0_1;
#[doc = "W0_2 register accessor: an alias for `Reg<W0_2_SPEC>`"]
pub type W0_2 = crate::Reg<w0_2::W0_2_SPEC>;
#[doc = "Word pin registers for all port 0 and 1 GPIO pins"]
pub mod w0_2;
#[doc = "W0_3 register accessor: an alias for `Reg<W0_3_SPEC>`"]
pub type W0_3 = crate::Reg<w0_3::W0_3_SPEC>;
#[doc = "Word pin registers for all port 0 and 1 GPIO pins"]
pub mod w0_3;
#[doc = "W0_4 register accessor: an alias for `Reg<W0_4_SPEC>`"]
pub type W0_4 = crate::Reg<w0_4::W0_4_SPEC>;
#[doc = "Word pin registers for all port 0 and 1 GPIO pins"]
pub mod w0_4;
#[doc = "W0_5 register accessor: an alias for `Reg<W0_5_SPEC>`"]
pub type W0_5 = crate::Reg<w0_5::W0_5_SPEC>;
#[doc = "Word pin registers for all port 0 and 1 GPIO pins"]
pub mod w0_5;
#[doc = "W0_6 register accessor: an alias for `Reg<W0_6_SPEC>`"]
pub type W0_6 = crate::Reg<w0_6::W0_6_SPEC>;
#[doc = "Word pin registers for all port 0 and 1 GPIO pins"]
pub mod w0_6;
#[doc = "W0_7 register accessor: an alias for `Reg<W0_7_SPEC>`"]
pub type W0_7 = crate::Reg<w0_7::W0_7_SPEC>;
#[doc = "Word pin registers for all port 0 and 1 GPIO pins"]
pub mod w0_7;
#[doc = "W0_8 register accessor: an alias for `Reg<W0_8_SPEC>`"]
pub type W0_8 = crate::Reg<w0_8::W0_8_SPEC>;
#[doc = "Word pin registers for all port 0 and 1 GPIO pins"]
pub mod w0_8;
#[doc = "W0_9 register accessor: an alias for `Reg<W0_9_SPEC>`"]
pub type W0_9 = crate::Reg<w0_9::W0_9_SPEC>;
#[doc = "Word pin registers for all port 0 and 1 GPIO pins"]
pub mod w0_9;
#[doc = "W0_10 register accessor: an alias for `Reg<W0_10_SPEC>`"]
pub type W0_10 = crate::Reg<w0_10::W0_10_SPEC>;
#[doc = "Word pin registers for all port 0 and 1 GPIO pins"]
pub mod w0_10;
#[doc = "W0_11 register accessor: an alias for `Reg<W0_11_SPEC>`"]
pub type W0_11 = crate::Reg<w0_11::W0_11_SPEC>;
#[doc = "Word pin registers for all port 0 and 1 GPIO pins"]
pub mod w0_11;
#[doc = "W0_12 register accessor: an alias for `Reg<W0_12_SPEC>`"]
pub type W0_12 = crate::Reg<w0_12::W0_12_SPEC>;
#[doc = "Word pin registers for all port 0 and 1 GPIO pins"]
pub mod w0_12;
#[doc = "W0_13 register accessor: an alias for `Reg<W0_13_SPEC>`"]
pub type W0_13 = crate::Reg<w0_13::W0_13_SPEC>;
#[doc = "Word pin registers for all port 0 and 1 GPIO pins"]
pub mod w0_13;
#[doc = "W0_14 register accessor: an alias for `Reg<W0_14_SPEC>`"]
pub type W0_14 = crate::Reg<w0_14::W0_14_SPEC>;
#[doc = "Word pin registers for all port 0 and 1 GPIO pins"]
pub mod w0_14;
#[doc = "W0_15 register accessor: an alias for `Reg<W0_15_SPEC>`"]
pub type W0_15 = crate::Reg<w0_15::W0_15_SPEC>;
#[doc = "Word pin registers for all port 0 and 1 GPIO pins"]
pub mod w0_15;
#[doc = "W0_16 register accessor: an alias for `Reg<W0_16_SPEC>`"]
pub type W0_16 = crate::Reg<w0_16::W0_16_SPEC>;
#[doc = "Word pin registers for all port 0 and 1 GPIO pins"]
pub mod w0_16;
#[doc = "W0_17 register accessor: an alias for `Reg<W0_17_SPEC>`"]
pub type W0_17 = crate::Reg<w0_17::W0_17_SPEC>;
#[doc = "Word pin registers for all port 0 and 1 GPIO pins"]
pub mod w0_17;
#[doc = "W0_18 register accessor: an alias for `Reg<W0_18_SPEC>`"]
pub type W0_18 = crate::Reg<w0_18::W0_18_SPEC>;
#[doc = "Word pin registers for all port 0 and 1 GPIO pins"]
pub mod w0_18;
#[doc = "W0_19 register accessor: an alias for `Reg<W0_19_SPEC>`"]
pub type W0_19 = crate::Reg<w0_19::W0_19_SPEC>;
#[doc = "Word pin registers for all port 0 and 1 GPIO pins"]
pub mod w0_19;
#[doc = "W0_20 register accessor: an alias for `Reg<W0_20_SPEC>`"]
pub type W0_20 = crate::Reg<w0_20::W0_20_SPEC>;
#[doc = "Word pin registers for all port 0 and 1 GPIO pins"]
pub mod w0_20;
#[doc = "W0_21 register accessor: an alias for `Reg<W0_21_SPEC>`"]
pub type W0_21 = crate::Reg<w0_21::W0_21_SPEC>;
#[doc = "Word pin registers for all port 0 and 1 GPIO pins"]
pub mod w0_21;
#[doc = "W0_22 register accessor: an alias for `Reg<W0_22_SPEC>`"]
pub type W0_22 = crate::Reg<w0_22::W0_22_SPEC>;
#[doc = "Word pin registers for all port 0 and 1 GPIO pins"]
pub mod w0_22;
#[doc = "W0_23 register accessor: an alias for `Reg<W0_23_SPEC>`"]
pub type W0_23 = crate::Reg<w0_23::W0_23_SPEC>;
#[doc = "Word pin registers for all port 0 and 1 GPIO pins"]
pub mod w0_23;
#[doc = "W0_24 register accessor: an alias for `Reg<W0_24_SPEC>`"]
pub type W0_24 = crate::Reg<w0_24::W0_24_SPEC>;
#[doc = "Word pin registers for all port 0 and 1 GPIO pins"]
pub mod w0_24;
#[doc = "W0_25 register accessor: an alias for `Reg<W0_25_SPEC>`"]
pub type W0_25 = crate::Reg<w0_25::W0_25_SPEC>;
#[doc = "Word pin registers for all port 0 and 1 GPIO pins"]
pub mod w0_25;
#[doc = "W0_26 register accessor: an alias for `Reg<W0_26_SPEC>`"]
pub type W0_26 = crate::Reg<w0_26::W0_26_SPEC>;
#[doc = "Word pin registers for all port 0 and 1 GPIO pins"]
pub mod w0_26;
#[doc = "W0_27 register accessor: an alias for `Reg<W0_27_SPEC>`"]
pub type W0_27 = crate::Reg<w0_27::W0_27_SPEC>;
#[doc = "Word pin registers for all port 0 and 1 GPIO pins"]
pub mod w0_27;
#[doc = "W0_28 register accessor: an alias for `Reg<W0_28_SPEC>`"]
pub type W0_28 = crate::Reg<w0_28::W0_28_SPEC>;
#[doc = "Word pin registers for all port 0 and 1 GPIO pins"]
pub mod w0_28;
#[doc = "DIR0 register accessor: an alias for `Reg<DIR0_SPEC>`"]
pub type DIR0 = crate::Reg<dir0::DIR0_SPEC>;
#[doc = "Direction registers"]
pub mod dir0;
#[doc = "MASK0 register accessor: an alias for `Reg<MASK0_SPEC>`"]
pub type MASK0 = crate::Reg<mask0::MASK0_SPEC>;
#[doc = "Mask register"]
pub mod mask0;
#[doc = "PIN0 register accessor: an alias for `Reg<PIN0_SPEC>`"]
pub type PIN0 = crate::Reg<pin0::PIN0_SPEC>;
#[doc = "Port pin register"]
pub mod pin0;
#[doc = "MPIN0 register accessor: an alias for `Reg<MPIN0_SPEC>`"]
pub type MPIN0 = crate::Reg<mpin0::MPIN0_SPEC>;
#[doc = "Masked port register"]
pub mod mpin0;
#[doc = "SET0 register accessor: an alias for `Reg<SET0_SPEC>`"]
pub type SET0 = crate::Reg<set0::SET0_SPEC>;
#[doc = "Write: Set register for port Read: output bits for port"]
pub mod set0;
#[doc = "CLR0 register accessor: an alias for `Reg<CLR0_SPEC>`"]
pub type CLR0 = crate::Reg<clr0::CLR0_SPEC>;
#[doc = "Clear port"]
pub mod clr0;
#[doc = "NOT0 register accessor: an alias for `Reg<NOT0_SPEC>`"]
pub type NOT0 = crate::Reg<not0::NOT0_SPEC>;
#[doc = "Toggle port"]
pub mod not0;
#[doc = "DIRSET0 register accessor: an alias for `Reg<DIRSET0_SPEC>`"]
pub type DIRSET0 = crate::Reg<dirset0::DIRSET0_SPEC>;
#[doc = "Set pin direction bits for port"]
pub mod dirset0;
#[doc = "DIRCLR0 register accessor: an alias for `Reg<DIRCLR0_SPEC>`"]
pub type DIRCLR0 = crate::Reg<dirclr0::DIRCLR0_SPEC>;
#[doc = "Clear pin direction bits for port"]
pub mod dirclr0;
#[doc = "DIRNOT0 register accessor: an alias for `Reg<DIRNOT0_SPEC>`"]
pub type DIRNOT0 = crate::Reg<dirnot0::DIRNOT0_SPEC>;
#[doc = "Toggle pin direction bits for port"]
pub mod dirnot0;
