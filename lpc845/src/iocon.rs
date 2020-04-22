#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Digital I/O control for pins PIO0_17"]
    pub pio0_17: PIO0_17,
    #[doc = "0x04 - Digital I/O control for pins PIO0_13"]
    pub pio0_13: PIO0_13,
    #[doc = "0x08 - Digital I/O control for pins PIO0_12"]
    pub pio0_12: PIO0_12,
    #[doc = "0x0c - Digital I/O control for pins PIO0_5"]
    pub pio0_5: PIO0_5,
    #[doc = "0x10 - Digital I/O control for pins PIO0_4"]
    pub pio0_4: PIO0_4,
    #[doc = "0x14 - Digital I/O control for pins PIO0_3"]
    pub pio0_3: PIO0_3,
    #[doc = "0x18 - Digital I/O control for pins PIO0_2"]
    pub pio0_2: PIO0_2,
    #[doc = "0x1c - Digital I/O control for pins PIO0_11"]
    pub pio0_11: PIO0_11,
    #[doc = "0x20 - Digital I/O control for pins PIO0_10"]
    pub pio0_10: PIO0_10,
    #[doc = "0x24 - Digital I/O control for pins PIO0_16"]
    pub pio0_16: PIO0_16,
    #[doc = "0x28 - Digital I/O control for pins PIO0_15"]
    pub pio0_15: PIO0_15,
    #[doc = "0x2c - Digital I/O control for pins PIO0_1"]
    pub pio0_1: PIO0_1,
    _reserved12: [u8; 4usize],
    #[doc = "0x34 - Digital I/O control for pins PIO0_9"]
    pub pio0_9: PIO0_9,
    #[doc = "0x38 - Digital I/O control for pins PIO0_8"]
    pub pio0_8: PIO0_8,
    #[doc = "0x3c - Digital I/O control for pins PIO0_7"]
    pub pio0_7: PIO0_7,
    #[doc = "0x40 - Digital I/O control for pins PIO0_6"]
    pub pio0_6: PIO0_6,
    #[doc = "0x44 - Digital I/O control for pins PIO0_0"]
    pub pio0_0: PIO0_0,
    #[doc = "0x48 - Digital I/O control for pins PIO0_14"]
    pub pio0_14: PIO0_14,
    _reserved18: [u8; 4usize],
    #[doc = "0x50 - Digital I/O control for pins PIO0_28"]
    pub pio0_28: PIO0_28,
    #[doc = "0x54 - Digital I/O control for pins PIO0_27"]
    pub pio0_27: PIO0_27,
    #[doc = "0x58 - Digital I/O control for pins PIO0_26"]
    pub pio0_26: PIO0_26,
    #[doc = "0x5c - Digital I/O control for pins PIO0_25"]
    pub pio0_25: PIO0_25,
    #[doc = "0x60 - Digital I/O control for pins PIO0_24"]
    pub pio0_24: PIO0_24,
    #[doc = "0x64 - Digital I/O control for pins PIO0_23"]
    pub pio0_23: PIO0_23,
    #[doc = "0x68 - Digital I/O control for pins PIO0_22"]
    pub pio0_22: PIO0_22,
    #[doc = "0x6c - Digital I/O control for pins PIO0_21"]
    pub pio0_21: PIO0_21,
    #[doc = "0x70 - Digital I/O control for pins PIO0_20"]
    pub pio0_20: PIO0_20,
    #[doc = "0x74 - Digital I/O control for pins PIO0_19"]
    pub pio0_19: PIO0_19,
    #[doc = "0x78 - Digital I/O control for pins PIO0_18"]
    pub pio0_18: PIO0_18,
    #[doc = "0x7c - Digital I/O control for pins PIO1_8"]
    pub pio1_8: PIO1_8,
    #[doc = "0x80 - Digital I/O control for pins PIO1_9"]
    pub pio1_9: PIO1_9,
    #[doc = "0x84 - Digital I/O control for pins PIO1_12"]
    pub pio1_12: PIO1_12,
    #[doc = "0x88 - Digital I/O control for pins PIO1_13"]
    pub pio1_13: PIO1_13,
    #[doc = "0x8c - Digital I/O control for pins PIO0_31"]
    pub pio0_31: PIO0_31,
    #[doc = "0x90 - Digital I/O control for pins PIO1_0"]
    pub pio1_0: PIO1_0,
    #[doc = "0x94 - Digital I/O control for pins PIO1_1"]
    pub pio1_1: PIO1_1,
    #[doc = "0x98 - Digital I/O control for pins PIO1_2"]
    pub pio1_2: PIO1_2,
    #[doc = "0x9c - Digital I/O control for pins PIO1_14"]
    pub pio1_14: PIO1_14,
    #[doc = "0xa0 - Digital I/O control for pins PIO1_15"]
    pub pio1_15: PIO1_15,
    #[doc = "0xa4 - Digital I/O control for pins PIO1_3"]
    pub pio1_3: PIO1_3,
    #[doc = "0xa8 - Digital I/O control for pins PIO1_4"]
    pub pio1_4: PIO1_4,
    #[doc = "0xac - Digital I/O control for pins PIO1_5"]
    pub pio1_5: PIO1_5,
    #[doc = "0xb0 - Digital I/O control for pins PIO1_16"]
    pub pio1_16: PIO1_16,
    #[doc = "0xb4 - Digital I/O control for pins PIO1_17"]
    pub pio1_17: PIO1_17,
    #[doc = "0xb8 - Digital I/O control for pins PIO1_6"]
    pub pio1_6: PIO1_6,
    #[doc = "0xbc - Digital I/O control for pins PIO1_18"]
    pub pio1_18: PIO1_18,
    #[doc = "0xc0 - Digital I/O control for pins PIO1_19"]
    pub pio1_19: PIO1_19,
    #[doc = "0xc4 - Digital I/O control for pins PIO1_7"]
    pub pio1_7: PIO1_7,
    #[doc = "0xc8 - Digital I/O control for pins PIO0_29"]
    pub pio0_29: PIO0_29,
    #[doc = "0xcc - Digital I/O control for pins PIO0_30"]
    pub pio0_30: PIO0_30,
    #[doc = "0xd0 - Digital I/O control for pins PIO1_20"]
    pub pio1_20: PIO1_20,
    #[doc = "0xd4 - Digital I/O control for pins PIO1_21"]
    pub pio1_21: PIO1_21,
    #[doc = "0xd8 - Digital I/O control for pins PIO1_11"]
    pub pio1_11: PIO1_11,
    #[doc = "0xdc - Digital I/O control for pins PIO1_10"]
    pub pio1_10: PIO1_10,
}
#[doc = "Digital I/O control for pins PIO0_17\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pio0_17](pio0_17) module"]
pub type PIO0_17 = crate::Reg<u32, _PIO0_17>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIO0_17;
#[doc = "`read()` method returns [pio0_17::R](pio0_17::R) reader structure"]
impl crate::Readable for PIO0_17 {}
#[doc = "`write(|w| ..)` method takes [pio0_17::W](pio0_17::W) writer structure"]
impl crate::Writable for PIO0_17 {}
#[doc = "Digital I/O control for pins PIO0_17"]
pub mod pio0_17;
#[doc = "Digital I/O control for pins PIO0_13\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pio0_13](pio0_13) module"]
pub type PIO0_13 = crate::Reg<u32, _PIO0_13>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIO0_13;
#[doc = "`read()` method returns [pio0_13::R](pio0_13::R) reader structure"]
impl crate::Readable for PIO0_13 {}
#[doc = "`write(|w| ..)` method takes [pio0_13::W](pio0_13::W) writer structure"]
impl crate::Writable for PIO0_13 {}
#[doc = "Digital I/O control for pins PIO0_13"]
pub mod pio0_13;
#[doc = "Digital I/O control for pins PIO0_12\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pio0_12](pio0_12) module"]
pub type PIO0_12 = crate::Reg<u32, _PIO0_12>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIO0_12;
#[doc = "`read()` method returns [pio0_12::R](pio0_12::R) reader structure"]
impl crate::Readable for PIO0_12 {}
#[doc = "`write(|w| ..)` method takes [pio0_12::W](pio0_12::W) writer structure"]
impl crate::Writable for PIO0_12 {}
#[doc = "Digital I/O control for pins PIO0_12"]
pub mod pio0_12;
#[doc = "Digital I/O control for pins PIO0_5\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pio0_5](pio0_5) module"]
pub type PIO0_5 = crate::Reg<u32, _PIO0_5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIO0_5;
#[doc = "`read()` method returns [pio0_5::R](pio0_5::R) reader structure"]
impl crate::Readable for PIO0_5 {}
#[doc = "`write(|w| ..)` method takes [pio0_5::W](pio0_5::W) writer structure"]
impl crate::Writable for PIO0_5 {}
#[doc = "Digital I/O control for pins PIO0_5"]
pub mod pio0_5;
#[doc = "Digital I/O control for pins PIO0_4\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pio0_4](pio0_4) module"]
pub type PIO0_4 = crate::Reg<u32, _PIO0_4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIO0_4;
#[doc = "`read()` method returns [pio0_4::R](pio0_4::R) reader structure"]
impl crate::Readable for PIO0_4 {}
#[doc = "`write(|w| ..)` method takes [pio0_4::W](pio0_4::W) writer structure"]
impl crate::Writable for PIO0_4 {}
#[doc = "Digital I/O control for pins PIO0_4"]
pub mod pio0_4;
#[doc = "Digital I/O control for pins PIO0_3\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pio0_3](pio0_3) module"]
pub type PIO0_3 = crate::Reg<u32, _PIO0_3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIO0_3;
#[doc = "`read()` method returns [pio0_3::R](pio0_3::R) reader structure"]
impl crate::Readable for PIO0_3 {}
#[doc = "`write(|w| ..)` method takes [pio0_3::W](pio0_3::W) writer structure"]
impl crate::Writable for PIO0_3 {}
#[doc = "Digital I/O control for pins PIO0_3"]
pub mod pio0_3;
#[doc = "Digital I/O control for pins PIO0_2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pio0_2](pio0_2) module"]
pub type PIO0_2 = crate::Reg<u32, _PIO0_2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIO0_2;
#[doc = "`read()` method returns [pio0_2::R](pio0_2::R) reader structure"]
impl crate::Readable for PIO0_2 {}
#[doc = "`write(|w| ..)` method takes [pio0_2::W](pio0_2::W) writer structure"]
impl crate::Writable for PIO0_2 {}
#[doc = "Digital I/O control for pins PIO0_2"]
pub mod pio0_2;
#[doc = "Digital I/O control for pins PIO0_11\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pio0_11](pio0_11) module"]
pub type PIO0_11 = crate::Reg<u32, _PIO0_11>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIO0_11;
#[doc = "`read()` method returns [pio0_11::R](pio0_11::R) reader structure"]
impl crate::Readable for PIO0_11 {}
#[doc = "`write(|w| ..)` method takes [pio0_11::W](pio0_11::W) writer structure"]
impl crate::Writable for PIO0_11 {}
#[doc = "Digital I/O control for pins PIO0_11"]
pub mod pio0_11;
#[doc = "Digital I/O control for pins PIO0_10\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pio0_10](pio0_10) module"]
pub type PIO0_10 = crate::Reg<u32, _PIO0_10>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIO0_10;
#[doc = "`read()` method returns [pio0_10::R](pio0_10::R) reader structure"]
impl crate::Readable for PIO0_10 {}
#[doc = "`write(|w| ..)` method takes [pio0_10::W](pio0_10::W) writer structure"]
impl crate::Writable for PIO0_10 {}
#[doc = "Digital I/O control for pins PIO0_10"]
pub mod pio0_10;
#[doc = "Digital I/O control for pins PIO0_16\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pio0_16](pio0_16) module"]
pub type PIO0_16 = crate::Reg<u32, _PIO0_16>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIO0_16;
#[doc = "`read()` method returns [pio0_16::R](pio0_16::R) reader structure"]
impl crate::Readable for PIO0_16 {}
#[doc = "`write(|w| ..)` method takes [pio0_16::W](pio0_16::W) writer structure"]
impl crate::Writable for PIO0_16 {}
#[doc = "Digital I/O control for pins PIO0_16"]
pub mod pio0_16;
#[doc = "Digital I/O control for pins PIO0_15\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pio0_15](pio0_15) module"]
pub type PIO0_15 = crate::Reg<u32, _PIO0_15>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIO0_15;
#[doc = "`read()` method returns [pio0_15::R](pio0_15::R) reader structure"]
impl crate::Readable for PIO0_15 {}
#[doc = "`write(|w| ..)` method takes [pio0_15::W](pio0_15::W) writer structure"]
impl crate::Writable for PIO0_15 {}
#[doc = "Digital I/O control for pins PIO0_15"]
pub mod pio0_15;
#[doc = "Digital I/O control for pins PIO0_1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pio0_1](pio0_1) module"]
pub type PIO0_1 = crate::Reg<u32, _PIO0_1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIO0_1;
#[doc = "`read()` method returns [pio0_1::R](pio0_1::R) reader structure"]
impl crate::Readable for PIO0_1 {}
#[doc = "`write(|w| ..)` method takes [pio0_1::W](pio0_1::W) writer structure"]
impl crate::Writable for PIO0_1 {}
#[doc = "Digital I/O control for pins PIO0_1"]
pub mod pio0_1;
#[doc = "Digital I/O control for pins PIO0_9\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pio0_9](pio0_9) module"]
pub type PIO0_9 = crate::Reg<u32, _PIO0_9>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIO0_9;
#[doc = "`read()` method returns [pio0_9::R](pio0_9::R) reader structure"]
impl crate::Readable for PIO0_9 {}
#[doc = "`write(|w| ..)` method takes [pio0_9::W](pio0_9::W) writer structure"]
impl crate::Writable for PIO0_9 {}
#[doc = "Digital I/O control for pins PIO0_9"]
pub mod pio0_9;
#[doc = "Digital I/O control for pins PIO0_8\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pio0_8](pio0_8) module"]
pub type PIO0_8 = crate::Reg<u32, _PIO0_8>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIO0_8;
#[doc = "`read()` method returns [pio0_8::R](pio0_8::R) reader structure"]
impl crate::Readable for PIO0_8 {}
#[doc = "`write(|w| ..)` method takes [pio0_8::W](pio0_8::W) writer structure"]
impl crate::Writable for PIO0_8 {}
#[doc = "Digital I/O control for pins PIO0_8"]
pub mod pio0_8;
#[doc = "Digital I/O control for pins PIO0_7\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pio0_7](pio0_7) module"]
pub type PIO0_7 = crate::Reg<u32, _PIO0_7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIO0_7;
#[doc = "`read()` method returns [pio0_7::R](pio0_7::R) reader structure"]
impl crate::Readable for PIO0_7 {}
#[doc = "`write(|w| ..)` method takes [pio0_7::W](pio0_7::W) writer structure"]
impl crate::Writable for PIO0_7 {}
#[doc = "Digital I/O control for pins PIO0_7"]
pub mod pio0_7;
#[doc = "Digital I/O control for pins PIO0_6\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pio0_6](pio0_6) module"]
pub type PIO0_6 = crate::Reg<u32, _PIO0_6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIO0_6;
#[doc = "`read()` method returns [pio0_6::R](pio0_6::R) reader structure"]
impl crate::Readable for PIO0_6 {}
#[doc = "`write(|w| ..)` method takes [pio0_6::W](pio0_6::W) writer structure"]
impl crate::Writable for PIO0_6 {}
#[doc = "Digital I/O control for pins PIO0_6"]
pub mod pio0_6;
#[doc = "Digital I/O control for pins PIO0_0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pio0_0](pio0_0) module"]
pub type PIO0_0 = crate::Reg<u32, _PIO0_0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIO0_0;
#[doc = "`read()` method returns [pio0_0::R](pio0_0::R) reader structure"]
impl crate::Readable for PIO0_0 {}
#[doc = "`write(|w| ..)` method takes [pio0_0::W](pio0_0::W) writer structure"]
impl crate::Writable for PIO0_0 {}
#[doc = "Digital I/O control for pins PIO0_0"]
pub mod pio0_0;
#[doc = "Digital I/O control for pins PIO0_14\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pio0_14](pio0_14) module"]
pub type PIO0_14 = crate::Reg<u32, _PIO0_14>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIO0_14;
#[doc = "`read()` method returns [pio0_14::R](pio0_14::R) reader structure"]
impl crate::Readable for PIO0_14 {}
#[doc = "`write(|w| ..)` method takes [pio0_14::W](pio0_14::W) writer structure"]
impl crate::Writable for PIO0_14 {}
#[doc = "Digital I/O control for pins PIO0_14"]
pub mod pio0_14;
#[doc = "Digital I/O control for pins PIO0_28\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pio0_28](pio0_28) module"]
pub type PIO0_28 = crate::Reg<u32, _PIO0_28>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIO0_28;
#[doc = "`read()` method returns [pio0_28::R](pio0_28::R) reader structure"]
impl crate::Readable for PIO0_28 {}
#[doc = "`write(|w| ..)` method takes [pio0_28::W](pio0_28::W) writer structure"]
impl crate::Writable for PIO0_28 {}
#[doc = "Digital I/O control for pins PIO0_28"]
pub mod pio0_28;
#[doc = "Digital I/O control for pins PIO0_27\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pio0_27](pio0_27) module"]
pub type PIO0_27 = crate::Reg<u32, _PIO0_27>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIO0_27;
#[doc = "`read()` method returns [pio0_27::R](pio0_27::R) reader structure"]
impl crate::Readable for PIO0_27 {}
#[doc = "`write(|w| ..)` method takes [pio0_27::W](pio0_27::W) writer structure"]
impl crate::Writable for PIO0_27 {}
#[doc = "Digital I/O control for pins PIO0_27"]
pub mod pio0_27;
#[doc = "Digital I/O control for pins PIO0_26\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pio0_26](pio0_26) module"]
pub type PIO0_26 = crate::Reg<u32, _PIO0_26>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIO0_26;
#[doc = "`read()` method returns [pio0_26::R](pio0_26::R) reader structure"]
impl crate::Readable for PIO0_26 {}
#[doc = "`write(|w| ..)` method takes [pio0_26::W](pio0_26::W) writer structure"]
impl crate::Writable for PIO0_26 {}
#[doc = "Digital I/O control for pins PIO0_26"]
pub mod pio0_26;
#[doc = "Digital I/O control for pins PIO0_25\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pio0_25](pio0_25) module"]
pub type PIO0_25 = crate::Reg<u32, _PIO0_25>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIO0_25;
#[doc = "`read()` method returns [pio0_25::R](pio0_25::R) reader structure"]
impl crate::Readable for PIO0_25 {}
#[doc = "`write(|w| ..)` method takes [pio0_25::W](pio0_25::W) writer structure"]
impl crate::Writable for PIO0_25 {}
#[doc = "Digital I/O control for pins PIO0_25"]
pub mod pio0_25;
#[doc = "Digital I/O control for pins PIO0_24\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pio0_24](pio0_24) module"]
pub type PIO0_24 = crate::Reg<u32, _PIO0_24>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIO0_24;
#[doc = "`read()` method returns [pio0_24::R](pio0_24::R) reader structure"]
impl crate::Readable for PIO0_24 {}
#[doc = "`write(|w| ..)` method takes [pio0_24::W](pio0_24::W) writer structure"]
impl crate::Writable for PIO0_24 {}
#[doc = "Digital I/O control for pins PIO0_24"]
pub mod pio0_24;
#[doc = "Digital I/O control for pins PIO0_23\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pio0_23](pio0_23) module"]
pub type PIO0_23 = crate::Reg<u32, _PIO0_23>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIO0_23;
#[doc = "`read()` method returns [pio0_23::R](pio0_23::R) reader structure"]
impl crate::Readable for PIO0_23 {}
#[doc = "`write(|w| ..)` method takes [pio0_23::W](pio0_23::W) writer structure"]
impl crate::Writable for PIO0_23 {}
#[doc = "Digital I/O control for pins PIO0_23"]
pub mod pio0_23;
#[doc = "Digital I/O control for pins PIO0_22\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pio0_22](pio0_22) module"]
pub type PIO0_22 = crate::Reg<u32, _PIO0_22>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIO0_22;
#[doc = "`read()` method returns [pio0_22::R](pio0_22::R) reader structure"]
impl crate::Readable for PIO0_22 {}
#[doc = "`write(|w| ..)` method takes [pio0_22::W](pio0_22::W) writer structure"]
impl crate::Writable for PIO0_22 {}
#[doc = "Digital I/O control for pins PIO0_22"]
pub mod pio0_22;
#[doc = "Digital I/O control for pins PIO0_21\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pio0_21](pio0_21) module"]
pub type PIO0_21 = crate::Reg<u32, _PIO0_21>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIO0_21;
#[doc = "`read()` method returns [pio0_21::R](pio0_21::R) reader structure"]
impl crate::Readable for PIO0_21 {}
#[doc = "`write(|w| ..)` method takes [pio0_21::W](pio0_21::W) writer structure"]
impl crate::Writable for PIO0_21 {}
#[doc = "Digital I/O control for pins PIO0_21"]
pub mod pio0_21;
#[doc = "Digital I/O control for pins PIO0_20\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pio0_20](pio0_20) module"]
pub type PIO0_20 = crate::Reg<u32, _PIO0_20>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIO0_20;
#[doc = "`read()` method returns [pio0_20::R](pio0_20::R) reader structure"]
impl crate::Readable for PIO0_20 {}
#[doc = "`write(|w| ..)` method takes [pio0_20::W](pio0_20::W) writer structure"]
impl crate::Writable for PIO0_20 {}
#[doc = "Digital I/O control for pins PIO0_20"]
pub mod pio0_20;
#[doc = "Digital I/O control for pins PIO0_19\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pio0_19](pio0_19) module"]
pub type PIO0_19 = crate::Reg<u32, _PIO0_19>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIO0_19;
#[doc = "`read()` method returns [pio0_19::R](pio0_19::R) reader structure"]
impl crate::Readable for PIO0_19 {}
#[doc = "`write(|w| ..)` method takes [pio0_19::W](pio0_19::W) writer structure"]
impl crate::Writable for PIO0_19 {}
#[doc = "Digital I/O control for pins PIO0_19"]
pub mod pio0_19;
#[doc = "Digital I/O control for pins PIO0_18\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pio0_18](pio0_18) module"]
pub type PIO0_18 = crate::Reg<u32, _PIO0_18>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIO0_18;
#[doc = "`read()` method returns [pio0_18::R](pio0_18::R) reader structure"]
impl crate::Readable for PIO0_18 {}
#[doc = "`write(|w| ..)` method takes [pio0_18::W](pio0_18::W) writer structure"]
impl crate::Writable for PIO0_18 {}
#[doc = "Digital I/O control for pins PIO0_18"]
pub mod pio0_18;
#[doc = "Digital I/O control for pins PIO1_8\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pio1_8](pio1_8) module"]
pub type PIO1_8 = crate::Reg<u32, _PIO1_8>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIO1_8;
#[doc = "`read()` method returns [pio1_8::R](pio1_8::R) reader structure"]
impl crate::Readable for PIO1_8 {}
#[doc = "`write(|w| ..)` method takes [pio1_8::W](pio1_8::W) writer structure"]
impl crate::Writable for PIO1_8 {}
#[doc = "Digital I/O control for pins PIO1_8"]
pub mod pio1_8;
#[doc = "Digital I/O control for pins PIO1_9\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pio1_9](pio1_9) module"]
pub type PIO1_9 = crate::Reg<u32, _PIO1_9>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIO1_9;
#[doc = "`read()` method returns [pio1_9::R](pio1_9::R) reader structure"]
impl crate::Readable for PIO1_9 {}
#[doc = "`write(|w| ..)` method takes [pio1_9::W](pio1_9::W) writer structure"]
impl crate::Writable for PIO1_9 {}
#[doc = "Digital I/O control for pins PIO1_9"]
pub mod pio1_9;
#[doc = "Digital I/O control for pins PIO1_12\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pio1_12](pio1_12) module"]
pub type PIO1_12 = crate::Reg<u32, _PIO1_12>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIO1_12;
#[doc = "`read()` method returns [pio1_12::R](pio1_12::R) reader structure"]
impl crate::Readable for PIO1_12 {}
#[doc = "`write(|w| ..)` method takes [pio1_12::W](pio1_12::W) writer structure"]
impl crate::Writable for PIO1_12 {}
#[doc = "Digital I/O control for pins PIO1_12"]
pub mod pio1_12;
#[doc = "Digital I/O control for pins PIO1_13\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pio1_13](pio1_13) module"]
pub type PIO1_13 = crate::Reg<u32, _PIO1_13>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIO1_13;
#[doc = "`read()` method returns [pio1_13::R](pio1_13::R) reader structure"]
impl crate::Readable for PIO1_13 {}
#[doc = "`write(|w| ..)` method takes [pio1_13::W](pio1_13::W) writer structure"]
impl crate::Writable for PIO1_13 {}
#[doc = "Digital I/O control for pins PIO1_13"]
pub mod pio1_13;
#[doc = "Digital I/O control for pins PIO0_31\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pio0_31](pio0_31) module"]
pub type PIO0_31 = crate::Reg<u32, _PIO0_31>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIO0_31;
#[doc = "`read()` method returns [pio0_31::R](pio0_31::R) reader structure"]
impl crate::Readable for PIO0_31 {}
#[doc = "`write(|w| ..)` method takes [pio0_31::W](pio0_31::W) writer structure"]
impl crate::Writable for PIO0_31 {}
#[doc = "Digital I/O control for pins PIO0_31"]
pub mod pio0_31;
#[doc = "Digital I/O control for pins PIO1_0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pio1_0](pio1_0) module"]
pub type PIO1_0 = crate::Reg<u32, _PIO1_0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIO1_0;
#[doc = "`read()` method returns [pio1_0::R](pio1_0::R) reader structure"]
impl crate::Readable for PIO1_0 {}
#[doc = "`write(|w| ..)` method takes [pio1_0::W](pio1_0::W) writer structure"]
impl crate::Writable for PIO1_0 {}
#[doc = "Digital I/O control for pins PIO1_0"]
pub mod pio1_0;
#[doc = "Digital I/O control for pins PIO1_1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pio1_1](pio1_1) module"]
pub type PIO1_1 = crate::Reg<u32, _PIO1_1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIO1_1;
#[doc = "`read()` method returns [pio1_1::R](pio1_1::R) reader structure"]
impl crate::Readable for PIO1_1 {}
#[doc = "`write(|w| ..)` method takes [pio1_1::W](pio1_1::W) writer structure"]
impl crate::Writable for PIO1_1 {}
#[doc = "Digital I/O control for pins PIO1_1"]
pub mod pio1_1;
#[doc = "Digital I/O control for pins PIO1_2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pio1_2](pio1_2) module"]
pub type PIO1_2 = crate::Reg<u32, _PIO1_2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIO1_2;
#[doc = "`read()` method returns [pio1_2::R](pio1_2::R) reader structure"]
impl crate::Readable for PIO1_2 {}
#[doc = "`write(|w| ..)` method takes [pio1_2::W](pio1_2::W) writer structure"]
impl crate::Writable for PIO1_2 {}
#[doc = "Digital I/O control for pins PIO1_2"]
pub mod pio1_2;
#[doc = "Digital I/O control for pins PIO1_14\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pio1_14](pio1_14) module"]
pub type PIO1_14 = crate::Reg<u32, _PIO1_14>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIO1_14;
#[doc = "`read()` method returns [pio1_14::R](pio1_14::R) reader structure"]
impl crate::Readable for PIO1_14 {}
#[doc = "`write(|w| ..)` method takes [pio1_14::W](pio1_14::W) writer structure"]
impl crate::Writable for PIO1_14 {}
#[doc = "Digital I/O control for pins PIO1_14"]
pub mod pio1_14;
#[doc = "Digital I/O control for pins PIO1_15\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pio1_15](pio1_15) module"]
pub type PIO1_15 = crate::Reg<u32, _PIO1_15>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIO1_15;
#[doc = "`read()` method returns [pio1_15::R](pio1_15::R) reader structure"]
impl crate::Readable for PIO1_15 {}
#[doc = "`write(|w| ..)` method takes [pio1_15::W](pio1_15::W) writer structure"]
impl crate::Writable for PIO1_15 {}
#[doc = "Digital I/O control for pins PIO1_15"]
pub mod pio1_15;
#[doc = "Digital I/O control for pins PIO1_3\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pio1_3](pio1_3) module"]
pub type PIO1_3 = crate::Reg<u32, _PIO1_3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIO1_3;
#[doc = "`read()` method returns [pio1_3::R](pio1_3::R) reader structure"]
impl crate::Readable for PIO1_3 {}
#[doc = "`write(|w| ..)` method takes [pio1_3::W](pio1_3::W) writer structure"]
impl crate::Writable for PIO1_3 {}
#[doc = "Digital I/O control for pins PIO1_3"]
pub mod pio1_3;
#[doc = "Digital I/O control for pins PIO1_4\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pio1_4](pio1_4) module"]
pub type PIO1_4 = crate::Reg<u32, _PIO1_4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIO1_4;
#[doc = "`read()` method returns [pio1_4::R](pio1_4::R) reader structure"]
impl crate::Readable for PIO1_4 {}
#[doc = "`write(|w| ..)` method takes [pio1_4::W](pio1_4::W) writer structure"]
impl crate::Writable for PIO1_4 {}
#[doc = "Digital I/O control for pins PIO1_4"]
pub mod pio1_4;
#[doc = "Digital I/O control for pins PIO1_5\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pio1_5](pio1_5) module"]
pub type PIO1_5 = crate::Reg<u32, _PIO1_5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIO1_5;
#[doc = "`read()` method returns [pio1_5::R](pio1_5::R) reader structure"]
impl crate::Readable for PIO1_5 {}
#[doc = "`write(|w| ..)` method takes [pio1_5::W](pio1_5::W) writer structure"]
impl crate::Writable for PIO1_5 {}
#[doc = "Digital I/O control for pins PIO1_5"]
pub mod pio1_5;
#[doc = "Digital I/O control for pins PIO1_16\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pio1_16](pio1_16) module"]
pub type PIO1_16 = crate::Reg<u32, _PIO1_16>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIO1_16;
#[doc = "`read()` method returns [pio1_16::R](pio1_16::R) reader structure"]
impl crate::Readable for PIO1_16 {}
#[doc = "`write(|w| ..)` method takes [pio1_16::W](pio1_16::W) writer structure"]
impl crate::Writable for PIO1_16 {}
#[doc = "Digital I/O control for pins PIO1_16"]
pub mod pio1_16;
#[doc = "Digital I/O control for pins PIO1_17\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pio1_17](pio1_17) module"]
pub type PIO1_17 = crate::Reg<u32, _PIO1_17>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIO1_17;
#[doc = "`read()` method returns [pio1_17::R](pio1_17::R) reader structure"]
impl crate::Readable for PIO1_17 {}
#[doc = "`write(|w| ..)` method takes [pio1_17::W](pio1_17::W) writer structure"]
impl crate::Writable for PIO1_17 {}
#[doc = "Digital I/O control for pins PIO1_17"]
pub mod pio1_17;
#[doc = "Digital I/O control for pins PIO1_6\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pio1_6](pio1_6) module"]
pub type PIO1_6 = crate::Reg<u32, _PIO1_6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIO1_6;
#[doc = "`read()` method returns [pio1_6::R](pio1_6::R) reader structure"]
impl crate::Readable for PIO1_6 {}
#[doc = "`write(|w| ..)` method takes [pio1_6::W](pio1_6::W) writer structure"]
impl crate::Writable for PIO1_6 {}
#[doc = "Digital I/O control for pins PIO1_6"]
pub mod pio1_6;
#[doc = "Digital I/O control for pins PIO1_18\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pio1_18](pio1_18) module"]
pub type PIO1_18 = crate::Reg<u32, _PIO1_18>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIO1_18;
#[doc = "`read()` method returns [pio1_18::R](pio1_18::R) reader structure"]
impl crate::Readable for PIO1_18 {}
#[doc = "`write(|w| ..)` method takes [pio1_18::W](pio1_18::W) writer structure"]
impl crate::Writable for PIO1_18 {}
#[doc = "Digital I/O control for pins PIO1_18"]
pub mod pio1_18;
#[doc = "Digital I/O control for pins PIO1_19\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pio1_19](pio1_19) module"]
pub type PIO1_19 = crate::Reg<u32, _PIO1_19>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIO1_19;
#[doc = "`read()` method returns [pio1_19::R](pio1_19::R) reader structure"]
impl crate::Readable for PIO1_19 {}
#[doc = "`write(|w| ..)` method takes [pio1_19::W](pio1_19::W) writer structure"]
impl crate::Writable for PIO1_19 {}
#[doc = "Digital I/O control for pins PIO1_19"]
pub mod pio1_19;
#[doc = "Digital I/O control for pins PIO1_7\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pio1_7](pio1_7) module"]
pub type PIO1_7 = crate::Reg<u32, _PIO1_7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIO1_7;
#[doc = "`read()` method returns [pio1_7::R](pio1_7::R) reader structure"]
impl crate::Readable for PIO1_7 {}
#[doc = "`write(|w| ..)` method takes [pio1_7::W](pio1_7::W) writer structure"]
impl crate::Writable for PIO1_7 {}
#[doc = "Digital I/O control for pins PIO1_7"]
pub mod pio1_7;
#[doc = "Digital I/O control for pins PIO0_29\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pio0_29](pio0_29) module"]
pub type PIO0_29 = crate::Reg<u32, _PIO0_29>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIO0_29;
#[doc = "`read()` method returns [pio0_29::R](pio0_29::R) reader structure"]
impl crate::Readable for PIO0_29 {}
#[doc = "`write(|w| ..)` method takes [pio0_29::W](pio0_29::W) writer structure"]
impl crate::Writable for PIO0_29 {}
#[doc = "Digital I/O control for pins PIO0_29"]
pub mod pio0_29;
#[doc = "Digital I/O control for pins PIO0_30\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pio0_30](pio0_30) module"]
pub type PIO0_30 = crate::Reg<u32, _PIO0_30>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIO0_30;
#[doc = "`read()` method returns [pio0_30::R](pio0_30::R) reader structure"]
impl crate::Readable for PIO0_30 {}
#[doc = "`write(|w| ..)` method takes [pio0_30::W](pio0_30::W) writer structure"]
impl crate::Writable for PIO0_30 {}
#[doc = "Digital I/O control for pins PIO0_30"]
pub mod pio0_30;
#[doc = "Digital I/O control for pins PIO1_20\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pio1_20](pio1_20) module"]
pub type PIO1_20 = crate::Reg<u32, _PIO1_20>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIO1_20;
#[doc = "`read()` method returns [pio1_20::R](pio1_20::R) reader structure"]
impl crate::Readable for PIO1_20 {}
#[doc = "`write(|w| ..)` method takes [pio1_20::W](pio1_20::W) writer structure"]
impl crate::Writable for PIO1_20 {}
#[doc = "Digital I/O control for pins PIO1_20"]
pub mod pio1_20;
#[doc = "Digital I/O control for pins PIO1_21\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pio1_21](pio1_21) module"]
pub type PIO1_21 = crate::Reg<u32, _PIO1_21>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIO1_21;
#[doc = "`read()` method returns [pio1_21::R](pio1_21::R) reader structure"]
impl crate::Readable for PIO1_21 {}
#[doc = "`write(|w| ..)` method takes [pio1_21::W](pio1_21::W) writer structure"]
impl crate::Writable for PIO1_21 {}
#[doc = "Digital I/O control for pins PIO1_21"]
pub mod pio1_21;
#[doc = "Digital I/O control for pins PIO1_11\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pio1_11](pio1_11) module"]
pub type PIO1_11 = crate::Reg<u32, _PIO1_11>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIO1_11;
#[doc = "`read()` method returns [pio1_11::R](pio1_11::R) reader structure"]
impl crate::Readable for PIO1_11 {}
#[doc = "`write(|w| ..)` method takes [pio1_11::W](pio1_11::W) writer structure"]
impl crate::Writable for PIO1_11 {}
#[doc = "Digital I/O control for pins PIO1_11"]
pub mod pio1_11;
#[doc = "Digital I/O control for pins PIO1_10\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pio1_10](pio1_10) module"]
pub type PIO1_10 = crate::Reg<u32, _PIO1_10>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIO1_10;
#[doc = "`read()` method returns [pio1_10::R](pio1_10::R) reader structure"]
impl crate::Readable for PIO1_10 {}
#[doc = "`write(|w| ..)` method takes [pio1_10::W](pio1_10::W) writer structure"]
impl crate::Writable for PIO1_10 {}
#[doc = "Digital I/O control for pins PIO1_10"]
pub mod pio1_10;
