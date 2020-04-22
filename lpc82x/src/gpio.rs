#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Byte pin registers for all port 0 and 1 GPIO pins"]
    pub b0_0: B0_0,
    #[doc = "0x01 - Byte pin registers for all port 0 and 1 GPIO pins"]
    pub b0_1: B0_1,
    #[doc = "0x02 - Byte pin registers for all port 0 and 1 GPIO pins"]
    pub b0_2: B0_2,
    #[doc = "0x03 - Byte pin registers for all port 0 and 1 GPIO pins"]
    pub b0_3: B0_3,
    #[doc = "0x04 - Byte pin registers for all port 0 and 1 GPIO pins"]
    pub b0_4: B0_4,
    #[doc = "0x05 - Byte pin registers for all port 0 and 1 GPIO pins"]
    pub b0_5: B0_5,
    #[doc = "0x06 - Byte pin registers for all port 0 and 1 GPIO pins"]
    pub b0_6: B0_6,
    #[doc = "0x07 - Byte pin registers for all port 0 and 1 GPIO pins"]
    pub b0_7: B0_7,
    #[doc = "0x08 - Byte pin registers for all port 0 and 1 GPIO pins"]
    pub b0_8: B0_8,
    #[doc = "0x09 - Byte pin registers for all port 0 and 1 GPIO pins"]
    pub b0_9: B0_9,
    #[doc = "0x0a - Byte pin registers for all port 0 and 1 GPIO pins"]
    pub b0_10: B0_10,
    #[doc = "0x0b - Byte pin registers for all port 0 and 1 GPIO pins"]
    pub b0_11: B0_11,
    #[doc = "0x0c - Byte pin registers for all port 0 and 1 GPIO pins"]
    pub b0_12: B0_12,
    #[doc = "0x0d - Byte pin registers for all port 0 and 1 GPIO pins"]
    pub b0_13: B0_13,
    #[doc = "0x0e - Byte pin registers for all port 0 and 1 GPIO pins"]
    pub b0_14: B0_14,
    #[doc = "0x0f - Byte pin registers for all port 0 and 1 GPIO pins"]
    pub b0_15: B0_15,
    #[doc = "0x10 - Byte pin registers for all port 0 and 1 GPIO pins"]
    pub b0_16: B0_16,
    #[doc = "0x11 - Byte pin registers for all port 0 and 1 GPIO pins"]
    pub b0_17: B0_17,
    #[doc = "0x12 - Byte pin registers for all port 0 and 1 GPIO pins"]
    pub b0_18: B0_18,
    #[doc = "0x13 - Byte pin registers for all port 0 and 1 GPIO pins"]
    pub b0_19: B0_19,
    #[doc = "0x14 - Byte pin registers for all port 0 and 1 GPIO pins"]
    pub b0_20: B0_20,
    #[doc = "0x15 - Byte pin registers for all port 0 and 1 GPIO pins"]
    pub b0_21: B0_21,
    #[doc = "0x16 - Byte pin registers for all port 0 and 1 GPIO pins"]
    pub b0_22: B0_22,
    #[doc = "0x17 - Byte pin registers for all port 0 and 1 GPIO pins"]
    pub b0_23: B0_23,
    #[doc = "0x18 - Byte pin registers for all port 0 and 1 GPIO pins"]
    pub b0_24: B0_24,
    #[doc = "0x19 - Byte pin registers for all port 0 and 1 GPIO pins"]
    pub b0_25: B0_25,
    #[doc = "0x1a - Byte pin registers for all port 0 and 1 GPIO pins"]
    pub b0_26: B0_26,
    #[doc = "0x1b - Byte pin registers for all port 0 and 1 GPIO pins"]
    pub b0_27: B0_27,
    #[doc = "0x1c - Byte pin registers for all port 0 and 1 GPIO pins"]
    pub b0_28: B0_28,
    _reserved29: [u8; 4067usize],
    #[doc = "0x1000 - Word pin registers for all port 0 and 1 GPIO pins"]
    pub w0_0: W0_0,
    #[doc = "0x1004 - Word pin registers for all port 0 and 1 GPIO pins"]
    pub w0_1: W0_1,
    #[doc = "0x1008 - Word pin registers for all port 0 and 1 GPIO pins"]
    pub w0_2: W0_2,
    #[doc = "0x100c - Word pin registers for all port 0 and 1 GPIO pins"]
    pub w0_3: W0_3,
    #[doc = "0x1010 - Word pin registers for all port 0 and 1 GPIO pins"]
    pub w0_4: W0_4,
    #[doc = "0x1014 - Word pin registers for all port 0 and 1 GPIO pins"]
    pub w0_5: W0_5,
    #[doc = "0x1018 - Word pin registers for all port 0 and 1 GPIO pins"]
    pub w0_6: W0_6,
    #[doc = "0x101c - Word pin registers for all port 0 and 1 GPIO pins"]
    pub w0_7: W0_7,
    #[doc = "0x1020 - Word pin registers for all port 0 and 1 GPIO pins"]
    pub w0_8: W0_8,
    #[doc = "0x1024 - Word pin registers for all port 0 and 1 GPIO pins"]
    pub w0_9: W0_9,
    #[doc = "0x1028 - Word pin registers for all port 0 and 1 GPIO pins"]
    pub w0_10: W0_10,
    #[doc = "0x102c - Word pin registers for all port 0 and 1 GPIO pins"]
    pub w0_11: W0_11,
    #[doc = "0x1030 - Word pin registers for all port 0 and 1 GPIO pins"]
    pub w0_12: W0_12,
    #[doc = "0x1034 - Word pin registers for all port 0 and 1 GPIO pins"]
    pub w0_13: W0_13,
    #[doc = "0x1038 - Word pin registers for all port 0 and 1 GPIO pins"]
    pub w0_14: W0_14,
    #[doc = "0x103c - Word pin registers for all port 0 and 1 GPIO pins"]
    pub w0_15: W0_15,
    #[doc = "0x1040 - Word pin registers for all port 0 and 1 GPIO pins"]
    pub w0_16: W0_16,
    #[doc = "0x1044 - Word pin registers for all port 0 and 1 GPIO pins"]
    pub w0_17: W0_17,
    #[doc = "0x1048 - Word pin registers for all port 0 and 1 GPIO pins"]
    pub w0_18: W0_18,
    #[doc = "0x104c - Word pin registers for all port 0 and 1 GPIO pins"]
    pub w0_19: W0_19,
    #[doc = "0x1050 - Word pin registers for all port 0 and 1 GPIO pins"]
    pub w0_20: W0_20,
    #[doc = "0x1054 - Word pin registers for all port 0 and 1 GPIO pins"]
    pub w0_21: W0_21,
    #[doc = "0x1058 - Word pin registers for all port 0 and 1 GPIO pins"]
    pub w0_22: W0_22,
    #[doc = "0x105c - Word pin registers for all port 0 and 1 GPIO pins"]
    pub w0_23: W0_23,
    #[doc = "0x1060 - Word pin registers for all port 0 and 1 GPIO pins"]
    pub w0_24: W0_24,
    #[doc = "0x1064 - Word pin registers for all port 0 and 1 GPIO pins"]
    pub w0_25: W0_25,
    #[doc = "0x1068 - Word pin registers for all port 0 and 1 GPIO pins"]
    pub w0_26: W0_26,
    #[doc = "0x106c - Word pin registers for all port 0 and 1 GPIO pins"]
    pub w0_27: W0_27,
    #[doc = "0x1070 - Word pin registers for all port 0 and 1 GPIO pins"]
    pub w0_28: W0_28,
    _reserved58: [u8; 3980usize],
    #[doc = "0x2000 - Direction registers"]
    pub dir0: DIR0,
    _reserved59: [u8; 124usize],
    #[doc = "0x2080 - Mask register"]
    pub mask0: MASK0,
    _reserved60: [u8; 124usize],
    #[doc = "0x2100 - Port pin register"]
    pub pin0: PIN0,
    _reserved61: [u8; 124usize],
    #[doc = "0x2180 - Masked port register"]
    pub mpin0: MPIN0,
    _reserved62: [u8; 124usize],
    #[doc = "0x2200 - Write: Set register for port Read: output bits for port"]
    pub set0: SET0,
    _reserved63: [u8; 124usize],
    #[doc = "0x2280 - Clear port"]
    pub clr0: CLR0,
    _reserved64: [u8; 124usize],
    #[doc = "0x2300 - Toggle port"]
    pub not0: NOT0,
    _reserved65: [u8; 124usize],
    #[doc = "0x2380 - Set pin direction bits for port"]
    pub dirset0: DIRSET0,
    _reserved66: [u8; 124usize],
    #[doc = "0x2400 - Clear pin direction bits for port"]
    pub dirclr0: DIRCLR0,
    _reserved67: [u8; 124usize],
    #[doc = "0x2480 - Toggle pin direction bits for port"]
    pub dirnot0: DIRNOT0,
}
#[doc = "Byte pin registers for all port 0 and 1 GPIO pins\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [b0_0](b0_0) module"]
pub type B0_0 = crate::Reg<u8, _B0_0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _B0_0;
#[doc = "`read()` method returns [b0_0::R](b0_0::R) reader structure"]
impl crate::Readable for B0_0 {}
#[doc = "`write(|w| ..)` method takes [b0_0::W](b0_0::W) writer structure"]
impl crate::Writable for B0_0 {}
#[doc = "Byte pin registers for all port 0 and 1 GPIO pins"]
pub mod b0_0;
#[doc = "Byte pin registers for all port 0 and 1 GPIO pins\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [b0_1](b0_1) module"]
pub type B0_1 = crate::Reg<u8, _B0_1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _B0_1;
#[doc = "`read()` method returns [b0_1::R](b0_1::R) reader structure"]
impl crate::Readable for B0_1 {}
#[doc = "`write(|w| ..)` method takes [b0_1::W](b0_1::W) writer structure"]
impl crate::Writable for B0_1 {}
#[doc = "Byte pin registers for all port 0 and 1 GPIO pins"]
pub mod b0_1;
#[doc = "Byte pin registers for all port 0 and 1 GPIO pins\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [b0_2](b0_2) module"]
pub type B0_2 = crate::Reg<u8, _B0_2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _B0_2;
#[doc = "`read()` method returns [b0_2::R](b0_2::R) reader structure"]
impl crate::Readable for B0_2 {}
#[doc = "`write(|w| ..)` method takes [b0_2::W](b0_2::W) writer structure"]
impl crate::Writable for B0_2 {}
#[doc = "Byte pin registers for all port 0 and 1 GPIO pins"]
pub mod b0_2;
#[doc = "Byte pin registers for all port 0 and 1 GPIO pins\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [b0_3](b0_3) module"]
pub type B0_3 = crate::Reg<u8, _B0_3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _B0_3;
#[doc = "`read()` method returns [b0_3::R](b0_3::R) reader structure"]
impl crate::Readable for B0_3 {}
#[doc = "`write(|w| ..)` method takes [b0_3::W](b0_3::W) writer structure"]
impl crate::Writable for B0_3 {}
#[doc = "Byte pin registers for all port 0 and 1 GPIO pins"]
pub mod b0_3;
#[doc = "Byte pin registers for all port 0 and 1 GPIO pins\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [b0_4](b0_4) module"]
pub type B0_4 = crate::Reg<u8, _B0_4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _B0_4;
#[doc = "`read()` method returns [b0_4::R](b0_4::R) reader structure"]
impl crate::Readable for B0_4 {}
#[doc = "`write(|w| ..)` method takes [b0_4::W](b0_4::W) writer structure"]
impl crate::Writable for B0_4 {}
#[doc = "Byte pin registers for all port 0 and 1 GPIO pins"]
pub mod b0_4;
#[doc = "Byte pin registers for all port 0 and 1 GPIO pins\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [b0_5](b0_5) module"]
pub type B0_5 = crate::Reg<u8, _B0_5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _B0_5;
#[doc = "`read()` method returns [b0_5::R](b0_5::R) reader structure"]
impl crate::Readable for B0_5 {}
#[doc = "`write(|w| ..)` method takes [b0_5::W](b0_5::W) writer structure"]
impl crate::Writable for B0_5 {}
#[doc = "Byte pin registers for all port 0 and 1 GPIO pins"]
pub mod b0_5;
#[doc = "Byte pin registers for all port 0 and 1 GPIO pins\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [b0_6](b0_6) module"]
pub type B0_6 = crate::Reg<u8, _B0_6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _B0_6;
#[doc = "`read()` method returns [b0_6::R](b0_6::R) reader structure"]
impl crate::Readable for B0_6 {}
#[doc = "`write(|w| ..)` method takes [b0_6::W](b0_6::W) writer structure"]
impl crate::Writable for B0_6 {}
#[doc = "Byte pin registers for all port 0 and 1 GPIO pins"]
pub mod b0_6;
#[doc = "Byte pin registers for all port 0 and 1 GPIO pins\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [b0_7](b0_7) module"]
pub type B0_7 = crate::Reg<u8, _B0_7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _B0_7;
#[doc = "`read()` method returns [b0_7::R](b0_7::R) reader structure"]
impl crate::Readable for B0_7 {}
#[doc = "`write(|w| ..)` method takes [b0_7::W](b0_7::W) writer structure"]
impl crate::Writable for B0_7 {}
#[doc = "Byte pin registers for all port 0 and 1 GPIO pins"]
pub mod b0_7;
#[doc = "Byte pin registers for all port 0 and 1 GPIO pins\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [b0_8](b0_8) module"]
pub type B0_8 = crate::Reg<u8, _B0_8>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _B0_8;
#[doc = "`read()` method returns [b0_8::R](b0_8::R) reader structure"]
impl crate::Readable for B0_8 {}
#[doc = "`write(|w| ..)` method takes [b0_8::W](b0_8::W) writer structure"]
impl crate::Writable for B0_8 {}
#[doc = "Byte pin registers for all port 0 and 1 GPIO pins"]
pub mod b0_8;
#[doc = "Byte pin registers for all port 0 and 1 GPIO pins\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [b0_9](b0_9) module"]
pub type B0_9 = crate::Reg<u8, _B0_9>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _B0_9;
#[doc = "`read()` method returns [b0_9::R](b0_9::R) reader structure"]
impl crate::Readable for B0_9 {}
#[doc = "`write(|w| ..)` method takes [b0_9::W](b0_9::W) writer structure"]
impl crate::Writable for B0_9 {}
#[doc = "Byte pin registers for all port 0 and 1 GPIO pins"]
pub mod b0_9;
#[doc = "Byte pin registers for all port 0 and 1 GPIO pins\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [b0_10](b0_10) module"]
pub type B0_10 = crate::Reg<u8, _B0_10>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _B0_10;
#[doc = "`read()` method returns [b0_10::R](b0_10::R) reader structure"]
impl crate::Readable for B0_10 {}
#[doc = "`write(|w| ..)` method takes [b0_10::W](b0_10::W) writer structure"]
impl crate::Writable for B0_10 {}
#[doc = "Byte pin registers for all port 0 and 1 GPIO pins"]
pub mod b0_10;
#[doc = "Byte pin registers for all port 0 and 1 GPIO pins\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [b0_11](b0_11) module"]
pub type B0_11 = crate::Reg<u8, _B0_11>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _B0_11;
#[doc = "`read()` method returns [b0_11::R](b0_11::R) reader structure"]
impl crate::Readable for B0_11 {}
#[doc = "`write(|w| ..)` method takes [b0_11::W](b0_11::W) writer structure"]
impl crate::Writable for B0_11 {}
#[doc = "Byte pin registers for all port 0 and 1 GPIO pins"]
pub mod b0_11;
#[doc = "Byte pin registers for all port 0 and 1 GPIO pins\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [b0_12](b0_12) module"]
pub type B0_12 = crate::Reg<u8, _B0_12>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _B0_12;
#[doc = "`read()` method returns [b0_12::R](b0_12::R) reader structure"]
impl crate::Readable for B0_12 {}
#[doc = "`write(|w| ..)` method takes [b0_12::W](b0_12::W) writer structure"]
impl crate::Writable for B0_12 {}
#[doc = "Byte pin registers for all port 0 and 1 GPIO pins"]
pub mod b0_12;
#[doc = "Byte pin registers for all port 0 and 1 GPIO pins\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [b0_13](b0_13) module"]
pub type B0_13 = crate::Reg<u8, _B0_13>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _B0_13;
#[doc = "`read()` method returns [b0_13::R](b0_13::R) reader structure"]
impl crate::Readable for B0_13 {}
#[doc = "`write(|w| ..)` method takes [b0_13::W](b0_13::W) writer structure"]
impl crate::Writable for B0_13 {}
#[doc = "Byte pin registers for all port 0 and 1 GPIO pins"]
pub mod b0_13;
#[doc = "Byte pin registers for all port 0 and 1 GPIO pins\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [b0_14](b0_14) module"]
pub type B0_14 = crate::Reg<u8, _B0_14>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _B0_14;
#[doc = "`read()` method returns [b0_14::R](b0_14::R) reader structure"]
impl crate::Readable for B0_14 {}
#[doc = "`write(|w| ..)` method takes [b0_14::W](b0_14::W) writer structure"]
impl crate::Writable for B0_14 {}
#[doc = "Byte pin registers for all port 0 and 1 GPIO pins"]
pub mod b0_14;
#[doc = "Byte pin registers for all port 0 and 1 GPIO pins\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [b0_15](b0_15) module"]
pub type B0_15 = crate::Reg<u8, _B0_15>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _B0_15;
#[doc = "`read()` method returns [b0_15::R](b0_15::R) reader structure"]
impl crate::Readable for B0_15 {}
#[doc = "`write(|w| ..)` method takes [b0_15::W](b0_15::W) writer structure"]
impl crate::Writable for B0_15 {}
#[doc = "Byte pin registers for all port 0 and 1 GPIO pins"]
pub mod b0_15;
#[doc = "Byte pin registers for all port 0 and 1 GPIO pins\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [b0_16](b0_16) module"]
pub type B0_16 = crate::Reg<u8, _B0_16>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _B0_16;
#[doc = "`read()` method returns [b0_16::R](b0_16::R) reader structure"]
impl crate::Readable for B0_16 {}
#[doc = "`write(|w| ..)` method takes [b0_16::W](b0_16::W) writer structure"]
impl crate::Writable for B0_16 {}
#[doc = "Byte pin registers for all port 0 and 1 GPIO pins"]
pub mod b0_16;
#[doc = "Byte pin registers for all port 0 and 1 GPIO pins\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [b0_17](b0_17) module"]
pub type B0_17 = crate::Reg<u8, _B0_17>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _B0_17;
#[doc = "`read()` method returns [b0_17::R](b0_17::R) reader structure"]
impl crate::Readable for B0_17 {}
#[doc = "`write(|w| ..)` method takes [b0_17::W](b0_17::W) writer structure"]
impl crate::Writable for B0_17 {}
#[doc = "Byte pin registers for all port 0 and 1 GPIO pins"]
pub mod b0_17;
#[doc = "Byte pin registers for all port 0 and 1 GPIO pins\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [b0_18](b0_18) module"]
pub type B0_18 = crate::Reg<u8, _B0_18>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _B0_18;
#[doc = "`read()` method returns [b0_18::R](b0_18::R) reader structure"]
impl crate::Readable for B0_18 {}
#[doc = "`write(|w| ..)` method takes [b0_18::W](b0_18::W) writer structure"]
impl crate::Writable for B0_18 {}
#[doc = "Byte pin registers for all port 0 and 1 GPIO pins"]
pub mod b0_18;
#[doc = "Byte pin registers for all port 0 and 1 GPIO pins\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [b0_19](b0_19) module"]
pub type B0_19 = crate::Reg<u8, _B0_19>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _B0_19;
#[doc = "`read()` method returns [b0_19::R](b0_19::R) reader structure"]
impl crate::Readable for B0_19 {}
#[doc = "`write(|w| ..)` method takes [b0_19::W](b0_19::W) writer structure"]
impl crate::Writable for B0_19 {}
#[doc = "Byte pin registers for all port 0 and 1 GPIO pins"]
pub mod b0_19;
#[doc = "Byte pin registers for all port 0 and 1 GPIO pins\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [b0_20](b0_20) module"]
pub type B0_20 = crate::Reg<u8, _B0_20>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _B0_20;
#[doc = "`read()` method returns [b0_20::R](b0_20::R) reader structure"]
impl crate::Readable for B0_20 {}
#[doc = "`write(|w| ..)` method takes [b0_20::W](b0_20::W) writer structure"]
impl crate::Writable for B0_20 {}
#[doc = "Byte pin registers for all port 0 and 1 GPIO pins"]
pub mod b0_20;
#[doc = "Byte pin registers for all port 0 and 1 GPIO pins\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [b0_21](b0_21) module"]
pub type B0_21 = crate::Reg<u8, _B0_21>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _B0_21;
#[doc = "`read()` method returns [b0_21::R](b0_21::R) reader structure"]
impl crate::Readable for B0_21 {}
#[doc = "`write(|w| ..)` method takes [b0_21::W](b0_21::W) writer structure"]
impl crate::Writable for B0_21 {}
#[doc = "Byte pin registers for all port 0 and 1 GPIO pins"]
pub mod b0_21;
#[doc = "Byte pin registers for all port 0 and 1 GPIO pins\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [b0_22](b0_22) module"]
pub type B0_22 = crate::Reg<u8, _B0_22>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _B0_22;
#[doc = "`read()` method returns [b0_22::R](b0_22::R) reader structure"]
impl crate::Readable for B0_22 {}
#[doc = "`write(|w| ..)` method takes [b0_22::W](b0_22::W) writer structure"]
impl crate::Writable for B0_22 {}
#[doc = "Byte pin registers for all port 0 and 1 GPIO pins"]
pub mod b0_22;
#[doc = "Byte pin registers for all port 0 and 1 GPIO pins\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [b0_23](b0_23) module"]
pub type B0_23 = crate::Reg<u8, _B0_23>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _B0_23;
#[doc = "`read()` method returns [b0_23::R](b0_23::R) reader structure"]
impl crate::Readable for B0_23 {}
#[doc = "`write(|w| ..)` method takes [b0_23::W](b0_23::W) writer structure"]
impl crate::Writable for B0_23 {}
#[doc = "Byte pin registers for all port 0 and 1 GPIO pins"]
pub mod b0_23;
#[doc = "Byte pin registers for all port 0 and 1 GPIO pins\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [b0_24](b0_24) module"]
pub type B0_24 = crate::Reg<u8, _B0_24>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _B0_24;
#[doc = "`read()` method returns [b0_24::R](b0_24::R) reader structure"]
impl crate::Readable for B0_24 {}
#[doc = "`write(|w| ..)` method takes [b0_24::W](b0_24::W) writer structure"]
impl crate::Writable for B0_24 {}
#[doc = "Byte pin registers for all port 0 and 1 GPIO pins"]
pub mod b0_24;
#[doc = "Byte pin registers for all port 0 and 1 GPIO pins\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [b0_25](b0_25) module"]
pub type B0_25 = crate::Reg<u8, _B0_25>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _B0_25;
#[doc = "`read()` method returns [b0_25::R](b0_25::R) reader structure"]
impl crate::Readable for B0_25 {}
#[doc = "`write(|w| ..)` method takes [b0_25::W](b0_25::W) writer structure"]
impl crate::Writable for B0_25 {}
#[doc = "Byte pin registers for all port 0 and 1 GPIO pins"]
pub mod b0_25;
#[doc = "Byte pin registers for all port 0 and 1 GPIO pins\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [b0_26](b0_26) module"]
pub type B0_26 = crate::Reg<u8, _B0_26>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _B0_26;
#[doc = "`read()` method returns [b0_26::R](b0_26::R) reader structure"]
impl crate::Readable for B0_26 {}
#[doc = "`write(|w| ..)` method takes [b0_26::W](b0_26::W) writer structure"]
impl crate::Writable for B0_26 {}
#[doc = "Byte pin registers for all port 0 and 1 GPIO pins"]
pub mod b0_26;
#[doc = "Byte pin registers for all port 0 and 1 GPIO pins\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [b0_27](b0_27) module"]
pub type B0_27 = crate::Reg<u8, _B0_27>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _B0_27;
#[doc = "`read()` method returns [b0_27::R](b0_27::R) reader structure"]
impl crate::Readable for B0_27 {}
#[doc = "`write(|w| ..)` method takes [b0_27::W](b0_27::W) writer structure"]
impl crate::Writable for B0_27 {}
#[doc = "Byte pin registers for all port 0 and 1 GPIO pins"]
pub mod b0_27;
#[doc = "Byte pin registers for all port 0 and 1 GPIO pins\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [b0_28](b0_28) module"]
pub type B0_28 = crate::Reg<u8, _B0_28>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _B0_28;
#[doc = "`read()` method returns [b0_28::R](b0_28::R) reader structure"]
impl crate::Readable for B0_28 {}
#[doc = "`write(|w| ..)` method takes [b0_28::W](b0_28::W) writer structure"]
impl crate::Writable for B0_28 {}
#[doc = "Byte pin registers for all port 0 and 1 GPIO pins"]
pub mod b0_28;
#[doc = "Word pin registers for all port 0 and 1 GPIO pins\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [w0_0](w0_0) module"]
pub type W0_0 = crate::Reg<u32, _W0_0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _W0_0;
#[doc = "`read()` method returns [w0_0::R](w0_0::R) reader structure"]
impl crate::Readable for W0_0 {}
#[doc = "`write(|w| ..)` method takes [w0_0::W](w0_0::W) writer structure"]
impl crate::Writable for W0_0 {}
#[doc = "Word pin registers for all port 0 and 1 GPIO pins"]
pub mod w0_0;
#[doc = "Word pin registers for all port 0 and 1 GPIO pins\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [w0_1](w0_1) module"]
pub type W0_1 = crate::Reg<u32, _W0_1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _W0_1;
#[doc = "`read()` method returns [w0_1::R](w0_1::R) reader structure"]
impl crate::Readable for W0_1 {}
#[doc = "`write(|w| ..)` method takes [w0_1::W](w0_1::W) writer structure"]
impl crate::Writable for W0_1 {}
#[doc = "Word pin registers for all port 0 and 1 GPIO pins"]
pub mod w0_1;
#[doc = "Word pin registers for all port 0 and 1 GPIO pins\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [w0_2](w0_2) module"]
pub type W0_2 = crate::Reg<u32, _W0_2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _W0_2;
#[doc = "`read()` method returns [w0_2::R](w0_2::R) reader structure"]
impl crate::Readable for W0_2 {}
#[doc = "`write(|w| ..)` method takes [w0_2::W](w0_2::W) writer structure"]
impl crate::Writable for W0_2 {}
#[doc = "Word pin registers for all port 0 and 1 GPIO pins"]
pub mod w0_2;
#[doc = "Word pin registers for all port 0 and 1 GPIO pins\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [w0_3](w0_3) module"]
pub type W0_3 = crate::Reg<u32, _W0_3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _W0_3;
#[doc = "`read()` method returns [w0_3::R](w0_3::R) reader structure"]
impl crate::Readable for W0_3 {}
#[doc = "`write(|w| ..)` method takes [w0_3::W](w0_3::W) writer structure"]
impl crate::Writable for W0_3 {}
#[doc = "Word pin registers for all port 0 and 1 GPIO pins"]
pub mod w0_3;
#[doc = "Word pin registers for all port 0 and 1 GPIO pins\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [w0_4](w0_4) module"]
pub type W0_4 = crate::Reg<u32, _W0_4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _W0_4;
#[doc = "`read()` method returns [w0_4::R](w0_4::R) reader structure"]
impl crate::Readable for W0_4 {}
#[doc = "`write(|w| ..)` method takes [w0_4::W](w0_4::W) writer structure"]
impl crate::Writable for W0_4 {}
#[doc = "Word pin registers for all port 0 and 1 GPIO pins"]
pub mod w0_4;
#[doc = "Word pin registers for all port 0 and 1 GPIO pins\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [w0_5](w0_5) module"]
pub type W0_5 = crate::Reg<u32, _W0_5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _W0_5;
#[doc = "`read()` method returns [w0_5::R](w0_5::R) reader structure"]
impl crate::Readable for W0_5 {}
#[doc = "`write(|w| ..)` method takes [w0_5::W](w0_5::W) writer structure"]
impl crate::Writable for W0_5 {}
#[doc = "Word pin registers for all port 0 and 1 GPIO pins"]
pub mod w0_5;
#[doc = "Word pin registers for all port 0 and 1 GPIO pins\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [w0_6](w0_6) module"]
pub type W0_6 = crate::Reg<u32, _W0_6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _W0_6;
#[doc = "`read()` method returns [w0_6::R](w0_6::R) reader structure"]
impl crate::Readable for W0_6 {}
#[doc = "`write(|w| ..)` method takes [w0_6::W](w0_6::W) writer structure"]
impl crate::Writable for W0_6 {}
#[doc = "Word pin registers for all port 0 and 1 GPIO pins"]
pub mod w0_6;
#[doc = "Word pin registers for all port 0 and 1 GPIO pins\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [w0_7](w0_7) module"]
pub type W0_7 = crate::Reg<u32, _W0_7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _W0_7;
#[doc = "`read()` method returns [w0_7::R](w0_7::R) reader structure"]
impl crate::Readable for W0_7 {}
#[doc = "`write(|w| ..)` method takes [w0_7::W](w0_7::W) writer structure"]
impl crate::Writable for W0_7 {}
#[doc = "Word pin registers for all port 0 and 1 GPIO pins"]
pub mod w0_7;
#[doc = "Word pin registers for all port 0 and 1 GPIO pins\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [w0_8](w0_8) module"]
pub type W0_8 = crate::Reg<u32, _W0_8>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _W0_8;
#[doc = "`read()` method returns [w0_8::R](w0_8::R) reader structure"]
impl crate::Readable for W0_8 {}
#[doc = "`write(|w| ..)` method takes [w0_8::W](w0_8::W) writer structure"]
impl crate::Writable for W0_8 {}
#[doc = "Word pin registers for all port 0 and 1 GPIO pins"]
pub mod w0_8;
#[doc = "Word pin registers for all port 0 and 1 GPIO pins\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [w0_9](w0_9) module"]
pub type W0_9 = crate::Reg<u32, _W0_9>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _W0_9;
#[doc = "`read()` method returns [w0_9::R](w0_9::R) reader structure"]
impl crate::Readable for W0_9 {}
#[doc = "`write(|w| ..)` method takes [w0_9::W](w0_9::W) writer structure"]
impl crate::Writable for W0_9 {}
#[doc = "Word pin registers for all port 0 and 1 GPIO pins"]
pub mod w0_9;
#[doc = "Word pin registers for all port 0 and 1 GPIO pins\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [w0_10](w0_10) module"]
pub type W0_10 = crate::Reg<u32, _W0_10>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _W0_10;
#[doc = "`read()` method returns [w0_10::R](w0_10::R) reader structure"]
impl crate::Readable for W0_10 {}
#[doc = "`write(|w| ..)` method takes [w0_10::W](w0_10::W) writer structure"]
impl crate::Writable for W0_10 {}
#[doc = "Word pin registers for all port 0 and 1 GPIO pins"]
pub mod w0_10;
#[doc = "Word pin registers for all port 0 and 1 GPIO pins\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [w0_11](w0_11) module"]
pub type W0_11 = crate::Reg<u32, _W0_11>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _W0_11;
#[doc = "`read()` method returns [w0_11::R](w0_11::R) reader structure"]
impl crate::Readable for W0_11 {}
#[doc = "`write(|w| ..)` method takes [w0_11::W](w0_11::W) writer structure"]
impl crate::Writable for W0_11 {}
#[doc = "Word pin registers for all port 0 and 1 GPIO pins"]
pub mod w0_11;
#[doc = "Word pin registers for all port 0 and 1 GPIO pins\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [w0_12](w0_12) module"]
pub type W0_12 = crate::Reg<u32, _W0_12>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _W0_12;
#[doc = "`read()` method returns [w0_12::R](w0_12::R) reader structure"]
impl crate::Readable for W0_12 {}
#[doc = "`write(|w| ..)` method takes [w0_12::W](w0_12::W) writer structure"]
impl crate::Writable for W0_12 {}
#[doc = "Word pin registers for all port 0 and 1 GPIO pins"]
pub mod w0_12;
#[doc = "Word pin registers for all port 0 and 1 GPIO pins\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [w0_13](w0_13) module"]
pub type W0_13 = crate::Reg<u32, _W0_13>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _W0_13;
#[doc = "`read()` method returns [w0_13::R](w0_13::R) reader structure"]
impl crate::Readable for W0_13 {}
#[doc = "`write(|w| ..)` method takes [w0_13::W](w0_13::W) writer structure"]
impl crate::Writable for W0_13 {}
#[doc = "Word pin registers for all port 0 and 1 GPIO pins"]
pub mod w0_13;
#[doc = "Word pin registers for all port 0 and 1 GPIO pins\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [w0_14](w0_14) module"]
pub type W0_14 = crate::Reg<u32, _W0_14>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _W0_14;
#[doc = "`read()` method returns [w0_14::R](w0_14::R) reader structure"]
impl crate::Readable for W0_14 {}
#[doc = "`write(|w| ..)` method takes [w0_14::W](w0_14::W) writer structure"]
impl crate::Writable for W0_14 {}
#[doc = "Word pin registers for all port 0 and 1 GPIO pins"]
pub mod w0_14;
#[doc = "Word pin registers for all port 0 and 1 GPIO pins\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [w0_15](w0_15) module"]
pub type W0_15 = crate::Reg<u32, _W0_15>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _W0_15;
#[doc = "`read()` method returns [w0_15::R](w0_15::R) reader structure"]
impl crate::Readable for W0_15 {}
#[doc = "`write(|w| ..)` method takes [w0_15::W](w0_15::W) writer structure"]
impl crate::Writable for W0_15 {}
#[doc = "Word pin registers for all port 0 and 1 GPIO pins"]
pub mod w0_15;
#[doc = "Word pin registers for all port 0 and 1 GPIO pins\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [w0_16](w0_16) module"]
pub type W0_16 = crate::Reg<u32, _W0_16>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _W0_16;
#[doc = "`read()` method returns [w0_16::R](w0_16::R) reader structure"]
impl crate::Readable for W0_16 {}
#[doc = "`write(|w| ..)` method takes [w0_16::W](w0_16::W) writer structure"]
impl crate::Writable for W0_16 {}
#[doc = "Word pin registers for all port 0 and 1 GPIO pins"]
pub mod w0_16;
#[doc = "Word pin registers for all port 0 and 1 GPIO pins\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [w0_17](w0_17) module"]
pub type W0_17 = crate::Reg<u32, _W0_17>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _W0_17;
#[doc = "`read()` method returns [w0_17::R](w0_17::R) reader structure"]
impl crate::Readable for W0_17 {}
#[doc = "`write(|w| ..)` method takes [w0_17::W](w0_17::W) writer structure"]
impl crate::Writable for W0_17 {}
#[doc = "Word pin registers for all port 0 and 1 GPIO pins"]
pub mod w0_17;
#[doc = "Word pin registers for all port 0 and 1 GPIO pins\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [w0_18](w0_18) module"]
pub type W0_18 = crate::Reg<u32, _W0_18>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _W0_18;
#[doc = "`read()` method returns [w0_18::R](w0_18::R) reader structure"]
impl crate::Readable for W0_18 {}
#[doc = "`write(|w| ..)` method takes [w0_18::W](w0_18::W) writer structure"]
impl crate::Writable for W0_18 {}
#[doc = "Word pin registers for all port 0 and 1 GPIO pins"]
pub mod w0_18;
#[doc = "Word pin registers for all port 0 and 1 GPIO pins\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [w0_19](w0_19) module"]
pub type W0_19 = crate::Reg<u32, _W0_19>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _W0_19;
#[doc = "`read()` method returns [w0_19::R](w0_19::R) reader structure"]
impl crate::Readable for W0_19 {}
#[doc = "`write(|w| ..)` method takes [w0_19::W](w0_19::W) writer structure"]
impl crate::Writable for W0_19 {}
#[doc = "Word pin registers for all port 0 and 1 GPIO pins"]
pub mod w0_19;
#[doc = "Word pin registers for all port 0 and 1 GPIO pins\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [w0_20](w0_20) module"]
pub type W0_20 = crate::Reg<u32, _W0_20>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _W0_20;
#[doc = "`read()` method returns [w0_20::R](w0_20::R) reader structure"]
impl crate::Readable for W0_20 {}
#[doc = "`write(|w| ..)` method takes [w0_20::W](w0_20::W) writer structure"]
impl crate::Writable for W0_20 {}
#[doc = "Word pin registers for all port 0 and 1 GPIO pins"]
pub mod w0_20;
#[doc = "Word pin registers for all port 0 and 1 GPIO pins\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [w0_21](w0_21) module"]
pub type W0_21 = crate::Reg<u32, _W0_21>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _W0_21;
#[doc = "`read()` method returns [w0_21::R](w0_21::R) reader structure"]
impl crate::Readable for W0_21 {}
#[doc = "`write(|w| ..)` method takes [w0_21::W](w0_21::W) writer structure"]
impl crate::Writable for W0_21 {}
#[doc = "Word pin registers for all port 0 and 1 GPIO pins"]
pub mod w0_21;
#[doc = "Word pin registers for all port 0 and 1 GPIO pins\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [w0_22](w0_22) module"]
pub type W0_22 = crate::Reg<u32, _W0_22>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _W0_22;
#[doc = "`read()` method returns [w0_22::R](w0_22::R) reader structure"]
impl crate::Readable for W0_22 {}
#[doc = "`write(|w| ..)` method takes [w0_22::W](w0_22::W) writer structure"]
impl crate::Writable for W0_22 {}
#[doc = "Word pin registers for all port 0 and 1 GPIO pins"]
pub mod w0_22;
#[doc = "Word pin registers for all port 0 and 1 GPIO pins\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [w0_23](w0_23) module"]
pub type W0_23 = crate::Reg<u32, _W0_23>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _W0_23;
#[doc = "`read()` method returns [w0_23::R](w0_23::R) reader structure"]
impl crate::Readable for W0_23 {}
#[doc = "`write(|w| ..)` method takes [w0_23::W](w0_23::W) writer structure"]
impl crate::Writable for W0_23 {}
#[doc = "Word pin registers for all port 0 and 1 GPIO pins"]
pub mod w0_23;
#[doc = "Word pin registers for all port 0 and 1 GPIO pins\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [w0_24](w0_24) module"]
pub type W0_24 = crate::Reg<u32, _W0_24>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _W0_24;
#[doc = "`read()` method returns [w0_24::R](w0_24::R) reader structure"]
impl crate::Readable for W0_24 {}
#[doc = "`write(|w| ..)` method takes [w0_24::W](w0_24::W) writer structure"]
impl crate::Writable for W0_24 {}
#[doc = "Word pin registers for all port 0 and 1 GPIO pins"]
pub mod w0_24;
#[doc = "Word pin registers for all port 0 and 1 GPIO pins\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [w0_25](w0_25) module"]
pub type W0_25 = crate::Reg<u32, _W0_25>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _W0_25;
#[doc = "`read()` method returns [w0_25::R](w0_25::R) reader structure"]
impl crate::Readable for W0_25 {}
#[doc = "`write(|w| ..)` method takes [w0_25::W](w0_25::W) writer structure"]
impl crate::Writable for W0_25 {}
#[doc = "Word pin registers for all port 0 and 1 GPIO pins"]
pub mod w0_25;
#[doc = "Word pin registers for all port 0 and 1 GPIO pins\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [w0_26](w0_26) module"]
pub type W0_26 = crate::Reg<u32, _W0_26>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _W0_26;
#[doc = "`read()` method returns [w0_26::R](w0_26::R) reader structure"]
impl crate::Readable for W0_26 {}
#[doc = "`write(|w| ..)` method takes [w0_26::W](w0_26::W) writer structure"]
impl crate::Writable for W0_26 {}
#[doc = "Word pin registers for all port 0 and 1 GPIO pins"]
pub mod w0_26;
#[doc = "Word pin registers for all port 0 and 1 GPIO pins\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [w0_27](w0_27) module"]
pub type W0_27 = crate::Reg<u32, _W0_27>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _W0_27;
#[doc = "`read()` method returns [w0_27::R](w0_27::R) reader structure"]
impl crate::Readable for W0_27 {}
#[doc = "`write(|w| ..)` method takes [w0_27::W](w0_27::W) writer structure"]
impl crate::Writable for W0_27 {}
#[doc = "Word pin registers for all port 0 and 1 GPIO pins"]
pub mod w0_27;
#[doc = "Word pin registers for all port 0 and 1 GPIO pins\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [w0_28](w0_28) module"]
pub type W0_28 = crate::Reg<u32, _W0_28>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _W0_28;
#[doc = "`read()` method returns [w0_28::R](w0_28::R) reader structure"]
impl crate::Readable for W0_28 {}
#[doc = "`write(|w| ..)` method takes [w0_28::W](w0_28::W) writer structure"]
impl crate::Writable for W0_28 {}
#[doc = "Word pin registers for all port 0 and 1 GPIO pins"]
pub mod w0_28;
#[doc = "Direction registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dir0](dir0) module"]
pub type DIR0 = crate::Reg<u32, _DIR0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DIR0;
#[doc = "`read()` method returns [dir0::R](dir0::R) reader structure"]
impl crate::Readable for DIR0 {}
#[doc = "`write(|w| ..)` method takes [dir0::W](dir0::W) writer structure"]
impl crate::Writable for DIR0 {}
#[doc = "Direction registers"]
pub mod dir0;
#[doc = "Mask register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mask0](mask0) module"]
pub type MASK0 = crate::Reg<u32, _MASK0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MASK0;
#[doc = "`read()` method returns [mask0::R](mask0::R) reader structure"]
impl crate::Readable for MASK0 {}
#[doc = "`write(|w| ..)` method takes [mask0::W](mask0::W) writer structure"]
impl crate::Writable for MASK0 {}
#[doc = "Mask register"]
pub mod mask0;
#[doc = "Port pin register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pin0](pin0) module"]
pub type PIN0 = crate::Reg<u32, _PIN0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIN0;
#[doc = "`read()` method returns [pin0::R](pin0::R) reader structure"]
impl crate::Readable for PIN0 {}
#[doc = "`write(|w| ..)` method takes [pin0::W](pin0::W) writer structure"]
impl crate::Writable for PIN0 {}
#[doc = "Port pin register"]
pub mod pin0;
#[doc = "Masked port register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mpin0](mpin0) module"]
pub type MPIN0 = crate::Reg<u32, _MPIN0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MPIN0;
#[doc = "`read()` method returns [mpin0::R](mpin0::R) reader structure"]
impl crate::Readable for MPIN0 {}
#[doc = "`write(|w| ..)` method takes [mpin0::W](mpin0::W) writer structure"]
impl crate::Writable for MPIN0 {}
#[doc = "Masked port register"]
pub mod mpin0;
#[doc = "Write: Set register for port Read: output bits for port\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [set0](set0) module"]
pub type SET0 = crate::Reg<u32, _SET0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SET0;
#[doc = "`read()` method returns [set0::R](set0::R) reader structure"]
impl crate::Readable for SET0 {}
#[doc = "`write(|w| ..)` method takes [set0::W](set0::W) writer structure"]
impl crate::Writable for SET0 {}
#[doc = "Write: Set register for port Read: output bits for port"]
pub mod set0;
#[doc = "Clear port\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clr0](clr0) module"]
pub type CLR0 = crate::Reg<u32, _CLR0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CLR0;
#[doc = "`read()` method returns [clr0::R](clr0::R) reader structure"]
impl crate::Readable for CLR0 {}
#[doc = "`write(|w| ..)` method takes [clr0::W](clr0::W) writer structure"]
impl crate::Writable for CLR0 {}
#[doc = "Clear port"]
pub mod clr0;
#[doc = "Toggle port\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [not0](not0) module"]
pub type NOT0 = crate::Reg<u32, _NOT0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _NOT0;
#[doc = "`read()` method returns [not0::R](not0::R) reader structure"]
impl crate::Readable for NOT0 {}
#[doc = "`write(|w| ..)` method takes [not0::W](not0::W) writer structure"]
impl crate::Writable for NOT0 {}
#[doc = "Toggle port"]
pub mod not0;
#[doc = "Set pin direction bits for port\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dirset0](dirset0) module"]
pub type DIRSET0 = crate::Reg<u32, _DIRSET0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DIRSET0;
#[doc = "`write(|w| ..)` method takes [dirset0::W](dirset0::W) writer structure"]
impl crate::Writable for DIRSET0 {}
#[doc = "Set pin direction bits for port"]
pub mod dirset0;
#[doc = "Clear pin direction bits for port\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dirclr0](dirclr0) module"]
pub type DIRCLR0 = crate::Reg<u32, _DIRCLR0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DIRCLR0;
#[doc = "`write(|w| ..)` method takes [dirclr0::W](dirclr0::W) writer structure"]
impl crate::Writable for DIRCLR0 {}
#[doc = "Clear pin direction bits for port"]
pub mod dirclr0;
#[doc = "Toggle pin direction bits for port\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dirnot0](dirnot0) module"]
pub type DIRNOT0 = crate::Reg<u32, _DIRNOT0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DIRNOT0;
#[doc = "`write(|w| ..)` method takes [dirnot0::W](dirnot0::W) writer structure"]
impl crate::Writable for DIRNOT0 {}
#[doc = "Toggle pin direction bits for port"]
pub mod dirnot0;
