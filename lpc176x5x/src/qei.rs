#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control register"]
    pub con: CON,
    #[doc = "0x04 - Status register"]
    pub stat: STAT,
    #[doc = "0x08 - Configuration register"]
    pub conf: CONF,
    #[doc = "0x0c - Position register"]
    pub pos: POS,
    #[doc = "0x10 - Maximum position register"]
    pub maxpos: MAXPOS,
    #[doc = "0x14 - Position compare register 0"]
    pub cmpos0: CMPOS0,
    #[doc = "0x18 - Position compare register 1"]
    pub cmpos1: CMPOS1,
    #[doc = "0x1c - Position compare register 2"]
    pub cmpos2: CMPOS2,
    #[doc = "0x20 - Index count register 0"]
    pub inxcnt: INXCNT,
    #[doc = "0x24 - Index compare register 0"]
    pub inxcmp0: INXCMP0,
    #[doc = "0x28 - Velocity timer reload register"]
    pub load: LOAD,
    #[doc = "0x2c - Velocity timer register"]
    pub time: TIME,
    #[doc = "0x30 - Velocity counter register"]
    pub vel: VEL,
    #[doc = "0x34 - Velocity capture register"]
    pub cap: CAP,
    #[doc = "0x38 - Velocity compare register"]
    pub velcomp: VELCOMP,
    #[doc = "0x3c - Digital filter register"]
    pub filter: FILTER,
    _reserved16: [u8; 3992usize],
    #[doc = "0xfd8 - Interrupt enable clear register"]
    pub iec: IEC,
    #[doc = "0xfdc - Interrupt enable set register"]
    pub ies: IES,
    #[doc = "0xfe0 - Interrupt status register"]
    pub intstat: INTSTAT,
    #[doc = "0xfe4 - Interrupt enable register"]
    pub ie: IE,
    #[doc = "0xfe8 - Interrupt status clear register"]
    pub clr: CLR,
    #[doc = "0xfec - Interrupt status set register"]
    pub set: SET,
}
#[doc = "Control register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [con](con) module"]
pub type CON = crate::Reg<u32, _CON>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CON;
#[doc = "`write(|w| ..)` method takes [con::W](con::W) writer structure"]
impl crate::Writable for CON {}
#[doc = "Control register"]
pub mod con;
#[doc = "Configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [conf](conf) module"]
pub type CONF = crate::Reg<u32, _CONF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CONF;
#[doc = "`read()` method returns [conf::R](conf::R) reader structure"]
impl crate::Readable for CONF {}
#[doc = "`write(|w| ..)` method takes [conf::W](conf::W) writer structure"]
impl crate::Writable for CONF {}
#[doc = "Configuration register"]
pub mod conf;
#[doc = "Status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [stat](stat) module"]
pub type STAT = crate::Reg<u32, _STAT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _STAT;
#[doc = "`read()` method returns [stat::R](stat::R) reader structure"]
impl crate::Readable for STAT {}
#[doc = "Status register"]
pub mod stat;
#[doc = "Position register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pos](pos) module"]
pub type POS = crate::Reg<u32, _POS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _POS;
#[doc = "`read()` method returns [pos::R](pos::R) reader structure"]
impl crate::Readable for POS {}
#[doc = "Position register"]
pub mod pos;
#[doc = "Maximum position register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [maxpos](maxpos) module"]
pub type MAXPOS = crate::Reg<u32, _MAXPOS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MAXPOS;
#[doc = "`read()` method returns [maxpos::R](maxpos::R) reader structure"]
impl crate::Readable for MAXPOS {}
#[doc = "`write(|w| ..)` method takes [maxpos::W](maxpos::W) writer structure"]
impl crate::Writable for MAXPOS {}
#[doc = "Maximum position register"]
pub mod maxpos;
#[doc = "Position compare register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmpos0](cmpos0) module"]
pub type CMPOS0 = crate::Reg<u32, _CMPOS0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CMPOS0;
#[doc = "`read()` method returns [cmpos0::R](cmpos0::R) reader structure"]
impl crate::Readable for CMPOS0 {}
#[doc = "`write(|w| ..)` method takes [cmpos0::W](cmpos0::W) writer structure"]
impl crate::Writable for CMPOS0 {}
#[doc = "Position compare register 0"]
pub mod cmpos0;
#[doc = "Position compare register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmpos1](cmpos1) module"]
pub type CMPOS1 = crate::Reg<u32, _CMPOS1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CMPOS1;
#[doc = "`read()` method returns [cmpos1::R](cmpos1::R) reader structure"]
impl crate::Readable for CMPOS1 {}
#[doc = "`write(|w| ..)` method takes [cmpos1::W](cmpos1::W) writer structure"]
impl crate::Writable for CMPOS1 {}
#[doc = "Position compare register 1"]
pub mod cmpos1;
#[doc = "Position compare register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmpos2](cmpos2) module"]
pub type CMPOS2 = crate::Reg<u32, _CMPOS2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CMPOS2;
#[doc = "`read()` method returns [cmpos2::R](cmpos2::R) reader structure"]
impl crate::Readable for CMPOS2 {}
#[doc = "`write(|w| ..)` method takes [cmpos2::W](cmpos2::W) writer structure"]
impl crate::Writable for CMPOS2 {}
#[doc = "Position compare register 2"]
pub mod cmpos2;
#[doc = "Index count register 0\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [inxcnt](inxcnt) module"]
pub type INXCNT = crate::Reg<u32, _INXCNT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INXCNT;
#[doc = "`read()` method returns [inxcnt::R](inxcnt::R) reader structure"]
impl crate::Readable for INXCNT {}
#[doc = "Index count register 0"]
pub mod inxcnt;
#[doc = "Index compare register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [inxcmp0](inxcmp0) module"]
pub type INXCMP0 = crate::Reg<u32, _INXCMP0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INXCMP0;
#[doc = "`read()` method returns [inxcmp0::R](inxcmp0::R) reader structure"]
impl crate::Readable for INXCMP0 {}
#[doc = "`write(|w| ..)` method takes [inxcmp0::W](inxcmp0::W) writer structure"]
impl crate::Writable for INXCMP0 {}
#[doc = "Index compare register 0"]
pub mod inxcmp0;
#[doc = "Velocity timer reload register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [load](load) module"]
pub type LOAD = crate::Reg<u32, _LOAD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LOAD;
#[doc = "`read()` method returns [load::R](load::R) reader structure"]
impl crate::Readable for LOAD {}
#[doc = "`write(|w| ..)` method takes [load::W](load::W) writer structure"]
impl crate::Writable for LOAD {}
#[doc = "Velocity timer reload register"]
pub mod load;
#[doc = "Velocity timer register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [time](time) module"]
pub type TIME = crate::Reg<u32, _TIME>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIME;
#[doc = "`read()` method returns [time::R](time::R) reader structure"]
impl crate::Readable for TIME {}
#[doc = "Velocity timer register"]
pub mod time;
#[doc = "Velocity counter register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [vel](vel) module"]
pub type VEL = crate::Reg<u32, _VEL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _VEL;
#[doc = "`read()` method returns [vel::R](vel::R) reader structure"]
impl crate::Readable for VEL {}
#[doc = "Velocity counter register"]
pub mod vel;
#[doc = "Velocity capture register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cap](cap) module"]
pub type CAP = crate::Reg<u32, _CAP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CAP;
#[doc = "`read()` method returns [cap::R](cap::R) reader structure"]
impl crate::Readable for CAP {}
#[doc = "Velocity capture register"]
pub mod cap;
#[doc = "Velocity compare register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [velcomp](velcomp) module"]
pub type VELCOMP = crate::Reg<u32, _VELCOMP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _VELCOMP;
#[doc = "`read()` method returns [velcomp::R](velcomp::R) reader structure"]
impl crate::Readable for VELCOMP {}
#[doc = "`write(|w| ..)` method takes [velcomp::W](velcomp::W) writer structure"]
impl crate::Writable for VELCOMP {}
#[doc = "Velocity compare register"]
pub mod velcomp;
#[doc = "Digital filter register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [filter](filter) module"]
pub type FILTER = crate::Reg<u32, _FILTER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FILTER;
#[doc = "`read()` method returns [filter::R](filter::R) reader structure"]
impl crate::Readable for FILTER {}
#[doc = "`write(|w| ..)` method takes [filter::W](filter::W) writer structure"]
impl crate::Writable for FILTER {}
#[doc = "Digital filter register"]
pub mod filter;
#[doc = "Interrupt status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intstat](intstat) module"]
pub type INTSTAT = crate::Reg<u32, _INTSTAT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTSTAT;
#[doc = "`read()` method returns [intstat::R](intstat::R) reader structure"]
impl crate::Readable for INTSTAT {}
#[doc = "Interrupt status register"]
pub mod intstat;
#[doc = "Interrupt status set register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [set](set) module"]
pub type SET = crate::Reg<u32, _SET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SET;
#[doc = "`write(|w| ..)` method takes [set::W](set::W) writer structure"]
impl crate::Writable for SET {}
#[doc = "Interrupt status set register"]
pub mod set;
#[doc = "Interrupt status clear register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clr](clr) module"]
pub type CLR = crate::Reg<u32, _CLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CLR;
#[doc = "`write(|w| ..)` method takes [clr::W](clr::W) writer structure"]
impl crate::Writable for CLR {}
#[doc = "Interrupt status clear register"]
pub mod clr;
#[doc = "Interrupt enable register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ie](ie) module"]
pub type IE = crate::Reg<u32, _IE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IE;
#[doc = "`read()` method returns [ie::R](ie::R) reader structure"]
impl crate::Readable for IE {}
#[doc = "Interrupt enable register"]
pub mod ie;
#[doc = "Interrupt enable set register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ies](ies) module"]
pub type IES = crate::Reg<u32, _IES>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IES;
#[doc = "`write(|w| ..)` method takes [ies::W](ies::W) writer structure"]
impl crate::Writable for IES {}
#[doc = "Interrupt enable set register"]
pub mod ies;
#[doc = "Interrupt enable clear register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [iec](iec) module"]
pub type IEC = crate::Reg<u32, _IEC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IEC;
#[doc = "`write(|w| ..)` method takes [iec::W](iec::W) writer structure"]
impl crate::Writable for IEC {}
#[doc = "Interrupt enable clear register"]
pub mod iec;
