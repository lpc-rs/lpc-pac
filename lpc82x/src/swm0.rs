#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Pin assign register 0. Assign movable functions U0_TXD, U0_RXD, U0_RTS, U0_CTS."]
    pub pinassign0: PINASSIGN0,
    #[doc = "0x04 - Pin assign register 1. Assign movable functions U0_SCLK, U1_TXD, U1_RXD, U1_RTS."]
    pub pinassign1: PINASSIGN1,
    #[doc = "0x08 - Pin assign register 2. Assign movable functions U1_CTS, U1_SCLK, U2_TXD, U2_RXD."]
    pub pinassign2: PINASSIGN2,
    #[doc = "0x0c - Pin assign register 3. Assign movable function U2_RTS, U2_CTS, U2_SCLK, SPI0_SCK."]
    pub pinassign3: PINASSIGN3,
    #[doc = "0x10 - Pin assign register 4. Assign movable functions SPI0_MOSI, SPI0_MISO, SPI0_SSEL0, SPI0_SSEL1."]
    pub pinassign4: PINASSIGN4,
    #[doc = "0x14 - Pin assign register 5. Assign movable functions SPI0_SSEL2, SPI0_SSEL3, SPI1_SCK, SPI1_MOSI"]
    pub pinassign5: PINASSIGN5,
    #[doc = "0x18 - Pin assign register 6. Assign movable functions SPI1_MISO, SPI1_SSEL0, SPI1_SSEL1, SCT0_IN0."]
    pub pinassign6: PINASSIGN6,
    #[doc = "0x1c - Pin assign register 7. Assign movable functions SCT_IN1, SCT_IN2, SCT_IN3, SCT_OUT0."]
    pub pinassign7: PINASSIGN7,
    #[doc = "0x20 - Pin assign register 8. Assign movable functions SCT_OUT1, SCT_OUT2, SCT_OUT3, SCT_OUT4."]
    pub pinassign8: PINASSIGN8,
    #[doc = "0x24 - Pin assign register 9. Assign movable functions SCT_OUT5, I2C1_SDA, I2C1_SCL, I2C2_SDA."]
    pub pinassign9: PINASSIGN9,
    #[doc = "0x28 - Pin assign register 10. Assign movable functions, I2C2_SCL, I2C3_SDA, I2C3_SCL, ADC_PINTRIG0."]
    pub pinassign10: PINASSIGN10,
    #[doc = "0x2c - Pin assign register 11. Assign movable functions ADC_PINTRIG1, ACMP_O, CLKOUT, GPIO_INT_BMAT"]
    pub pinassign11: PINASSIGN11,
    _reserved12: [u8; 400usize],
    #[doc = "0x1c0 - Pin enable register 0. Enables fixed-pin functions ACMP_I0, ACMP_I1, SWCLK, SWDIO, XTALIN, XTALOUT, RESET, CLKIN, VDDCMP and so on."]
    pub pinenable0: PINENABLE0,
}
#[doc = "Pin assign register 0. Assign movable functions U0_TXD, U0_RXD, U0_RTS, U0_CTS.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pinassign0](pinassign0) module"]
pub type PINASSIGN0 = crate::Reg<u32, _PINASSIGN0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PINASSIGN0;
#[doc = "`read()` method returns [pinassign0::R](pinassign0::R) reader structure"]
impl crate::Readable for PINASSIGN0 {}
#[doc = "`write(|w| ..)` method takes [pinassign0::W](pinassign0::W) writer structure"]
impl crate::Writable for PINASSIGN0 {}
#[doc = "Pin assign register 0. Assign movable functions U0_TXD, U0_RXD, U0_RTS, U0_CTS."]
pub mod pinassign0;
#[doc = "Pin assign register 1. Assign movable functions U0_SCLK, U1_TXD, U1_RXD, U1_RTS.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pinassign1](pinassign1) module"]
pub type PINASSIGN1 = crate::Reg<u32, _PINASSIGN1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PINASSIGN1;
#[doc = "`read()` method returns [pinassign1::R](pinassign1::R) reader structure"]
impl crate::Readable for PINASSIGN1 {}
#[doc = "`write(|w| ..)` method takes [pinassign1::W](pinassign1::W) writer structure"]
impl crate::Writable for PINASSIGN1 {}
#[doc = "Pin assign register 1. Assign movable functions U0_SCLK, U1_TXD, U1_RXD, U1_RTS."]
pub mod pinassign1;
#[doc = "Pin assign register 2. Assign movable functions U1_CTS, U1_SCLK, U2_TXD, U2_RXD.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pinassign2](pinassign2) module"]
pub type PINASSIGN2 = crate::Reg<u32, _PINASSIGN2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PINASSIGN2;
#[doc = "`read()` method returns [pinassign2::R](pinassign2::R) reader structure"]
impl crate::Readable for PINASSIGN2 {}
#[doc = "`write(|w| ..)` method takes [pinassign2::W](pinassign2::W) writer structure"]
impl crate::Writable for PINASSIGN2 {}
#[doc = "Pin assign register 2. Assign movable functions U1_CTS, U1_SCLK, U2_TXD, U2_RXD."]
pub mod pinassign2;
#[doc = "Pin assign register 3. Assign movable function U2_RTS, U2_CTS, U2_SCLK, SPI0_SCK.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pinassign3](pinassign3) module"]
pub type PINASSIGN3 = crate::Reg<u32, _PINASSIGN3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PINASSIGN3;
#[doc = "`read()` method returns [pinassign3::R](pinassign3::R) reader structure"]
impl crate::Readable for PINASSIGN3 {}
#[doc = "`write(|w| ..)` method takes [pinassign3::W](pinassign3::W) writer structure"]
impl crate::Writable for PINASSIGN3 {}
#[doc = "Pin assign register 3. Assign movable function U2_RTS, U2_CTS, U2_SCLK, SPI0_SCK."]
pub mod pinassign3;
#[doc = "Pin assign register 4. Assign movable functions SPI0_MOSI, SPI0_MISO, SPI0_SSEL0, SPI0_SSEL1.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pinassign4](pinassign4) module"]
pub type PINASSIGN4 = crate::Reg<u32, _PINASSIGN4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PINASSIGN4;
#[doc = "`read()` method returns [pinassign4::R](pinassign4::R) reader structure"]
impl crate::Readable for PINASSIGN4 {}
#[doc = "`write(|w| ..)` method takes [pinassign4::W](pinassign4::W) writer structure"]
impl crate::Writable for PINASSIGN4 {}
#[doc = "Pin assign register 4. Assign movable functions SPI0_MOSI, SPI0_MISO, SPI0_SSEL0, SPI0_SSEL1."]
pub mod pinassign4;
#[doc = "Pin assign register 5. Assign movable functions SPI0_SSEL2, SPI0_SSEL3, SPI1_SCK, SPI1_MOSI\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pinassign5](pinassign5) module"]
pub type PINASSIGN5 = crate::Reg<u32, _PINASSIGN5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PINASSIGN5;
#[doc = "`read()` method returns [pinassign5::R](pinassign5::R) reader structure"]
impl crate::Readable for PINASSIGN5 {}
#[doc = "`write(|w| ..)` method takes [pinassign5::W](pinassign5::W) writer structure"]
impl crate::Writable for PINASSIGN5 {}
#[doc = "Pin assign register 5. Assign movable functions SPI0_SSEL2, SPI0_SSEL3, SPI1_SCK, SPI1_MOSI"]
pub mod pinassign5;
#[doc = "Pin assign register 6. Assign movable functions SPI1_MISO, SPI1_SSEL0, SPI1_SSEL1, SCT0_IN0.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pinassign6](pinassign6) module"]
pub type PINASSIGN6 = crate::Reg<u32, _PINASSIGN6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PINASSIGN6;
#[doc = "`read()` method returns [pinassign6::R](pinassign6::R) reader structure"]
impl crate::Readable for PINASSIGN6 {}
#[doc = "`write(|w| ..)` method takes [pinassign6::W](pinassign6::W) writer structure"]
impl crate::Writable for PINASSIGN6 {}
#[doc = "Pin assign register 6. Assign movable functions SPI1_MISO, SPI1_SSEL0, SPI1_SSEL1, SCT0_IN0."]
pub mod pinassign6;
#[doc = "Pin assign register 7. Assign movable functions SCT_IN1, SCT_IN2, SCT_IN3, SCT_OUT0.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pinassign7](pinassign7) module"]
pub type PINASSIGN7 = crate::Reg<u32, _PINASSIGN7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PINASSIGN7;
#[doc = "`read()` method returns [pinassign7::R](pinassign7::R) reader structure"]
impl crate::Readable for PINASSIGN7 {}
#[doc = "`write(|w| ..)` method takes [pinassign7::W](pinassign7::W) writer structure"]
impl crate::Writable for PINASSIGN7 {}
#[doc = "Pin assign register 7. Assign movable functions SCT_IN1, SCT_IN2, SCT_IN3, SCT_OUT0."]
pub mod pinassign7;
#[doc = "Pin assign register 8. Assign movable functions SCT_OUT1, SCT_OUT2, SCT_OUT3, SCT_OUT4.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pinassign8](pinassign8) module"]
pub type PINASSIGN8 = crate::Reg<u32, _PINASSIGN8>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PINASSIGN8;
#[doc = "`read()` method returns [pinassign8::R](pinassign8::R) reader structure"]
impl crate::Readable for PINASSIGN8 {}
#[doc = "`write(|w| ..)` method takes [pinassign8::W](pinassign8::W) writer structure"]
impl crate::Writable for PINASSIGN8 {}
#[doc = "Pin assign register 8. Assign movable functions SCT_OUT1, SCT_OUT2, SCT_OUT3, SCT_OUT4."]
pub mod pinassign8;
#[doc = "Pin assign register 9. Assign movable functions SCT_OUT5, I2C1_SDA, I2C1_SCL, I2C2_SDA.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pinassign9](pinassign9) module"]
pub type PINASSIGN9 = crate::Reg<u32, _PINASSIGN9>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PINASSIGN9;
#[doc = "`read()` method returns [pinassign9::R](pinassign9::R) reader structure"]
impl crate::Readable for PINASSIGN9 {}
#[doc = "`write(|w| ..)` method takes [pinassign9::W](pinassign9::W) writer structure"]
impl crate::Writable for PINASSIGN9 {}
#[doc = "Pin assign register 9. Assign movable functions SCT_OUT5, I2C1_SDA, I2C1_SCL, I2C2_SDA."]
pub mod pinassign9;
#[doc = "Pin assign register 10. Assign movable functions, I2C2_SCL, I2C3_SDA, I2C3_SCL, ADC_PINTRIG0.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pinassign10](pinassign10) module"]
pub type PINASSIGN10 = crate::Reg<u32, _PINASSIGN10>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PINASSIGN10;
#[doc = "`read()` method returns [pinassign10::R](pinassign10::R) reader structure"]
impl crate::Readable for PINASSIGN10 {}
#[doc = "`write(|w| ..)` method takes [pinassign10::W](pinassign10::W) writer structure"]
impl crate::Writable for PINASSIGN10 {}
#[doc = "Pin assign register 10. Assign movable functions, I2C2_SCL, I2C3_SDA, I2C3_SCL, ADC_PINTRIG0."]
pub mod pinassign10;
#[doc = "Pin assign register 11. Assign movable functions ADC_PINTRIG1, ACMP_O, CLKOUT, GPIO_INT_BMAT\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pinassign11](pinassign11) module"]
pub type PINASSIGN11 = crate::Reg<u32, _PINASSIGN11>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PINASSIGN11;
#[doc = "`read()` method returns [pinassign11::R](pinassign11::R) reader structure"]
impl crate::Readable for PINASSIGN11 {}
#[doc = "`write(|w| ..)` method takes [pinassign11::W](pinassign11::W) writer structure"]
impl crate::Writable for PINASSIGN11 {}
#[doc = "Pin assign register 11. Assign movable functions ADC_PINTRIG1, ACMP_O, CLKOUT, GPIO_INT_BMAT"]
pub mod pinassign11;
#[doc = "Pin enable register 0. Enables fixed-pin functions ACMP_I0, ACMP_I1, SWCLK, SWDIO, XTALIN, XTALOUT, RESET, CLKIN, VDDCMP and so on.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pinenable0](pinenable0) module"]
pub type PINENABLE0 = crate::Reg<u32, _PINENABLE0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PINENABLE0;
#[doc = "`read()` method returns [pinenable0::R](pinenable0::R) reader structure"]
impl crate::Readable for PINENABLE0 {}
#[doc = "`write(|w| ..)` method takes [pinenable0::W](pinenable0::W) writer structure"]
impl crate::Writable for PINENABLE0 {}
#[doc = "Pin enable register 0. Enables fixed-pin functions ACMP_I0, ACMP_I1, SWCLK, SWDIO, XTALIN, XTALOUT, RESET, CLKIN, VDDCMP and so on."]
pub mod pinenable0;
