#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 16usize],
    #[doc = "0x10 - Flash memory access time configuration register"]
    pub flashcfg: FLASHCFG,
    _reserved1: [u8; 12usize],
    #[doc = "0x20 - Signature start address register"]
    pub fmsstart: FMSSTART,
    #[doc = "0x24 - Signature stop-address register"]
    pub fmsstop: FMSSTOP,
    _reserved3: [u8; 4usize],
    #[doc = "0x2c - Word 0 \\[31:0\\]"]
    pub fmsw0: FMSW0,
    #[doc = "0x30 - Word 1 \\[63:32\\]"]
    pub fmsw1: FMSW1,
    #[doc = "0x34 - Word 2 \\[95:64\\]"]
    pub fmsw2: FMSW2,
    #[doc = "0x38 - Word 3 \\[127:96\\]"]
    pub fmsw3: FMSW3,
    _reserved7: [u8; 96usize],
    #[doc = "0x9c - EEPROM BIST start address register"]
    pub eemsstart: EEMSSTART,
    #[doc = "0xa0 - EEPROM BIST stop address register"]
    pub eemsstop: EEMSSTOP,
    #[doc = "0xa4 - EEPROM 24-bit BIST signature register"]
    pub eemssig: EEMSSIG,
    _reserved10: [u8; 3896usize],
    #[doc = "0xfe0 - Signature generation status register"]
    pub fmstat: FMSTAT,
    _reserved11: [u8; 4usize],
    #[doc = "0xfe8 - Signature generation status clear register"]
    pub fmstatclr: FMSTATCLR,
}
#[doc = "EEPROM BIST start address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eemsstart](eemsstart) module"]
pub type EEMSSTART = crate::Reg<u32, _EEMSSTART>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EEMSSTART;
#[doc = "`read()` method returns [eemsstart::R](eemsstart::R) reader structure"]
impl crate::Readable for EEMSSTART {}
#[doc = "`write(|w| ..)` method takes [eemsstart::W](eemsstart::W) writer structure"]
impl crate::Writable for EEMSSTART {}
#[doc = "EEPROM BIST start address register"]
pub mod eemsstart;
#[doc = "EEPROM BIST stop address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eemsstop](eemsstop) module"]
pub type EEMSSTOP = crate::Reg<u32, _EEMSSTOP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EEMSSTOP;
#[doc = "`read()` method returns [eemsstop::R](eemsstop::R) reader structure"]
impl crate::Readable for EEMSSTOP {}
#[doc = "`write(|w| ..)` method takes [eemsstop::W](eemsstop::W) writer structure"]
impl crate::Writable for EEMSSTOP {}
#[doc = "EEPROM BIST stop address register"]
pub mod eemsstop;
#[doc = "EEPROM 24-bit BIST signature register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eemssig](eemssig) module"]
pub type EEMSSIG = crate::Reg<u32, _EEMSSIG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EEMSSIG;
#[doc = "`read()` method returns [eemssig::R](eemssig::R) reader structure"]
impl crate::Readable for EEMSSIG {}
#[doc = "EEPROM 24-bit BIST signature register"]
pub mod eemssig;
#[doc = "Flash memory access time configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [flashcfg](flashcfg) module"]
pub type FLASHCFG = crate::Reg<u32, _FLASHCFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FLASHCFG;
#[doc = "`read()` method returns [flashcfg::R](flashcfg::R) reader structure"]
impl crate::Readable for FLASHCFG {}
#[doc = "`write(|w| ..)` method takes [flashcfg::W](flashcfg::W) writer structure"]
impl crate::Writable for FLASHCFG {}
#[doc = "Flash memory access time configuration register"]
pub mod flashcfg;
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
#[doc = "Word 0 \\[31:0\\]\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fmsw0](fmsw0) module"]
pub type FMSW0 = crate::Reg<u32, _FMSW0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FMSW0;
#[doc = "`read()` method returns [fmsw0::R](fmsw0::R) reader structure"]
impl crate::Readable for FMSW0 {}
#[doc = "Word 0 \\[31:0\\]"]
pub mod fmsw0;
#[doc = "Word 1 \\[63:32\\]\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fmsw1](fmsw1) module"]
pub type FMSW1 = crate::Reg<u32, _FMSW1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FMSW1;
#[doc = "`read()` method returns [fmsw1::R](fmsw1::R) reader structure"]
impl crate::Readable for FMSW1 {}
#[doc = "Word 1 \\[63:32\\]"]
pub mod fmsw1;
#[doc = "Word 2 \\[95:64\\]\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fmsw2](fmsw2) module"]
pub type FMSW2 = crate::Reg<u32, _FMSW2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FMSW2;
#[doc = "`read()` method returns [fmsw2::R](fmsw2::R) reader structure"]
impl crate::Readable for FMSW2 {}
#[doc = "Word 2 \\[95:64\\]"]
pub mod fmsw2;
#[doc = "Word 3 \\[127:96\\]\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fmsw3](fmsw3) module"]
pub type FMSW3 = crate::Reg<u32, _FMSW3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FMSW3;
#[doc = "`read()` method returns [fmsw3::R](fmsw3::R) reader structure"]
impl crate::Readable for FMSW3 {}
#[doc = "Word 3 \\[127:96\\]"]
pub mod fmsw3;
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
