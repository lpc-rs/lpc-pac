#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Digital I/O control for pins PIO0_17"]
    pub pio0_17: crate::Reg<pio0_17::PIO0_17_SPEC>,
    #[doc = "0x04 - Digital I/O control for pins PIO0_13"]
    pub pio0_13: crate::Reg<pio0_13::PIO0_13_SPEC>,
    #[doc = "0x08 - Digital I/O control for pins PIO0_12"]
    pub pio0_12: crate::Reg<pio0_12::PIO0_12_SPEC>,
    #[doc = "0x0c - Digital I/O control for pins PIO0_5"]
    pub pio0_5: crate::Reg<pio0_5::PIO0_5_SPEC>,
    #[doc = "0x10 - Digital I/O control for pins PIO0_4"]
    pub pio0_4: crate::Reg<pio0_4::PIO0_4_SPEC>,
    #[doc = "0x14 - Digital I/O control for pins PIO0_3"]
    pub pio0_3: crate::Reg<pio0_3::PIO0_3_SPEC>,
    #[doc = "0x18 - Digital I/O control for pins PIO0_2"]
    pub pio0_2: crate::Reg<pio0_2::PIO0_2_SPEC>,
    #[doc = "0x1c - Digital I/O control for pins PIO0_11"]
    pub pio0_11: crate::Reg<pio0_11::PIO0_11_SPEC>,
    #[doc = "0x20 - Digital I/O control for pins PIO0_10"]
    pub pio0_10: crate::Reg<pio0_10::PIO0_10_SPEC>,
    #[doc = "0x24 - Digital I/O control for pins PIO0_16"]
    pub pio0_16: crate::Reg<pio0_16::PIO0_16_SPEC>,
    #[doc = "0x28 - Digital I/O control for pins PIO0_15"]
    pub pio0_15: crate::Reg<pio0_15::PIO0_15_SPEC>,
    #[doc = "0x2c - Digital I/O control for pins PIO0_1"]
    pub pio0_1: crate::Reg<pio0_1::PIO0_1_SPEC>,
    _reserved12: [u8; 0x04],
    #[doc = "0x34 - Digital I/O control for pins PIO0_9"]
    pub pio0_9: crate::Reg<pio0_9::PIO0_9_SPEC>,
    #[doc = "0x38 - Digital I/O control for pins PIO0_8"]
    pub pio0_8: crate::Reg<pio0_8::PIO0_8_SPEC>,
    #[doc = "0x3c - Digital I/O control for pins PIO0_7"]
    pub pio0_7: crate::Reg<pio0_7::PIO0_7_SPEC>,
    #[doc = "0x40 - Digital I/O control for pins PIO0_6"]
    pub pio0_6: crate::Reg<pio0_6::PIO0_6_SPEC>,
    #[doc = "0x44 - Digital I/O control for pins PIO0_0"]
    pub pio0_0: crate::Reg<pio0_0::PIO0_0_SPEC>,
    #[doc = "0x48 - Digital I/O control for pins PIO0_14"]
    pub pio0_14: crate::Reg<pio0_14::PIO0_14_SPEC>,
    _reserved18: [u8; 0x04],
    #[doc = "0x50 - Digital I/O control for pins PIO0_28"]
    pub pio0_28: crate::Reg<pio0_28::PIO0_28_SPEC>,
    #[doc = "0x54 - Digital I/O control for pins PIO0_27"]
    pub pio0_27: crate::Reg<pio0_27::PIO0_27_SPEC>,
    #[doc = "0x58 - Digital I/O control for pins PIO0_26"]
    pub pio0_26: crate::Reg<pio0_26::PIO0_26_SPEC>,
    #[doc = "0x5c - Digital I/O control for pins PIO0_25"]
    pub pio0_25: crate::Reg<pio0_25::PIO0_25_SPEC>,
    #[doc = "0x60 - Digital I/O control for pins PIO0_24"]
    pub pio0_24: crate::Reg<pio0_24::PIO0_24_SPEC>,
    #[doc = "0x64 - Digital I/O control for pins PIO0_23"]
    pub pio0_23: crate::Reg<pio0_23::PIO0_23_SPEC>,
    #[doc = "0x68 - Digital I/O control for pins PIO0_22"]
    pub pio0_22: crate::Reg<pio0_22::PIO0_22_SPEC>,
    #[doc = "0x6c - Digital I/O control for pins PIO0_21"]
    pub pio0_21: crate::Reg<pio0_21::PIO0_21_SPEC>,
    #[doc = "0x70 - Digital I/O control for pins PIO0_20"]
    pub pio0_20: crate::Reg<pio0_20::PIO0_20_SPEC>,
    #[doc = "0x74 - Digital I/O control for pins PIO0_19"]
    pub pio0_19: crate::Reg<pio0_19::PIO0_19_SPEC>,
    #[doc = "0x78 - Digital I/O control for pins PIO0_18"]
    pub pio0_18: crate::Reg<pio0_18::PIO0_18_SPEC>,
    #[doc = "0x7c - Digital I/O control for pins PIO1_8"]
    pub pio1_8: crate::Reg<pio1_8::PIO1_8_SPEC>,
    #[doc = "0x80 - Digital I/O control for pins PIO1_9"]
    pub pio1_9: crate::Reg<pio1_9::PIO1_9_SPEC>,
    #[doc = "0x84 - Digital I/O control for pins PIO1_12"]
    pub pio1_12: crate::Reg<pio1_12::PIO1_12_SPEC>,
    #[doc = "0x88 - Digital I/O control for pins PIO1_13"]
    pub pio1_13: crate::Reg<pio1_13::PIO1_13_SPEC>,
    #[doc = "0x8c - Digital I/O control for pins PIO0_31"]
    pub pio0_31: crate::Reg<pio0_31::PIO0_31_SPEC>,
    #[doc = "0x90 - Digital I/O control for pins PIO1_0"]
    pub pio1_0: crate::Reg<pio1_0::PIO1_0_SPEC>,
    #[doc = "0x94 - Digital I/O control for pins PIO1_1"]
    pub pio1_1: crate::Reg<pio1_1::PIO1_1_SPEC>,
    #[doc = "0x98 - Digital I/O control for pins PIO1_2"]
    pub pio1_2: crate::Reg<pio1_2::PIO1_2_SPEC>,
    #[doc = "0x9c - Digital I/O control for pins PIO1_14"]
    pub pio1_14: crate::Reg<pio1_14::PIO1_14_SPEC>,
    #[doc = "0xa0 - Digital I/O control for pins PIO1_15"]
    pub pio1_15: crate::Reg<pio1_15::PIO1_15_SPEC>,
    #[doc = "0xa4 - Digital I/O control for pins PIO1_3"]
    pub pio1_3: crate::Reg<pio1_3::PIO1_3_SPEC>,
    #[doc = "0xa8 - Digital I/O control for pins PIO1_4"]
    pub pio1_4: crate::Reg<pio1_4::PIO1_4_SPEC>,
    #[doc = "0xac - Digital I/O control for pins PIO1_5"]
    pub pio1_5: crate::Reg<pio1_5::PIO1_5_SPEC>,
    #[doc = "0xb0 - Digital I/O control for pins PIO1_16"]
    pub pio1_16: crate::Reg<pio1_16::PIO1_16_SPEC>,
    #[doc = "0xb4 - Digital I/O control for pins PIO1_17"]
    pub pio1_17: crate::Reg<pio1_17::PIO1_17_SPEC>,
    #[doc = "0xb8 - Digital I/O control for pins PIO1_6"]
    pub pio1_6: crate::Reg<pio1_6::PIO1_6_SPEC>,
    #[doc = "0xbc - Digital I/O control for pins PIO1_18"]
    pub pio1_18: crate::Reg<pio1_18::PIO1_18_SPEC>,
    #[doc = "0xc0 - Digital I/O control for pins PIO1_19"]
    pub pio1_19: crate::Reg<pio1_19::PIO1_19_SPEC>,
    #[doc = "0xc4 - Digital I/O control for pins PIO1_7"]
    pub pio1_7: crate::Reg<pio1_7::PIO1_7_SPEC>,
    #[doc = "0xc8 - Digital I/O control for pins PIO0_29"]
    pub pio0_29: crate::Reg<pio0_29::PIO0_29_SPEC>,
    #[doc = "0xcc - Digital I/O control for pins PIO0_30"]
    pub pio0_30: crate::Reg<pio0_30::PIO0_30_SPEC>,
    #[doc = "0xd0 - Digital I/O control for pins PIO1_20"]
    pub pio1_20: crate::Reg<pio1_20::PIO1_20_SPEC>,
    #[doc = "0xd4 - Digital I/O control for pins PIO1_21"]
    pub pio1_21: crate::Reg<pio1_21::PIO1_21_SPEC>,
    #[doc = "0xd8 - Digital I/O control for pins PIO1_11"]
    pub pio1_11: crate::Reg<pio1_11::PIO1_11_SPEC>,
    #[doc = "0xdc - Digital I/O control for pins PIO1_10"]
    pub pio1_10: crate::Reg<pio1_10::PIO1_10_SPEC>,
}
#[doc = "PIO0_17 register accessor: an alias for `Reg<PIO0_17_SPEC>`"]
pub type PIO0_17 = crate::Reg<pio0_17::PIO0_17_SPEC>;
#[doc = "Digital I/O control for pins PIO0_17"]
pub mod pio0_17;
#[doc = "PIO0_13 register accessor: an alias for `Reg<PIO0_13_SPEC>`"]
pub type PIO0_13 = crate::Reg<pio0_13::PIO0_13_SPEC>;
#[doc = "Digital I/O control for pins PIO0_13"]
pub mod pio0_13;
#[doc = "PIO0_12 register accessor: an alias for `Reg<PIO0_12_SPEC>`"]
pub type PIO0_12 = crate::Reg<pio0_12::PIO0_12_SPEC>;
#[doc = "Digital I/O control for pins PIO0_12"]
pub mod pio0_12;
#[doc = "PIO0_5 register accessor: an alias for `Reg<PIO0_5_SPEC>`"]
pub type PIO0_5 = crate::Reg<pio0_5::PIO0_5_SPEC>;
#[doc = "Digital I/O control for pins PIO0_5"]
pub mod pio0_5;
#[doc = "PIO0_4 register accessor: an alias for `Reg<PIO0_4_SPEC>`"]
pub type PIO0_4 = crate::Reg<pio0_4::PIO0_4_SPEC>;
#[doc = "Digital I/O control for pins PIO0_4"]
pub mod pio0_4;
#[doc = "PIO0_3 register accessor: an alias for `Reg<PIO0_3_SPEC>`"]
pub type PIO0_3 = crate::Reg<pio0_3::PIO0_3_SPEC>;
#[doc = "Digital I/O control for pins PIO0_3"]
pub mod pio0_3;
#[doc = "PIO0_2 register accessor: an alias for `Reg<PIO0_2_SPEC>`"]
pub type PIO0_2 = crate::Reg<pio0_2::PIO0_2_SPEC>;
#[doc = "Digital I/O control for pins PIO0_2"]
pub mod pio0_2;
#[doc = "PIO0_11 register accessor: an alias for `Reg<PIO0_11_SPEC>`"]
pub type PIO0_11 = crate::Reg<pio0_11::PIO0_11_SPEC>;
#[doc = "Digital I/O control for pins PIO0_11"]
pub mod pio0_11;
#[doc = "PIO0_10 register accessor: an alias for `Reg<PIO0_10_SPEC>`"]
pub type PIO0_10 = crate::Reg<pio0_10::PIO0_10_SPEC>;
#[doc = "Digital I/O control for pins PIO0_10"]
pub mod pio0_10;
#[doc = "PIO0_16 register accessor: an alias for `Reg<PIO0_16_SPEC>`"]
pub type PIO0_16 = crate::Reg<pio0_16::PIO0_16_SPEC>;
#[doc = "Digital I/O control for pins PIO0_16"]
pub mod pio0_16;
#[doc = "PIO0_15 register accessor: an alias for `Reg<PIO0_15_SPEC>`"]
pub type PIO0_15 = crate::Reg<pio0_15::PIO0_15_SPEC>;
#[doc = "Digital I/O control for pins PIO0_15"]
pub mod pio0_15;
#[doc = "PIO0_1 register accessor: an alias for `Reg<PIO0_1_SPEC>`"]
pub type PIO0_1 = crate::Reg<pio0_1::PIO0_1_SPEC>;
#[doc = "Digital I/O control for pins PIO0_1"]
pub mod pio0_1;
#[doc = "PIO0_9 register accessor: an alias for `Reg<PIO0_9_SPEC>`"]
pub type PIO0_9 = crate::Reg<pio0_9::PIO0_9_SPEC>;
#[doc = "Digital I/O control for pins PIO0_9"]
pub mod pio0_9;
#[doc = "PIO0_8 register accessor: an alias for `Reg<PIO0_8_SPEC>`"]
pub type PIO0_8 = crate::Reg<pio0_8::PIO0_8_SPEC>;
#[doc = "Digital I/O control for pins PIO0_8"]
pub mod pio0_8;
#[doc = "PIO0_7 register accessor: an alias for `Reg<PIO0_7_SPEC>`"]
pub type PIO0_7 = crate::Reg<pio0_7::PIO0_7_SPEC>;
#[doc = "Digital I/O control for pins PIO0_7"]
pub mod pio0_7;
#[doc = "PIO0_6 register accessor: an alias for `Reg<PIO0_6_SPEC>`"]
pub type PIO0_6 = crate::Reg<pio0_6::PIO0_6_SPEC>;
#[doc = "Digital I/O control for pins PIO0_6"]
pub mod pio0_6;
#[doc = "PIO0_0 register accessor: an alias for `Reg<PIO0_0_SPEC>`"]
pub type PIO0_0 = crate::Reg<pio0_0::PIO0_0_SPEC>;
#[doc = "Digital I/O control for pins PIO0_0"]
pub mod pio0_0;
#[doc = "PIO0_14 register accessor: an alias for `Reg<PIO0_14_SPEC>`"]
pub type PIO0_14 = crate::Reg<pio0_14::PIO0_14_SPEC>;
#[doc = "Digital I/O control for pins PIO0_14"]
pub mod pio0_14;
#[doc = "PIO0_28 register accessor: an alias for `Reg<PIO0_28_SPEC>`"]
pub type PIO0_28 = crate::Reg<pio0_28::PIO0_28_SPEC>;
#[doc = "Digital I/O control for pins PIO0_28"]
pub mod pio0_28;
#[doc = "PIO0_27 register accessor: an alias for `Reg<PIO0_27_SPEC>`"]
pub type PIO0_27 = crate::Reg<pio0_27::PIO0_27_SPEC>;
#[doc = "Digital I/O control for pins PIO0_27"]
pub mod pio0_27;
#[doc = "PIO0_26 register accessor: an alias for `Reg<PIO0_26_SPEC>`"]
pub type PIO0_26 = crate::Reg<pio0_26::PIO0_26_SPEC>;
#[doc = "Digital I/O control for pins PIO0_26"]
pub mod pio0_26;
#[doc = "PIO0_25 register accessor: an alias for `Reg<PIO0_25_SPEC>`"]
pub type PIO0_25 = crate::Reg<pio0_25::PIO0_25_SPEC>;
#[doc = "Digital I/O control for pins PIO0_25"]
pub mod pio0_25;
#[doc = "PIO0_24 register accessor: an alias for `Reg<PIO0_24_SPEC>`"]
pub type PIO0_24 = crate::Reg<pio0_24::PIO0_24_SPEC>;
#[doc = "Digital I/O control for pins PIO0_24"]
pub mod pio0_24;
#[doc = "PIO0_23 register accessor: an alias for `Reg<PIO0_23_SPEC>`"]
pub type PIO0_23 = crate::Reg<pio0_23::PIO0_23_SPEC>;
#[doc = "Digital I/O control for pins PIO0_23"]
pub mod pio0_23;
#[doc = "PIO0_22 register accessor: an alias for `Reg<PIO0_22_SPEC>`"]
pub type PIO0_22 = crate::Reg<pio0_22::PIO0_22_SPEC>;
#[doc = "Digital I/O control for pins PIO0_22"]
pub mod pio0_22;
#[doc = "PIO0_21 register accessor: an alias for `Reg<PIO0_21_SPEC>`"]
pub type PIO0_21 = crate::Reg<pio0_21::PIO0_21_SPEC>;
#[doc = "Digital I/O control for pins PIO0_21"]
pub mod pio0_21;
#[doc = "PIO0_20 register accessor: an alias for `Reg<PIO0_20_SPEC>`"]
pub type PIO0_20 = crate::Reg<pio0_20::PIO0_20_SPEC>;
#[doc = "Digital I/O control for pins PIO0_20"]
pub mod pio0_20;
#[doc = "PIO0_19 register accessor: an alias for `Reg<PIO0_19_SPEC>`"]
pub type PIO0_19 = crate::Reg<pio0_19::PIO0_19_SPEC>;
#[doc = "Digital I/O control for pins PIO0_19"]
pub mod pio0_19;
#[doc = "PIO0_18 register accessor: an alias for `Reg<PIO0_18_SPEC>`"]
pub type PIO0_18 = crate::Reg<pio0_18::PIO0_18_SPEC>;
#[doc = "Digital I/O control for pins PIO0_18"]
pub mod pio0_18;
#[doc = "PIO1_8 register accessor: an alias for `Reg<PIO1_8_SPEC>`"]
pub type PIO1_8 = crate::Reg<pio1_8::PIO1_8_SPEC>;
#[doc = "Digital I/O control for pins PIO1_8"]
pub mod pio1_8;
#[doc = "PIO1_9 register accessor: an alias for `Reg<PIO1_9_SPEC>`"]
pub type PIO1_9 = crate::Reg<pio1_9::PIO1_9_SPEC>;
#[doc = "Digital I/O control for pins PIO1_9"]
pub mod pio1_9;
#[doc = "PIO1_12 register accessor: an alias for `Reg<PIO1_12_SPEC>`"]
pub type PIO1_12 = crate::Reg<pio1_12::PIO1_12_SPEC>;
#[doc = "Digital I/O control for pins PIO1_12"]
pub mod pio1_12;
#[doc = "PIO1_13 register accessor: an alias for `Reg<PIO1_13_SPEC>`"]
pub type PIO1_13 = crate::Reg<pio1_13::PIO1_13_SPEC>;
#[doc = "Digital I/O control for pins PIO1_13"]
pub mod pio1_13;
#[doc = "PIO0_31 register accessor: an alias for `Reg<PIO0_31_SPEC>`"]
pub type PIO0_31 = crate::Reg<pio0_31::PIO0_31_SPEC>;
#[doc = "Digital I/O control for pins PIO0_31"]
pub mod pio0_31;
#[doc = "PIO1_0 register accessor: an alias for `Reg<PIO1_0_SPEC>`"]
pub type PIO1_0 = crate::Reg<pio1_0::PIO1_0_SPEC>;
#[doc = "Digital I/O control for pins PIO1_0"]
pub mod pio1_0;
#[doc = "PIO1_1 register accessor: an alias for `Reg<PIO1_1_SPEC>`"]
pub type PIO1_1 = crate::Reg<pio1_1::PIO1_1_SPEC>;
#[doc = "Digital I/O control for pins PIO1_1"]
pub mod pio1_1;
#[doc = "PIO1_2 register accessor: an alias for `Reg<PIO1_2_SPEC>`"]
pub type PIO1_2 = crate::Reg<pio1_2::PIO1_2_SPEC>;
#[doc = "Digital I/O control for pins PIO1_2"]
pub mod pio1_2;
#[doc = "PIO1_14 register accessor: an alias for `Reg<PIO1_14_SPEC>`"]
pub type PIO1_14 = crate::Reg<pio1_14::PIO1_14_SPEC>;
#[doc = "Digital I/O control for pins PIO1_14"]
pub mod pio1_14;
#[doc = "PIO1_15 register accessor: an alias for `Reg<PIO1_15_SPEC>`"]
pub type PIO1_15 = crate::Reg<pio1_15::PIO1_15_SPEC>;
#[doc = "Digital I/O control for pins PIO1_15"]
pub mod pio1_15;
#[doc = "PIO1_3 register accessor: an alias for `Reg<PIO1_3_SPEC>`"]
pub type PIO1_3 = crate::Reg<pio1_3::PIO1_3_SPEC>;
#[doc = "Digital I/O control for pins PIO1_3"]
pub mod pio1_3;
#[doc = "PIO1_4 register accessor: an alias for `Reg<PIO1_4_SPEC>`"]
pub type PIO1_4 = crate::Reg<pio1_4::PIO1_4_SPEC>;
#[doc = "Digital I/O control for pins PIO1_4"]
pub mod pio1_4;
#[doc = "PIO1_5 register accessor: an alias for `Reg<PIO1_5_SPEC>`"]
pub type PIO1_5 = crate::Reg<pio1_5::PIO1_5_SPEC>;
#[doc = "Digital I/O control for pins PIO1_5"]
pub mod pio1_5;
#[doc = "PIO1_16 register accessor: an alias for `Reg<PIO1_16_SPEC>`"]
pub type PIO1_16 = crate::Reg<pio1_16::PIO1_16_SPEC>;
#[doc = "Digital I/O control for pins PIO1_16"]
pub mod pio1_16;
#[doc = "PIO1_17 register accessor: an alias for `Reg<PIO1_17_SPEC>`"]
pub type PIO1_17 = crate::Reg<pio1_17::PIO1_17_SPEC>;
#[doc = "Digital I/O control for pins PIO1_17"]
pub mod pio1_17;
#[doc = "PIO1_6 register accessor: an alias for `Reg<PIO1_6_SPEC>`"]
pub type PIO1_6 = crate::Reg<pio1_6::PIO1_6_SPEC>;
#[doc = "Digital I/O control for pins PIO1_6"]
pub mod pio1_6;
#[doc = "PIO1_18 register accessor: an alias for `Reg<PIO1_18_SPEC>`"]
pub type PIO1_18 = crate::Reg<pio1_18::PIO1_18_SPEC>;
#[doc = "Digital I/O control for pins PIO1_18"]
pub mod pio1_18;
#[doc = "PIO1_19 register accessor: an alias for `Reg<PIO1_19_SPEC>`"]
pub type PIO1_19 = crate::Reg<pio1_19::PIO1_19_SPEC>;
#[doc = "Digital I/O control for pins PIO1_19"]
pub mod pio1_19;
#[doc = "PIO1_7 register accessor: an alias for `Reg<PIO1_7_SPEC>`"]
pub type PIO1_7 = crate::Reg<pio1_7::PIO1_7_SPEC>;
#[doc = "Digital I/O control for pins PIO1_7"]
pub mod pio1_7;
#[doc = "PIO0_29 register accessor: an alias for `Reg<PIO0_29_SPEC>`"]
pub type PIO0_29 = crate::Reg<pio0_29::PIO0_29_SPEC>;
#[doc = "Digital I/O control for pins PIO0_29"]
pub mod pio0_29;
#[doc = "PIO0_30 register accessor: an alias for `Reg<PIO0_30_SPEC>`"]
pub type PIO0_30 = crate::Reg<pio0_30::PIO0_30_SPEC>;
#[doc = "Digital I/O control for pins PIO0_30"]
pub mod pio0_30;
#[doc = "PIO1_20 register accessor: an alias for `Reg<PIO1_20_SPEC>`"]
pub type PIO1_20 = crate::Reg<pio1_20::PIO1_20_SPEC>;
#[doc = "Digital I/O control for pins PIO1_20"]
pub mod pio1_20;
#[doc = "PIO1_21 register accessor: an alias for `Reg<PIO1_21_SPEC>`"]
pub type PIO1_21 = crate::Reg<pio1_21::PIO1_21_SPEC>;
#[doc = "Digital I/O control for pins PIO1_21"]
pub mod pio1_21;
#[doc = "PIO1_11 register accessor: an alias for `Reg<PIO1_11_SPEC>`"]
pub type PIO1_11 = crate::Reg<pio1_11::PIO1_11_SPEC>;
#[doc = "Digital I/O control for pins PIO1_11"]
pub mod pio1_11;
#[doc = "PIO1_10 register accessor: an alias for `Reg<PIO1_10_SPEC>`"]
pub type PIO1_10 = crate::Reg<pio1_10::PIO1_10_SPEC>;
#[doc = "Digital I/O control for pins PIO1_10"]
pub mod pio1_10;
