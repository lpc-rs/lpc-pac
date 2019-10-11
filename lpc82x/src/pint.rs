#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Pin Interrupt Mode register"]
    pub isel: ISEL,
    #[doc = "0x04 - Pin interrupt level or rising edge interrupt enable register"]
    pub ienr: IENR,
    #[doc = "0x08 - Pin interrupt level or rising edge interrupt set register"]
    pub sienr: SIENR,
    #[doc = "0x0c - Pin interrupt level (rising edge interrupt) clear register"]
    pub cienr: CIENR,
    #[doc = "0x10 - Pin interrupt active level or falling edge interrupt enable register"]
    pub ienf: IENF,
    #[doc = "0x14 - Pin interrupt active level or falling edge interrupt set register"]
    pub sienf: SIENF,
    #[doc = "0x18 - Pin interrupt active level or falling edge interrupt clear register"]
    pub cienf: CIENF,
    #[doc = "0x1c - Pin interrupt rising edge register"]
    pub rise: RISE,
    #[doc = "0x20 - Pin interrupt falling edge register"]
    pub fall: FALL,
    #[doc = "0x24 - Pin interrupt status register"]
    pub ist: IST,
    #[doc = "0x28 - Pattern match interrupt control register"]
    pub pmctrl: PMCTRL,
    #[doc = "0x2c - Pattern match interrupt bit-slice source register"]
    pub pmsrc: PMSRC,
    #[doc = "0x30 - Pattern match interrupt bit slice configuration register"]
    pub pmcfg: PMCFG,
}
#[doc = "Pin Interrupt Mode register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [isel](isel) module"]
pub type ISEL = crate::Reg<u32, _ISEL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ISEL;
#[doc = "`read()` method returns [isel::R](isel::R) reader structure"]
impl crate::Readable for ISEL {}
#[doc = "`write(|w| ..)` method takes [isel::W](isel::W) writer structure"]
impl crate::Writable for ISEL {}
#[doc = "Pin Interrupt Mode register"]
pub mod isel;
#[doc = "Pin interrupt level or rising edge interrupt enable register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ienr](ienr) module"]
pub type IENR = crate::Reg<u32, _IENR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IENR;
#[doc = "`read()` method returns [ienr::R](ienr::R) reader structure"]
impl crate::Readable for IENR {}
#[doc = "`write(|w| ..)` method takes [ienr::W](ienr::W) writer structure"]
impl crate::Writable for IENR {}
#[doc = "Pin interrupt level or rising edge interrupt enable register"]
pub mod ienr;
#[doc = "Pin interrupt level or rising edge interrupt set register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sienr](sienr) module"]
pub type SIENR = crate::Reg<u32, _SIENR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SIENR;
#[doc = "`write(|w| ..)` method takes [sienr::W](sienr::W) writer structure"]
impl crate::Writable for SIENR {}
#[doc = "Pin interrupt level or rising edge interrupt set register"]
pub mod sienr;
#[doc = "Pin interrupt level (rising edge interrupt) clear register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cienr](cienr) module"]
pub type CIENR = crate::Reg<u32, _CIENR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CIENR;
#[doc = "`write(|w| ..)` method takes [cienr::W](cienr::W) writer structure"]
impl crate::Writable for CIENR {}
#[doc = "Pin interrupt level (rising edge interrupt) clear register"]
pub mod cienr;
#[doc = "Pin interrupt active level or falling edge interrupt enable register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ienf](ienf) module"]
pub type IENF = crate::Reg<u32, _IENF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IENF;
#[doc = "`read()` method returns [ienf::R](ienf::R) reader structure"]
impl crate::Readable for IENF {}
#[doc = "`write(|w| ..)` method takes [ienf::W](ienf::W) writer structure"]
impl crate::Writable for IENF {}
#[doc = "Pin interrupt active level or falling edge interrupt enable register"]
pub mod ienf;
#[doc = "Pin interrupt active level or falling edge interrupt set register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sienf](sienf) module"]
pub type SIENF = crate::Reg<u32, _SIENF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SIENF;
#[doc = "`write(|w| ..)` method takes [sienf::W](sienf::W) writer structure"]
impl crate::Writable for SIENF {}
#[doc = "Pin interrupt active level or falling edge interrupt set register"]
pub mod sienf;
#[doc = "Pin interrupt active level or falling edge interrupt clear register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cienf](cienf) module"]
pub type CIENF = crate::Reg<u32, _CIENF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CIENF;
#[doc = "`write(|w| ..)` method takes [cienf::W](cienf::W) writer structure"]
impl crate::Writable for CIENF {}
#[doc = "Pin interrupt active level or falling edge interrupt clear register"]
pub mod cienf;
#[doc = "Pin interrupt rising edge register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rise](rise) module"]
pub type RISE = crate::Reg<u32, _RISE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RISE;
#[doc = "`read()` method returns [rise::R](rise::R) reader structure"]
impl crate::Readable for RISE {}
#[doc = "`write(|w| ..)` method takes [rise::W](rise::W) writer structure"]
impl crate::Writable for RISE {}
#[doc = "Pin interrupt rising edge register"]
pub mod rise;
#[doc = "Pin interrupt falling edge register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [fall](fall) module"]
pub type FALL = crate::Reg<u32, _FALL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FALL;
#[doc = "`read()` method returns [fall::R](fall::R) reader structure"]
impl crate::Readable for FALL {}
#[doc = "`write(|w| ..)` method takes [fall::W](fall::W) writer structure"]
impl crate::Writable for FALL {}
#[doc = "Pin interrupt falling edge register"]
pub mod fall;
#[doc = "Pin interrupt status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ist](ist) module"]
pub type IST = crate::Reg<u32, _IST>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IST;
#[doc = "`read()` method returns [ist::R](ist::R) reader structure"]
impl crate::Readable for IST {}
#[doc = "`write(|w| ..)` method takes [ist::W](ist::W) writer structure"]
impl crate::Writable for IST {}
#[doc = "Pin interrupt status register"]
pub mod ist;
#[doc = "Pattern match interrupt control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pmctrl](pmctrl) module"]
pub type PMCTRL = crate::Reg<u32, _PMCTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PMCTRL;
#[doc = "`read()` method returns [pmctrl::R](pmctrl::R) reader structure"]
impl crate::Readable for PMCTRL {}
#[doc = "`write(|w| ..)` method takes [pmctrl::W](pmctrl::W) writer structure"]
impl crate::Writable for PMCTRL {}
#[doc = "Pattern match interrupt control register"]
pub mod pmctrl;
#[doc = "Pattern match interrupt bit-slice source register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pmsrc](pmsrc) module"]
pub type PMSRC = crate::Reg<u32, _PMSRC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PMSRC;
#[doc = "`read()` method returns [pmsrc::R](pmsrc::R) reader structure"]
impl crate::Readable for PMSRC {}
#[doc = "`write(|w| ..)` method takes [pmsrc::W](pmsrc::W) writer structure"]
impl crate::Writable for PMSRC {}
#[doc = "Pattern match interrupt bit-slice source register"]
pub mod pmsrc;
#[doc = "Pattern match interrupt bit slice configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pmcfg](pmcfg) module"]
pub type PMCFG = crate::Reg<u32, _PMCFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PMCFG;
#[doc = "`read()` method returns [pmcfg::R](pmcfg::R) reader structure"]
impl crate::Readable for PMCFG {}
#[doc = "`write(|w| ..)` method takes [pmcfg::W](pmcfg::W) writer structure"]
impl crate::Writable for PMCFG {}
#[doc = "Pattern match interrupt bit slice configuration register"]
pub mod pmcfg;
