#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Horizontal Timing Control register"]
    pub timh: crate::Reg<timh::TIMH_SPEC>,
    #[doc = "0x04 - Vertical Timing Control register"]
    pub timv: crate::Reg<timv::TIMV_SPEC>,
    #[doc = "0x08 - Clock and Signal Polarity Control register"]
    pub pol: crate::Reg<pol::POL_SPEC>,
    #[doc = "0x0c - Line End Control register"]
    pub le: crate::Reg<le::LE_SPEC>,
    #[doc = "0x10 - Upper Panel Frame Base Address register"]
    pub upbase: crate::Reg<upbase::UPBASE_SPEC>,
    #[doc = "0x14 - Lower Panel Frame Base Address register"]
    pub lpbase: crate::Reg<lpbase::LPBASE_SPEC>,
    #[doc = "0x18 - LCD Control register"]
    pub ctrl: crate::Reg<ctrl::CTRL_SPEC>,
    #[doc = "0x1c - Interrupt Mask register"]
    pub intmsk: crate::Reg<intmsk::INTMSK_SPEC>,
    #[doc = "0x20 - Raw Interrupt Status register"]
    pub intraw: crate::Reg<intraw::INTRAW_SPEC>,
    #[doc = "0x24 - Masked Interrupt Status register"]
    pub intstat: crate::Reg<intstat::INTSTAT_SPEC>,
    #[doc = "0x28 - Interrupt Clear register"]
    pub intclr: crate::Reg<intclr::INTCLR_SPEC>,
    #[doc = "0x2c - Upper Panel Current Address Value register"]
    pub upcurr: crate::Reg<upcurr::UPCURR_SPEC>,
    #[doc = "0x30 - Lower Panel Current Address Value register"]
    pub lpcurr: crate::Reg<lpcurr::LPCURR_SPEC>,
    _reserved13: [u8; 0x01cc],
    #[doc = "0x200..0x400 - 256x16-bit Color Palette registers"]
    pub pal: [crate::Reg<pal::PAL_SPEC>; 128],
    _reserved14: [u8; 0x0400],
    #[doc = "0x800..0xc00 - Cursor Image registers"]
    pub crsr_img: [crate::Reg<crsr_img::CRSR_IMG_SPEC>; 256],
    #[doc = "0xc00 - Cursor Control register"]
    pub crsr_ctrl: crate::Reg<crsr_ctrl::CRSR_CTRL_SPEC>,
    #[doc = "0xc04 - Cursor Configuration register"]
    pub crsr_cfg: crate::Reg<crsr_cfg::CRSR_CFG_SPEC>,
    #[doc = "0xc08 - Cursor Palette register 0"]
    pub crsr_pal0: crate::Reg<crsr_pal0::CRSR_PAL0_SPEC>,
    #[doc = "0xc0c - Cursor Palette register 1"]
    pub crsr_pal1: crate::Reg<crsr_pal1::CRSR_PAL1_SPEC>,
    #[doc = "0xc10 - Cursor XY Position register"]
    pub crsr_xy: crate::Reg<crsr_xy::CRSR_XY_SPEC>,
    #[doc = "0xc14 - Cursor Clip Position register"]
    pub crsr_clip: crate::Reg<crsr_clip::CRSR_CLIP_SPEC>,
    _reserved21: [u8; 0x08],
    #[doc = "0xc20 - Cursor Interrupt Mask register"]
    pub crsr_intmsk: crate::Reg<crsr_intmsk::CRSR_INTMSK_SPEC>,
    #[doc = "0xc24 - Cursor Interrupt Clear register"]
    pub crsr_intclr: crate::Reg<crsr_intclr::CRSR_INTCLR_SPEC>,
    #[doc = "0xc28 - Cursor Raw Interrupt Status register"]
    pub crsr_intraw: crate::Reg<crsr_intraw::CRSR_INTRAW_SPEC>,
    #[doc = "0xc2c - Cursor Masked Interrupt Status register"]
    pub crsr_intstat: crate::Reg<crsr_intstat::CRSR_INTSTAT_SPEC>,
}
#[doc = "TIMH register accessor: an alias for `Reg<TIMH_SPEC>`"]
pub type TIMH = crate::Reg<timh::TIMH_SPEC>;
#[doc = "Horizontal Timing Control register"]
pub mod timh;
#[doc = "TIMV register accessor: an alias for `Reg<TIMV_SPEC>`"]
pub type TIMV = crate::Reg<timv::TIMV_SPEC>;
#[doc = "Vertical Timing Control register"]
pub mod timv;
#[doc = "POL register accessor: an alias for `Reg<POL_SPEC>`"]
pub type POL = crate::Reg<pol::POL_SPEC>;
#[doc = "Clock and Signal Polarity Control register"]
pub mod pol;
#[doc = "LE register accessor: an alias for `Reg<LE_SPEC>`"]
pub type LE = crate::Reg<le::LE_SPEC>;
#[doc = "Line End Control register"]
pub mod le;
#[doc = "UPBASE register accessor: an alias for `Reg<UPBASE_SPEC>`"]
pub type UPBASE = crate::Reg<upbase::UPBASE_SPEC>;
#[doc = "Upper Panel Frame Base Address register"]
pub mod upbase;
#[doc = "LPBASE register accessor: an alias for `Reg<LPBASE_SPEC>`"]
pub type LPBASE = crate::Reg<lpbase::LPBASE_SPEC>;
#[doc = "Lower Panel Frame Base Address register"]
pub mod lpbase;
#[doc = "CTRL register accessor: an alias for `Reg<CTRL_SPEC>`"]
pub type CTRL = crate::Reg<ctrl::CTRL_SPEC>;
#[doc = "LCD Control register"]
pub mod ctrl;
#[doc = "INTMSK register accessor: an alias for `Reg<INTMSK_SPEC>`"]
pub type INTMSK = crate::Reg<intmsk::INTMSK_SPEC>;
#[doc = "Interrupt Mask register"]
pub mod intmsk;
#[doc = "INTRAW register accessor: an alias for `Reg<INTRAW_SPEC>`"]
pub type INTRAW = crate::Reg<intraw::INTRAW_SPEC>;
#[doc = "Raw Interrupt Status register"]
pub mod intraw;
#[doc = "INTSTAT register accessor: an alias for `Reg<INTSTAT_SPEC>`"]
pub type INTSTAT = crate::Reg<intstat::INTSTAT_SPEC>;
#[doc = "Masked Interrupt Status register"]
pub mod intstat;
#[doc = "INTCLR register accessor: an alias for `Reg<INTCLR_SPEC>`"]
pub type INTCLR = crate::Reg<intclr::INTCLR_SPEC>;
#[doc = "Interrupt Clear register"]
pub mod intclr;
#[doc = "UPCURR register accessor: an alias for `Reg<UPCURR_SPEC>`"]
pub type UPCURR = crate::Reg<upcurr::UPCURR_SPEC>;
#[doc = "Upper Panel Current Address Value register"]
pub mod upcurr;
#[doc = "LPCURR register accessor: an alias for `Reg<LPCURR_SPEC>`"]
pub type LPCURR = crate::Reg<lpcurr::LPCURR_SPEC>;
#[doc = "Lower Panel Current Address Value register"]
pub mod lpcurr;
#[doc = "PAL register accessor: an alias for `Reg<PAL_SPEC>`"]
pub type PAL = crate::Reg<pal::PAL_SPEC>;
#[doc = "256x16-bit Color Palette registers"]
pub mod pal;
#[doc = "CRSR_IMG register accessor: an alias for `Reg<CRSR_IMG_SPEC>`"]
pub type CRSR_IMG = crate::Reg<crsr_img::CRSR_IMG_SPEC>;
#[doc = "Cursor Image registers"]
pub mod crsr_img;
#[doc = "CRSR_CTRL register accessor: an alias for `Reg<CRSR_CTRL_SPEC>`"]
pub type CRSR_CTRL = crate::Reg<crsr_ctrl::CRSR_CTRL_SPEC>;
#[doc = "Cursor Control register"]
pub mod crsr_ctrl;
#[doc = "CRSR_CFG register accessor: an alias for `Reg<CRSR_CFG_SPEC>`"]
pub type CRSR_CFG = crate::Reg<crsr_cfg::CRSR_CFG_SPEC>;
#[doc = "Cursor Configuration register"]
pub mod crsr_cfg;
#[doc = "CRSR_PAL0 register accessor: an alias for `Reg<CRSR_PAL0_SPEC>`"]
pub type CRSR_PAL0 = crate::Reg<crsr_pal0::CRSR_PAL0_SPEC>;
#[doc = "Cursor Palette register 0"]
pub mod crsr_pal0;
#[doc = "CRSR_PAL1 register accessor: an alias for `Reg<CRSR_PAL1_SPEC>`"]
pub type CRSR_PAL1 = crate::Reg<crsr_pal1::CRSR_PAL1_SPEC>;
#[doc = "Cursor Palette register 1"]
pub mod crsr_pal1;
#[doc = "CRSR_XY register accessor: an alias for `Reg<CRSR_XY_SPEC>`"]
pub type CRSR_XY = crate::Reg<crsr_xy::CRSR_XY_SPEC>;
#[doc = "Cursor XY Position register"]
pub mod crsr_xy;
#[doc = "CRSR_CLIP register accessor: an alias for `Reg<CRSR_CLIP_SPEC>`"]
pub type CRSR_CLIP = crate::Reg<crsr_clip::CRSR_CLIP_SPEC>;
#[doc = "Cursor Clip Position register"]
pub mod crsr_clip;
#[doc = "CRSR_INTMSK register accessor: an alias for `Reg<CRSR_INTMSK_SPEC>`"]
pub type CRSR_INTMSK = crate::Reg<crsr_intmsk::CRSR_INTMSK_SPEC>;
#[doc = "Cursor Interrupt Mask register"]
pub mod crsr_intmsk;
#[doc = "CRSR_INTCLR register accessor: an alias for `Reg<CRSR_INTCLR_SPEC>`"]
pub type CRSR_INTCLR = crate::Reg<crsr_intclr::CRSR_INTCLR_SPEC>;
#[doc = "Cursor Interrupt Clear register"]
pub mod crsr_intclr;
#[doc = "CRSR_INTRAW register accessor: an alias for `Reg<CRSR_INTRAW_SPEC>`"]
pub type CRSR_INTRAW = crate::Reg<crsr_intraw::CRSR_INTRAW_SPEC>;
#[doc = "Cursor Raw Interrupt Status register"]
pub mod crsr_intraw;
#[doc = "CRSR_INTSTAT register accessor: an alias for `Reg<CRSR_INTSTAT_SPEC>`"]
pub type CRSR_INTSTAT = crate::Reg<crsr_intstat::CRSR_INTSTAT_SPEC>;
#[doc = "Cursor Masked Interrupt Status register"]
pub mod crsr_intstat;
