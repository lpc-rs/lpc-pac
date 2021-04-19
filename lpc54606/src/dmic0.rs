#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - no description available"]
    pub channel0: CHANNEL,
    _reserved1: [u8; 108usize],
    #[doc = "0x100 - no description available"]
    pub channel1: CHANNEL,
    _reserved2: [u8; 3436usize],
    #[doc = "0xf00 - Channel Enable register"]
    pub chanen: crate::Reg<chanen::CHANEN_SPEC>,
    _reserved3: [u8; 8usize],
    #[doc = "0xf0c - I/O Configuration register"]
    pub iocfg: crate::Reg<iocfg::IOCFG_SPEC>,
    #[doc = "0xf10 - Use 2FS register"]
    pub use2fs: crate::Reg<use2fs::USE2FS_SPEC>,
    _reserved5: [u8; 108usize],
    #[doc = "0xf80 - HWVAD input gain register"]
    pub hwvadgain: crate::Reg<hwvadgain::HWVADGAIN_SPEC>,
    #[doc = "0xf84 - HWVAD filter control register"]
    pub hwvadhpfs: crate::Reg<hwvadhpfs::HWVADHPFS_SPEC>,
    #[doc = "0xf88 - HWVAD control register"]
    pub hwvadst10: crate::Reg<hwvadst10::HWVADST10_SPEC>,
    #[doc = "0xf8c - HWVAD filter reset register"]
    pub hwvadrstt: crate::Reg<hwvadrstt::HWVADRSTT_SPEC>,
    #[doc = "0xf90 - HWVAD noise estimator gain register"]
    pub hwvadthgn: crate::Reg<hwvadthgn::HWVADTHGN_SPEC>,
    #[doc = "0xf94 - HWVAD signal estimator gain register"]
    pub hwvadthgs: crate::Reg<hwvadthgs::HWVADTHGS_SPEC>,
    #[doc = "0xf98 - HWVAD noise envelope estimator register"]
    pub hwvadlowz: crate::Reg<hwvadlowz::HWVADLOWZ_SPEC>,
    _reserved12: [u8; 96usize],
    #[doc = "0xffc - Module Identification register"]
    pub id: crate::Reg<id::ID_SPEC>,
}
#[doc = r"Register block"]
#[repr(C)]
pub struct CHANNEL {
    #[doc = "0x00 - Oversample Rate register 0"]
    pub osr: crate::Reg<self::channel::osr::OSR_SPEC>,
    #[doc = "0x04 - DMIC Clock Register 0"]
    pub divhfclk: crate::Reg<self::channel::divhfclk::DIVHFCLK_SPEC>,
    #[doc = "0x08 - Pre-Emphasis Filter Coefficient for 2 FS register"]
    pub preac2fscoef: crate::Reg<self::channel::preac2fscoef::PREAC2FSCOEF_SPEC>,
    #[doc = "0x0c - Pre-Emphasis Filter Coefficient for 4 FS register"]
    pub preac4fscoef: crate::Reg<self::channel::preac4fscoef::PREAC4FSCOEF_SPEC>,
    #[doc = "0x10 - Decimator Gain Shift register"]
    pub gainshift: crate::Reg<self::channel::gainshift::GAINSHIFT_SPEC>,
    _reserved5: [u8; 108usize],
    #[doc = "0x80 - FIFO Control register 0"]
    pub fifo_ctrl: crate::Reg<self::channel::fifo_ctrl::FIFO_CTRL_SPEC>,
    #[doc = "0x84 - FIFO Status register 0"]
    pub fifo_status: crate::Reg<self::channel::fifo_status::FIFO_STATUS_SPEC>,
    #[doc = "0x88 - FIFO Data Register 0"]
    pub fifo_data: crate::Reg<self::channel::fifo_data::FIFO_DATA_SPEC>,
    #[doc = "0x8c - PDM Source Configuration register 0"]
    pub phy_ctrl: crate::Reg<self::channel::phy_ctrl::PHY_CTRL_SPEC>,
    #[doc = "0x90 - DC Control register 0"]
    pub dc_ctrl: crate::Reg<self::channel::dc_ctrl::DC_CTRL_SPEC>,
}
#[doc = r"Register block"]
#[doc = "no description available"]
pub mod channel;
#[doc = "CHANEN register accessor: an alias for `Reg<CHANEN_SPEC>`"]
pub type CHANEN = crate::Reg<chanen::CHANEN_SPEC>;
#[doc = "Channel Enable register"]
pub mod chanen;
#[doc = "IOCFG register accessor: an alias for `Reg<IOCFG_SPEC>`"]
pub type IOCFG = crate::Reg<iocfg::IOCFG_SPEC>;
#[doc = "I/O Configuration register"]
pub mod iocfg;
#[doc = "USE2FS register accessor: an alias for `Reg<USE2FS_SPEC>`"]
pub type USE2FS = crate::Reg<use2fs::USE2FS_SPEC>;
#[doc = "Use 2FS register"]
pub mod use2fs;
#[doc = "HWVADGAIN register accessor: an alias for `Reg<HWVADGAIN_SPEC>`"]
pub type HWVADGAIN = crate::Reg<hwvadgain::HWVADGAIN_SPEC>;
#[doc = "HWVAD input gain register"]
pub mod hwvadgain;
#[doc = "HWVADHPFS register accessor: an alias for `Reg<HWVADHPFS_SPEC>`"]
pub type HWVADHPFS = crate::Reg<hwvadhpfs::HWVADHPFS_SPEC>;
#[doc = "HWVAD filter control register"]
pub mod hwvadhpfs;
#[doc = "HWVADST10 register accessor: an alias for `Reg<HWVADST10_SPEC>`"]
pub type HWVADST10 = crate::Reg<hwvadst10::HWVADST10_SPEC>;
#[doc = "HWVAD control register"]
pub mod hwvadst10;
#[doc = "HWVADRSTT register accessor: an alias for `Reg<HWVADRSTT_SPEC>`"]
pub type HWVADRSTT = crate::Reg<hwvadrstt::HWVADRSTT_SPEC>;
#[doc = "HWVAD filter reset register"]
pub mod hwvadrstt;
#[doc = "HWVADTHGN register accessor: an alias for `Reg<HWVADTHGN_SPEC>`"]
pub type HWVADTHGN = crate::Reg<hwvadthgn::HWVADTHGN_SPEC>;
#[doc = "HWVAD noise estimator gain register"]
pub mod hwvadthgn;
#[doc = "HWVADTHGS register accessor: an alias for `Reg<HWVADTHGS_SPEC>`"]
pub type HWVADTHGS = crate::Reg<hwvadthgs::HWVADTHGS_SPEC>;
#[doc = "HWVAD signal estimator gain register"]
pub mod hwvadthgs;
#[doc = "HWVADLOWZ register accessor: an alias for `Reg<HWVADLOWZ_SPEC>`"]
pub type HWVADLOWZ = crate::Reg<hwvadlowz::HWVADLOWZ_SPEC>;
#[doc = "HWVAD noise envelope estimator register"]
pub mod hwvadlowz;
#[doc = "ID register accessor: an alias for `Reg<ID_SPEC>`"]
pub type ID = crate::Reg<id::ID_SPEC>;
#[doc = "Module Identification register"]
pub mod id;
