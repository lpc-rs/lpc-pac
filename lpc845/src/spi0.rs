#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - SPI Configuration register"]
    pub cfg: CFG,
    #[doc = "0x04 - SPI Delay register"]
    pub dly: DLY,
    #[doc = "0x08 - SPI Status. Some status flags can be cleared by writing a 1 to that bit position"]
    pub stat: STAT,
    #[doc = "0x0c - SPI Interrupt Enable read and Set. A complete value may be read from this register. Writing a 1 to any implemented bit position causes that bit to be set."]
    pub intenset: INTENSET,
    #[doc = "0x10 - SPI Interrupt Enable Clear. Writing a 1 to any implemented bit position causes the corresponding bit in INTENSET to be cleared."]
    pub intenclr: INTENCLR,
    #[doc = "0x14 - SPI Receive Data"]
    pub rxdat: RXDAT,
    #[doc = "0x18 - SPI Transmit Data with Control"]
    pub txdatctl: TXDATCTL,
    #[doc = "0x1c - SPI Transmit Data."]
    pub txdat: TXDAT,
    #[doc = "0x20 - SPI Transmit Control"]
    pub txctl: TXCTL,
    #[doc = "0x24 - SPI clock Divider"]
    pub div: DIV,
    #[doc = "0x28 - SPI Interrupt Status"]
    pub intstat: INTSTAT,
}
#[doc = "SPI Configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub type CFG = crate::Reg<u32, _CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CFG;
#[doc = "`read()` method returns [cfg::R](cfg::R) reader structure"]
impl crate::Readable for CFG {}
#[doc = "`write(|w| ..)` method takes [cfg::W](cfg::W) writer structure"]
impl crate::Writable for CFG {}
#[doc = "SPI Configuration register"]
pub mod cfg;
#[doc = "SPI Delay register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dly](dly) module"]
pub type DLY = crate::Reg<u32, _DLY>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DLY;
#[doc = "`read()` method returns [dly::R](dly::R) reader structure"]
impl crate::Readable for DLY {}
#[doc = "`write(|w| ..)` method takes [dly::W](dly::W) writer structure"]
impl crate::Writable for DLY {}
#[doc = "SPI Delay register"]
pub mod dly;
#[doc = "SPI Status. Some status flags can be cleared by writing a 1 to that bit position\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [stat](stat) module"]
pub type STAT = crate::Reg<u32, _STAT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _STAT;
#[doc = "`read()` method returns [stat::R](stat::R) reader structure"]
impl crate::Readable for STAT {}
#[doc = "`write(|w| ..)` method takes [stat::W](stat::W) writer structure"]
impl crate::Writable for STAT {}
#[doc = "SPI Status. Some status flags can be cleared by writing a 1 to that bit position"]
pub mod stat;
#[doc = "SPI Interrupt Enable read and Set. A complete value may be read from this register. Writing a 1 to any implemented bit position causes that bit to be set.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intenset](intenset) module"]
pub type INTENSET = crate::Reg<u32, _INTENSET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTENSET;
#[doc = "`read()` method returns [intenset::R](intenset::R) reader structure"]
impl crate::Readable for INTENSET {}
#[doc = "`write(|w| ..)` method takes [intenset::W](intenset::W) writer structure"]
impl crate::Writable for INTENSET {}
#[doc = "SPI Interrupt Enable read and Set. A complete value may be read from this register. Writing a 1 to any implemented bit position causes that bit to be set."]
pub mod intenset;
#[doc = "SPI Interrupt Enable Clear. Writing a 1 to any implemented bit position causes the corresponding bit in INTENSET to be cleared.\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intenclr](intenclr) module"]
pub type INTENCLR = crate::Reg<u32, _INTENCLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTENCLR;
#[doc = "`write(|w| ..)` method takes [intenclr::W](intenclr::W) writer structure"]
impl crate::Writable for INTENCLR {}
#[doc = "SPI Interrupt Enable Clear. Writing a 1 to any implemented bit position causes the corresponding bit in INTENSET to be cleared."]
pub mod intenclr;
#[doc = "SPI Receive Data\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rxdat](rxdat) module"]
pub type RXDAT = crate::Reg<u32, _RXDAT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RXDAT;
#[doc = "`read()` method returns [rxdat::R](rxdat::R) reader structure"]
impl crate::Readable for RXDAT {}
#[doc = "SPI Receive Data"]
pub mod rxdat;
#[doc = "SPI Transmit Data with Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [txdatctl](txdatctl) module"]
pub type TXDATCTL = crate::Reg<u32, _TXDATCTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TXDATCTL;
#[doc = "`read()` method returns [txdatctl::R](txdatctl::R) reader structure"]
impl crate::Readable for TXDATCTL {}
#[doc = "`write(|w| ..)` method takes [txdatctl::W](txdatctl::W) writer structure"]
impl crate::Writable for TXDATCTL {}
#[doc = "SPI Transmit Data with Control"]
pub mod txdatctl;
#[doc = "SPI Transmit Data.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [txdat](txdat) module"]
pub type TXDAT = crate::Reg<u32, _TXDAT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TXDAT;
#[doc = "`read()` method returns [txdat::R](txdat::R) reader structure"]
impl crate::Readable for TXDAT {}
#[doc = "`write(|w| ..)` method takes [txdat::W](txdat::W) writer structure"]
impl crate::Writable for TXDAT {}
#[doc = "SPI Transmit Data."]
pub mod txdat;
#[doc = "SPI Transmit Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [txctl](txctl) module"]
pub type TXCTL = crate::Reg<u32, _TXCTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TXCTL;
#[doc = "`read()` method returns [txctl::R](txctl::R) reader structure"]
impl crate::Readable for TXCTL {}
#[doc = "`write(|w| ..)` method takes [txctl::W](txctl::W) writer structure"]
impl crate::Writable for TXCTL {}
#[doc = "SPI Transmit Control"]
pub mod txctl;
#[doc = "SPI clock Divider\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [div](div) module"]
pub type DIV = crate::Reg<u32, _DIV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DIV;
#[doc = "`read()` method returns [div::R](div::R) reader structure"]
impl crate::Readable for DIV {}
#[doc = "`write(|w| ..)` method takes [div::W](div::W) writer structure"]
impl crate::Writable for DIV {}
#[doc = "SPI clock Divider"]
pub mod div;
#[doc = "SPI Interrupt Status\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intstat](intstat) module"]
pub type INTSTAT = crate::Reg<u32, _INTSTAT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTSTAT;
#[doc = "`read()` method returns [intstat::R](intstat::R) reader structure"]
impl crate::Readable for INTSTAT {}
#[doc = "`write(|w| ..)` method takes [intstat::W](intstat::W) writer structure"]
impl crate::Writable for INTSTAT {}
#[doc = "SPI Interrupt Status"]
pub mod intstat;
