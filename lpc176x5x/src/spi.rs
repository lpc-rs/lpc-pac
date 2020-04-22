#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - SPI Control Register. This register controls the operation of the SPI."]
    pub cr: CR,
    #[doc = "0x04 - SPI Status Register. This register shows the status of the SPI."]
    pub sr: SR,
    #[doc = "0x08 - SPI Data Register. This bi-directional register provides the transmit and receive data for the SPI. Transmit data is provided to the SPI0 by writing to this register. Data received by the SPI0 can be read from this register."]
    pub dr: DR,
    #[doc = "0x0c - SPI Clock Counter Register. This register controls the frequency of a master's SCK0."]
    pub ccr: CCR,
    _reserved4: [u8; 12usize],
    #[doc = "0x1c - SPI Interrupt Flag. This register contains the interrupt flag for the SPI interface."]
    pub int: INT,
}
#[doc = "SPI Control Register. This register controls the operation of the SPI.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cr](cr) module"]
pub type CR = crate::Reg<u32, _CR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CR;
#[doc = "`read()` method returns [cr::R](cr::R) reader structure"]
impl crate::Readable for CR {}
#[doc = "`write(|w| ..)` method takes [cr::W](cr::W) writer structure"]
impl crate::Writable for CR {}
#[doc = "SPI Control Register. This register controls the operation of the SPI."]
pub mod cr;
#[doc = "SPI Status Register. This register shows the status of the SPI.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sr](sr) module"]
pub type SR = crate::Reg<u32, _SR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SR;
#[doc = "`read()` method returns [sr::R](sr::R) reader structure"]
impl crate::Readable for SR {}
#[doc = "SPI Status Register. This register shows the status of the SPI."]
pub mod sr;
#[doc = "SPI Data Register. This bi-directional register provides the transmit and receive data for the SPI. Transmit data is provided to the SPI0 by writing to this register. Data received by the SPI0 can be read from this register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dr](dr) module"]
pub type DR = crate::Reg<u32, _DR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DR;
#[doc = "`read()` method returns [dr::R](dr::R) reader structure"]
impl crate::Readable for DR {}
#[doc = "`write(|w| ..)` method takes [dr::W](dr::W) writer structure"]
impl crate::Writable for DR {}
#[doc = "SPI Data Register. This bi-directional register provides the transmit and receive data for the SPI. Transmit data is provided to the SPI0 by writing to this register. Data received by the SPI0 can be read from this register."]
pub mod dr;
#[doc = "SPI Clock Counter Register. This register controls the frequency of a master's SCK0.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ccr](ccr) module"]
pub type CCR = crate::Reg<u32, _CCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CCR;
#[doc = "`read()` method returns [ccr::R](ccr::R) reader structure"]
impl crate::Readable for CCR {}
#[doc = "`write(|w| ..)` method takes [ccr::W](ccr::W) writer structure"]
impl crate::Writable for CCR {}
#[doc = "SPI Clock Counter Register. This register controls the frequency of a master's SCK0."]
pub mod ccr;
#[doc = "SPI Interrupt Flag. This register contains the interrupt flag for the SPI interface.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [int](int) module"]
pub type INT = crate::Reg<u32, _INT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INT;
#[doc = "`read()` method returns [int::R](int::R) reader structure"]
impl crate::Readable for INT {}
#[doc = "`write(|w| ..)` method takes [int::W](int::W) writer structure"]
impl crate::Writable for INT {}
#[doc = "SPI Interrupt Flag. This register contains the interrupt flag for the SPI interface."]
pub mod int;
