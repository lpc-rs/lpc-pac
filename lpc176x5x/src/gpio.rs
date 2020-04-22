#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - GPIO Port Direction control register."]
    pub dir0: DIR,
    _reserved1: [u8; 12usize],
    #[doc = "0x10 - Mask register for Port."]
    pub mask0: MASK,
    #[doc = "0x14 - Port Pin value register using FIOMASK."]
    pub pin0: PIN,
    #[doc = "0x18 - Port Output Set register using FIOMASK."]
    pub set0: SET,
    #[doc = "0x1c - Port Output Clear register using FIOMASK."]
    pub clr0: CLR,
    #[doc = "0x20 - GPIO Port Direction control register."]
    pub dir1: DIR,
    _reserved6: [u8; 12usize],
    #[doc = "0x30 - Mask register for Port."]
    pub mask1: MASK,
    #[doc = "0x34 - Port Pin value register using FIOMASK."]
    pub pin1: PIN,
    #[doc = "0x38 - Port Output Set register using FIOMASK."]
    pub set1: SET,
    #[doc = "0x3c - Port Output Clear register using FIOMASK."]
    pub clr1: CLR,
    #[doc = "0x40 - GPIO Port Direction control register."]
    pub dir2: DIR,
    _reserved11: [u8; 12usize],
    #[doc = "0x50 - Mask register for Port."]
    pub mask2: MASK,
    #[doc = "0x54 - Port Pin value register using FIOMASK."]
    pub pin2: PIN,
    #[doc = "0x58 - Port Output Set register using FIOMASK."]
    pub set2: SET,
    #[doc = "0x5c - Port Output Clear register using FIOMASK."]
    pub clr2: CLR,
    #[doc = "0x60 - GPIO Port Direction control register."]
    pub dir3: DIR,
    _reserved16: [u8; 12usize],
    #[doc = "0x70 - Mask register for Port."]
    pub mask3: MASK,
    #[doc = "0x74 - Port Pin value register using FIOMASK."]
    pub pin3: PIN,
    #[doc = "0x78 - Port Output Set register using FIOMASK."]
    pub set3: SET,
    #[doc = "0x7c - Port Output Clear register using FIOMASK."]
    pub clr3: CLR,
    #[doc = "0x80 - GPIO Port Direction control register."]
    pub dir4: DIR,
    _reserved21: [u8; 12usize],
    #[doc = "0x90 - Mask register for Port."]
    pub mask4: MASK,
    #[doc = "0x94 - Port Pin value register using FIOMASK."]
    pub pin4: PIN,
    #[doc = "0x98 - Port Output Set register using FIOMASK."]
    pub set4: SET,
    #[doc = "0x9c - Port Output Clear register using FIOMASK."]
    pub clr4: CLR,
}
#[doc = "GPIO Port Direction control register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dir](dir) module"]
pub type DIR = crate::Reg<u32, _DIR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DIR;
#[doc = "`read()` method returns [dir::R](dir::R) reader structure"]
impl crate::Readable for DIR {}
#[doc = "`write(|w| ..)` method takes [dir::W](dir::W) writer structure"]
impl crate::Writable for DIR {}
#[doc = "GPIO Port Direction control register."]
pub mod dir;
#[doc = "Mask register for Port.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mask](mask) module"]
pub type MASK = crate::Reg<u32, _MASK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MASK;
#[doc = "`read()` method returns [mask::R](mask::R) reader structure"]
impl crate::Readable for MASK {}
#[doc = "`write(|w| ..)` method takes [mask::W](mask::W) writer structure"]
impl crate::Writable for MASK {}
#[doc = "Mask register for Port."]
pub mod mask;
#[doc = "Port Pin value register using FIOMASK.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pin](pin) module"]
pub type PIN = crate::Reg<u32, _PIN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIN;
#[doc = "`read()` method returns [pin::R](pin::R) reader structure"]
impl crate::Readable for PIN {}
#[doc = "`write(|w| ..)` method takes [pin::W](pin::W) writer structure"]
impl crate::Writable for PIN {}
#[doc = "Port Pin value register using FIOMASK."]
pub mod pin;
#[doc = "Port Output Set register using FIOMASK.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [set](set) module"]
pub type SET = crate::Reg<u32, _SET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SET;
#[doc = "`read()` method returns [set::R](set::R) reader structure"]
impl crate::Readable for SET {}
#[doc = "`write(|w| ..)` method takes [set::W](set::W) writer structure"]
impl crate::Writable for SET {}
#[doc = "Port Output Set register using FIOMASK."]
pub mod set;
#[doc = "Port Output Clear register using FIOMASK.\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clr](clr) module"]
pub type CLR = crate::Reg<u32, _CLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CLR;
#[doc = "`write(|w| ..)` method takes [clr::W](clr::W) writer structure"]
impl crate::Writable for CLR {}
#[doc = "Port Output Clear register using FIOMASK."]
pub mod clr;
