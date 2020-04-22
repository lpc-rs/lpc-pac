#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 16usize],
    #[doc = "0x10 - Flash configuration register"]
    pub flashcfg: FLASHCFG,
    _reserved1: [u8; 12usize],
    #[doc = "0x20 - Flash signature start address register"]
    pub fmsstart: FMSSTART,
    #[doc = "0x24 - Flash signaure stop address register"]
    pub fmsstop: FMSSTOP,
    _reserved3: [u8; 4usize],
    #[doc = "0x2c - Flash signature generation result register returns the flash signature produced by the embedded signature generator.."]
    pub fmsw0: FMSW0,
    _reserved4: [u8; 4016usize],
    #[doc = "0xfe0 - Flash signature generation status bit"]
    pub fmstat: FMSTAT,
    _reserved5: [u8; 4usize],
    #[doc = "0xfe8 - Clear FLASH signature generation status bit"]
    pub fmstatclr: FMSTATCLR,
}
#[doc = "Flash configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [flashcfg](flashcfg) module"]
pub type FLASHCFG = crate::Reg<u32, _FLASHCFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FLASHCFG;
#[doc = "`read()` method returns [flashcfg::R](flashcfg::R) reader structure"]
impl crate::Readable for FLASHCFG {}
#[doc = "`write(|w| ..)` method takes [flashcfg::W](flashcfg::W) writer structure"]
impl crate::Writable for FLASHCFG {}
#[doc = "Flash configuration register"]
pub mod flashcfg;
#[doc = "Flash signature start address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fmsstart](fmsstart) module"]
pub type FMSSTART = crate::Reg<u32, _FMSSTART>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FMSSTART;
#[doc = "`read()` method returns [fmsstart::R](fmsstart::R) reader structure"]
impl crate::Readable for FMSSTART {}
#[doc = "`write(|w| ..)` method takes [fmsstart::W](fmsstart::W) writer structure"]
impl crate::Writable for FMSSTART {}
#[doc = "Flash signature start address register"]
pub mod fmsstart;
#[doc = "Flash signaure stop address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fmsstop](fmsstop) module"]
pub type FMSSTOP = crate::Reg<u32, _FMSSTOP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FMSSTOP;
#[doc = "`read()` method returns [fmsstop::R](fmsstop::R) reader structure"]
impl crate::Readable for FMSSTOP {}
#[doc = "`write(|w| ..)` method takes [fmsstop::W](fmsstop::W) writer structure"]
impl crate::Writable for FMSSTOP {}
#[doc = "Flash signaure stop address register"]
pub mod fmsstop;
#[doc = "Flash signature generation result register returns the flash signature produced by the embedded signature generator..\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fmsw0](fmsw0) module"]
pub type FMSW0 = crate::Reg<u32, _FMSW0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FMSW0;
#[doc = "`read()` method returns [fmsw0::R](fmsw0::R) reader structure"]
impl crate::Readable for FMSW0 {}
#[doc = "Flash signature generation result register returns the flash signature produced by the embedded signature generator.."]
pub mod fmsw0;
#[doc = "Flash signature generation status bit\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fmstat](fmstat) module"]
pub type FMSTAT = crate::Reg<u32, _FMSTAT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FMSTAT;
#[doc = "`read()` method returns [fmstat::R](fmstat::R) reader structure"]
impl crate::Readable for FMSTAT {}
#[doc = "Flash signature generation status bit"]
pub mod fmstat;
#[doc = "Clear FLASH signature generation status bit\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fmstatclr](fmstatclr) module"]
pub type FMSTATCLR = crate::Reg<u32, _FMSTATCLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FMSTATCLR;
#[doc = "`write(|w| ..)` method takes [fmstatclr::W](fmstatclr::W) writer structure"]
impl crate::Writable for FMSTATCLR {}
#[doc = "Clear FLASH signature generation status bit"]
pub mod fmstatclr;
