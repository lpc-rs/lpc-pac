#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - I/O configuration for pin RESET/PIO0_0"]
    pub reset_pio0_0: RESET_PIO0_0,
    #[doc = "0x04 - I/O configuration for pin PIO0_1/CLKOUT/CT32B0_MAT2/USB_FTOGGLE"]
    pub pio0_1: PIO0_1,
    #[doc = "0x08 - I/O configuration for pin PIO0_2/SSEL0/CT16B0_CAP0"]
    pub pio0_2: PIO0_2,
    #[doc = "0x0c - I/O configuration for pin PIO0_3/USB_VBUS"]
    pub pio0_3: PIO0_3,
    #[doc = "0x10 - I/O configuration for pin PIO0_4/SCL"]
    pub pio0_4: PIO0_4,
    #[doc = "0x14 - I/O configuration for pin PIO0_5/SDA"]
    pub pio0_5: PIO0_5,
    #[doc = "0x18 - I/O configuration for pin PIO0_6/USB_CONNECT/SCK0"]
    pub pio0_6: PIO0_6,
    #[doc = "0x1c - I/O configuration for pin PIO0_7/CTS"]
    pub pio0_7: PIO0_7,
    #[doc = "0x20 - I/O configuration for pin PIO0_8/MISO0/CT16B0_MAT0"]
    pub pio0_8: PIO0_8,
    #[doc = "0x24 - I/O configuration for pin PIO0_9/MOSI0/CT16B0_MAT1"]
    pub pio0_9: PIO0_9,
    #[doc = "0x28 - I/O configuration for pin SWCLK/PIO0_10/ SCK0/CT16B0_MAT2"]
    pub swclk_pio0_10: SWCLK_PIO0_10,
    #[doc = "0x2c - I/O configuration for pin TDI/PIO0_11/AD0/CT32B0_MAT3"]
    pub tdi_pio0_11: TDI_PIO0_11,
    #[doc = "0x30 - I/O configuration for pin TMS/PIO0_12/AD1/CT32B1_CAP0"]
    pub tms_pio0_12: TMS_PIO0_12,
    #[doc = "0x34 - I/O configuration for pin TDO/PIO0_13/AD2/CT32B1_MAT0"]
    pub tdo_pio0_13: TDO_PIO0_13,
    #[doc = "0x38 - I/O configuration for pin TRST/PIO0_14/AD3/CT32B1_MAT1"]
    pub trst_pio0_14: TRST_PIO0_14,
    #[doc = "0x3c - I/O configuration for pin SWDIO/PIO0_15/AD4/CT32B1_MAT2"]
    pub swdio_pio0_15: SWDIO_PIO0_15,
    #[doc = "0x40 - I/O configuration for pin PIO0_16/AD5/CT32B1_MAT3/ WAKEUP"]
    pub pio0_16: PIO0_16,
    #[doc = "0x44 - I/O configuration for pin PIO0_17/RTS/CT32B0_CAP0/SCLK"]
    pub pio0_17: PIO0_17,
    #[doc = "0x48 - I/O configuration for pin PIO0_18/RXD/CT32B0_MAT0"]
    pub pio0_18: PIO0_18,
    #[doc = "0x4c - I/O configuration for pin PIO0_19/TXD/CT32B0_MAT1"]
    pub pio0_19: PIO0_19,
    #[doc = "0x50 - I/O configuration for pin PIO0_20/CT16B1_CAP0"]
    pub pio0_20: PIO0_20,
    #[doc = "0x54 - I/O configuration for pin PIO0_21/CT16B1_MAT0/MOSI1"]
    pub pio0_21: PIO0_21,
    #[doc = "0x58 - I/O configuration for pin PIO0_22/AD6/CT16B1_MAT1/MISO1"]
    pub pio0_22: PIO0_22,
    #[doc = "0x5c - I/O configuration for pin PIO0_23/AD7"]
    pub pio0_23: PIO0_23,
    #[doc = "0x60 - I/O configuration for pin PIO1_0/CT32B1_MAT0"]
    pub pio1_0: PIO1_0,
    #[doc = "0x64 - I/O configuration for pin PIO1_1/CT32B1_MAT1"]
    pub pio1_1: PIO1_1,
    #[doc = "0x68 - I/O configuration for pin PIO1_2/CT32B1_MAT2"]
    pub pio1_2: PIO1_2,
    #[doc = "0x6c - I/O configuration for pin PIO1_3/CT32B1_MAT3"]
    pub pio1_3: PIO1_3,
    #[doc = "0x70 - I/O configuration for pin PIO1_4/CT32B1_CAP0"]
    pub pio1_4: PIO1_4,
    #[doc = "0x74 - I/O configuration for pin PIO1_5/CT32B1_CAP1"]
    pub pio1_5: PIO1_5,
    #[doc = "0x78 - I/O configuration for pin PIO1_6"]
    pub pio1_6: PIO1_6,
    #[doc = "0x7c - I/O configuration for pin PIO1_7"]
    pub pio1_7: PIO1_7,
    #[doc = "0x80 - I/O configuration for pin PIO1_8"]
    pub pio1_8: PIO1_8,
    #[doc = "0x84 - I/O configuration for pin PIO1_9"]
    pub pio1_9: PIO1_9,
    #[doc = "0x88 - I/O configuration for pin PIO1_10"]
    pub pio1_10: PIO1_10,
    #[doc = "0x8c - I/O configuration for pin PIO1_11"]
    pub pio1_11: PIO1_11,
    #[doc = "0x90 - I/O configuration for pin PIO1_12"]
    pub pio1_12: PIO1_12,
    #[doc = "0x94 - I/O configuration for pin PIO1_13/DTR/CT16B0_MAT0/TXD"]
    pub pio1_13: PIO1_13,
    #[doc = "0x98 - I/O configuration for pin PIO1_14/DSR/CT16B0_MAT1/RXD"]
    pub pio1_14: PIO1_14,
    #[doc = "0x9c - I/O configuration for pin PIO1_15/DCD/ CT16B0_MAT2/SCK1"]
    pub pio1_15: PIO1_15,
    #[doc = "0xa0 - I/O configuration for pin PIO1_16/RI/CT16B0_CAP0"]
    pub pio1_16: PIO1_16,
    #[doc = "0xa4 - I/O configuration for PIO1_17/CT16B0_CAP1/RXD"]
    pub pio1_17: PIO1_17,
    #[doc = "0xa8 - I/O configuration for PIO1_18/CT16B1_CAP1/TXD"]
    pub pio1_18: PIO1_18,
    #[doc = "0xac - I/O configuration for pin PIO1_19/DTR/SSEL1"]
    pub pio1_19: PIO1_19,
    #[doc = "0xb0 - I/O configuration for pin PIO1_20/DSR/SCK1"]
    pub pio1_20: PIO1_20,
    #[doc = "0xb4 - I/O configuration for pin PIO1_21/DCD/MISO1"]
    pub pio1_21: PIO1_21,
    #[doc = "0xb8 - I/O configuration for pin PIO1_22/RI/MOSI1"]
    pub pio1_22: PIO1_22,
    #[doc = "0xbc - I/O configuration for pin PIO1_23/CT16B1_MAT1/SSEL1"]
    pub pio1_23: PIO1_23,
    #[doc = "0xc0 - I/O configuration for pin PIO1_24/ CT32B0_MAT0"]
    pub pio1_24: PIO1_24,
    #[doc = "0xc4 - I/O configuration for pin PIO1_25/CT32B0_MAT1"]
    pub pio1_25: PIO1_25,
    #[doc = "0xc8 - I/O configuration for pin PIO1_26/CT32B0_MAT2/ RXD"]
    pub pio1_26: PIO1_26,
    #[doc = "0xcc - I/O configuration for pin PIO1_27/CT32B0_MAT3/ TXD"]
    pub pio1_27: PIO1_27,
    #[doc = "0xd0 - I/O configuration for pin PIO1_28/CT32B0_CAP0/ SCLK"]
    pub pio1_28: PIO1_28,
    #[doc = "0xd4 - I/O configuration for pin PIO1_29/SCK0/ CT32B0_CAP1"]
    pub pio1_29: PIO1_29,
    _reserved54: [u8; 4usize],
    #[doc = "0xdc - I/O configuration for pin PIO1_31"]
    pub pio1_31: PIO1_31,
}
#[doc = "I/O configuration for pin RESET/PIO0_0"]
pub struct RESET_PIO0_0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I/O configuration for pin RESET/PIO0_0"]
pub mod reset_pio0_0;
#[doc = "I/O configuration for pin PIO0_1/CLKOUT/CT32B0_MAT2/USB_FTOGGLE"]
pub struct PIO0_1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I/O configuration for pin PIO0_1/CLKOUT/CT32B0_MAT2/USB_FTOGGLE"]
pub mod pio0_1;
#[doc = "I/O configuration for pin PIO0_2/SSEL0/CT16B0_CAP0"]
pub struct PIO0_2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I/O configuration for pin PIO0_2/SSEL0/CT16B0_CAP0"]
pub mod pio0_2;
#[doc = "I/O configuration for pin PIO0_3/USB_VBUS"]
pub struct PIO0_3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I/O configuration for pin PIO0_3/USB_VBUS"]
pub mod pio0_3;
#[doc = "I/O configuration for pin PIO0_4/SCL"]
pub struct PIO0_4 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I/O configuration for pin PIO0_4/SCL"]
pub mod pio0_4;
#[doc = "I/O configuration for pin PIO0_5/SDA"]
pub struct PIO0_5 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I/O configuration for pin PIO0_5/SDA"]
pub mod pio0_5;
#[doc = "I/O configuration for pin PIO0_6/USB_CONNECT/SCK0"]
pub struct PIO0_6 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I/O configuration for pin PIO0_6/USB_CONNECT/SCK0"]
pub mod pio0_6;
#[doc = "I/O configuration for pin PIO0_7/CTS"]
pub struct PIO0_7 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I/O configuration for pin PIO0_7/CTS"]
pub mod pio0_7;
#[doc = "I/O configuration for pin PIO0_8/MISO0/CT16B0_MAT0"]
pub struct PIO0_8 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I/O configuration for pin PIO0_8/MISO0/CT16B0_MAT0"]
pub mod pio0_8;
#[doc = "I/O configuration for pin PIO0_9/MOSI0/CT16B0_MAT1"]
pub struct PIO0_9 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I/O configuration for pin PIO0_9/MOSI0/CT16B0_MAT1"]
pub mod pio0_9;
#[doc = "I/O configuration for pin SWCLK/PIO0_10/ SCK0/CT16B0_MAT2"]
pub struct SWCLK_PIO0_10 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I/O configuration for pin SWCLK/PIO0_10/ SCK0/CT16B0_MAT2"]
pub mod swclk_pio0_10;
#[doc = "I/O configuration for pin TDI/PIO0_11/AD0/CT32B0_MAT3"]
pub struct TDI_PIO0_11 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I/O configuration for pin TDI/PIO0_11/AD0/CT32B0_MAT3"]
pub mod tdi_pio0_11;
#[doc = "I/O configuration for pin TMS/PIO0_12/AD1/CT32B1_CAP0"]
pub struct TMS_PIO0_12 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I/O configuration for pin TMS/PIO0_12/AD1/CT32B1_CAP0"]
pub mod tms_pio0_12;
#[doc = "I/O configuration for pin TDO/PIO0_13/AD2/CT32B1_MAT0"]
pub struct TDO_PIO0_13 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I/O configuration for pin TDO/PIO0_13/AD2/CT32B1_MAT0"]
pub mod tdo_pio0_13;
#[doc = "I/O configuration for pin TRST/PIO0_14/AD3/CT32B1_MAT1"]
pub struct TRST_PIO0_14 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I/O configuration for pin TRST/PIO0_14/AD3/CT32B1_MAT1"]
pub mod trst_pio0_14;
#[doc = "I/O configuration for pin SWDIO/PIO0_15/AD4/CT32B1_MAT2"]
pub struct SWDIO_PIO0_15 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I/O configuration for pin SWDIO/PIO0_15/AD4/CT32B1_MAT2"]
pub mod swdio_pio0_15;
#[doc = "I/O configuration for pin PIO0_16/AD5/CT32B1_MAT3/ WAKEUP"]
pub struct PIO0_16 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I/O configuration for pin PIO0_16/AD5/CT32B1_MAT3/ WAKEUP"]
pub mod pio0_16;
#[doc = "I/O configuration for pin PIO0_17/RTS/CT32B0_CAP0/SCLK"]
pub struct PIO0_17 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I/O configuration for pin PIO0_17/RTS/CT32B0_CAP0/SCLK"]
pub mod pio0_17;
#[doc = "I/O configuration for pin PIO0_18/RXD/CT32B0_MAT0"]
pub struct PIO0_18 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I/O configuration for pin PIO0_18/RXD/CT32B0_MAT0"]
pub mod pio0_18;
#[doc = "I/O configuration for pin PIO0_19/TXD/CT32B0_MAT1"]
pub struct PIO0_19 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I/O configuration for pin PIO0_19/TXD/CT32B0_MAT1"]
pub mod pio0_19;
#[doc = "I/O configuration for pin PIO0_20/CT16B1_CAP0"]
pub struct PIO0_20 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I/O configuration for pin PIO0_20/CT16B1_CAP0"]
pub mod pio0_20;
#[doc = "I/O configuration for pin PIO0_21/CT16B1_MAT0/MOSI1"]
pub struct PIO0_21 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I/O configuration for pin PIO0_21/CT16B1_MAT0/MOSI1"]
pub mod pio0_21;
#[doc = "I/O configuration for pin PIO0_22/AD6/CT16B1_MAT1/MISO1"]
pub struct PIO0_22 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I/O configuration for pin PIO0_22/AD6/CT16B1_MAT1/MISO1"]
pub mod pio0_22;
#[doc = "I/O configuration for pin PIO0_23/AD7"]
pub struct PIO0_23 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I/O configuration for pin PIO0_23/AD7"]
pub mod pio0_23;
#[doc = "I/O configuration for pin PIO1_0/CT32B1_MAT0"]
pub struct PIO1_0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I/O configuration for pin PIO1_0/CT32B1_MAT0"]
pub mod pio1_0;
#[doc = "I/O configuration for pin PIO1_1/CT32B1_MAT1"]
pub struct PIO1_1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I/O configuration for pin PIO1_1/CT32B1_MAT1"]
pub mod pio1_1;
#[doc = "I/O configuration for pin PIO1_2/CT32B1_MAT2"]
pub struct PIO1_2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I/O configuration for pin PIO1_2/CT32B1_MAT2"]
pub mod pio1_2;
#[doc = "I/O configuration for pin PIO1_3/CT32B1_MAT3"]
pub struct PIO1_3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I/O configuration for pin PIO1_3/CT32B1_MAT3"]
pub mod pio1_3;
#[doc = "I/O configuration for pin PIO1_4/CT32B1_CAP0"]
pub struct PIO1_4 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I/O configuration for pin PIO1_4/CT32B1_CAP0"]
pub mod pio1_4;
#[doc = "I/O configuration for pin PIO1_5/CT32B1_CAP1"]
pub struct PIO1_5 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I/O configuration for pin PIO1_5/CT32B1_CAP1"]
pub mod pio1_5;
#[doc = "I/O configuration for pin PIO1_6"]
pub struct PIO1_6 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I/O configuration for pin PIO1_6"]
pub mod pio1_6;
#[doc = "I/O configuration for pin PIO1_7"]
pub struct PIO1_7 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I/O configuration for pin PIO1_7"]
pub mod pio1_7;
#[doc = "I/O configuration for pin PIO1_8"]
pub struct PIO1_8 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I/O configuration for pin PIO1_8"]
pub mod pio1_8;
#[doc = "I/O configuration for pin PIO1_9"]
pub struct PIO1_9 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I/O configuration for pin PIO1_9"]
pub mod pio1_9;
#[doc = "I/O configuration for pin PIO1_10"]
pub struct PIO1_10 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I/O configuration for pin PIO1_10"]
pub mod pio1_10;
#[doc = "I/O configuration for pin PIO1_11"]
pub struct PIO1_11 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I/O configuration for pin PIO1_11"]
pub mod pio1_11;
#[doc = "I/O configuration for pin PIO1_12"]
pub struct PIO1_12 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I/O configuration for pin PIO1_12"]
pub mod pio1_12;
#[doc = "I/O configuration for pin PIO1_13/DTR/CT16B0_MAT0/TXD"]
pub struct PIO1_13 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I/O configuration for pin PIO1_13/DTR/CT16B0_MAT0/TXD"]
pub mod pio1_13;
#[doc = "I/O configuration for pin PIO1_14/DSR/CT16B0_MAT1/RXD"]
pub struct PIO1_14 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I/O configuration for pin PIO1_14/DSR/CT16B0_MAT1/RXD"]
pub mod pio1_14;
#[doc = "I/O configuration for pin PIO1_15/DCD/ CT16B0_MAT2/SCK1"]
pub struct PIO1_15 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I/O configuration for pin PIO1_15/DCD/ CT16B0_MAT2/SCK1"]
pub mod pio1_15;
#[doc = "I/O configuration for pin PIO1_16/RI/CT16B0_CAP0"]
pub struct PIO1_16 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I/O configuration for pin PIO1_16/RI/CT16B0_CAP0"]
pub mod pio1_16;
#[doc = "I/O configuration for PIO1_17/CT16B0_CAP1/RXD"]
pub struct PIO1_17 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I/O configuration for PIO1_17/CT16B0_CAP1/RXD"]
pub mod pio1_17;
#[doc = "I/O configuration for PIO1_18/CT16B1_CAP1/TXD"]
pub struct PIO1_18 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I/O configuration for PIO1_18/CT16B1_CAP1/TXD"]
pub mod pio1_18;
#[doc = "I/O configuration for pin PIO1_19/DTR/SSEL1"]
pub struct PIO1_19 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I/O configuration for pin PIO1_19/DTR/SSEL1"]
pub mod pio1_19;
#[doc = "I/O configuration for pin PIO1_20/DSR/SCK1"]
pub struct PIO1_20 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I/O configuration for pin PIO1_20/DSR/SCK1"]
pub mod pio1_20;
#[doc = "I/O configuration for pin PIO1_21/DCD/MISO1"]
pub struct PIO1_21 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I/O configuration for pin PIO1_21/DCD/MISO1"]
pub mod pio1_21;
#[doc = "I/O configuration for pin PIO1_22/RI/MOSI1"]
pub struct PIO1_22 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I/O configuration for pin PIO1_22/RI/MOSI1"]
pub mod pio1_22;
#[doc = "I/O configuration for pin PIO1_23/CT16B1_MAT1/SSEL1"]
pub struct PIO1_23 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I/O configuration for pin PIO1_23/CT16B1_MAT1/SSEL1"]
pub mod pio1_23;
#[doc = "I/O configuration for pin PIO1_24/ CT32B0_MAT0"]
pub struct PIO1_24 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I/O configuration for pin PIO1_24/ CT32B0_MAT0"]
pub mod pio1_24;
#[doc = "I/O configuration for pin PIO1_25/CT32B0_MAT1"]
pub struct PIO1_25 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I/O configuration for pin PIO1_25/CT32B0_MAT1"]
pub mod pio1_25;
#[doc = "I/O configuration for pin PIO1_26/CT32B0_MAT2/ RXD"]
pub struct PIO1_26 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I/O configuration for pin PIO1_26/CT32B0_MAT2/ RXD"]
pub mod pio1_26;
#[doc = "I/O configuration for pin PIO1_27/CT32B0_MAT3/ TXD"]
pub struct PIO1_27 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I/O configuration for pin PIO1_27/CT32B0_MAT3/ TXD"]
pub mod pio1_27;
#[doc = "I/O configuration for pin PIO1_28/CT32B0_CAP0/ SCLK"]
pub struct PIO1_28 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I/O configuration for pin PIO1_28/CT32B0_CAP0/ SCLK"]
pub mod pio1_28;
#[doc = "I/O configuration for pin PIO1_29/SCK0/ CT32B0_CAP1"]
pub struct PIO1_29 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I/O configuration for pin PIO1_29/SCK0/ CT32B0_CAP1"]
pub mod pio1_29;
#[doc = "I/O configuration for pin PIO1_31"]
pub struct PIO1_31 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I/O configuration for pin PIO1_31"]
pub mod pio1_31;
