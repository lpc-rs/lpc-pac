///Register block
#[repr(C)]
pub struct RegisterBlock {
    ///0x00 - Control register
    pub fctr: crate::Reg<fctr::FCTR_SPEC>,
    _reserved1: [u8; 0x0c],
    ///0x10 - Wait state register
    pub fbwst: crate::Reg<fbwst::FBWST_SPEC>,
    _reserved2: [u8; 0x0c],
    ///0x20 - Signature start address register
    pub fmsstart: crate::Reg<fmsstart::FMSSTART_SPEC>,
    ///0x24 - Signature stop-address register
    pub fmsstop: crate::Reg<fmsstop::FMSSTOP_SPEC>,
    _reserved4: [u8; 0x04],
    ///0x2c..0x3c - Words of 128-bit signature word
    pub fmsw: [crate::Reg<fmsw::FMSW_SPEC>; 4],
    _reserved5: [u8; 0x0fa4],
    ///0xfe0 - Signature generation status register
    pub fmstat: crate::Reg<fmstat::FMSTAT_SPEC>,
    _reserved6: [u8; 0x04],
    ///0xfe8 - Signature generation status clear register
    pub fmstatclr: crate::Reg<fmstatclr::FMSTATCLR_SPEC>,
}
///FCTR register accessor: an alias for `Reg<FCTR_SPEC>`
pub type FCTR = crate::Reg<fctr::FCTR_SPEC>;
///Control register
pub mod fctr;
///FBWST register accessor: an alias for `Reg<FBWST_SPEC>`
pub type FBWST = crate::Reg<fbwst::FBWST_SPEC>;
///Wait state register
pub mod fbwst;
///FMSSTART register accessor: an alias for `Reg<FMSSTART_SPEC>`
pub type FMSSTART = crate::Reg<fmsstart::FMSSTART_SPEC>;
///Signature start address register
pub mod fmsstart;
///FMSSTOP register accessor: an alias for `Reg<FMSSTOP_SPEC>`
pub type FMSSTOP = crate::Reg<fmsstop::FMSSTOP_SPEC>;
///Signature stop-address register
pub mod fmsstop;
///FMSW register accessor: an alias for `Reg<FMSW_SPEC>`
pub type FMSW = crate::Reg<fmsw::FMSW_SPEC>;
///Words of 128-bit signature word
pub mod fmsw;
///FMSTAT register accessor: an alias for `Reg<FMSTAT_SPEC>`
pub type FMSTAT = crate::Reg<fmstat::FMSTAT_SPEC>;
///Signature generation status register
pub mod fmstat;
///FMSTATCLR register accessor: an alias for `Reg<FMSTATCLR_SPEC>`
pub type FMSTATCLR = crate::Reg<fmstatclr::FMSTATCLR_SPEC>;
///Signature generation status clear register
pub mod fmstatclr;
