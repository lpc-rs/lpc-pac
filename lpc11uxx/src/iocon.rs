#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - I/O configuration for pin RESET/PIO0_0"]
    pub reset_pio0_0: crate::Reg<reset_pio0_0::RESET_PIO0_0_SPEC>,
    #[doc = "0x04 - I/O configuration for pin PIO0_1/CLKOUT/CT32B0_MAT2/USB_FTOGGLE"]
    pub pio0_1: crate::Reg<pio0_1::PIO0_1_SPEC>,
    #[doc = "0x08 - I/O configuration for pin PIO0_2/SSEL0/CT16B0_CAP0"]
    pub pio0_2: crate::Reg<pio0_2::PIO0_2_SPEC>,
    #[doc = "0x0c - I/O configuration for pin PIO0_3/USB_VBUS"]
    pub pio0_3: crate::Reg<pio0_3::PIO0_3_SPEC>,
    #[doc = "0x10 - I/O configuration for pin PIO0_4/SCL"]
    pub pio0_4: crate::Reg<pio0_4::PIO0_4_SPEC>,
    #[doc = "0x14 - I/O configuration for pin PIO0_5/SDA"]
    pub pio0_5: crate::Reg<pio0_5::PIO0_5_SPEC>,
    #[doc = "0x18 - I/O configuration for pin PIO0_6/USB_CONNECT/SCK0"]
    pub pio0_6: crate::Reg<pio0_6::PIO0_6_SPEC>,
    #[doc = "0x1c - I/O configuration for pin PIO0_7/CTS"]
    pub pio0_7: crate::Reg<pio0_7::PIO0_7_SPEC>,
    #[doc = "0x20 - I/O configuration for pin PIO0_8/MISO0/CT16B0_MAT0"]
    pub pio0_8: crate::Reg<pio0_8::PIO0_8_SPEC>,
    #[doc = "0x24 - I/O configuration for pin PIO0_9/MOSI0/CT16B0_MAT1"]
    pub pio0_9: crate::Reg<pio0_9::PIO0_9_SPEC>,
    #[doc = "0x28 - I/O configuration for pin SWCLK/PIO0_10/ SCK0/CT16B0_MAT2"]
    pub swclk_pio0_10: crate::Reg<swclk_pio0_10::SWCLK_PIO0_10_SPEC>,
    #[doc = "0x2c - I/O configuration for pin TDI/PIO0_11/AD0/CT32B0_MAT3"]
    pub tdi_pio0_11: crate::Reg<tdi_pio0_11::TDI_PIO0_11_SPEC>,
    #[doc = "0x30 - I/O configuration for pin TMS/PIO0_12/AD1/CT32B1_CAP0"]
    pub tms_pio0_12: crate::Reg<tms_pio0_12::TMS_PIO0_12_SPEC>,
    #[doc = "0x34 - I/O configuration for pin TDO/PIO0_13/AD2/CT32B1_MAT0"]
    pub tdo_pio0_13: crate::Reg<tdo_pio0_13::TDO_PIO0_13_SPEC>,
    #[doc = "0x38 - I/O configuration for pin TRST/PIO0_14/AD3/CT32B1_MAT1"]
    pub trst_pio0_14: crate::Reg<trst_pio0_14::TRST_PIO0_14_SPEC>,
    #[doc = "0x3c - I/O configuration for pin SWDIO/PIO0_15/AD4/CT32B1_MAT2"]
    pub swdio_pio0_15: crate::Reg<swdio_pio0_15::SWDIO_PIO0_15_SPEC>,
    #[doc = "0x40 - I/O configuration for pin PIO0_16/AD5/CT32B1_MAT3/ WAKEUP"]
    pub pio0_16: crate::Reg<pio0_16::PIO0_16_SPEC>,
    #[doc = "0x44 - I/O configuration for pin PIO0_17/RTS/CT32B0_CAP0/SCLK"]
    pub pio0_17: crate::Reg<pio0_17::PIO0_17_SPEC>,
    #[doc = "0x48 - I/O configuration for pin PIO0_18/RXD/CT32B0_MAT0"]
    pub pio0_18: crate::Reg<pio0_18::PIO0_18_SPEC>,
    #[doc = "0x4c - I/O configuration for pin PIO0_19/TXD/CT32B0_MAT1"]
    pub pio0_19: crate::Reg<pio0_19::PIO0_19_SPEC>,
    #[doc = "0x50 - I/O configuration for pin PIO0_20/CT16B1_CAP0"]
    pub pio0_20: crate::Reg<pio0_20::PIO0_20_SPEC>,
    #[doc = "0x54 - I/O configuration for pin PIO0_21/CT16B1_MAT0/MOSI1"]
    pub pio0_21: crate::Reg<pio0_21::PIO0_21_SPEC>,
    #[doc = "0x58 - I/O configuration for pin PIO0_22/AD6/CT16B1_MAT1/MISO1"]
    pub pio0_22: crate::Reg<pio0_22::PIO0_22_SPEC>,
    #[doc = "0x5c - I/O configuration for pin PIO0_23/AD7"]
    pub pio0_23: crate::Reg<pio0_23::PIO0_23_SPEC>,
    #[doc = "0x60 - I/O configuration for pin PIO1_0/CT32B1_MAT0"]
    pub pio1_0: crate::Reg<pio1_0::PIO1_0_SPEC>,
    #[doc = "0x64 - I/O configuration for pin PIO1_1/CT32B1_MAT1"]
    pub pio1_1: crate::Reg<pio1_1::PIO1_1_SPEC>,
    #[doc = "0x68 - I/O configuration for pin PIO1_2/CT32B1_MAT2"]
    pub pio1_2: crate::Reg<pio1_2::PIO1_2_SPEC>,
    #[doc = "0x6c - I/O configuration for pin PIO1_3/CT32B1_MAT3"]
    pub pio1_3: crate::Reg<pio1_3::PIO1_3_SPEC>,
    #[doc = "0x70 - I/O configuration for pin PIO1_4/CT32B1_CAP0"]
    pub pio1_4: crate::Reg<pio1_4::PIO1_4_SPEC>,
    #[doc = "0x74 - I/O configuration for pin PIO1_5/CT32B1_CAP1"]
    pub pio1_5: crate::Reg<pio1_5::PIO1_5_SPEC>,
    #[doc = "0x78 - I/O configuration for pin PIO1_6"]
    pub pio1_6: crate::Reg<pio1_6::PIO1_6_SPEC>,
    #[doc = "0x7c - I/O configuration for pin PIO1_7"]
    pub pio1_7: crate::Reg<pio1_7::PIO1_7_SPEC>,
    #[doc = "0x80 - I/O configuration for pin PIO1_8"]
    pub pio1_8: crate::Reg<pio1_8::PIO1_8_SPEC>,
    #[doc = "0x84 - I/O configuration for pin PIO1_9"]
    pub pio1_9: crate::Reg<pio1_9::PIO1_9_SPEC>,
    #[doc = "0x88 - I/O configuration for pin PIO1_10"]
    pub pio1_10: crate::Reg<pio1_10::PIO1_10_SPEC>,
    #[doc = "0x8c - I/O configuration for pin PIO1_11"]
    pub pio1_11: crate::Reg<pio1_11::PIO1_11_SPEC>,
    #[doc = "0x90 - I/O configuration for pin PIO1_12"]
    pub pio1_12: crate::Reg<pio1_12::PIO1_12_SPEC>,
    #[doc = "0x94 - I/O configuration for pin PIO1_13/DTR/CT16B0_MAT0/TXD"]
    pub pio1_13: crate::Reg<pio1_13::PIO1_13_SPEC>,
    #[doc = "0x98 - I/O configuration for pin PIO1_14/DSR/CT16B0_MAT1/RXD"]
    pub pio1_14: crate::Reg<pio1_14::PIO1_14_SPEC>,
    #[doc = "0x9c - I/O configuration for pin PIO1_15/DCD/ CT16B0_MAT2/SCK1"]
    pub pio1_15: crate::Reg<pio1_15::PIO1_15_SPEC>,
    #[doc = "0xa0 - I/O configuration for pin PIO1_16/RI/CT16B0_CAP0"]
    pub pio1_16: crate::Reg<pio1_16::PIO1_16_SPEC>,
    #[doc = "0xa4 - I/O configuration for PIO1_17/CT16B0_CAP1/RXD"]
    pub pio1_17: crate::Reg<pio1_17::PIO1_17_SPEC>,
    #[doc = "0xa8 - I/O configuration for PIO1_18/CT16B1_CAP1/TXD"]
    pub pio1_18: crate::Reg<pio1_18::PIO1_18_SPEC>,
    #[doc = "0xac - I/O configuration for pin PIO1_19/DTR/SSEL1"]
    pub pio1_19: crate::Reg<pio1_19::PIO1_19_SPEC>,
    #[doc = "0xb0 - I/O configuration for pin PIO1_20/DSR/SCK1"]
    pub pio1_20: crate::Reg<pio1_20::PIO1_20_SPEC>,
    #[doc = "0xb4 - I/O configuration for pin PIO1_21/DCD/MISO1"]
    pub pio1_21: crate::Reg<pio1_21::PIO1_21_SPEC>,
    #[doc = "0xb8 - I/O configuration for pin PIO1_22/RI/MOSI1"]
    pub pio1_22: crate::Reg<pio1_22::PIO1_22_SPEC>,
    #[doc = "0xbc - I/O configuration for pin PIO1_23/CT16B1_MAT1/SSEL1"]
    pub pio1_23: crate::Reg<pio1_23::PIO1_23_SPEC>,
    #[doc = "0xc0 - I/O configuration for pin PIO1_24/ CT32B0_MAT0"]
    pub pio1_24: crate::Reg<pio1_24::PIO1_24_SPEC>,
    #[doc = "0xc4 - I/O configuration for pin PIO1_25/CT32B0_MAT1"]
    pub pio1_25: crate::Reg<pio1_25::PIO1_25_SPEC>,
    #[doc = "0xc8 - I/O configuration for pin PIO1_26/CT32B0_MAT2/ RXD"]
    pub pio1_26: crate::Reg<pio1_26::PIO1_26_SPEC>,
    #[doc = "0xcc - I/O configuration for pin PIO1_27/CT32B0_MAT3/ TXD"]
    pub pio1_27: crate::Reg<pio1_27::PIO1_27_SPEC>,
    #[doc = "0xd0 - I/O configuration for pin PIO1_28/CT32B0_CAP0/ SCLK"]
    pub pio1_28: crate::Reg<pio1_28::PIO1_28_SPEC>,
    #[doc = "0xd4 - I/O configuration for pin PIO1_29/SCK0/ CT32B0_CAP1"]
    pub pio1_29: crate::Reg<pio1_29::PIO1_29_SPEC>,
    _reserved54: [u8; 0x04],
    #[doc = "0xdc - I/O configuration for pin PIO1_31"]
    pub pio1_31: crate::Reg<pio1_31::PIO1_31_SPEC>,
}
#[doc = "RESET_PIO0_0 register accessor: an alias for `Reg<RESET_PIO0_0_SPEC>`"]
pub type RESET_PIO0_0 = crate::Reg<reset_pio0_0::RESET_PIO0_0_SPEC>;
#[doc = "I/O configuration for pin RESET/PIO0_0"]
pub mod reset_pio0_0;
#[doc = "PIO0_1 register accessor: an alias for `Reg<PIO0_1_SPEC>`"]
pub type PIO0_1 = crate::Reg<pio0_1::PIO0_1_SPEC>;
#[doc = "I/O configuration for pin PIO0_1/CLKOUT/CT32B0_MAT2/USB_FTOGGLE"]
pub mod pio0_1;
#[doc = "PIO0_2 register accessor: an alias for `Reg<PIO0_2_SPEC>`"]
pub type PIO0_2 = crate::Reg<pio0_2::PIO0_2_SPEC>;
#[doc = "I/O configuration for pin PIO0_2/SSEL0/CT16B0_CAP0"]
pub mod pio0_2;
#[doc = "PIO0_3 register accessor: an alias for `Reg<PIO0_3_SPEC>`"]
pub type PIO0_3 = crate::Reg<pio0_3::PIO0_3_SPEC>;
#[doc = "I/O configuration for pin PIO0_3/USB_VBUS"]
pub mod pio0_3;
#[doc = "PIO0_4 register accessor: an alias for `Reg<PIO0_4_SPEC>`"]
pub type PIO0_4 = crate::Reg<pio0_4::PIO0_4_SPEC>;
#[doc = "I/O configuration for pin PIO0_4/SCL"]
pub mod pio0_4;
#[doc = "PIO0_5 register accessor: an alias for `Reg<PIO0_5_SPEC>`"]
pub type PIO0_5 = crate::Reg<pio0_5::PIO0_5_SPEC>;
#[doc = "I/O configuration for pin PIO0_5/SDA"]
pub mod pio0_5;
#[doc = "PIO0_6 register accessor: an alias for `Reg<PIO0_6_SPEC>`"]
pub type PIO0_6 = crate::Reg<pio0_6::PIO0_6_SPEC>;
#[doc = "I/O configuration for pin PIO0_6/USB_CONNECT/SCK0"]
pub mod pio0_6;
#[doc = "PIO0_7 register accessor: an alias for `Reg<PIO0_7_SPEC>`"]
pub type PIO0_7 = crate::Reg<pio0_7::PIO0_7_SPEC>;
#[doc = "I/O configuration for pin PIO0_7/CTS"]
pub mod pio0_7;
#[doc = "PIO0_8 register accessor: an alias for `Reg<PIO0_8_SPEC>`"]
pub type PIO0_8 = crate::Reg<pio0_8::PIO0_8_SPEC>;
#[doc = "I/O configuration for pin PIO0_8/MISO0/CT16B0_MAT0"]
pub mod pio0_8;
#[doc = "PIO0_9 register accessor: an alias for `Reg<PIO0_9_SPEC>`"]
pub type PIO0_9 = crate::Reg<pio0_9::PIO0_9_SPEC>;
#[doc = "I/O configuration for pin PIO0_9/MOSI0/CT16B0_MAT1"]
pub mod pio0_9;
#[doc = "SWCLK_PIO0_10 register accessor: an alias for `Reg<SWCLK_PIO0_10_SPEC>`"]
pub type SWCLK_PIO0_10 = crate::Reg<swclk_pio0_10::SWCLK_PIO0_10_SPEC>;
#[doc = "I/O configuration for pin SWCLK/PIO0_10/ SCK0/CT16B0_MAT2"]
pub mod swclk_pio0_10;
#[doc = "TDI_PIO0_11 register accessor: an alias for `Reg<TDI_PIO0_11_SPEC>`"]
pub type TDI_PIO0_11 = crate::Reg<tdi_pio0_11::TDI_PIO0_11_SPEC>;
#[doc = "I/O configuration for pin TDI/PIO0_11/AD0/CT32B0_MAT3"]
pub mod tdi_pio0_11;
#[doc = "TMS_PIO0_12 register accessor: an alias for `Reg<TMS_PIO0_12_SPEC>`"]
pub type TMS_PIO0_12 = crate::Reg<tms_pio0_12::TMS_PIO0_12_SPEC>;
#[doc = "I/O configuration for pin TMS/PIO0_12/AD1/CT32B1_CAP0"]
pub mod tms_pio0_12;
#[doc = "TDO_PIO0_13 register accessor: an alias for `Reg<TDO_PIO0_13_SPEC>`"]
pub type TDO_PIO0_13 = crate::Reg<tdo_pio0_13::TDO_PIO0_13_SPEC>;
#[doc = "I/O configuration for pin TDO/PIO0_13/AD2/CT32B1_MAT0"]
pub mod tdo_pio0_13;
#[doc = "TRST_PIO0_14 register accessor: an alias for `Reg<TRST_PIO0_14_SPEC>`"]
pub type TRST_PIO0_14 = crate::Reg<trst_pio0_14::TRST_PIO0_14_SPEC>;
#[doc = "I/O configuration for pin TRST/PIO0_14/AD3/CT32B1_MAT1"]
pub mod trst_pio0_14;
#[doc = "SWDIO_PIO0_15 register accessor: an alias for `Reg<SWDIO_PIO0_15_SPEC>`"]
pub type SWDIO_PIO0_15 = crate::Reg<swdio_pio0_15::SWDIO_PIO0_15_SPEC>;
#[doc = "I/O configuration for pin SWDIO/PIO0_15/AD4/CT32B1_MAT2"]
pub mod swdio_pio0_15;
#[doc = "PIO0_16 register accessor: an alias for `Reg<PIO0_16_SPEC>`"]
pub type PIO0_16 = crate::Reg<pio0_16::PIO0_16_SPEC>;
#[doc = "I/O configuration for pin PIO0_16/AD5/CT32B1_MAT3/ WAKEUP"]
pub mod pio0_16;
#[doc = "PIO0_17 register accessor: an alias for `Reg<PIO0_17_SPEC>`"]
pub type PIO0_17 = crate::Reg<pio0_17::PIO0_17_SPEC>;
#[doc = "I/O configuration for pin PIO0_17/RTS/CT32B0_CAP0/SCLK"]
pub mod pio0_17;
#[doc = "PIO0_18 register accessor: an alias for `Reg<PIO0_18_SPEC>`"]
pub type PIO0_18 = crate::Reg<pio0_18::PIO0_18_SPEC>;
#[doc = "I/O configuration for pin PIO0_18/RXD/CT32B0_MAT0"]
pub mod pio0_18;
#[doc = "PIO0_19 register accessor: an alias for `Reg<PIO0_19_SPEC>`"]
pub type PIO0_19 = crate::Reg<pio0_19::PIO0_19_SPEC>;
#[doc = "I/O configuration for pin PIO0_19/TXD/CT32B0_MAT1"]
pub mod pio0_19;
#[doc = "PIO0_20 register accessor: an alias for `Reg<PIO0_20_SPEC>`"]
pub type PIO0_20 = crate::Reg<pio0_20::PIO0_20_SPEC>;
#[doc = "I/O configuration for pin PIO0_20/CT16B1_CAP0"]
pub mod pio0_20;
#[doc = "PIO0_21 register accessor: an alias for `Reg<PIO0_21_SPEC>`"]
pub type PIO0_21 = crate::Reg<pio0_21::PIO0_21_SPEC>;
#[doc = "I/O configuration for pin PIO0_21/CT16B1_MAT0/MOSI1"]
pub mod pio0_21;
#[doc = "PIO0_22 register accessor: an alias for `Reg<PIO0_22_SPEC>`"]
pub type PIO0_22 = crate::Reg<pio0_22::PIO0_22_SPEC>;
#[doc = "I/O configuration for pin PIO0_22/AD6/CT16B1_MAT1/MISO1"]
pub mod pio0_22;
#[doc = "PIO0_23 register accessor: an alias for `Reg<PIO0_23_SPEC>`"]
pub type PIO0_23 = crate::Reg<pio0_23::PIO0_23_SPEC>;
#[doc = "I/O configuration for pin PIO0_23/AD7"]
pub mod pio0_23;
#[doc = "PIO1_0 register accessor: an alias for `Reg<PIO1_0_SPEC>`"]
pub type PIO1_0 = crate::Reg<pio1_0::PIO1_0_SPEC>;
#[doc = "I/O configuration for pin PIO1_0/CT32B1_MAT0"]
pub mod pio1_0;
#[doc = "PIO1_1 register accessor: an alias for `Reg<PIO1_1_SPEC>`"]
pub type PIO1_1 = crate::Reg<pio1_1::PIO1_1_SPEC>;
#[doc = "I/O configuration for pin PIO1_1/CT32B1_MAT1"]
pub mod pio1_1;
#[doc = "PIO1_2 register accessor: an alias for `Reg<PIO1_2_SPEC>`"]
pub type PIO1_2 = crate::Reg<pio1_2::PIO1_2_SPEC>;
#[doc = "I/O configuration for pin PIO1_2/CT32B1_MAT2"]
pub mod pio1_2;
#[doc = "PIO1_3 register accessor: an alias for `Reg<PIO1_3_SPEC>`"]
pub type PIO1_3 = crate::Reg<pio1_3::PIO1_3_SPEC>;
#[doc = "I/O configuration for pin PIO1_3/CT32B1_MAT3"]
pub mod pio1_3;
#[doc = "PIO1_4 register accessor: an alias for `Reg<PIO1_4_SPEC>`"]
pub type PIO1_4 = crate::Reg<pio1_4::PIO1_4_SPEC>;
#[doc = "I/O configuration for pin PIO1_4/CT32B1_CAP0"]
pub mod pio1_4;
#[doc = "PIO1_5 register accessor: an alias for `Reg<PIO1_5_SPEC>`"]
pub type PIO1_5 = crate::Reg<pio1_5::PIO1_5_SPEC>;
#[doc = "I/O configuration for pin PIO1_5/CT32B1_CAP1"]
pub mod pio1_5;
#[doc = "PIO1_6 register accessor: an alias for `Reg<PIO1_6_SPEC>`"]
pub type PIO1_6 = crate::Reg<pio1_6::PIO1_6_SPEC>;
#[doc = "I/O configuration for pin PIO1_6"]
pub mod pio1_6;
#[doc = "PIO1_7 register accessor: an alias for `Reg<PIO1_7_SPEC>`"]
pub type PIO1_7 = crate::Reg<pio1_7::PIO1_7_SPEC>;
#[doc = "I/O configuration for pin PIO1_7"]
pub mod pio1_7;
#[doc = "PIO1_8 register accessor: an alias for `Reg<PIO1_8_SPEC>`"]
pub type PIO1_8 = crate::Reg<pio1_8::PIO1_8_SPEC>;
#[doc = "I/O configuration for pin PIO1_8"]
pub mod pio1_8;
#[doc = "PIO1_9 register accessor: an alias for `Reg<PIO1_9_SPEC>`"]
pub type PIO1_9 = crate::Reg<pio1_9::PIO1_9_SPEC>;
#[doc = "I/O configuration for pin PIO1_9"]
pub mod pio1_9;
#[doc = "PIO1_10 register accessor: an alias for `Reg<PIO1_10_SPEC>`"]
pub type PIO1_10 = crate::Reg<pio1_10::PIO1_10_SPEC>;
#[doc = "I/O configuration for pin PIO1_10"]
pub mod pio1_10;
#[doc = "PIO1_11 register accessor: an alias for `Reg<PIO1_11_SPEC>`"]
pub type PIO1_11 = crate::Reg<pio1_11::PIO1_11_SPEC>;
#[doc = "I/O configuration for pin PIO1_11"]
pub mod pio1_11;
#[doc = "PIO1_12 register accessor: an alias for `Reg<PIO1_12_SPEC>`"]
pub type PIO1_12 = crate::Reg<pio1_12::PIO1_12_SPEC>;
#[doc = "I/O configuration for pin PIO1_12"]
pub mod pio1_12;
#[doc = "PIO1_13 register accessor: an alias for `Reg<PIO1_13_SPEC>`"]
pub type PIO1_13 = crate::Reg<pio1_13::PIO1_13_SPEC>;
#[doc = "I/O configuration for pin PIO1_13/DTR/CT16B0_MAT0/TXD"]
pub mod pio1_13;
#[doc = "PIO1_14 register accessor: an alias for `Reg<PIO1_14_SPEC>`"]
pub type PIO1_14 = crate::Reg<pio1_14::PIO1_14_SPEC>;
#[doc = "I/O configuration for pin PIO1_14/DSR/CT16B0_MAT1/RXD"]
pub mod pio1_14;
#[doc = "PIO1_15 register accessor: an alias for `Reg<PIO1_15_SPEC>`"]
pub type PIO1_15 = crate::Reg<pio1_15::PIO1_15_SPEC>;
#[doc = "I/O configuration for pin PIO1_15/DCD/ CT16B0_MAT2/SCK1"]
pub mod pio1_15;
#[doc = "PIO1_16 register accessor: an alias for `Reg<PIO1_16_SPEC>`"]
pub type PIO1_16 = crate::Reg<pio1_16::PIO1_16_SPEC>;
#[doc = "I/O configuration for pin PIO1_16/RI/CT16B0_CAP0"]
pub mod pio1_16;
#[doc = "PIO1_17 register accessor: an alias for `Reg<PIO1_17_SPEC>`"]
pub type PIO1_17 = crate::Reg<pio1_17::PIO1_17_SPEC>;
#[doc = "I/O configuration for PIO1_17/CT16B0_CAP1/RXD"]
pub mod pio1_17;
#[doc = "PIO1_18 register accessor: an alias for `Reg<PIO1_18_SPEC>`"]
pub type PIO1_18 = crate::Reg<pio1_18::PIO1_18_SPEC>;
#[doc = "I/O configuration for PIO1_18/CT16B1_CAP1/TXD"]
pub mod pio1_18;
#[doc = "PIO1_19 register accessor: an alias for `Reg<PIO1_19_SPEC>`"]
pub type PIO1_19 = crate::Reg<pio1_19::PIO1_19_SPEC>;
#[doc = "I/O configuration for pin PIO1_19/DTR/SSEL1"]
pub mod pio1_19;
#[doc = "PIO1_20 register accessor: an alias for `Reg<PIO1_20_SPEC>`"]
pub type PIO1_20 = crate::Reg<pio1_20::PIO1_20_SPEC>;
#[doc = "I/O configuration for pin PIO1_20/DSR/SCK1"]
pub mod pio1_20;
#[doc = "PIO1_21 register accessor: an alias for `Reg<PIO1_21_SPEC>`"]
pub type PIO1_21 = crate::Reg<pio1_21::PIO1_21_SPEC>;
#[doc = "I/O configuration for pin PIO1_21/DCD/MISO1"]
pub mod pio1_21;
#[doc = "PIO1_22 register accessor: an alias for `Reg<PIO1_22_SPEC>`"]
pub type PIO1_22 = crate::Reg<pio1_22::PIO1_22_SPEC>;
#[doc = "I/O configuration for pin PIO1_22/RI/MOSI1"]
pub mod pio1_22;
#[doc = "PIO1_23 register accessor: an alias for `Reg<PIO1_23_SPEC>`"]
pub type PIO1_23 = crate::Reg<pio1_23::PIO1_23_SPEC>;
#[doc = "I/O configuration for pin PIO1_23/CT16B1_MAT1/SSEL1"]
pub mod pio1_23;
#[doc = "PIO1_24 register accessor: an alias for `Reg<PIO1_24_SPEC>`"]
pub type PIO1_24 = crate::Reg<pio1_24::PIO1_24_SPEC>;
#[doc = "I/O configuration for pin PIO1_24/ CT32B0_MAT0"]
pub mod pio1_24;
#[doc = "PIO1_25 register accessor: an alias for `Reg<PIO1_25_SPEC>`"]
pub type PIO1_25 = crate::Reg<pio1_25::PIO1_25_SPEC>;
#[doc = "I/O configuration for pin PIO1_25/CT32B0_MAT1"]
pub mod pio1_25;
#[doc = "PIO1_26 register accessor: an alias for `Reg<PIO1_26_SPEC>`"]
pub type PIO1_26 = crate::Reg<pio1_26::PIO1_26_SPEC>;
#[doc = "I/O configuration for pin PIO1_26/CT32B0_MAT2/ RXD"]
pub mod pio1_26;
#[doc = "PIO1_27 register accessor: an alias for `Reg<PIO1_27_SPEC>`"]
pub type PIO1_27 = crate::Reg<pio1_27::PIO1_27_SPEC>;
#[doc = "I/O configuration for pin PIO1_27/CT32B0_MAT3/ TXD"]
pub mod pio1_27;
#[doc = "PIO1_28 register accessor: an alias for `Reg<PIO1_28_SPEC>`"]
pub type PIO1_28 = crate::Reg<pio1_28::PIO1_28_SPEC>;
#[doc = "I/O configuration for pin PIO1_28/CT32B0_CAP0/ SCLK"]
pub mod pio1_28;
#[doc = "PIO1_29 register accessor: an alias for `Reg<PIO1_29_SPEC>`"]
pub type PIO1_29 = crate::Reg<pio1_29::PIO1_29_SPEC>;
#[doc = "I/O configuration for pin PIO1_29/SCK0/ CT32B0_CAP1"]
pub mod pio1_29;
#[doc = "PIO1_31 register accessor: an alias for `Reg<PIO1_31_SPEC>`"]
pub type PIO1_31 = crate::Reg<pio1_31::PIO1_31_SPEC>;
#[doc = "I/O configuration for pin PIO1_31"]
pub mod pio1_31;
