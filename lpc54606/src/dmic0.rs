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
    pub chanen: CHANEN,
    _reserved3: [u8; 8usize],
    #[doc = "0xf0c - I/O Configuration register"]
    pub iocfg: IOCFG,
    #[doc = "0xf10 - Use 2FS register"]
    pub use2fs: USE2FS,
    _reserved5: [u8; 108usize],
    #[doc = "0xf80 - HWVAD input gain register"]
    pub hwvadgain: HWVADGAIN,
    #[doc = "0xf84 - HWVAD filter control register"]
    pub hwvadhpfs: HWVADHPFS,
    #[doc = "0xf88 - HWVAD control register"]
    pub hwvadst10: HWVADST10,
    #[doc = "0xf8c - HWVAD filter reset register"]
    pub hwvadrstt: HWVADRSTT,
    #[doc = "0xf90 - HWVAD noise estimator gain register"]
    pub hwvadthgn: HWVADTHGN,
    #[doc = "0xf94 - HWVAD signal estimator gain register"]
    pub hwvadthgs: HWVADTHGS,
    #[doc = "0xf98 - HWVAD noise envelope estimator register"]
    pub hwvadlowz: HWVADLOWZ,
    _reserved12: [u8; 96usize],
    #[doc = "0xffc - Module Identification register"]
    pub id: ID,
}
#[doc = r"Register block"]
#[repr(C)]
pub struct CHANNEL {
    #[doc = "0x00 - Oversample Rate register 0"]
    pub osr: self::channel::OSR,
    #[doc = "0x04 - DMIC Clock Register 0"]
    pub divhfclk: self::channel::DIVHFCLK,
    #[doc = "0x08 - Pre-Emphasis Filter Coefficient for 2 FS register"]
    pub preac2fscoef: self::channel::PREAC2FSCOEF,
    #[doc = "0x0c - Pre-Emphasis Filter Coefficient for 4 FS register"]
    pub preac4fscoef: self::channel::PREAC4FSCOEF,
    #[doc = "0x10 - Decimator Gain Shift register"]
    pub gainshift: self::channel::GAINSHIFT,
    _reserved5: [u8; 108usize],
    #[doc = "0x80 - FIFO Control register 0"]
    pub fifo_ctrl: self::channel::FIFO_CTRL,
    #[doc = "0x84 - FIFO Status register 0"]
    pub fifo_status: self::channel::FIFO_STATUS,
    #[doc = "0x88 - FIFO Data Register 0"]
    pub fifo_data: self::channel::FIFO_DATA,
    #[doc = "0x8c - PDM Source Configuration register 0"]
    pub phy_ctrl: self::channel::PHY_CTRL,
    #[doc = "0x90 - DC Control register 0"]
    pub dc_ctrl: self::channel::DC_CTRL,
}
#[doc = r"Register block"]
#[doc = "no description available"]
pub mod channel;
#[doc = "Channel Enable register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [chanen](chanen) module"]
pub type CHANEN = crate::Reg<u32, _CHANEN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CHANEN;
#[doc = "`read()` method returns [chanen::R](chanen::R) reader structure"]
impl crate::Readable for CHANEN {}
#[doc = "`write(|w| ..)` method takes [chanen::W](chanen::W) writer structure"]
impl crate::Writable for CHANEN {}
#[doc = "Channel Enable register"]
pub mod chanen;
#[doc = "I/O Configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [iocfg](iocfg) module"]
pub type IOCFG = crate::Reg<u32, _IOCFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IOCFG;
#[doc = "`read()` method returns [iocfg::R](iocfg::R) reader structure"]
impl crate::Readable for IOCFG {}
#[doc = "`write(|w| ..)` method takes [iocfg::W](iocfg::W) writer structure"]
impl crate::Writable for IOCFG {}
#[doc = "I/O Configuration register"]
pub mod iocfg;
#[doc = "Use 2FS register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [use2fs](use2fs) module"]
pub type USE2FS = crate::Reg<u32, _USE2FS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USE2FS;
#[doc = "`read()` method returns [use2fs::R](use2fs::R) reader structure"]
impl crate::Readable for USE2FS {}
#[doc = "`write(|w| ..)` method takes [use2fs::W](use2fs::W) writer structure"]
impl crate::Writable for USE2FS {}
#[doc = "Use 2FS register"]
pub mod use2fs;
#[doc = "HWVAD input gain register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hwvadgain](hwvadgain) module"]
pub type HWVADGAIN = crate::Reg<u32, _HWVADGAIN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HWVADGAIN;
#[doc = "`read()` method returns [hwvadgain::R](hwvadgain::R) reader structure"]
impl crate::Readable for HWVADGAIN {}
#[doc = "`write(|w| ..)` method takes [hwvadgain::W](hwvadgain::W) writer structure"]
impl crate::Writable for HWVADGAIN {}
#[doc = "HWVAD input gain register"]
pub mod hwvadgain;
#[doc = "HWVAD filter control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hwvadhpfs](hwvadhpfs) module"]
pub type HWVADHPFS = crate::Reg<u32, _HWVADHPFS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HWVADHPFS;
#[doc = "`read()` method returns [hwvadhpfs::R](hwvadhpfs::R) reader structure"]
impl crate::Readable for HWVADHPFS {}
#[doc = "`write(|w| ..)` method takes [hwvadhpfs::W](hwvadhpfs::W) writer structure"]
impl crate::Writable for HWVADHPFS {}
#[doc = "HWVAD filter control register"]
pub mod hwvadhpfs;
#[doc = "HWVAD control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hwvadst10](hwvadst10) module"]
pub type HWVADST10 = crate::Reg<u32, _HWVADST10>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HWVADST10;
#[doc = "`read()` method returns [hwvadst10::R](hwvadst10::R) reader structure"]
impl crate::Readable for HWVADST10 {}
#[doc = "`write(|w| ..)` method takes [hwvadst10::W](hwvadst10::W) writer structure"]
impl crate::Writable for HWVADST10 {}
#[doc = "HWVAD control register"]
pub mod hwvadst10;
#[doc = "HWVAD filter reset register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hwvadrstt](hwvadrstt) module"]
pub type HWVADRSTT = crate::Reg<u32, _HWVADRSTT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HWVADRSTT;
#[doc = "`read()` method returns [hwvadrstt::R](hwvadrstt::R) reader structure"]
impl crate::Readable for HWVADRSTT {}
#[doc = "`write(|w| ..)` method takes [hwvadrstt::W](hwvadrstt::W) writer structure"]
impl crate::Writable for HWVADRSTT {}
#[doc = "HWVAD filter reset register"]
pub mod hwvadrstt;
#[doc = "HWVAD noise estimator gain register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hwvadthgn](hwvadthgn) module"]
pub type HWVADTHGN = crate::Reg<u32, _HWVADTHGN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HWVADTHGN;
#[doc = "`read()` method returns [hwvadthgn::R](hwvadthgn::R) reader structure"]
impl crate::Readable for HWVADTHGN {}
#[doc = "`write(|w| ..)` method takes [hwvadthgn::W](hwvadthgn::W) writer structure"]
impl crate::Writable for HWVADTHGN {}
#[doc = "HWVAD noise estimator gain register"]
pub mod hwvadthgn;
#[doc = "HWVAD signal estimator gain register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hwvadthgs](hwvadthgs) module"]
pub type HWVADTHGS = crate::Reg<u32, _HWVADTHGS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HWVADTHGS;
#[doc = "`read()` method returns [hwvadthgs::R](hwvadthgs::R) reader structure"]
impl crate::Readable for HWVADTHGS {}
#[doc = "`write(|w| ..)` method takes [hwvadthgs::W](hwvadthgs::W) writer structure"]
impl crate::Writable for HWVADTHGS {}
#[doc = "HWVAD signal estimator gain register"]
pub mod hwvadthgs;
#[doc = "HWVAD noise envelope estimator register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hwvadlowz](hwvadlowz) module"]
pub type HWVADLOWZ = crate::Reg<u32, _HWVADLOWZ>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HWVADLOWZ;
#[doc = "`read()` method returns [hwvadlowz::R](hwvadlowz::R) reader structure"]
impl crate::Readable for HWVADLOWZ {}
#[doc = "HWVAD noise envelope estimator register"]
pub mod hwvadlowz;
#[doc = "Module Identification register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [id](id) module"]
pub type ID = crate::Reg<u32, _ID>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ID;
#[doc = "`read()` method returns [id::R](id::R) reader structure"]
impl crate::Readable for ID {}
#[doc = "Module Identification register"]
pub mod id;
