#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control register"]
    pub fctr: FCTR,
    _reserved1: [u8; 12usize],
    #[doc = "0x10 - Wait state register"]
    pub fbwst: FBWST,
    _reserved2: [u8; 12usize],
    #[doc = "0x20 - Signature start address register"]
    pub fmsstart: FMSSTART,
    #[doc = "0x24 - Signature stop-address register"]
    pub fmsstop: FMSSTOP,
    _reserved4: [u8; 4usize],
    #[doc = "0x2c - Words of 128-bit signature word"]
    pub fmsw: [FMSW; 4],
    _reserved5: [u8; 4004usize],
    #[doc = "0xfe0 - Signature generation status register"]
    pub fmstat: FMSTAT,
    _reserved6: [u8; 4usize],
    #[doc = "0xfe8 - Signature generation status clear register"]
    pub fmstatclr: FMSTATCLR,
}
#[doc = "Control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fctr](fctr) module"]
pub type FCTR = crate::Reg<u32, _FCTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FCTR;
#[doc = "`read()` method returns [fctr::R](fctr::R) reader structure"]
impl crate::Readable for FCTR {}
#[doc = "`write(|w| ..)` method takes [fctr::W](fctr::W) writer structure"]
impl crate::Writable for FCTR {}
#[doc = "Control register"]
pub mod fctr;
#[doc = "Wait state register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fbwst](fbwst) module"]
pub type FBWST = crate::Reg<u32, _FBWST>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FBWST;
#[doc = "`read()` method returns [fbwst::R](fbwst::R) reader structure"]
impl crate::Readable for FBWST {}
#[doc = "`write(|w| ..)` method takes [fbwst::W](fbwst::W) writer structure"]
impl crate::Writable for FBWST {}
#[doc = "Wait state register"]
pub mod fbwst;
#[doc = "Signature start address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fmsstart](fmsstart) module"]
pub type FMSSTART = crate::Reg<u32, _FMSSTART>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FMSSTART;
#[doc = "`read()` method returns [fmsstart::R](fmsstart::R) reader structure"]
impl crate::Readable for FMSSTART {}
#[doc = "`write(|w| ..)` method takes [fmsstart::W](fmsstart::W) writer structure"]
impl crate::Writable for FMSSTART {}
#[doc = "Signature start address register"]
pub mod fmsstart;
#[doc = "Signature stop-address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fmsstop](fmsstop) module"]
pub type FMSSTOP = crate::Reg<u32, _FMSSTOP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FMSSTOP;
#[doc = "`read()` method returns [fmsstop::R](fmsstop::R) reader structure"]
impl crate::Readable for FMSSTOP {}
#[doc = "`write(|w| ..)` method takes [fmsstop::W](fmsstop::W) writer structure"]
impl crate::Writable for FMSSTOP {}
#[doc = "Signature stop-address register"]
pub mod fmsstop;
#[doc = "Words of 128-bit signature word\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fmsw](fmsw) module"]
pub type FMSW = crate::Reg<u32, _FMSW>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FMSW;
#[doc = "`read()` method returns [fmsw::R](fmsw::R) reader structure"]
impl crate::Readable for FMSW {}
#[doc = "Words of 128-bit signature word"]
pub mod fmsw;
#[doc = "Signature generation status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fmstat](fmstat) module"]
pub type FMSTAT = crate::Reg<u32, _FMSTAT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FMSTAT;
#[doc = "`read()` method returns [fmstat::R](fmstat::R) reader structure"]
impl crate::Readable for FMSTAT {}
#[doc = "Signature generation status register"]
pub mod fmstat;
#[doc = "Signature generation status clear register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fmstatclr](fmstatclr) module"]
pub type FMSTATCLR = crate::Reg<u32, _FMSTATCLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FMSTATCLR;
#[doc = "`write(|w| ..)` method takes [fmstatclr::W](fmstatclr::W) writer structure"]
impl crate::Writable for FMSTATCLR {}
#[doc = "Signature generation status clear register"]
pub mod fmstatclr;
