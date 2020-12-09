#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 1024usize],
    #[doc = "0x400 - SPI Configuration register"]
    pub cfg: CFG,
    #[doc = "0x404 - SPI Delay register"]
    pub dly: DLY,
    #[doc = "0x408 - SPI Status. Some status flags can be cleared by writing a 1 to that bit position."]
    pub stat: STAT,
    #[doc = "0x40c - SPI Interrupt Enable read and Set. A complete value may be read from this register. Writing a 1 to any implemented bit position causes that bit to be set."]
    pub intenset: INTENSET,
    #[doc = "0x410 - SPI Interrupt Enable Clear. Writing a 1 to any implemented bit position causes the corresponding bit in INTENSET to be cleared."]
    pub intenclr: INTENCLR,
    _reserved5: [u8; 16usize],
    #[doc = "0x424 - SPI clock Divider"]
    pub div: DIV,
    #[doc = "0x428 - SPI Interrupt Status"]
    pub intstat: INTSTAT,
    _reserved7: [u8; 2516usize],
    #[doc = "0xe00 - FIFO configuration and enable register."]
    pub fifocfg: FIFOCFG,
    #[doc = "0xe04 - FIFO status register."]
    pub fifostat: FIFOSTAT,
    #[doc = "0xe08 - FIFO trigger settings for interrupt and DMA request."]
    pub fifotrig: FIFOTRIG,
    _reserved10: [u8; 4usize],
    #[doc = "0xe10 - FIFO interrupt enable set (enable) and read register."]
    pub fifointenset: FIFOINTENSET,
    #[doc = "0xe14 - FIFO interrupt enable clear (disable) and read register."]
    pub fifointenclr: FIFOINTENCLR,
    #[doc = "0xe18 - FIFO interrupt status register."]
    pub fifointstat: FIFOINTSTAT,
    _reserved13: [u8; 4usize],
    #[doc = "0xe20 - FIFO write data."]
    pub fifowr: FIFOWR,
    _reserved14: [u8; 12usize],
    #[doc = "0xe30 - FIFO read data."]
    pub fiford: FIFORD,
    _reserved15: [u8; 12usize],
    #[doc = "0xe40 - FIFO data read with no FIFO pop."]
    pub fifordnopop: FIFORDNOPOP,
    _reserved16: [u8; 440usize],
    #[doc = "0xffc - Peripheral identification register."]
    pub id: ID,
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
#[doc = "SPI Status. Some status flags can be cleared by writing a 1 to that bit position.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [stat](stat) module"]
pub type STAT = crate::Reg<u32, _STAT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _STAT;
#[doc = "`read()` method returns [stat::R](stat::R) reader structure"]
impl crate::Readable for STAT {}
#[doc = "`write(|w| ..)` method takes [stat::W](stat::W) writer structure"]
impl crate::Writable for STAT {}
#[doc = "SPI Status. Some status flags can be cleared by writing a 1 to that bit position."]
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
#[doc = "SPI Interrupt Status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intstat](intstat) module"]
pub type INTSTAT = crate::Reg<u32, _INTSTAT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTSTAT;
#[doc = "`read()` method returns [intstat::R](intstat::R) reader structure"]
impl crate::Readable for INTSTAT {}
#[doc = "SPI Interrupt Status"]
pub mod intstat;
#[doc = "FIFO configuration and enable register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fifocfg](fifocfg) module"]
pub type FIFOCFG = crate::Reg<u32, _FIFOCFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FIFOCFG;
#[doc = "`read()` method returns [fifocfg::R](fifocfg::R) reader structure"]
impl crate::Readable for FIFOCFG {}
#[doc = "`write(|w| ..)` method takes [fifocfg::W](fifocfg::W) writer structure"]
impl crate::Writable for FIFOCFG {}
#[doc = "FIFO configuration and enable register."]
pub mod fifocfg;
#[doc = "FIFO status register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fifostat](fifostat) module"]
pub type FIFOSTAT = crate::Reg<u32, _FIFOSTAT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FIFOSTAT;
#[doc = "`read()` method returns [fifostat::R](fifostat::R) reader structure"]
impl crate::Readable for FIFOSTAT {}
#[doc = "`write(|w| ..)` method takes [fifostat::W](fifostat::W) writer structure"]
impl crate::Writable for FIFOSTAT {}
#[doc = "FIFO status register."]
pub mod fifostat;
#[doc = "FIFO trigger settings for interrupt and DMA request.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fifotrig](fifotrig) module"]
pub type FIFOTRIG = crate::Reg<u32, _FIFOTRIG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FIFOTRIG;
#[doc = "`read()` method returns [fifotrig::R](fifotrig::R) reader structure"]
impl crate::Readable for FIFOTRIG {}
#[doc = "`write(|w| ..)` method takes [fifotrig::W](fifotrig::W) writer structure"]
impl crate::Writable for FIFOTRIG {}
#[doc = "FIFO trigger settings for interrupt and DMA request."]
pub mod fifotrig;
#[doc = "FIFO interrupt enable set (enable) and read register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fifointenset](fifointenset) module"]
pub type FIFOINTENSET = crate::Reg<u32, _FIFOINTENSET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FIFOINTENSET;
#[doc = "`read()` method returns [fifointenset::R](fifointenset::R) reader structure"]
impl crate::Readable for FIFOINTENSET {}
#[doc = "`write(|w| ..)` method takes [fifointenset::W](fifointenset::W) writer structure"]
impl crate::Writable for FIFOINTENSET {}
#[doc = "FIFO interrupt enable set (enable) and read register."]
pub mod fifointenset;
#[doc = "FIFO interrupt enable clear (disable) and read register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fifointenclr](fifointenclr) module"]
pub type FIFOINTENCLR = crate::Reg<u32, _FIFOINTENCLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FIFOINTENCLR;
#[doc = "`read()` method returns [fifointenclr::R](fifointenclr::R) reader structure"]
impl crate::Readable for FIFOINTENCLR {}
#[doc = "`write(|w| ..)` method takes [fifointenclr::W](fifointenclr::W) writer structure"]
impl crate::Writable for FIFOINTENCLR {}
#[doc = "FIFO interrupt enable clear (disable) and read register."]
pub mod fifointenclr;
#[doc = "FIFO interrupt status register.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fifointstat](fifointstat) module"]
pub type FIFOINTSTAT = crate::Reg<u32, _FIFOINTSTAT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FIFOINTSTAT;
#[doc = "`read()` method returns [fifointstat::R](fifointstat::R) reader structure"]
impl crate::Readable for FIFOINTSTAT {}
#[doc = "FIFO interrupt status register."]
pub mod fifointstat;
#[doc = "FIFO write data.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fifowr](fifowr) module"]
pub type FIFOWR = crate::Reg<u32, _FIFOWR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FIFOWR;
#[doc = "`read()` method returns [fifowr::R](fifowr::R) reader structure"]
impl crate::Readable for FIFOWR {}
#[doc = "`write(|w| ..)` method takes [fifowr::W](fifowr::W) writer structure"]
impl crate::Writable for FIFOWR {}
#[doc = "FIFO write data."]
pub mod fifowr;
#[doc = "FIFO read data.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fiford](fiford) module"]
pub type FIFORD = crate::Reg<u32, _FIFORD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FIFORD;
#[doc = "`read()` method returns [fiford::R](fiford::R) reader structure"]
impl crate::Readable for FIFORD {}
#[doc = "FIFO read data."]
pub mod fiford;
#[doc = "FIFO data read with no FIFO pop.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fifordnopop](fifordnopop) module"]
pub type FIFORDNOPOP = crate::Reg<u32, _FIFORDNOPOP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FIFORDNOPOP;
#[doc = "`read()` method returns [fifordnopop::R](fifordnopop::R) reader structure"]
impl crate::Readable for FIFORDNOPOP {}
#[doc = "FIFO data read with no FIFO pop."]
pub mod fifordnopop;
#[doc = "Peripheral identification register.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [id](id) module"]
pub type ID = crate::Reg<u32, _ID>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ID;
#[doc = "`read()` method returns [id::R](id::R) reader structure"]
impl crate::Readable for ID {}
#[doc = "Peripheral identification register."]
pub mod id;
