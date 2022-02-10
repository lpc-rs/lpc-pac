///Register block
#[repr(C)]
pub struct RegisterBlock {
    ///0x00 - Horizontal Timing Control register
    pub timh: crate::Reg<timh::TIMH_SPEC>,
    ///0x04 - Vertical Timing Control register
    pub timv: crate::Reg<timv::TIMV_SPEC>,
    ///0x08 - Clock and Signal Polarity Control register
    pub pol: crate::Reg<pol::POL_SPEC>,
    ///0x0c - Line End Control register
    pub le: crate::Reg<le::LE_SPEC>,
    ///0x10 - Upper Panel Frame Base Address register
    pub upbase: crate::Reg<upbase::UPBASE_SPEC>,
    ///0x14 - Lower Panel Frame Base Address register
    pub lpbase: crate::Reg<lpbase::LPBASE_SPEC>,
    ///0x18 - LCD Control register
    pub ctrl: crate::Reg<ctrl::CTRL_SPEC>,
    ///0x1c - Interrupt Mask register
    pub intmsk: crate::Reg<intmsk::INTMSK_SPEC>,
    ///0x20 - Raw Interrupt Status register
    pub intraw: crate::Reg<intraw::INTRAW_SPEC>,
    ///0x24 - Masked Interrupt Status register
    pub intstat: crate::Reg<intstat::INTSTAT_SPEC>,
    ///0x28 - Interrupt Clear register
    pub intclr: crate::Reg<intclr::INTCLR_SPEC>,
    ///0x2c - Upper Panel Current Address Value register
    pub upcurr: crate::Reg<upcurr::UPCURR_SPEC>,
    ///0x30 - Lower Panel Current Address Value register
    pub lpcurr: crate::Reg<lpcurr::LPCURR_SPEC>,
    _reserved13: [u8; 0x01cc],
    ///0x200..0x400 - 256x16-bit Color Palette registers
    pub pal: [crate::Reg<pal::PAL_SPEC>; 128],
    _reserved14: [u8; 0x0400],
    ///0x800..0xc00 - Cursor Image registers
    pub crsr_img: [crate::Reg<crsr_img::CRSR_IMG_SPEC>; 256],
    ///0xc00 - Cursor Control register
    pub crsr_ctrl: crate::Reg<crsr_ctrl::CRSR_CTRL_SPEC>,
    ///0xc04 - Cursor Configuration register
    pub crsr_cfg: crate::Reg<crsr_cfg::CRSR_CFG_SPEC>,
    ///0xc08 - Cursor Palette register 0
    pub crsr_pal0: crate::Reg<crsr_pal0::CRSR_PAL0_SPEC>,
    ///0xc0c - Cursor Palette register 1
    pub crsr_pal1: crate::Reg<crsr_pal1::CRSR_PAL1_SPEC>,
    ///0xc10 - Cursor XY Position register
    pub crsr_xy: crate::Reg<crsr_xy::CRSR_XY_SPEC>,
    ///0xc14 - Cursor Clip Position register
    pub crsr_clip: crate::Reg<crsr_clip::CRSR_CLIP_SPEC>,
    _reserved21: [u8; 0x08],
    ///0xc20 - Cursor Interrupt Mask register
    pub crsr_intmsk: crate::Reg<crsr_intmsk::CRSR_INTMSK_SPEC>,
    ///0xc24 - Cursor Interrupt Clear register
    pub crsr_intclr: crate::Reg<crsr_intclr::CRSR_INTCLR_SPEC>,
    ///0xc28 - Cursor Raw Interrupt Status register
    pub crsr_intraw: crate::Reg<crsr_intraw::CRSR_INTRAW_SPEC>,
    ///0xc2c - Cursor Masked Interrupt Status register
    pub crsr_intstat: crate::Reg<crsr_intstat::CRSR_INTSTAT_SPEC>,
}
///TIMH register accessor: an alias for `Reg<TIMH_SPEC>`
pub type TIMH = crate::Reg<timh::TIMH_SPEC>;
///Horizontal Timing Control register
pub mod timh;
///TIMV register accessor: an alias for `Reg<TIMV_SPEC>`
pub type TIMV = crate::Reg<timv::TIMV_SPEC>;
///Vertical Timing Control register
pub mod timv;
///POL register accessor: an alias for `Reg<POL_SPEC>`
pub type POL = crate::Reg<pol::POL_SPEC>;
///Clock and Signal Polarity Control register
pub mod pol;
///LE register accessor: an alias for `Reg<LE_SPEC>`
pub type LE = crate::Reg<le::LE_SPEC>;
///Line End Control register
pub mod le;
///UPBASE register accessor: an alias for `Reg<UPBASE_SPEC>`
pub type UPBASE = crate::Reg<upbase::UPBASE_SPEC>;
///Upper Panel Frame Base Address register
pub mod upbase;
///LPBASE register accessor: an alias for `Reg<LPBASE_SPEC>`
pub type LPBASE = crate::Reg<lpbase::LPBASE_SPEC>;
///Lower Panel Frame Base Address register
pub mod lpbase;
///CTRL register accessor: an alias for `Reg<CTRL_SPEC>`
pub type CTRL = crate::Reg<ctrl::CTRL_SPEC>;
///LCD Control register
pub mod ctrl;
///INTMSK register accessor: an alias for `Reg<INTMSK_SPEC>`
pub type INTMSK = crate::Reg<intmsk::INTMSK_SPEC>;
///Interrupt Mask register
pub mod intmsk;
///INTRAW register accessor: an alias for `Reg<INTRAW_SPEC>`
pub type INTRAW = crate::Reg<intraw::INTRAW_SPEC>;
///Raw Interrupt Status register
pub mod intraw;
///INTSTAT register accessor: an alias for `Reg<INTSTAT_SPEC>`
pub type INTSTAT = crate::Reg<intstat::INTSTAT_SPEC>;
///Masked Interrupt Status register
pub mod intstat;
///INTCLR register accessor: an alias for `Reg<INTCLR_SPEC>`
pub type INTCLR = crate::Reg<intclr::INTCLR_SPEC>;
///Interrupt Clear register
pub mod intclr;
///UPCURR register accessor: an alias for `Reg<UPCURR_SPEC>`
pub type UPCURR = crate::Reg<upcurr::UPCURR_SPEC>;
///Upper Panel Current Address Value register
pub mod upcurr;
///LPCURR register accessor: an alias for `Reg<LPCURR_SPEC>`
pub type LPCURR = crate::Reg<lpcurr::LPCURR_SPEC>;
///Lower Panel Current Address Value register
pub mod lpcurr;
///PAL register accessor: an alias for `Reg<PAL_SPEC>`
pub type PAL = crate::Reg<pal::PAL_SPEC>;
///256x16-bit Color Palette registers
pub mod pal;
///CRSR_IMG register accessor: an alias for `Reg<CRSR_IMG_SPEC>`
pub type CRSR_IMG = crate::Reg<crsr_img::CRSR_IMG_SPEC>;
///Cursor Image registers
pub mod crsr_img;
///CRSR_CTRL register accessor: an alias for `Reg<CRSR_CTRL_SPEC>`
pub type CRSR_CTRL = crate::Reg<crsr_ctrl::CRSR_CTRL_SPEC>;
///Cursor Control register
pub mod crsr_ctrl;
///CRSR_CFG register accessor: an alias for `Reg<CRSR_CFG_SPEC>`
pub type CRSR_CFG = crate::Reg<crsr_cfg::CRSR_CFG_SPEC>;
///Cursor Configuration register
pub mod crsr_cfg;
///CRSR_PAL0 register accessor: an alias for `Reg<CRSR_PAL0_SPEC>`
pub type CRSR_PAL0 = crate::Reg<crsr_pal0::CRSR_PAL0_SPEC>;
///Cursor Palette register 0
pub mod crsr_pal0;
///CRSR_PAL1 register accessor: an alias for `Reg<CRSR_PAL1_SPEC>`
pub type CRSR_PAL1 = crate::Reg<crsr_pal1::CRSR_PAL1_SPEC>;
///Cursor Palette register 1
pub mod crsr_pal1;
///CRSR_XY register accessor: an alias for `Reg<CRSR_XY_SPEC>`
pub type CRSR_XY = crate::Reg<crsr_xy::CRSR_XY_SPEC>;
///Cursor XY Position register
pub mod crsr_xy;
///CRSR_CLIP register accessor: an alias for `Reg<CRSR_CLIP_SPEC>`
pub type CRSR_CLIP = crate::Reg<crsr_clip::CRSR_CLIP_SPEC>;
///Cursor Clip Position register
pub mod crsr_clip;
///CRSR_INTMSK register accessor: an alias for `Reg<CRSR_INTMSK_SPEC>`
pub type CRSR_INTMSK = crate::Reg<crsr_intmsk::CRSR_INTMSK_SPEC>;
///Cursor Interrupt Mask register
pub mod crsr_intmsk;
///CRSR_INTCLR register accessor: an alias for `Reg<CRSR_INTCLR_SPEC>`
pub type CRSR_INTCLR = crate::Reg<crsr_intclr::CRSR_INTCLR_SPEC>;
///Cursor Interrupt Clear register
pub mod crsr_intclr;
///CRSR_INTRAW register accessor: an alias for `Reg<CRSR_INTRAW_SPEC>`
pub type CRSR_INTRAW = crate::Reg<crsr_intraw::CRSR_INTRAW_SPEC>;
///Cursor Raw Interrupt Status register
pub mod crsr_intraw;
///CRSR_INTSTAT register accessor: an alias for `Reg<CRSR_INTSTAT_SPEC>`
pub type CRSR_INTSTAT = crate::Reg<crsr_intstat::CRSR_INTSTAT_SPEC>;
///Cursor Masked Interrupt Status register
pub mod crsr_intstat;
