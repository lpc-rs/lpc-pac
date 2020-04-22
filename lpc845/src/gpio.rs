#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - no description available"]
    pub b: [B; 2],
    _reserved1: [u8; 4032usize],
    #[doc = "0x1000 - no description available"]
    pub w: [W; 2],
    _reserved2: [u8; 3840usize],
    #[doc = "0x2000 - Direction registers"]
    pub dir: [DIR; 2],
    _reserved3: [u8; 120usize],
    #[doc = "0x2080 - Mask register"]
    pub mask: [MASK; 2],
    _reserved4: [u8; 120usize],
    #[doc = "0x2100 - Port pin register"]
    pub pin: [PIN; 2],
    _reserved5: [u8; 120usize],
    #[doc = "0x2180 - Masked port register"]
    pub mpin: [MPIN; 2],
    _reserved6: [u8; 120usize],
    #[doc = "0x2200 - Write: Set register for port Read: output bits for port"]
    pub set: [SET; 2],
    _reserved7: [u8; 120usize],
    #[doc = "0x2280 - Clear port"]
    pub clr: [CLR; 2],
    _reserved8: [u8; 120usize],
    #[doc = "0x2300 - Toggle port"]
    pub not: [NOT; 2],
    _reserved9: [u8; 120usize],
    #[doc = "0x2380 - Set pin direction bits for port"]
    pub dirset: [DIRSET; 2],
    _reserved10: [u8; 120usize],
    #[doc = "0x2400 - Clear pin direction bits for port"]
    pub dirclr: [DIRCLR; 2],
    _reserved11: [u8; 120usize],
    #[doc = "0x2480 - Toggle pin direction bits for port"]
    pub dirnot: [DIRNOT; 2],
}
#[doc = r"Register block"]
#[repr(C)]
pub struct B {
    #[doc = "0x00 - Byte pin registers for all port 0 and 1 GPIO pins"]
    pub b_: [self::b::B_; 32],
}
#[doc = r"Register block"]
#[doc = "no description available"]
pub mod b;
#[doc = r"Register block"]
#[repr(C)]
pub struct W {
    #[doc = "0x00 - Word pin registers for all port 0 and 1 GPIO pins"]
    pub w_: [self::w::W_; 32],
}
#[doc = r"Register block"]
#[doc = "no description available"]
pub mod w;
#[doc = "Direction registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dir](dir) module"]
pub type DIR = crate::Reg<u32, _DIR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DIR;
#[doc = "`read()` method returns [dir::R](dir::R) reader structure"]
impl crate::Readable for DIR {}
#[doc = "`write(|w| ..)` method takes [dir::W](dir::W) writer structure"]
impl crate::Writable for DIR {}
#[doc = "Direction registers"]
pub mod dir;
#[doc = "Mask register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mask](mask) module"]
pub type MASK = crate::Reg<u32, _MASK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MASK;
#[doc = "`read()` method returns [mask::R](mask::R) reader structure"]
impl crate::Readable for MASK {}
#[doc = "`write(|w| ..)` method takes [mask::W](mask::W) writer structure"]
impl crate::Writable for MASK {}
#[doc = "Mask register"]
pub mod mask;
#[doc = "Port pin register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pin](pin) module"]
pub type PIN = crate::Reg<u32, _PIN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIN;
#[doc = "`read()` method returns [pin::R](pin::R) reader structure"]
impl crate::Readable for PIN {}
#[doc = "`write(|w| ..)` method takes [pin::W](pin::W) writer structure"]
impl crate::Writable for PIN {}
#[doc = "Port pin register"]
pub mod pin;
#[doc = "Masked port register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mpin](mpin) module"]
pub type MPIN = crate::Reg<u32, _MPIN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MPIN;
#[doc = "`read()` method returns [mpin::R](mpin::R) reader structure"]
impl crate::Readable for MPIN {}
#[doc = "`write(|w| ..)` method takes [mpin::W](mpin::W) writer structure"]
impl crate::Writable for MPIN {}
#[doc = "Masked port register"]
pub mod mpin;
#[doc = "Write: Set register for port Read: output bits for port\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [set](set) module"]
pub type SET = crate::Reg<u32, _SET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SET;
#[doc = "`read()` method returns [set::R](set::R) reader structure"]
impl crate::Readable for SET {}
#[doc = "`write(|w| ..)` method takes [set::W](set::W) writer structure"]
impl crate::Writable for SET {}
#[doc = "Write: Set register for port Read: output bits for port"]
pub mod set;
#[doc = "Clear port\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clr](clr) module"]
pub type CLR = crate::Reg<u32, _CLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CLR;
#[doc = "`write(|w| ..)` method takes [clr::W](clr::W) writer structure"]
impl crate::Writable for CLR {}
#[doc = "Clear port"]
pub mod clr;
#[doc = "Toggle port\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [not](not) module"]
pub type NOT = crate::Reg<u32, _NOT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _NOT;
#[doc = "`write(|w| ..)` method takes [not::W](not::W) writer structure"]
impl crate::Writable for NOT {}
#[doc = "Toggle port"]
pub mod not;
#[doc = "Set pin direction bits for port\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dirset](dirset) module"]
pub type DIRSET = crate::Reg<u32, _DIRSET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DIRSET;
#[doc = "`write(|w| ..)` method takes [dirset::W](dirset::W) writer structure"]
impl crate::Writable for DIRSET {}
#[doc = "Set pin direction bits for port"]
pub mod dirset;
#[doc = "Clear pin direction bits for port\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dirclr](dirclr) module"]
pub type DIRCLR = crate::Reg<u32, _DIRCLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DIRCLR;
#[doc = "`write(|w| ..)` method takes [dirclr::W](dirclr::W) writer structure"]
impl crate::Writable for DIRCLR {}
#[doc = "Clear pin direction bits for port"]
pub mod dirclr;
#[doc = "Toggle pin direction bits for port\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dirnot](dirnot) module"]
pub type DIRNOT = crate::Reg<u32, _DIRNOT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DIRNOT;
#[doc = "`write(|w| ..)` method takes [dirnot::W](dirnot::W) writer structure"]
impl crate::Writable for DIRNOT {}
#[doc = "Toggle pin direction bits for port"]
pub mod dirnot;
