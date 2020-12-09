#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Horizontal Timing Control register"]
    pub timh: TIMH,
    #[doc = "0x04 - Vertical Timing Control register"]
    pub timv: TIMV,
    #[doc = "0x08 - Clock and Signal Polarity Control register"]
    pub pol: POL,
    #[doc = "0x0c - Line End Control register"]
    pub le: LE,
    #[doc = "0x10 - Upper Panel Frame Base Address register"]
    pub upbase: UPBASE,
    #[doc = "0x14 - Lower Panel Frame Base Address register"]
    pub lpbase: LPBASE,
    #[doc = "0x18 - LCD Control register"]
    pub ctrl: CTRL,
    #[doc = "0x1c - Interrupt Mask register"]
    pub intmsk: INTMSK,
    #[doc = "0x20 - Raw Interrupt Status register"]
    pub intraw: INTRAW,
    #[doc = "0x24 - Masked Interrupt Status register"]
    pub intstat: INTSTAT,
    #[doc = "0x28 - Interrupt Clear register"]
    pub intclr: INTCLR,
    #[doc = "0x2c - Upper Panel Current Address Value register"]
    pub upcurr: UPCURR,
    #[doc = "0x30 - Lower Panel Current Address Value register"]
    pub lpcurr: LPCURR,
    _reserved13: [u8; 460usize],
    #[doc = "0x200 - 256x16-bit Color Palette registers"]
    pub pal: [PAL; 128],
    _reserved14: [u8; 1024usize],
    #[doc = "0x800 - Cursor Image registers"]
    pub crsr_img: [CRSR_IMG; 256],
    #[doc = "0xc00 - Cursor Control register"]
    pub crsr_ctrl: CRSR_CTRL,
    #[doc = "0xc04 - Cursor Configuration register"]
    pub crsr_cfg: CRSR_CFG,
    #[doc = "0xc08 - Cursor Palette register 0"]
    pub crsr_pal0: CRSR_PAL0,
    #[doc = "0xc0c - Cursor Palette register 1"]
    pub crsr_pal1: CRSR_PAL1,
    #[doc = "0xc10 - Cursor XY Position register"]
    pub crsr_xy: CRSR_XY,
    #[doc = "0xc14 - Cursor Clip Position register"]
    pub crsr_clip: CRSR_CLIP,
    _reserved21: [u8; 8usize],
    #[doc = "0xc20 - Cursor Interrupt Mask register"]
    pub crsr_intmsk: CRSR_INTMSK,
    #[doc = "0xc24 - Cursor Interrupt Clear register"]
    pub crsr_intclr: CRSR_INTCLR,
    #[doc = "0xc28 - Cursor Raw Interrupt Status register"]
    pub crsr_intraw: CRSR_INTRAW,
    #[doc = "0xc2c - Cursor Masked Interrupt Status register"]
    pub crsr_intstat: CRSR_INTSTAT,
}
#[doc = "Horizontal Timing Control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [timh](timh) module"]
pub type TIMH = crate::Reg<u32, _TIMH>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIMH;
#[doc = "`read()` method returns [timh::R](timh::R) reader structure"]
impl crate::Readable for TIMH {}
#[doc = "`write(|w| ..)` method takes [timh::W](timh::W) writer structure"]
impl crate::Writable for TIMH {}
#[doc = "Horizontal Timing Control register"]
pub mod timh;
#[doc = "Vertical Timing Control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [timv](timv) module"]
pub type TIMV = crate::Reg<u32, _TIMV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIMV;
#[doc = "`read()` method returns [timv::R](timv::R) reader structure"]
impl crate::Readable for TIMV {}
#[doc = "`write(|w| ..)` method takes [timv::W](timv::W) writer structure"]
impl crate::Writable for TIMV {}
#[doc = "Vertical Timing Control register"]
pub mod timv;
#[doc = "Clock and Signal Polarity Control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pol](pol) module"]
pub type POL = crate::Reg<u32, _POL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _POL;
#[doc = "`read()` method returns [pol::R](pol::R) reader structure"]
impl crate::Readable for POL {}
#[doc = "`write(|w| ..)` method takes [pol::W](pol::W) writer structure"]
impl crate::Writable for POL {}
#[doc = "Clock and Signal Polarity Control register"]
pub mod pol;
#[doc = "Line End Control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [le](le) module"]
pub type LE = crate::Reg<u32, _LE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LE;
#[doc = "`read()` method returns [le::R](le::R) reader structure"]
impl crate::Readable for LE {}
#[doc = "`write(|w| ..)` method takes [le::W](le::W) writer structure"]
impl crate::Writable for LE {}
#[doc = "Line End Control register"]
pub mod le;
#[doc = "Upper Panel Frame Base Address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [upbase](upbase) module"]
pub type UPBASE = crate::Reg<u32, _UPBASE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UPBASE;
#[doc = "`read()` method returns [upbase::R](upbase::R) reader structure"]
impl crate::Readable for UPBASE {}
#[doc = "`write(|w| ..)` method takes [upbase::W](upbase::W) writer structure"]
impl crate::Writable for UPBASE {}
#[doc = "Upper Panel Frame Base Address register"]
pub mod upbase;
#[doc = "Lower Panel Frame Base Address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lpbase](lpbase) module"]
pub type LPBASE = crate::Reg<u32, _LPBASE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LPBASE;
#[doc = "`read()` method returns [lpbase::R](lpbase::R) reader structure"]
impl crate::Readable for LPBASE {}
#[doc = "`write(|w| ..)` method takes [lpbase::W](lpbase::W) writer structure"]
impl crate::Writable for LPBASE {}
#[doc = "Lower Panel Frame Base Address register"]
pub mod lpbase;
#[doc = "LCD Control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrl](ctrl) module"]
pub type CTRL = crate::Reg<u32, _CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CTRL;
#[doc = "`read()` method returns [ctrl::R](ctrl::R) reader structure"]
impl crate::Readable for CTRL {}
#[doc = "`write(|w| ..)` method takes [ctrl::W](ctrl::W) writer structure"]
impl crate::Writable for CTRL {}
#[doc = "LCD Control register"]
pub mod ctrl;
#[doc = "Interrupt Mask register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intmsk](intmsk) module"]
pub type INTMSK = crate::Reg<u32, _INTMSK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTMSK;
#[doc = "`read()` method returns [intmsk::R](intmsk::R) reader structure"]
impl crate::Readable for INTMSK {}
#[doc = "`write(|w| ..)` method takes [intmsk::W](intmsk::W) writer structure"]
impl crate::Writable for INTMSK {}
#[doc = "Interrupt Mask register"]
pub mod intmsk;
#[doc = "Raw Interrupt Status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intraw](intraw) module"]
pub type INTRAW = crate::Reg<u32, _INTRAW>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTRAW;
#[doc = "`read()` method returns [intraw::R](intraw::R) reader structure"]
impl crate::Readable for INTRAW {}
#[doc = "Raw Interrupt Status register"]
pub mod intraw;
#[doc = "Masked Interrupt Status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intstat](intstat) module"]
pub type INTSTAT = crate::Reg<u32, _INTSTAT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTSTAT;
#[doc = "`read()` method returns [intstat::R](intstat::R) reader structure"]
impl crate::Readable for INTSTAT {}
#[doc = "Masked Interrupt Status register"]
pub mod intstat;
#[doc = "Interrupt Clear register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intclr](intclr) module"]
pub type INTCLR = crate::Reg<u32, _INTCLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTCLR;
#[doc = "`read()` method returns [intclr::R](intclr::R) reader structure"]
impl crate::Readable for INTCLR {}
#[doc = "`write(|w| ..)` method takes [intclr::W](intclr::W) writer structure"]
impl crate::Writable for INTCLR {}
#[doc = "Interrupt Clear register"]
pub mod intclr;
#[doc = "Upper Panel Current Address Value register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [upcurr](upcurr) module"]
pub type UPCURR = crate::Reg<u32, _UPCURR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UPCURR;
#[doc = "`read()` method returns [upcurr::R](upcurr::R) reader structure"]
impl crate::Readable for UPCURR {}
#[doc = "Upper Panel Current Address Value register"]
pub mod upcurr;
#[doc = "Lower Panel Current Address Value register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lpcurr](lpcurr) module"]
pub type LPCURR = crate::Reg<u32, _LPCURR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LPCURR;
#[doc = "`read()` method returns [lpcurr::R](lpcurr::R) reader structure"]
impl crate::Readable for LPCURR {}
#[doc = "Lower Panel Current Address Value register"]
pub mod lpcurr;
#[doc = "256x16-bit Color Palette registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pal](pal) module"]
pub type PAL = crate::Reg<u32, _PAL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PAL;
#[doc = "`read()` method returns [pal::R](pal::R) reader structure"]
impl crate::Readable for PAL {}
#[doc = "`write(|w| ..)` method takes [pal::W](pal::W) writer structure"]
impl crate::Writable for PAL {}
#[doc = "256x16-bit Color Palette registers"]
pub mod pal;
#[doc = "Cursor Image registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [crsr_img](crsr_img) module"]
pub type CRSR_IMG = crate::Reg<u32, _CRSR_IMG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CRSR_IMG;
#[doc = "`read()` method returns [crsr_img::R](crsr_img::R) reader structure"]
impl crate::Readable for CRSR_IMG {}
#[doc = "`write(|w| ..)` method takes [crsr_img::W](crsr_img::W) writer structure"]
impl crate::Writable for CRSR_IMG {}
#[doc = "Cursor Image registers"]
pub mod crsr_img;
#[doc = "Cursor Control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [crsr_ctrl](crsr_ctrl) module"]
pub type CRSR_CTRL = crate::Reg<u32, _CRSR_CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CRSR_CTRL;
#[doc = "`read()` method returns [crsr_ctrl::R](crsr_ctrl::R) reader structure"]
impl crate::Readable for CRSR_CTRL {}
#[doc = "`write(|w| ..)` method takes [crsr_ctrl::W](crsr_ctrl::W) writer structure"]
impl crate::Writable for CRSR_CTRL {}
#[doc = "Cursor Control register"]
pub mod crsr_ctrl;
#[doc = "Cursor Configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [crsr_cfg](crsr_cfg) module"]
pub type CRSR_CFG = crate::Reg<u32, _CRSR_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CRSR_CFG;
#[doc = "`read()` method returns [crsr_cfg::R](crsr_cfg::R) reader structure"]
impl crate::Readable for CRSR_CFG {}
#[doc = "`write(|w| ..)` method takes [crsr_cfg::W](crsr_cfg::W) writer structure"]
impl crate::Writable for CRSR_CFG {}
#[doc = "Cursor Configuration register"]
pub mod crsr_cfg;
#[doc = "Cursor Palette register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [crsr_pal0](crsr_pal0) module"]
pub type CRSR_PAL0 = crate::Reg<u32, _CRSR_PAL0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CRSR_PAL0;
#[doc = "`read()` method returns [crsr_pal0::R](crsr_pal0::R) reader structure"]
impl crate::Readable for CRSR_PAL0 {}
#[doc = "`write(|w| ..)` method takes [crsr_pal0::W](crsr_pal0::W) writer structure"]
impl crate::Writable for CRSR_PAL0 {}
#[doc = "Cursor Palette register 0"]
pub mod crsr_pal0;
#[doc = "Cursor Palette register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [crsr_pal1](crsr_pal1) module"]
pub type CRSR_PAL1 = crate::Reg<u32, _CRSR_PAL1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CRSR_PAL1;
#[doc = "`read()` method returns [crsr_pal1::R](crsr_pal1::R) reader structure"]
impl crate::Readable for CRSR_PAL1 {}
#[doc = "`write(|w| ..)` method takes [crsr_pal1::W](crsr_pal1::W) writer structure"]
impl crate::Writable for CRSR_PAL1 {}
#[doc = "Cursor Palette register 1"]
pub mod crsr_pal1;
#[doc = "Cursor XY Position register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [crsr_xy](crsr_xy) module"]
pub type CRSR_XY = crate::Reg<u32, _CRSR_XY>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CRSR_XY;
#[doc = "`read()` method returns [crsr_xy::R](crsr_xy::R) reader structure"]
impl crate::Readable for CRSR_XY {}
#[doc = "`write(|w| ..)` method takes [crsr_xy::W](crsr_xy::W) writer structure"]
impl crate::Writable for CRSR_XY {}
#[doc = "Cursor XY Position register"]
pub mod crsr_xy;
#[doc = "Cursor Clip Position register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [crsr_clip](crsr_clip) module"]
pub type CRSR_CLIP = crate::Reg<u32, _CRSR_CLIP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CRSR_CLIP;
#[doc = "`read()` method returns [crsr_clip::R](crsr_clip::R) reader structure"]
impl crate::Readable for CRSR_CLIP {}
#[doc = "`write(|w| ..)` method takes [crsr_clip::W](crsr_clip::W) writer structure"]
impl crate::Writable for CRSR_CLIP {}
#[doc = "Cursor Clip Position register"]
pub mod crsr_clip;
#[doc = "Cursor Interrupt Mask register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [crsr_intmsk](crsr_intmsk) module"]
pub type CRSR_INTMSK = crate::Reg<u32, _CRSR_INTMSK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CRSR_INTMSK;
#[doc = "`read()` method returns [crsr_intmsk::R](crsr_intmsk::R) reader structure"]
impl crate::Readable for CRSR_INTMSK {}
#[doc = "`write(|w| ..)` method takes [crsr_intmsk::W](crsr_intmsk::W) writer structure"]
impl crate::Writable for CRSR_INTMSK {}
#[doc = "Cursor Interrupt Mask register"]
pub mod crsr_intmsk;
#[doc = "Cursor Interrupt Clear register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [crsr_intclr](crsr_intclr) module"]
pub type CRSR_INTCLR = crate::Reg<u32, _CRSR_INTCLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CRSR_INTCLR;
#[doc = "`write(|w| ..)` method takes [crsr_intclr::W](crsr_intclr::W) writer structure"]
impl crate::Writable for CRSR_INTCLR {}
#[doc = "Cursor Interrupt Clear register"]
pub mod crsr_intclr;
#[doc = "Cursor Raw Interrupt Status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [crsr_intraw](crsr_intraw) module"]
pub type CRSR_INTRAW = crate::Reg<u32, _CRSR_INTRAW>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CRSR_INTRAW;
#[doc = "`read()` method returns [crsr_intraw::R](crsr_intraw::R) reader structure"]
impl crate::Readable for CRSR_INTRAW {}
#[doc = "Cursor Raw Interrupt Status register"]
pub mod crsr_intraw;
#[doc = "Cursor Masked Interrupt Status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [crsr_intstat](crsr_intstat) module"]
pub type CRSR_INTSTAT = crate::Reg<u32, _CRSR_INTSTAT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CRSR_INTSTAT;
#[doc = "`read()` method returns [crsr_intstat::R](crsr_intstat::R) reader structure"]
impl crate::Readable for CRSR_INTSTAT {}
#[doc = "Cursor Masked Interrupt Status register"]
pub mod crsr_intstat;
