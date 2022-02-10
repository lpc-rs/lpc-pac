///Register block
#[repr(C)]
pub struct RegisterBlock {
    ///0x00..0x94 - no description available
    pub channel0: CHANNEL,
    _reserved1: [u8; 0x6c],
    ///0x100..0x194 - no description available
    pub channel1: CHANNEL,
    _reserved2: [u8; 0x0d6c],
    ///0xf00 - Channel Enable register
    pub chanen: crate::Reg<chanen::CHANEN_SPEC>,
    _reserved3: [u8; 0x08],
    ///0xf0c - I/O Configuration register
    pub iocfg: crate::Reg<iocfg::IOCFG_SPEC>,
    ///0xf10 - Use 2FS register
    pub use2fs: crate::Reg<use2fs::USE2FS_SPEC>,
    _reserved5: [u8; 0x6c],
    ///0xf80 - HWVAD input gain register
    pub hwvadgain: crate::Reg<hwvadgain::HWVADGAIN_SPEC>,
    ///0xf84 - HWVAD filter control register
    pub hwvadhpfs: crate::Reg<hwvadhpfs::HWVADHPFS_SPEC>,
    ///0xf88 - HWVAD control register
    pub hwvadst10: crate::Reg<hwvadst10::HWVADST10_SPEC>,
    ///0xf8c - HWVAD filter reset register
    pub hwvadrstt: crate::Reg<hwvadrstt::HWVADRSTT_SPEC>,
    ///0xf90 - HWVAD noise estimator gain register
    pub hwvadthgn: crate::Reg<hwvadthgn::HWVADTHGN_SPEC>,
    ///0xf94 - HWVAD signal estimator gain register
    pub hwvadthgs: crate::Reg<hwvadthgs::HWVADTHGS_SPEC>,
    ///0xf98 - HWVAD noise envelope estimator register
    pub hwvadlowz: crate::Reg<hwvadlowz::HWVADLOWZ_SPEC>,
    _reserved12: [u8; 0x60],
    ///0xffc - Module Identification register
    pub id: crate::Reg<id::ID_SPEC>,
}
///Register block
#[repr(C)]
pub struct CHANNEL {
    ///0x00 - Oversample Rate register 0
    pub osr: crate::Reg<self::channel::osr::OSR_SPEC>,
    ///0x04 - DMIC Clock Register 0
    pub divhfclk: crate::Reg<self::channel::divhfclk::DIVHFCLK_SPEC>,
    ///0x08 - Pre-Emphasis Filter Coefficient for 2 FS register
    pub preac2fscoef: crate::Reg<self::channel::preac2fscoef::PREAC2FSCOEF_SPEC>,
    ///0x0c - Pre-Emphasis Filter Coefficient for 4 FS register
    pub preac4fscoef: crate::Reg<self::channel::preac4fscoef::PREAC4FSCOEF_SPEC>,
    ///0x10 - Decimator Gain Shift register
    pub gainshift: crate::Reg<self::channel::gainshift::GAINSHIFT_SPEC>,
    _reserved5: [u8; 0x6c],
    ///0x80 - FIFO Control register 0
    pub fifo_ctrl: crate::Reg<self::channel::fifo_ctrl::FIFO_CTRL_SPEC>,
    ///0x84 - FIFO Status register 0
    pub fifo_status: crate::Reg<self::channel::fifo_status::FIFO_STATUS_SPEC>,
    ///0x88 - FIFO Data Register 0
    pub fifo_data: crate::Reg<self::channel::fifo_data::FIFO_DATA_SPEC>,
    ///0x8c - PDM Source Configuration register 0
    pub phy_ctrl: crate::Reg<self::channel::phy_ctrl::PHY_CTRL_SPEC>,
    ///0x90 - DC Control register 0
    pub dc_ctrl: crate::Reg<self::channel::dc_ctrl::DC_CTRL_SPEC>,
}
///Register block
///no description available
pub mod channel;
///CHANEN register accessor: an alias for `Reg<CHANEN_SPEC>`
pub type CHANEN = crate::Reg<chanen::CHANEN_SPEC>;
///Channel Enable register
pub mod chanen;
///IOCFG register accessor: an alias for `Reg<IOCFG_SPEC>`
pub type IOCFG = crate::Reg<iocfg::IOCFG_SPEC>;
///I/O Configuration register
pub mod iocfg;
///USE2FS register accessor: an alias for `Reg<USE2FS_SPEC>`
pub type USE2FS = crate::Reg<use2fs::USE2FS_SPEC>;
///Use 2FS register
pub mod use2fs;
///HWVADGAIN register accessor: an alias for `Reg<HWVADGAIN_SPEC>`
pub type HWVADGAIN = crate::Reg<hwvadgain::HWVADGAIN_SPEC>;
///HWVAD input gain register
pub mod hwvadgain;
///HWVADHPFS register accessor: an alias for `Reg<HWVADHPFS_SPEC>`
pub type HWVADHPFS = crate::Reg<hwvadhpfs::HWVADHPFS_SPEC>;
///HWVAD filter control register
pub mod hwvadhpfs;
///HWVADST10 register accessor: an alias for `Reg<HWVADST10_SPEC>`
pub type HWVADST10 = crate::Reg<hwvadst10::HWVADST10_SPEC>;
///HWVAD control register
pub mod hwvadst10;
///HWVADRSTT register accessor: an alias for `Reg<HWVADRSTT_SPEC>`
pub type HWVADRSTT = crate::Reg<hwvadrstt::HWVADRSTT_SPEC>;
///HWVAD filter reset register
pub mod hwvadrstt;
///HWVADTHGN register accessor: an alias for `Reg<HWVADTHGN_SPEC>`
pub type HWVADTHGN = crate::Reg<hwvadthgn::HWVADTHGN_SPEC>;
///HWVAD noise estimator gain register
pub mod hwvadthgn;
///HWVADTHGS register accessor: an alias for `Reg<HWVADTHGS_SPEC>`
pub type HWVADTHGS = crate::Reg<hwvadthgs::HWVADTHGS_SPEC>;
///HWVAD signal estimator gain register
pub mod hwvadthgs;
///HWVADLOWZ register accessor: an alias for `Reg<HWVADLOWZ_SPEC>`
pub type HWVADLOWZ = crate::Reg<hwvadlowz::HWVADLOWZ_SPEC>;
///HWVAD noise envelope estimator register
pub mod hwvadlowz;
///ID register accessor: an alias for `Reg<ID_SPEC>`
pub type ID = crate::Reg<id::ID_SPEC>;
///Module Identification register
pub mod id;
