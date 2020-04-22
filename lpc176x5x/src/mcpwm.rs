#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - PWM Control read address"]
    pub con: CON,
    #[doc = "0x04 - PWM Control set address"]
    pub con_set: CON_SET,
    #[doc = "0x08 - PWM Control clear address"]
    pub con_clr: CON_CLR,
    #[doc = "0x0c - Capture Control read address"]
    pub capcon: CAPCON,
    #[doc = "0x10 - Capture Control set address"]
    pub capcon_set: CAPCON_SET,
    #[doc = "0x14 - Event Control clear address"]
    pub capcon_clr: CAPCON_CLR,
    #[doc = "0x18 - Timer Counter register"]
    pub tc: [TC; 3],
    #[doc = "0x24 - Limit register"]
    pub lim: [LIM; 3],
    #[doc = "0x30 - Match register"]
    pub mat: [MAT; 3],
    #[doc = "0x3c - Dead time register"]
    pub dt: DT,
    #[doc = "0x40 - Communication Pattern register"]
    pub cp: CP,
    #[doc = "0x44 - Capture register"]
    pub cap: [CAP; 3],
    #[doc = "0x50 - Interrupt Enable read address"]
    pub inten: INTEN,
    #[doc = "0x54 - Interrupt Enable set address"]
    pub inten_set: INTEN_SET,
    #[doc = "0x58 - Interrupt Enable clear address"]
    pub inten_clr: INTEN_CLR,
    #[doc = "0x5c - Count Control read address"]
    pub cntcon: CNTCON,
    #[doc = "0x60 - Count Control set address"]
    pub cntcon_set: CNTCON_SET,
    #[doc = "0x64 - Count Control clear address"]
    pub cntcon_clr: CNTCON_CLR,
    #[doc = "0x68 - Interrupt flags read address"]
    pub intf: INTF,
    #[doc = "0x6c - Interrupt flags set address"]
    pub intf_set: INTF_SET,
    #[doc = "0x70 - Interrupt flags clear address"]
    pub intf_clr: INTF_CLR,
    #[doc = "0x74 - Capture clear address"]
    pub cap_clr: CAP_CLR,
}
#[doc = "PWM Control read address\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [con](con) module"]
pub type CON = crate::Reg<u32, _CON>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CON;
#[doc = "`read()` method returns [con::R](con::R) reader structure"]
impl crate::Readable for CON {}
#[doc = "PWM Control read address"]
pub mod con;
#[doc = "PWM Control set address\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [con_set](con_set) module"]
pub type CON_SET = crate::Reg<u32, _CON_SET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CON_SET;
#[doc = "`write(|w| ..)` method takes [con_set::W](con_set::W) writer structure"]
impl crate::Writable for CON_SET {}
#[doc = "PWM Control set address"]
pub mod con_set;
#[doc = "PWM Control clear address\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [con_clr](con_clr) module"]
pub type CON_CLR = crate::Reg<u32, _CON_CLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CON_CLR;
#[doc = "`write(|w| ..)` method takes [con_clr::W](con_clr::W) writer structure"]
impl crate::Writable for CON_CLR {}
#[doc = "PWM Control clear address"]
pub mod con_clr;
#[doc = "Capture Control read address\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [capcon](capcon) module"]
pub type CAPCON = crate::Reg<u32, _CAPCON>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CAPCON;
#[doc = "`read()` method returns [capcon::R](capcon::R) reader structure"]
impl crate::Readable for CAPCON {}
#[doc = "Capture Control read address"]
pub mod capcon;
#[doc = "Capture Control set address\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [capcon_set](capcon_set) module"]
pub type CAPCON_SET = crate::Reg<u32, _CAPCON_SET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CAPCON_SET;
#[doc = "`write(|w| ..)` method takes [capcon_set::W](capcon_set::W) writer structure"]
impl crate::Writable for CAPCON_SET {}
#[doc = "Capture Control set address"]
pub mod capcon_set;
#[doc = "Event Control clear address\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [capcon_clr](capcon_clr) module"]
pub type CAPCON_CLR = crate::Reg<u32, _CAPCON_CLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CAPCON_CLR;
#[doc = "`write(|w| ..)` method takes [capcon_clr::W](capcon_clr::W) writer structure"]
impl crate::Writable for CAPCON_CLR {}
#[doc = "Event Control clear address"]
pub mod capcon_clr;
#[doc = "Timer Counter register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tc](tc) module"]
pub type TC = crate::Reg<u32, _TC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TC;
#[doc = "`read()` method returns [tc::R](tc::R) reader structure"]
impl crate::Readable for TC {}
#[doc = "`write(|w| ..)` method takes [tc::W](tc::W) writer structure"]
impl crate::Writable for TC {}
#[doc = "Timer Counter register"]
pub mod tc;
#[doc = "Limit register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lim](lim) module"]
pub type LIM = crate::Reg<u32, _LIM>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LIM;
#[doc = "`read()` method returns [lim::R](lim::R) reader structure"]
impl crate::Readable for LIM {}
#[doc = "`write(|w| ..)` method takes [lim::W](lim::W) writer structure"]
impl crate::Writable for LIM {}
#[doc = "Limit register"]
pub mod lim;
#[doc = "Match register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mat](mat) module"]
pub type MAT = crate::Reg<u32, _MAT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MAT;
#[doc = "`read()` method returns [mat::R](mat::R) reader structure"]
impl crate::Readable for MAT {}
#[doc = "`write(|w| ..)` method takes [mat::W](mat::W) writer structure"]
impl crate::Writable for MAT {}
#[doc = "Match register"]
pub mod mat;
#[doc = "Dead time register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dt](dt) module"]
pub type DT = crate::Reg<u32, _DT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DT;
#[doc = "`read()` method returns [dt::R](dt::R) reader structure"]
impl crate::Readable for DT {}
#[doc = "`write(|w| ..)` method takes [dt::W](dt::W) writer structure"]
impl crate::Writable for DT {}
#[doc = "Dead time register"]
pub mod dt;
#[doc = "Communication Pattern register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cp](cp) module"]
pub type CP = crate::Reg<u32, _CP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CP;
#[doc = "`read()` method returns [cp::R](cp::R) reader structure"]
impl crate::Readable for CP {}
#[doc = "`write(|w| ..)` method takes [cp::W](cp::W) writer structure"]
impl crate::Writable for CP {}
#[doc = "Communication Pattern register"]
pub mod cp;
#[doc = "Capture register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cap](cap) module"]
pub type CAP = crate::Reg<u32, _CAP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CAP;
#[doc = "`read()` method returns [cap::R](cap::R) reader structure"]
impl crate::Readable for CAP {}
#[doc = "Capture register"]
pub mod cap;
#[doc = "Interrupt Enable read address\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [inten](inten) module"]
pub type INTEN = crate::Reg<u32, _INTEN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTEN;
#[doc = "`read()` method returns [inten::R](inten::R) reader structure"]
impl crate::Readable for INTEN {}
#[doc = "Interrupt Enable read address"]
pub mod inten;
#[doc = "Interrupt Enable set address\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [inten_set](inten_set) module"]
pub type INTEN_SET = crate::Reg<u32, _INTEN_SET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTEN_SET;
#[doc = "`write(|w| ..)` method takes [inten_set::W](inten_set::W) writer structure"]
impl crate::Writable for INTEN_SET {}
#[doc = "Interrupt Enable set address"]
pub mod inten_set;
#[doc = "Interrupt Enable clear address\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [inten_clr](inten_clr) module"]
pub type INTEN_CLR = crate::Reg<u32, _INTEN_CLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTEN_CLR;
#[doc = "`write(|w| ..)` method takes [inten_clr::W](inten_clr::W) writer structure"]
impl crate::Writable for INTEN_CLR {}
#[doc = "Interrupt Enable clear address"]
pub mod inten_clr;
#[doc = "Interrupt flags read address\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intf](intf) module"]
pub type INTF = crate::Reg<u32, _INTF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTF;
#[doc = "`read()` method returns [intf::R](intf::R) reader structure"]
impl crate::Readable for INTF {}
#[doc = "Interrupt flags read address"]
pub mod intf;
#[doc = "Interrupt flags set address\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intf_set](intf_set) module"]
pub type INTF_SET = crate::Reg<u32, _INTF_SET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTF_SET;
#[doc = "`write(|w| ..)` method takes [intf_set::W](intf_set::W) writer structure"]
impl crate::Writable for INTF_SET {}
#[doc = "Interrupt flags set address"]
pub mod intf_set;
#[doc = "Interrupt flags clear address\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intf_clr](intf_clr) module"]
pub type INTF_CLR = crate::Reg<u32, _INTF_CLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTF_CLR;
#[doc = "`write(|w| ..)` method takes [intf_clr::W](intf_clr::W) writer structure"]
impl crate::Writable for INTF_CLR {}
#[doc = "Interrupt flags clear address"]
pub mod intf_clr;
#[doc = "Count Control read address\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cntcon](cntcon) module"]
pub type CNTCON = crate::Reg<u32, _CNTCON>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CNTCON;
#[doc = "`read()` method returns [cntcon::R](cntcon::R) reader structure"]
impl crate::Readable for CNTCON {}
#[doc = "Count Control read address"]
pub mod cntcon;
#[doc = "Count Control set address\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cntcon_set](cntcon_set) module"]
pub type CNTCON_SET = crate::Reg<u32, _CNTCON_SET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CNTCON_SET;
#[doc = "`write(|w| ..)` method takes [cntcon_set::W](cntcon_set::W) writer structure"]
impl crate::Writable for CNTCON_SET {}
#[doc = "Count Control set address"]
pub mod cntcon_set;
#[doc = "Count Control clear address\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cntcon_clr](cntcon_clr) module"]
pub type CNTCON_CLR = crate::Reg<u32, _CNTCON_CLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CNTCON_CLR;
#[doc = "`write(|w| ..)` method takes [cntcon_clr::W](cntcon_clr::W) writer structure"]
impl crate::Writable for CNTCON_CLR {}
#[doc = "Count Control clear address"]
pub mod cntcon_clr;
#[doc = "Capture clear address\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cap_clr](cap_clr) module"]
pub type CAP_CLR = crate::Reg<u32, _CAP_CLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CAP_CLR;
#[doc = "`write(|w| ..)` method takes [cap_clr::W](cap_clr::W) writer structure"]
impl crate::Writable for CAP_CLR {}
#[doc = "Capture clear address"]
pub mod cap_clr;
