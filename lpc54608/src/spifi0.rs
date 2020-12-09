#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - SPIFI control register"]
    pub ctrl: CTRL,
    #[doc = "0x04 - SPIFI command register"]
    pub cmd: CMD,
    #[doc = "0x08 - SPIFI address register"]
    pub addr: ADDR,
    #[doc = "0x0c - SPIFI intermediate data register"]
    pub idata: IDATA,
    #[doc = "0x10 - SPIFI limit register"]
    pub climit: CLIMIT,
    #[doc = "0x14 - SPIFI data register"]
    pub data: DATA,
    #[doc = "0x18 - SPIFI memory command register"]
    pub mcmd: MCMD,
    #[doc = "0x1c - SPIFI status register"]
    pub stat: STAT,
}
#[doc = "SPIFI control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrl](ctrl) module"]
pub type CTRL = crate::Reg<u32, _CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CTRL;
#[doc = "`read()` method returns [ctrl::R](ctrl::R) reader structure"]
impl crate::Readable for CTRL {}
#[doc = "`write(|w| ..)` method takes [ctrl::W](ctrl::W) writer structure"]
impl crate::Writable for CTRL {}
#[doc = "SPIFI control register"]
pub mod ctrl;
#[doc = "SPIFI command register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmd](cmd) module"]
pub type CMD = crate::Reg<u32, _CMD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CMD;
#[doc = "`read()` method returns [cmd::R](cmd::R) reader structure"]
impl crate::Readable for CMD {}
#[doc = "`write(|w| ..)` method takes [cmd::W](cmd::W) writer structure"]
impl crate::Writable for CMD {}
#[doc = "SPIFI command register"]
pub mod cmd;
#[doc = "SPIFI address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [addr](addr) module"]
pub type ADDR = crate::Reg<u32, _ADDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADDR;
#[doc = "`read()` method returns [addr::R](addr::R) reader structure"]
impl crate::Readable for ADDR {}
#[doc = "`write(|w| ..)` method takes [addr::W](addr::W) writer structure"]
impl crate::Writable for ADDR {}
#[doc = "SPIFI address register"]
pub mod addr;
#[doc = "SPIFI intermediate data register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [idata](idata) module"]
pub type IDATA = crate::Reg<u32, _IDATA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IDATA;
#[doc = "`read()` method returns [idata::R](idata::R) reader structure"]
impl crate::Readable for IDATA {}
#[doc = "`write(|w| ..)` method takes [idata::W](idata::W) writer structure"]
impl crate::Writable for IDATA {}
#[doc = "SPIFI intermediate data register"]
pub mod idata;
#[doc = "SPIFI limit register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [climit](climit) module"]
pub type CLIMIT = crate::Reg<u32, _CLIMIT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CLIMIT;
#[doc = "`read()` method returns [climit::R](climit::R) reader structure"]
impl crate::Readable for CLIMIT {}
#[doc = "`write(|w| ..)` method takes [climit::W](climit::W) writer structure"]
impl crate::Writable for CLIMIT {}
#[doc = "SPIFI limit register"]
pub mod climit;
#[doc = "SPIFI data register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [data](data) module"]
pub type DATA = crate::Reg<u32, _DATA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DATA;
#[doc = "`read()` method returns [data::R](data::R) reader structure"]
impl crate::Readable for DATA {}
#[doc = "`write(|w| ..)` method takes [data::W](data::W) writer structure"]
impl crate::Writable for DATA {}
#[doc = "SPIFI data register"]
pub mod data;
#[doc = "SPIFI memory command register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mcmd](mcmd) module"]
pub type MCMD = crate::Reg<u32, _MCMD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MCMD;
#[doc = "`read()` method returns [mcmd::R](mcmd::R) reader structure"]
impl crate::Readable for MCMD {}
#[doc = "`write(|w| ..)` method takes [mcmd::W](mcmd::W) writer structure"]
impl crate::Writable for MCMD {}
#[doc = "SPIFI memory command register"]
pub mod mcmd;
#[doc = "SPIFI status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [stat](stat) module"]
pub type STAT = crate::Reg<u32, _STAT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _STAT;
#[doc = "`read()` method returns [stat::R](stat::R) reader structure"]
impl crate::Readable for STAT {}
#[doc = "`write(|w| ..)` method takes [stat::W](stat::W) writer structure"]
impl crate::Writable for STAT {}
#[doc = "SPIFI status register"]
pub mod stat;
