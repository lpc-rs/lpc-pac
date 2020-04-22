#[doc = r"Register block"]
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
#[doc = "I/O configuration for pin RESET/PIO0_0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [reset_pio0_0](reset_pio0_0) module"]
pub type RESET_PIO0_0 = crate::Reg<u32, _RESET_PIO0_0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RESET_PIO0_0;
#[doc = "`read()` method returns [reset_pio0_0::R](reset_pio0_0::R) reader structure"]
impl crate::Readable for RESET_PIO0_0 {}
#[doc = "`write(|w| ..)` method takes [reset_pio0_0::W](reset_pio0_0::W) writer structure"]
impl crate::Writable for RESET_PIO0_0 {}
#[doc = "I/O configuration for pin RESET/PIO0_0"]
pub mod reset_pio0_0;
#[doc = "I/O configuration for pin PIO0_1/CLKOUT/CT32B0_MAT2/USB_FTOGGLE\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pio0_1](pio0_1) module"]
pub type PIO0_1 = crate::Reg<u32, _PIO0_1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIO0_1;
#[doc = "`read()` method returns [pio0_1::R](pio0_1::R) reader structure"]
impl crate::Readable for PIO0_1 {}
#[doc = "`write(|w| ..)` method takes [pio0_1::W](pio0_1::W) writer structure"]
impl crate::Writable for PIO0_1 {}
#[doc = "I/O configuration for pin PIO0_1/CLKOUT/CT32B0_MAT2/USB_FTOGGLE"]
pub mod pio0_1;
#[doc = "I/O configuration for pin PIO0_2/SSEL0/CT16B0_CAP0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pio0_2](pio0_2) module"]
pub type PIO0_2 = crate::Reg<u32, _PIO0_2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIO0_2;
#[doc = "`read()` method returns [pio0_2::R](pio0_2::R) reader structure"]
impl crate::Readable for PIO0_2 {}
#[doc = "`write(|w| ..)` method takes [pio0_2::W](pio0_2::W) writer structure"]
impl crate::Writable for PIO0_2 {}
#[doc = "I/O configuration for pin PIO0_2/SSEL0/CT16B0_CAP0"]
pub mod pio0_2;
#[doc = "I/O configuration for pin PIO0_3/USB_VBUS\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pio0_3](pio0_3) module"]
pub type PIO0_3 = crate::Reg<u32, _PIO0_3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIO0_3;
#[doc = "`read()` method returns [pio0_3::R](pio0_3::R) reader structure"]
impl crate::Readable for PIO0_3 {}
#[doc = "`write(|w| ..)` method takes [pio0_3::W](pio0_3::W) writer structure"]
impl crate::Writable for PIO0_3 {}
#[doc = "I/O configuration for pin PIO0_3/USB_VBUS"]
pub mod pio0_3;
#[doc = "I/O configuration for pin PIO0_4/SCL\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pio0_4](pio0_4) module"]
pub type PIO0_4 = crate::Reg<u32, _PIO0_4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIO0_4;
#[doc = "`read()` method returns [pio0_4::R](pio0_4::R) reader structure"]
impl crate::Readable for PIO0_4 {}
#[doc = "`write(|w| ..)` method takes [pio0_4::W](pio0_4::W) writer structure"]
impl crate::Writable for PIO0_4 {}
#[doc = "I/O configuration for pin PIO0_4/SCL"]
pub mod pio0_4;
#[doc = "I/O configuration for pin PIO0_5/SDA\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pio0_5](pio0_5) module"]
pub type PIO0_5 = crate::Reg<u32, _PIO0_5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIO0_5;
#[doc = "`read()` method returns [pio0_5::R](pio0_5::R) reader structure"]
impl crate::Readable for PIO0_5 {}
#[doc = "`write(|w| ..)` method takes [pio0_5::W](pio0_5::W) writer structure"]
impl crate::Writable for PIO0_5 {}
#[doc = "I/O configuration for pin PIO0_5/SDA"]
pub mod pio0_5;
#[doc = "I/O configuration for pin PIO0_6/USB_CONNECT/SCK0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pio0_6](pio0_6) module"]
pub type PIO0_6 = crate::Reg<u32, _PIO0_6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIO0_6;
#[doc = "`read()` method returns [pio0_6::R](pio0_6::R) reader structure"]
impl crate::Readable for PIO0_6 {}
#[doc = "`write(|w| ..)` method takes [pio0_6::W](pio0_6::W) writer structure"]
impl crate::Writable for PIO0_6 {}
#[doc = "I/O configuration for pin PIO0_6/USB_CONNECT/SCK0"]
pub mod pio0_6;
#[doc = "I/O configuration for pin PIO0_7/CTS\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pio0_7](pio0_7) module"]
pub type PIO0_7 = crate::Reg<u32, _PIO0_7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIO0_7;
#[doc = "`read()` method returns [pio0_7::R](pio0_7::R) reader structure"]
impl crate::Readable for PIO0_7 {}
#[doc = "`write(|w| ..)` method takes [pio0_7::W](pio0_7::W) writer structure"]
impl crate::Writable for PIO0_7 {}
#[doc = "I/O configuration for pin PIO0_7/CTS"]
pub mod pio0_7;
#[doc = "I/O configuration for pin PIO0_8/MISO0/CT16B0_MAT0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pio0_8](pio0_8) module"]
pub type PIO0_8 = crate::Reg<u32, _PIO0_8>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIO0_8;
#[doc = "`read()` method returns [pio0_8::R](pio0_8::R) reader structure"]
impl crate::Readable for PIO0_8 {}
#[doc = "`write(|w| ..)` method takes [pio0_8::W](pio0_8::W) writer structure"]
impl crate::Writable for PIO0_8 {}
#[doc = "I/O configuration for pin PIO0_8/MISO0/CT16B0_MAT0"]
pub mod pio0_8;
#[doc = "I/O configuration for pin PIO0_9/MOSI0/CT16B0_MAT1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pio0_9](pio0_9) module"]
pub type PIO0_9 = crate::Reg<u32, _PIO0_9>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIO0_9;
#[doc = "`read()` method returns [pio0_9::R](pio0_9::R) reader structure"]
impl crate::Readable for PIO0_9 {}
#[doc = "`write(|w| ..)` method takes [pio0_9::W](pio0_9::W) writer structure"]
impl crate::Writable for PIO0_9 {}
#[doc = "I/O configuration for pin PIO0_9/MOSI0/CT16B0_MAT1"]
pub mod pio0_9;
#[doc = "I/O configuration for pin SWCLK/PIO0_10/ SCK0/CT16B0_MAT2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [swclk_pio0_10](swclk_pio0_10) module"]
pub type SWCLK_PIO0_10 = crate::Reg<u32, _SWCLK_PIO0_10>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SWCLK_PIO0_10;
#[doc = "`read()` method returns [swclk_pio0_10::R](swclk_pio0_10::R) reader structure"]
impl crate::Readable for SWCLK_PIO0_10 {}
#[doc = "`write(|w| ..)` method takes [swclk_pio0_10::W](swclk_pio0_10::W) writer structure"]
impl crate::Writable for SWCLK_PIO0_10 {}
#[doc = "I/O configuration for pin SWCLK/PIO0_10/ SCK0/CT16B0_MAT2"]
pub mod swclk_pio0_10;
#[doc = "I/O configuration for pin TDI/PIO0_11/AD0/CT32B0_MAT3\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tdi_pio0_11](tdi_pio0_11) module"]
pub type TDI_PIO0_11 = crate::Reg<u32, _TDI_PIO0_11>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TDI_PIO0_11;
#[doc = "`read()` method returns [tdi_pio0_11::R](tdi_pio0_11::R) reader structure"]
impl crate::Readable for TDI_PIO0_11 {}
#[doc = "`write(|w| ..)` method takes [tdi_pio0_11::W](tdi_pio0_11::W) writer structure"]
impl crate::Writable for TDI_PIO0_11 {}
#[doc = "I/O configuration for pin TDI/PIO0_11/AD0/CT32B0_MAT3"]
pub mod tdi_pio0_11;
#[doc = "I/O configuration for pin TMS/PIO0_12/AD1/CT32B1_CAP0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tms_pio0_12](tms_pio0_12) module"]
pub type TMS_PIO0_12 = crate::Reg<u32, _TMS_PIO0_12>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TMS_PIO0_12;
#[doc = "`read()` method returns [tms_pio0_12::R](tms_pio0_12::R) reader structure"]
impl crate::Readable for TMS_PIO0_12 {}
#[doc = "`write(|w| ..)` method takes [tms_pio0_12::W](tms_pio0_12::W) writer structure"]
impl crate::Writable for TMS_PIO0_12 {}
#[doc = "I/O configuration for pin TMS/PIO0_12/AD1/CT32B1_CAP0"]
pub mod tms_pio0_12;
#[doc = "I/O configuration for pin TDO/PIO0_13/AD2/CT32B1_MAT0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tdo_pio0_13](tdo_pio0_13) module"]
pub type TDO_PIO0_13 = crate::Reg<u32, _TDO_PIO0_13>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TDO_PIO0_13;
#[doc = "`read()` method returns [tdo_pio0_13::R](tdo_pio0_13::R) reader structure"]
impl crate::Readable for TDO_PIO0_13 {}
#[doc = "`write(|w| ..)` method takes [tdo_pio0_13::W](tdo_pio0_13::W) writer structure"]
impl crate::Writable for TDO_PIO0_13 {}
#[doc = "I/O configuration for pin TDO/PIO0_13/AD2/CT32B1_MAT0"]
pub mod tdo_pio0_13;
#[doc = "I/O configuration for pin TRST/PIO0_14/AD3/CT32B1_MAT1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [trst_pio0_14](trst_pio0_14) module"]
pub type TRST_PIO0_14 = crate::Reg<u32, _TRST_PIO0_14>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TRST_PIO0_14;
#[doc = "`read()` method returns [trst_pio0_14::R](trst_pio0_14::R) reader structure"]
impl crate::Readable for TRST_PIO0_14 {}
#[doc = "`write(|w| ..)` method takes [trst_pio0_14::W](trst_pio0_14::W) writer structure"]
impl crate::Writable for TRST_PIO0_14 {}
#[doc = "I/O configuration for pin TRST/PIO0_14/AD3/CT32B1_MAT1"]
pub mod trst_pio0_14;
#[doc = "I/O configuration for pin SWDIO/PIO0_15/AD4/CT32B1_MAT2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [swdio_pio0_15](swdio_pio0_15) module"]
pub type SWDIO_PIO0_15 = crate::Reg<u32, _SWDIO_PIO0_15>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SWDIO_PIO0_15;
#[doc = "`read()` method returns [swdio_pio0_15::R](swdio_pio0_15::R) reader structure"]
impl crate::Readable for SWDIO_PIO0_15 {}
#[doc = "`write(|w| ..)` method takes [swdio_pio0_15::W](swdio_pio0_15::W) writer structure"]
impl crate::Writable for SWDIO_PIO0_15 {}
#[doc = "I/O configuration for pin SWDIO/PIO0_15/AD4/CT32B1_MAT2"]
pub mod swdio_pio0_15;
#[doc = "I/O configuration for pin PIO0_16/AD5/CT32B1_MAT3/ WAKEUP\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pio0_16](pio0_16) module"]
pub type PIO0_16 = crate::Reg<u32, _PIO0_16>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIO0_16;
#[doc = "`read()` method returns [pio0_16::R](pio0_16::R) reader structure"]
impl crate::Readable for PIO0_16 {}
#[doc = "`write(|w| ..)` method takes [pio0_16::W](pio0_16::W) writer structure"]
impl crate::Writable for PIO0_16 {}
#[doc = "I/O configuration for pin PIO0_16/AD5/CT32B1_MAT3/ WAKEUP"]
pub mod pio0_16;
#[doc = "I/O configuration for pin PIO0_17/RTS/CT32B0_CAP0/SCLK\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pio0_17](pio0_17) module"]
pub type PIO0_17 = crate::Reg<u32, _PIO0_17>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIO0_17;
#[doc = "`read()` method returns [pio0_17::R](pio0_17::R) reader structure"]
impl crate::Readable for PIO0_17 {}
#[doc = "`write(|w| ..)` method takes [pio0_17::W](pio0_17::W) writer structure"]
impl crate::Writable for PIO0_17 {}
#[doc = "I/O configuration for pin PIO0_17/RTS/CT32B0_CAP0/SCLK"]
pub mod pio0_17;
#[doc = "I/O configuration for pin PIO0_18/RXD/CT32B0_MAT0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pio0_18](pio0_18) module"]
pub type PIO0_18 = crate::Reg<u32, _PIO0_18>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIO0_18;
#[doc = "`read()` method returns [pio0_18::R](pio0_18::R) reader structure"]
impl crate::Readable for PIO0_18 {}
#[doc = "`write(|w| ..)` method takes [pio0_18::W](pio0_18::W) writer structure"]
impl crate::Writable for PIO0_18 {}
#[doc = "I/O configuration for pin PIO0_18/RXD/CT32B0_MAT0"]
pub mod pio0_18;
#[doc = "I/O configuration for pin PIO0_19/TXD/CT32B0_MAT1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pio0_19](pio0_19) module"]
pub type PIO0_19 = crate::Reg<u32, _PIO0_19>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIO0_19;
#[doc = "`read()` method returns [pio0_19::R](pio0_19::R) reader structure"]
impl crate::Readable for PIO0_19 {}
#[doc = "`write(|w| ..)` method takes [pio0_19::W](pio0_19::W) writer structure"]
impl crate::Writable for PIO0_19 {}
#[doc = "I/O configuration for pin PIO0_19/TXD/CT32B0_MAT1"]
pub mod pio0_19;
#[doc = "I/O configuration for pin PIO0_20/CT16B1_CAP0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pio0_20](pio0_20) module"]
pub type PIO0_20 = crate::Reg<u32, _PIO0_20>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIO0_20;
#[doc = "`read()` method returns [pio0_20::R](pio0_20::R) reader structure"]
impl crate::Readable for PIO0_20 {}
#[doc = "`write(|w| ..)` method takes [pio0_20::W](pio0_20::W) writer structure"]
impl crate::Writable for PIO0_20 {}
#[doc = "I/O configuration for pin PIO0_20/CT16B1_CAP0"]
pub mod pio0_20;
#[doc = "I/O configuration for pin PIO0_21/CT16B1_MAT0/MOSI1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pio0_21](pio0_21) module"]
pub type PIO0_21 = crate::Reg<u32, _PIO0_21>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIO0_21;
#[doc = "`read()` method returns [pio0_21::R](pio0_21::R) reader structure"]
impl crate::Readable for PIO0_21 {}
#[doc = "`write(|w| ..)` method takes [pio0_21::W](pio0_21::W) writer structure"]
impl crate::Writable for PIO0_21 {}
#[doc = "I/O configuration for pin PIO0_21/CT16B1_MAT0/MOSI1"]
pub mod pio0_21;
#[doc = "I/O configuration for pin PIO0_22/AD6/CT16B1_MAT1/MISO1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pio0_22](pio0_22) module"]
pub type PIO0_22 = crate::Reg<u32, _PIO0_22>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIO0_22;
#[doc = "`read()` method returns [pio0_22::R](pio0_22::R) reader structure"]
impl crate::Readable for PIO0_22 {}
#[doc = "`write(|w| ..)` method takes [pio0_22::W](pio0_22::W) writer structure"]
impl crate::Writable for PIO0_22 {}
#[doc = "I/O configuration for pin PIO0_22/AD6/CT16B1_MAT1/MISO1"]
pub mod pio0_22;
#[doc = "I/O configuration for pin PIO0_23/AD7\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pio0_23](pio0_23) module"]
pub type PIO0_23 = crate::Reg<u32, _PIO0_23>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIO0_23;
#[doc = "`read()` method returns [pio0_23::R](pio0_23::R) reader structure"]
impl crate::Readable for PIO0_23 {}
#[doc = "`write(|w| ..)` method takes [pio0_23::W](pio0_23::W) writer structure"]
impl crate::Writable for PIO0_23 {}
#[doc = "I/O configuration for pin PIO0_23/AD7"]
pub mod pio0_23;
#[doc = "I/O configuration for pin PIO1_0/CT32B1_MAT0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pio1_0](pio1_0) module"]
pub type PIO1_0 = crate::Reg<u32, _PIO1_0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIO1_0;
#[doc = "`read()` method returns [pio1_0::R](pio1_0::R) reader structure"]
impl crate::Readable for PIO1_0 {}
#[doc = "`write(|w| ..)` method takes [pio1_0::W](pio1_0::W) writer structure"]
impl crate::Writable for PIO1_0 {}
#[doc = "I/O configuration for pin PIO1_0/CT32B1_MAT0"]
pub mod pio1_0;
#[doc = "I/O configuration for pin PIO1_1/CT32B1_MAT1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pio1_1](pio1_1) module"]
pub type PIO1_1 = crate::Reg<u32, _PIO1_1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIO1_1;
#[doc = "`read()` method returns [pio1_1::R](pio1_1::R) reader structure"]
impl crate::Readable for PIO1_1 {}
#[doc = "`write(|w| ..)` method takes [pio1_1::W](pio1_1::W) writer structure"]
impl crate::Writable for PIO1_1 {}
#[doc = "I/O configuration for pin PIO1_1/CT32B1_MAT1"]
pub mod pio1_1;
#[doc = "I/O configuration for pin PIO1_2/CT32B1_MAT2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pio1_2](pio1_2) module"]
pub type PIO1_2 = crate::Reg<u32, _PIO1_2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIO1_2;
#[doc = "`read()` method returns [pio1_2::R](pio1_2::R) reader structure"]
impl crate::Readable for PIO1_2 {}
#[doc = "`write(|w| ..)` method takes [pio1_2::W](pio1_2::W) writer structure"]
impl crate::Writable for PIO1_2 {}
#[doc = "I/O configuration for pin PIO1_2/CT32B1_MAT2"]
pub mod pio1_2;
#[doc = "I/O configuration for pin PIO1_3/CT32B1_MAT3\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pio1_3](pio1_3) module"]
pub type PIO1_3 = crate::Reg<u32, _PIO1_3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIO1_3;
#[doc = "`read()` method returns [pio1_3::R](pio1_3::R) reader structure"]
impl crate::Readable for PIO1_3 {}
#[doc = "`write(|w| ..)` method takes [pio1_3::W](pio1_3::W) writer structure"]
impl crate::Writable for PIO1_3 {}
#[doc = "I/O configuration for pin PIO1_3/CT32B1_MAT3"]
pub mod pio1_3;
#[doc = "I/O configuration for pin PIO1_4/CT32B1_CAP0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pio1_4](pio1_4) module"]
pub type PIO1_4 = crate::Reg<u32, _PIO1_4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIO1_4;
#[doc = "`read()` method returns [pio1_4::R](pio1_4::R) reader structure"]
impl crate::Readable for PIO1_4 {}
#[doc = "`write(|w| ..)` method takes [pio1_4::W](pio1_4::W) writer structure"]
impl crate::Writable for PIO1_4 {}
#[doc = "I/O configuration for pin PIO1_4/CT32B1_CAP0"]
pub mod pio1_4;
#[doc = "I/O configuration for pin PIO1_5/CT32B1_CAP1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pio1_5](pio1_5) module"]
pub type PIO1_5 = crate::Reg<u32, _PIO1_5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIO1_5;
#[doc = "`read()` method returns [pio1_5::R](pio1_5::R) reader structure"]
impl crate::Readable for PIO1_5 {}
#[doc = "`write(|w| ..)` method takes [pio1_5::W](pio1_5::W) writer structure"]
impl crate::Writable for PIO1_5 {}
#[doc = "I/O configuration for pin PIO1_5/CT32B1_CAP1"]
pub mod pio1_5;
#[doc = "I/O configuration for pin PIO1_6\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pio1_6](pio1_6) module"]
pub type PIO1_6 = crate::Reg<u32, _PIO1_6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIO1_6;
#[doc = "`read()` method returns [pio1_6::R](pio1_6::R) reader structure"]
impl crate::Readable for PIO1_6 {}
#[doc = "`write(|w| ..)` method takes [pio1_6::W](pio1_6::W) writer structure"]
impl crate::Writable for PIO1_6 {}
#[doc = "I/O configuration for pin PIO1_6"]
pub mod pio1_6;
#[doc = "I/O configuration for pin PIO1_7\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pio1_7](pio1_7) module"]
pub type PIO1_7 = crate::Reg<u32, _PIO1_7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIO1_7;
#[doc = "`read()` method returns [pio1_7::R](pio1_7::R) reader structure"]
impl crate::Readable for PIO1_7 {}
#[doc = "`write(|w| ..)` method takes [pio1_7::W](pio1_7::W) writer structure"]
impl crate::Writable for PIO1_7 {}
#[doc = "I/O configuration for pin PIO1_7"]
pub mod pio1_7;
#[doc = "I/O configuration for pin PIO1_8\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pio1_8](pio1_8) module"]
pub type PIO1_8 = crate::Reg<u32, _PIO1_8>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIO1_8;
#[doc = "`read()` method returns [pio1_8::R](pio1_8::R) reader structure"]
impl crate::Readable for PIO1_8 {}
#[doc = "`write(|w| ..)` method takes [pio1_8::W](pio1_8::W) writer structure"]
impl crate::Writable for PIO1_8 {}
#[doc = "I/O configuration for pin PIO1_8"]
pub mod pio1_8;
#[doc = "I/O configuration for pin PIO1_9\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pio1_9](pio1_9) module"]
pub type PIO1_9 = crate::Reg<u32, _PIO1_9>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIO1_9;
#[doc = "`read()` method returns [pio1_9::R](pio1_9::R) reader structure"]
impl crate::Readable for PIO1_9 {}
#[doc = "`write(|w| ..)` method takes [pio1_9::W](pio1_9::W) writer structure"]
impl crate::Writable for PIO1_9 {}
#[doc = "I/O configuration for pin PIO1_9"]
pub mod pio1_9;
#[doc = "I/O configuration for pin PIO1_10\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pio1_10](pio1_10) module"]
pub type PIO1_10 = crate::Reg<u32, _PIO1_10>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIO1_10;
#[doc = "`read()` method returns [pio1_10::R](pio1_10::R) reader structure"]
impl crate::Readable for PIO1_10 {}
#[doc = "`write(|w| ..)` method takes [pio1_10::W](pio1_10::W) writer structure"]
impl crate::Writable for PIO1_10 {}
#[doc = "I/O configuration for pin PIO1_10"]
pub mod pio1_10;
#[doc = "I/O configuration for pin PIO1_11\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pio1_11](pio1_11) module"]
pub type PIO1_11 = crate::Reg<u32, _PIO1_11>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIO1_11;
#[doc = "`read()` method returns [pio1_11::R](pio1_11::R) reader structure"]
impl crate::Readable for PIO1_11 {}
#[doc = "`write(|w| ..)` method takes [pio1_11::W](pio1_11::W) writer structure"]
impl crate::Writable for PIO1_11 {}
#[doc = "I/O configuration for pin PIO1_11"]
pub mod pio1_11;
#[doc = "I/O configuration for pin PIO1_12\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pio1_12](pio1_12) module"]
pub type PIO1_12 = crate::Reg<u32, _PIO1_12>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIO1_12;
#[doc = "`read()` method returns [pio1_12::R](pio1_12::R) reader structure"]
impl crate::Readable for PIO1_12 {}
#[doc = "`write(|w| ..)` method takes [pio1_12::W](pio1_12::W) writer structure"]
impl crate::Writable for PIO1_12 {}
#[doc = "I/O configuration for pin PIO1_12"]
pub mod pio1_12;
#[doc = "I/O configuration for pin PIO1_13/DTR/CT16B0_MAT0/TXD\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pio1_13](pio1_13) module"]
pub type PIO1_13 = crate::Reg<u32, _PIO1_13>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIO1_13;
#[doc = "`read()` method returns [pio1_13::R](pio1_13::R) reader structure"]
impl crate::Readable for PIO1_13 {}
#[doc = "`write(|w| ..)` method takes [pio1_13::W](pio1_13::W) writer structure"]
impl crate::Writable for PIO1_13 {}
#[doc = "I/O configuration for pin PIO1_13/DTR/CT16B0_MAT0/TXD"]
pub mod pio1_13;
#[doc = "I/O configuration for pin PIO1_14/DSR/CT16B0_MAT1/RXD\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pio1_14](pio1_14) module"]
pub type PIO1_14 = crate::Reg<u32, _PIO1_14>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIO1_14;
#[doc = "`read()` method returns [pio1_14::R](pio1_14::R) reader structure"]
impl crate::Readable for PIO1_14 {}
#[doc = "`write(|w| ..)` method takes [pio1_14::W](pio1_14::W) writer structure"]
impl crate::Writable for PIO1_14 {}
#[doc = "I/O configuration for pin PIO1_14/DSR/CT16B0_MAT1/RXD"]
pub mod pio1_14;
#[doc = "I/O configuration for pin PIO1_15/DCD/ CT16B0_MAT2/SCK1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pio1_15](pio1_15) module"]
pub type PIO1_15 = crate::Reg<u32, _PIO1_15>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIO1_15;
#[doc = "`read()` method returns [pio1_15::R](pio1_15::R) reader structure"]
impl crate::Readable for PIO1_15 {}
#[doc = "`write(|w| ..)` method takes [pio1_15::W](pio1_15::W) writer structure"]
impl crate::Writable for PIO1_15 {}
#[doc = "I/O configuration for pin PIO1_15/DCD/ CT16B0_MAT2/SCK1"]
pub mod pio1_15;
#[doc = "I/O configuration for pin PIO1_16/RI/CT16B0_CAP0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pio1_16](pio1_16) module"]
pub type PIO1_16 = crate::Reg<u32, _PIO1_16>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIO1_16;
#[doc = "`read()` method returns [pio1_16::R](pio1_16::R) reader structure"]
impl crate::Readable for PIO1_16 {}
#[doc = "`write(|w| ..)` method takes [pio1_16::W](pio1_16::W) writer structure"]
impl crate::Writable for PIO1_16 {}
#[doc = "I/O configuration for pin PIO1_16/RI/CT16B0_CAP0"]
pub mod pio1_16;
#[doc = "I/O configuration for PIO1_17/CT16B0_CAP1/RXD\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pio1_17](pio1_17) module"]
pub type PIO1_17 = crate::Reg<u32, _PIO1_17>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIO1_17;
#[doc = "`read()` method returns [pio1_17::R](pio1_17::R) reader structure"]
impl crate::Readable for PIO1_17 {}
#[doc = "`write(|w| ..)` method takes [pio1_17::W](pio1_17::W) writer structure"]
impl crate::Writable for PIO1_17 {}
#[doc = "I/O configuration for PIO1_17/CT16B0_CAP1/RXD"]
pub mod pio1_17;
#[doc = "I/O configuration for PIO1_18/CT16B1_CAP1/TXD\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pio1_18](pio1_18) module"]
pub type PIO1_18 = crate::Reg<u32, _PIO1_18>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIO1_18;
#[doc = "`read()` method returns [pio1_18::R](pio1_18::R) reader structure"]
impl crate::Readable for PIO1_18 {}
#[doc = "`write(|w| ..)` method takes [pio1_18::W](pio1_18::W) writer structure"]
impl crate::Writable for PIO1_18 {}
#[doc = "I/O configuration for PIO1_18/CT16B1_CAP1/TXD"]
pub mod pio1_18;
#[doc = "I/O configuration for pin PIO1_19/DTR/SSEL1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pio1_19](pio1_19) module"]
pub type PIO1_19 = crate::Reg<u32, _PIO1_19>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIO1_19;
#[doc = "`read()` method returns [pio1_19::R](pio1_19::R) reader structure"]
impl crate::Readable for PIO1_19 {}
#[doc = "`write(|w| ..)` method takes [pio1_19::W](pio1_19::W) writer structure"]
impl crate::Writable for PIO1_19 {}
#[doc = "I/O configuration for pin PIO1_19/DTR/SSEL1"]
pub mod pio1_19;
#[doc = "I/O configuration for pin PIO1_20/DSR/SCK1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pio1_20](pio1_20) module"]
pub type PIO1_20 = crate::Reg<u32, _PIO1_20>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIO1_20;
#[doc = "`read()` method returns [pio1_20::R](pio1_20::R) reader structure"]
impl crate::Readable for PIO1_20 {}
#[doc = "`write(|w| ..)` method takes [pio1_20::W](pio1_20::W) writer structure"]
impl crate::Writable for PIO1_20 {}
#[doc = "I/O configuration for pin PIO1_20/DSR/SCK1"]
pub mod pio1_20;
#[doc = "I/O configuration for pin PIO1_21/DCD/MISO1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pio1_21](pio1_21) module"]
pub type PIO1_21 = crate::Reg<u32, _PIO1_21>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIO1_21;
#[doc = "`read()` method returns [pio1_21::R](pio1_21::R) reader structure"]
impl crate::Readable for PIO1_21 {}
#[doc = "`write(|w| ..)` method takes [pio1_21::W](pio1_21::W) writer structure"]
impl crate::Writable for PIO1_21 {}
#[doc = "I/O configuration for pin PIO1_21/DCD/MISO1"]
pub mod pio1_21;
#[doc = "I/O configuration for pin PIO1_22/RI/MOSI1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pio1_22](pio1_22) module"]
pub type PIO1_22 = crate::Reg<u32, _PIO1_22>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIO1_22;
#[doc = "`read()` method returns [pio1_22::R](pio1_22::R) reader structure"]
impl crate::Readable for PIO1_22 {}
#[doc = "`write(|w| ..)` method takes [pio1_22::W](pio1_22::W) writer structure"]
impl crate::Writable for PIO1_22 {}
#[doc = "I/O configuration for pin PIO1_22/RI/MOSI1"]
pub mod pio1_22;
#[doc = "I/O configuration for pin PIO1_23/CT16B1_MAT1/SSEL1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pio1_23](pio1_23) module"]
pub type PIO1_23 = crate::Reg<u32, _PIO1_23>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIO1_23;
#[doc = "`read()` method returns [pio1_23::R](pio1_23::R) reader structure"]
impl crate::Readable for PIO1_23 {}
#[doc = "`write(|w| ..)` method takes [pio1_23::W](pio1_23::W) writer structure"]
impl crate::Writable for PIO1_23 {}
#[doc = "I/O configuration for pin PIO1_23/CT16B1_MAT1/SSEL1"]
pub mod pio1_23;
#[doc = "I/O configuration for pin PIO1_24/ CT32B0_MAT0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pio1_24](pio1_24) module"]
pub type PIO1_24 = crate::Reg<u32, _PIO1_24>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIO1_24;
#[doc = "`read()` method returns [pio1_24::R](pio1_24::R) reader structure"]
impl crate::Readable for PIO1_24 {}
#[doc = "`write(|w| ..)` method takes [pio1_24::W](pio1_24::W) writer structure"]
impl crate::Writable for PIO1_24 {}
#[doc = "I/O configuration for pin PIO1_24/ CT32B0_MAT0"]
pub mod pio1_24;
#[doc = "I/O configuration for pin PIO1_25/CT32B0_MAT1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pio1_25](pio1_25) module"]
pub type PIO1_25 = crate::Reg<u32, _PIO1_25>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIO1_25;
#[doc = "`read()` method returns [pio1_25::R](pio1_25::R) reader structure"]
impl crate::Readable for PIO1_25 {}
#[doc = "`write(|w| ..)` method takes [pio1_25::W](pio1_25::W) writer structure"]
impl crate::Writable for PIO1_25 {}
#[doc = "I/O configuration for pin PIO1_25/CT32B0_MAT1"]
pub mod pio1_25;
#[doc = "I/O configuration for pin PIO1_26/CT32B0_MAT2/ RXD\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pio1_26](pio1_26) module"]
pub type PIO1_26 = crate::Reg<u32, _PIO1_26>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIO1_26;
#[doc = "`read()` method returns [pio1_26::R](pio1_26::R) reader structure"]
impl crate::Readable for PIO1_26 {}
#[doc = "`write(|w| ..)` method takes [pio1_26::W](pio1_26::W) writer structure"]
impl crate::Writable for PIO1_26 {}
#[doc = "I/O configuration for pin PIO1_26/CT32B0_MAT2/ RXD"]
pub mod pio1_26;
#[doc = "I/O configuration for pin PIO1_27/CT32B0_MAT3/ TXD\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pio1_27](pio1_27) module"]
pub type PIO1_27 = crate::Reg<u32, _PIO1_27>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIO1_27;
#[doc = "`read()` method returns [pio1_27::R](pio1_27::R) reader structure"]
impl crate::Readable for PIO1_27 {}
#[doc = "`write(|w| ..)` method takes [pio1_27::W](pio1_27::W) writer structure"]
impl crate::Writable for PIO1_27 {}
#[doc = "I/O configuration for pin PIO1_27/CT32B0_MAT3/ TXD"]
pub mod pio1_27;
#[doc = "I/O configuration for pin PIO1_28/CT32B0_CAP0/ SCLK\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pio1_28](pio1_28) module"]
pub type PIO1_28 = crate::Reg<u32, _PIO1_28>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIO1_28;
#[doc = "`read()` method returns [pio1_28::R](pio1_28::R) reader structure"]
impl crate::Readable for PIO1_28 {}
#[doc = "`write(|w| ..)` method takes [pio1_28::W](pio1_28::W) writer structure"]
impl crate::Writable for PIO1_28 {}
#[doc = "I/O configuration for pin PIO1_28/CT32B0_CAP0/ SCLK"]
pub mod pio1_28;
#[doc = "I/O configuration for pin PIO1_29/SCK0/ CT32B0_CAP1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pio1_29](pio1_29) module"]
pub type PIO1_29 = crate::Reg<u32, _PIO1_29>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIO1_29;
#[doc = "`read()` method returns [pio1_29::R](pio1_29::R) reader structure"]
impl crate::Readable for PIO1_29 {}
#[doc = "`write(|w| ..)` method takes [pio1_29::W](pio1_29::W) writer structure"]
impl crate::Writable for PIO1_29 {}
#[doc = "I/O configuration for pin PIO1_29/SCK0/ CT32B0_CAP1"]
pub mod pio1_29;
#[doc = "I/O configuration for pin PIO1_31\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pio1_31](pio1_31) module"]
pub type PIO1_31 = crate::Reg<u32, _PIO1_31>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIO1_31;
#[doc = "`read()` method returns [pio1_31::R](pio1_31::R) reader structure"]
impl crate::Readable for PIO1_31 {}
#[doc = "`write(|w| ..)` method takes [pio1_31::W](pio1_31::W) writer structure"]
impl crate::Writable for PIO1_31 {}
#[doc = "I/O configuration for pin PIO1_31"]
pub mod pio1_31;
