#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Pin function select register 0."]
    pub pinsel0: PINSEL0,
    #[doc = "0x04 - Pin function select register 1."]
    pub pinsel1: PINSEL1,
    #[doc = "0x08 - Pin function select register 2."]
    pub pinsel2: PINSEL2,
    #[doc = "0x0c - Pin function select register 3."]
    pub pinsel3: PINSEL3,
    #[doc = "0x10 - Pin function select register 4"]
    pub pinsel4: PINSEL4,
    _reserved5: [u8; 8usize],
    #[doc = "0x1c - Pin function select register 7"]
    pub pinsel7: PINSEL7,
    _reserved6: [u8; 4usize],
    #[doc = "0x24 - Pin function select register 9"]
    pub pinsel9: PINSEL9,
    #[doc = "0x28 - Pin function select register 10"]
    pub pinsel10: PINSEL10,
    _reserved8: [u8; 20usize],
    #[doc = "0x40 - Pin mode select register 0"]
    pub pinmode0: PINMODE0,
    #[doc = "0x44 - Pin mode select register 1"]
    pub pinmode1: PINMODE1,
    #[doc = "0x48 - Pin mode select register 2"]
    pub pinmode2: PINMODE2,
    #[doc = "0x4c - Pin mode select register 3."]
    pub pinmode3: PINMODE3,
    #[doc = "0x50 - Pin mode select register 4"]
    pub pinmode4: PINMODE4,
    _reserved13: [u8; 8usize],
    #[doc = "0x5c - Pin mode select register 7"]
    pub pinmode7: PINMODE7,
    _reserved14: [u8; 4usize],
    #[doc = "0x64 - Pin mode select register 9"]
    pub pinmode9: PINMODE9,
    #[doc = "0x68 - Open drain mode control register 0"]
    pub pinmode_od0: PINMODE_OD0,
    #[doc = "0x6c - Open drain mode control register 1"]
    pub pinmode_od1: PINMODE_OD1,
    #[doc = "0x70 - Open drain mode control register 2"]
    pub pinmode_od2: PINMODE_OD2,
    #[doc = "0x74 - Open drain mode control register 3"]
    pub pinmode_od3: PINMODE_OD3,
    #[doc = "0x78 - Open drain mode control register 4"]
    pub pinmode_od4: PINMODE_OD4,
    #[doc = "0x7c - I2C Pin Configuration register"]
    pub i2cpadcfg: I2CPADCFG,
}
#[doc = "Pin function select register 0.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pinsel0](pinsel0) module"]
pub type PINSEL0 = crate::Reg<u32, _PINSEL0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PINSEL0;
#[doc = "`read()` method returns [pinsel0::R](pinsel0::R) reader structure"]
impl crate::Readable for PINSEL0 {}
#[doc = "`write(|w| ..)` method takes [pinsel0::W](pinsel0::W) writer structure"]
impl crate::Writable for PINSEL0 {}
#[doc = "Pin function select register 0."]
pub mod pinsel0;
#[doc = "Pin function select register 1.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pinsel1](pinsel1) module"]
pub type PINSEL1 = crate::Reg<u32, _PINSEL1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PINSEL1;
#[doc = "`read()` method returns [pinsel1::R](pinsel1::R) reader structure"]
impl crate::Readable for PINSEL1 {}
#[doc = "`write(|w| ..)` method takes [pinsel1::W](pinsel1::W) writer structure"]
impl crate::Writable for PINSEL1 {}
#[doc = "Pin function select register 1."]
pub mod pinsel1;
#[doc = "Pin function select register 2.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pinsel2](pinsel2) module"]
pub type PINSEL2 = crate::Reg<u32, _PINSEL2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PINSEL2;
#[doc = "`read()` method returns [pinsel2::R](pinsel2::R) reader structure"]
impl crate::Readable for PINSEL2 {}
#[doc = "`write(|w| ..)` method takes [pinsel2::W](pinsel2::W) writer structure"]
impl crate::Writable for PINSEL2 {}
#[doc = "Pin function select register 2."]
pub mod pinsel2;
#[doc = "Pin function select register 3.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pinsel3](pinsel3) module"]
pub type PINSEL3 = crate::Reg<u32, _PINSEL3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PINSEL3;
#[doc = "`read()` method returns [pinsel3::R](pinsel3::R) reader structure"]
impl crate::Readable for PINSEL3 {}
#[doc = "`write(|w| ..)` method takes [pinsel3::W](pinsel3::W) writer structure"]
impl crate::Writable for PINSEL3 {}
#[doc = "Pin function select register 3."]
pub mod pinsel3;
#[doc = "Pin function select register 4\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pinsel4](pinsel4) module"]
pub type PINSEL4 = crate::Reg<u32, _PINSEL4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PINSEL4;
#[doc = "`read()` method returns [pinsel4::R](pinsel4::R) reader structure"]
impl crate::Readable for PINSEL4 {}
#[doc = "`write(|w| ..)` method takes [pinsel4::W](pinsel4::W) writer structure"]
impl crate::Writable for PINSEL4 {}
#[doc = "Pin function select register 4"]
pub mod pinsel4;
#[doc = "Pin function select register 7\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pinsel7](pinsel7) module"]
pub type PINSEL7 = crate::Reg<u32, _PINSEL7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PINSEL7;
#[doc = "`read()` method returns [pinsel7::R](pinsel7::R) reader structure"]
impl crate::Readable for PINSEL7 {}
#[doc = "`write(|w| ..)` method takes [pinsel7::W](pinsel7::W) writer structure"]
impl crate::Writable for PINSEL7 {}
#[doc = "Pin function select register 7"]
pub mod pinsel7;
#[doc = "Pin function select register 9\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pinsel9](pinsel9) module"]
pub type PINSEL9 = crate::Reg<u32, _PINSEL9>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PINSEL9;
#[doc = "`read()` method returns [pinsel9::R](pinsel9::R) reader structure"]
impl crate::Readable for PINSEL9 {}
#[doc = "`write(|w| ..)` method takes [pinsel9::W](pinsel9::W) writer structure"]
impl crate::Writable for PINSEL9 {}
#[doc = "Pin function select register 9"]
pub mod pinsel9;
#[doc = "Pin function select register 10\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pinsel10](pinsel10) module"]
pub type PINSEL10 = crate::Reg<u32, _PINSEL10>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PINSEL10;
#[doc = "`read()` method returns [pinsel10::R](pinsel10::R) reader structure"]
impl crate::Readable for PINSEL10 {}
#[doc = "`write(|w| ..)` method takes [pinsel10::W](pinsel10::W) writer structure"]
impl crate::Writable for PINSEL10 {}
#[doc = "Pin function select register 10"]
pub mod pinsel10;
#[doc = "Pin mode select register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pinmode0](pinmode0) module"]
pub type PINMODE0 = crate::Reg<u32, _PINMODE0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PINMODE0;
#[doc = "`read()` method returns [pinmode0::R](pinmode0::R) reader structure"]
impl crate::Readable for PINMODE0 {}
#[doc = "`write(|w| ..)` method takes [pinmode0::W](pinmode0::W) writer structure"]
impl crate::Writable for PINMODE0 {}
#[doc = "Pin mode select register 0"]
pub mod pinmode0;
#[doc = "Pin mode select register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pinmode1](pinmode1) module"]
pub type PINMODE1 = crate::Reg<u32, _PINMODE1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PINMODE1;
#[doc = "`read()` method returns [pinmode1::R](pinmode1::R) reader structure"]
impl crate::Readable for PINMODE1 {}
#[doc = "`write(|w| ..)` method takes [pinmode1::W](pinmode1::W) writer structure"]
impl crate::Writable for PINMODE1 {}
#[doc = "Pin mode select register 1"]
pub mod pinmode1;
#[doc = "Pin mode select register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pinmode2](pinmode2) module"]
pub type PINMODE2 = crate::Reg<u32, _PINMODE2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PINMODE2;
#[doc = "`read()` method returns [pinmode2::R](pinmode2::R) reader structure"]
impl crate::Readable for PINMODE2 {}
#[doc = "`write(|w| ..)` method takes [pinmode2::W](pinmode2::W) writer structure"]
impl crate::Writable for PINMODE2 {}
#[doc = "Pin mode select register 2"]
pub mod pinmode2;
#[doc = "Pin mode select register 3.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pinmode3](pinmode3) module"]
pub type PINMODE3 = crate::Reg<u32, _PINMODE3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PINMODE3;
#[doc = "`read()` method returns [pinmode3::R](pinmode3::R) reader structure"]
impl crate::Readable for PINMODE3 {}
#[doc = "`write(|w| ..)` method takes [pinmode3::W](pinmode3::W) writer structure"]
impl crate::Writable for PINMODE3 {}
#[doc = "Pin mode select register 3."]
pub mod pinmode3;
#[doc = "Pin mode select register 4\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pinmode4](pinmode4) module"]
pub type PINMODE4 = crate::Reg<u32, _PINMODE4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PINMODE4;
#[doc = "`read()` method returns [pinmode4::R](pinmode4::R) reader structure"]
impl crate::Readable for PINMODE4 {}
#[doc = "`write(|w| ..)` method takes [pinmode4::W](pinmode4::W) writer structure"]
impl crate::Writable for PINMODE4 {}
#[doc = "Pin mode select register 4"]
pub mod pinmode4;
#[doc = "Pin mode select register 7\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pinmode7](pinmode7) module"]
pub type PINMODE7 = crate::Reg<u32, _PINMODE7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PINMODE7;
#[doc = "`read()` method returns [pinmode7::R](pinmode7::R) reader structure"]
impl crate::Readable for PINMODE7 {}
#[doc = "`write(|w| ..)` method takes [pinmode7::W](pinmode7::W) writer structure"]
impl crate::Writable for PINMODE7 {}
#[doc = "Pin mode select register 7"]
pub mod pinmode7;
#[doc = "Pin mode select register 9\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pinmode9](pinmode9) module"]
pub type PINMODE9 = crate::Reg<u32, _PINMODE9>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PINMODE9;
#[doc = "`read()` method returns [pinmode9::R](pinmode9::R) reader structure"]
impl crate::Readable for PINMODE9 {}
#[doc = "`write(|w| ..)` method takes [pinmode9::W](pinmode9::W) writer structure"]
impl crate::Writable for PINMODE9 {}
#[doc = "Pin mode select register 9"]
pub mod pinmode9;
#[doc = "Open drain mode control register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pinmode_od0](pinmode_od0) module"]
pub type PINMODE_OD0 = crate::Reg<u32, _PINMODE_OD0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PINMODE_OD0;
#[doc = "`read()` method returns [pinmode_od0::R](pinmode_od0::R) reader structure"]
impl crate::Readable for PINMODE_OD0 {}
#[doc = "`write(|w| ..)` method takes [pinmode_od0::W](pinmode_od0::W) writer structure"]
impl crate::Writable for PINMODE_OD0 {}
#[doc = "Open drain mode control register 0"]
pub mod pinmode_od0;
#[doc = "Open drain mode control register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pinmode_od1](pinmode_od1) module"]
pub type PINMODE_OD1 = crate::Reg<u32, _PINMODE_OD1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PINMODE_OD1;
#[doc = "`read()` method returns [pinmode_od1::R](pinmode_od1::R) reader structure"]
impl crate::Readable for PINMODE_OD1 {}
#[doc = "`write(|w| ..)` method takes [pinmode_od1::W](pinmode_od1::W) writer structure"]
impl crate::Writable for PINMODE_OD1 {}
#[doc = "Open drain mode control register 1"]
pub mod pinmode_od1;
#[doc = "Open drain mode control register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pinmode_od2](pinmode_od2) module"]
pub type PINMODE_OD2 = crate::Reg<u32, _PINMODE_OD2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PINMODE_OD2;
#[doc = "`read()` method returns [pinmode_od2::R](pinmode_od2::R) reader structure"]
impl crate::Readable for PINMODE_OD2 {}
#[doc = "`write(|w| ..)` method takes [pinmode_od2::W](pinmode_od2::W) writer structure"]
impl crate::Writable for PINMODE_OD2 {}
#[doc = "Open drain mode control register 2"]
pub mod pinmode_od2;
#[doc = "Open drain mode control register 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pinmode_od3](pinmode_od3) module"]
pub type PINMODE_OD3 = crate::Reg<u32, _PINMODE_OD3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PINMODE_OD3;
#[doc = "`read()` method returns [pinmode_od3::R](pinmode_od3::R) reader structure"]
impl crate::Readable for PINMODE_OD3 {}
#[doc = "`write(|w| ..)` method takes [pinmode_od3::W](pinmode_od3::W) writer structure"]
impl crate::Writable for PINMODE_OD3 {}
#[doc = "Open drain mode control register 3"]
pub mod pinmode_od3;
#[doc = "Open drain mode control register 4\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pinmode_od4](pinmode_od4) module"]
pub type PINMODE_OD4 = crate::Reg<u32, _PINMODE_OD4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PINMODE_OD4;
#[doc = "`read()` method returns [pinmode_od4::R](pinmode_od4::R) reader structure"]
impl crate::Readable for PINMODE_OD4 {}
#[doc = "`write(|w| ..)` method takes [pinmode_od4::W](pinmode_od4::W) writer structure"]
impl crate::Writable for PINMODE_OD4 {}
#[doc = "Open drain mode control register 4"]
pub mod pinmode_od4;
#[doc = "I2C Pin Configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [i2cpadcfg](i2cpadcfg) module"]
pub type I2CPADCFG = crate::Reg<u32, _I2CPADCFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _I2CPADCFG;
#[doc = "`read()` method returns [i2cpadcfg::R](i2cpadcfg::R) reader structure"]
impl crate::Readable for I2CPADCFG {}
#[doc = "`write(|w| ..)` method takes [i2cpadcfg::W](i2cpadcfg::W) writer structure"]
impl crate::Writable for I2CPADCFG {}
#[doc = "I2C Pin Configuration register"]
pub mod i2cpadcfg;
