#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - EEPROM command register"]
    pub cmd: CMD,
    _reserved1: [u8; 4usize],
    #[doc = "0x08 - EEPROM read wait state register"]
    pub rwstate: RWSTATE,
    #[doc = "0x0c - EEPROM auto programming register"]
    pub autoprog: AUTOPROG,
    #[doc = "0x10 - EEPROM wait state register"]
    pub wstate: WSTATE,
    #[doc = "0x14 - EEPROM clock divider register"]
    pub clkdiv: CLKDIV,
    #[doc = "0x18 - EEPROM power-down register"]
    pub pwrdwn: PWRDWN,
    _reserved6: [u8; 4028usize],
    #[doc = "0xfd8 - EEPROM interrupt enable clear"]
    pub intenclr: INTENCLR,
    #[doc = "0xfdc - EEPROM interrupt enable set"]
    pub intenset: INTENSET,
    #[doc = "0xfe0 - EEPROM interrupt status"]
    pub intstat: INTSTAT,
    #[doc = "0xfe4 - EEPROM interrupt enable"]
    pub inten: INTEN,
    #[doc = "0xfe8 - EEPROM interrupt status clear"]
    pub intstatclr: INTSTATCLR,
    #[doc = "0xfec - EEPROM interrupt status set"]
    pub intstatset: INTSTATSET,
}
#[doc = "EEPROM command register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmd](cmd) module"]
pub type CMD = crate::Reg<u32, _CMD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CMD;
#[doc = "`read()` method returns [cmd::R](cmd::R) reader structure"]
impl crate::Readable for CMD {}
#[doc = "`write(|w| ..)` method takes [cmd::W](cmd::W) writer structure"]
impl crate::Writable for CMD {}
#[doc = "EEPROM command register"]
pub mod cmd;
#[doc = "EEPROM read wait state register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rwstate](rwstate) module"]
pub type RWSTATE = crate::Reg<u32, _RWSTATE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RWSTATE;
#[doc = "`read()` method returns [rwstate::R](rwstate::R) reader structure"]
impl crate::Readable for RWSTATE {}
#[doc = "`write(|w| ..)` method takes [rwstate::W](rwstate::W) writer structure"]
impl crate::Writable for RWSTATE {}
#[doc = "EEPROM read wait state register"]
pub mod rwstate;
#[doc = "EEPROM auto programming register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [autoprog](autoprog) module"]
pub type AUTOPROG = crate::Reg<u32, _AUTOPROG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AUTOPROG;
#[doc = "`read()` method returns [autoprog::R](autoprog::R) reader structure"]
impl crate::Readable for AUTOPROG {}
#[doc = "`write(|w| ..)` method takes [autoprog::W](autoprog::W) writer structure"]
impl crate::Writable for AUTOPROG {}
#[doc = "EEPROM auto programming register"]
pub mod autoprog;
#[doc = "EEPROM wait state register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wstate](wstate) module"]
pub type WSTATE = crate::Reg<u32, _WSTATE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WSTATE;
#[doc = "`read()` method returns [wstate::R](wstate::R) reader structure"]
impl crate::Readable for WSTATE {}
#[doc = "`write(|w| ..)` method takes [wstate::W](wstate::W) writer structure"]
impl crate::Writable for WSTATE {}
#[doc = "EEPROM wait state register"]
pub mod wstate;
#[doc = "EEPROM clock divider register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clkdiv](clkdiv) module"]
pub type CLKDIV = crate::Reg<u32, _CLKDIV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CLKDIV;
#[doc = "`read()` method returns [clkdiv::R](clkdiv::R) reader structure"]
impl crate::Readable for CLKDIV {}
#[doc = "`write(|w| ..)` method takes [clkdiv::W](clkdiv::W) writer structure"]
impl crate::Writable for CLKDIV {}
#[doc = "EEPROM clock divider register"]
pub mod clkdiv;
#[doc = "EEPROM power-down register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pwrdwn](pwrdwn) module"]
pub type PWRDWN = crate::Reg<u32, _PWRDWN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PWRDWN;
#[doc = "`read()` method returns [pwrdwn::R](pwrdwn::R) reader structure"]
impl crate::Readable for PWRDWN {}
#[doc = "`write(|w| ..)` method takes [pwrdwn::W](pwrdwn::W) writer structure"]
impl crate::Writable for PWRDWN {}
#[doc = "EEPROM power-down register"]
pub mod pwrdwn;
#[doc = "EEPROM interrupt enable clear\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intenclr](intenclr) module"]
pub type INTENCLR = crate::Reg<u32, _INTENCLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTENCLR;
#[doc = "`write(|w| ..)` method takes [intenclr::W](intenclr::W) writer structure"]
impl crate::Writable for INTENCLR {}
#[doc = "EEPROM interrupt enable clear"]
pub mod intenclr;
#[doc = "EEPROM interrupt enable set\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intenset](intenset) module"]
pub type INTENSET = crate::Reg<u32, _INTENSET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTENSET;
#[doc = "`write(|w| ..)` method takes [intenset::W](intenset::W) writer structure"]
impl crate::Writable for INTENSET {}
#[doc = "EEPROM interrupt enable set"]
pub mod intenset;
#[doc = "EEPROM interrupt status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intstat](intstat) module"]
pub type INTSTAT = crate::Reg<u32, _INTSTAT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTSTAT;
#[doc = "`read()` method returns [intstat::R](intstat::R) reader structure"]
impl crate::Readable for INTSTAT {}
#[doc = "EEPROM interrupt status"]
pub mod intstat;
#[doc = "EEPROM interrupt enable\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [inten](inten) module"]
pub type INTEN = crate::Reg<u32, _INTEN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTEN;
#[doc = "`read()` method returns [inten::R](inten::R) reader structure"]
impl crate::Readable for INTEN {}
#[doc = "EEPROM interrupt enable"]
pub mod inten;
#[doc = "EEPROM interrupt status clear\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intstatclr](intstatclr) module"]
pub type INTSTATCLR = crate::Reg<u32, _INTSTATCLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTSTATCLR;
#[doc = "`write(|w| ..)` method takes [intstatclr::W](intstatclr::W) writer structure"]
impl crate::Writable for INTSTATCLR {}
#[doc = "EEPROM interrupt status clear"]
pub mod intstatclr;
#[doc = "EEPROM interrupt status set\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intstatset](intstatset) module"]
pub type INTSTATSET = crate::Reg<u32, _INTSTATSET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTSTATSET;
#[doc = "`write(|w| ..)` method takes [intstatset::W](intstatset::W) writer structure"]
impl crate::Writable for INTSTATSET {}
#[doc = "EEPROM interrupt status set"]
pub mod intstatset;
