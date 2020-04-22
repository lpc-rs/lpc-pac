#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Controls operation of the memory controller"]
    pub control: CONTROL,
    #[doc = "0x04 - Provides EMC status information"]
    pub status: STATUS,
    #[doc = "0x08 - Configures operation of the memory controller"]
    pub config: CONFIG,
    _reserved3: [u8; 20usize],
    #[doc = "0x20 - Controls dynamic memory operation"]
    pub dynamiccontrol: DYNAMICCONTROL,
    #[doc = "0x24 - Configures dynamic memory refresh"]
    pub dynamicrefresh: DYNAMICREFRESH,
    #[doc = "0x28 - Configures dynamic memory read strategy"]
    pub dynamicreadconfig: DYNAMICREADCONFIG,
    _reserved6: [u8; 4usize],
    #[doc = "0x30 - Precharge command period"]
    pub dynamicrp: DYNAMICRP,
    #[doc = "0x34 - Active to precharge command period"]
    pub dynamicras: DYNAMICRAS,
    #[doc = "0x38 - Self-refresh exit time"]
    pub dynamicsrex: DYNAMICSREX,
    #[doc = "0x3c - Last-data-out to active command time"]
    pub dynamicapr: DYNAMICAPR,
    #[doc = "0x40 - Data-in to active command time"]
    pub dynamicdal: DYNAMICDAL,
    #[doc = "0x44 - Write recovery time"]
    pub dynamicwr: DYNAMICWR,
    #[doc = "0x48 - Selects the active to active command period"]
    pub dynamicrc: DYNAMICRC,
    #[doc = "0x4c - Selects the auto-refresh period"]
    pub dynamicrfc: DYNAMICRFC,
    #[doc = "0x50 - Time for exit self-refresh to active command"]
    pub dynamicxsr: DYNAMICXSR,
    #[doc = "0x54 - Latency for active bank A to active bank B"]
    pub dynamicrrd: DYNAMICRRD,
    #[doc = "0x58 - Time for load mode register to active command"]
    pub dynamicmrd: DYNAMICMRD,
    _reserved17: [u8; 36usize],
    #[doc = "0x80 - Time for long static memory read and write transfers"]
    pub staticextendedwait: STATICEXTENDEDWAIT,
    _reserved18: [u8; 124usize],
    #[doc = "0x100 - no description available"]
    pub dynamic0: DYNAMIC,
    _reserved19: [u8; 24usize],
    #[doc = "0x120 - no description available"]
    pub dynamic1: DYNAMIC,
    _reserved20: [u8; 24usize],
    #[doc = "0x140 - no description available"]
    pub dynamic2: DYNAMIC,
    _reserved21: [u8; 24usize],
    #[doc = "0x160 - no description available"]
    pub dynamic3: DYNAMIC,
    _reserved22: [u8; 152usize],
    #[doc = "0x200 - no description available"]
    pub static0: STATIC,
    _reserved23: [u8; 4usize],
    #[doc = "0x220 - no description available"]
    pub static1: STATIC,
    _reserved24: [u8; 4usize],
    #[doc = "0x240 - no description available"]
    pub static2: STATIC,
    _reserved25: [u8; 4usize],
    #[doc = "0x260 - no description available"]
    pub static3: STATIC,
}
#[doc = r"Register block"]
#[repr(C)]
pub struct DYNAMIC {
    #[doc = "0x00 - Configuration information for EMC_DYCSx"]
    pub dynamicconfig: self::dynamic::DYNAMICCONFIG,
    #[doc = "0x04 - RAS and CAS latencies for EMC_DYCSx"]
    pub dynamicrascas: self::dynamic::DYNAMICRASCAS,
}
#[doc = r"Register block"]
#[doc = "no description available"]
pub mod dynamic;
#[doc = r"Register block"]
#[repr(C)]
pub struct STATIC {
    #[doc = "0x00 - Configuration for EMC_CSx"]
    pub staticconfig: self::static_::STATICCONFIG,
    #[doc = "0x04 - Delay from EMC_CSx to write enable"]
    pub staticwaitwen: self::static_::STATICWAITWEN,
    #[doc = "0x08 - Delay from EMC_CSx or address change, whichever is later, to output enable"]
    pub staticwaitoen: self::static_::STATICWAITOEN,
    #[doc = "0x0c - Delay from EMC_CSx to a read access"]
    pub staticwaitrd: self::static_::STATICWAITRD,
    #[doc = "0x10 - Delay for asynchronous page mode sequential accesses for EMC_CSx"]
    pub staticwaitpage: self::static_::STATICWAITPAGE,
    #[doc = "0x14 - Delay from EMC_CSx to a write access"]
    pub staticwaitwr: self::static_::STATICWAITWR,
    #[doc = "0x18 - Number of bus turnaround cycles EMC_CSx"]
    pub staticwaitturn: self::static_::STATICWAITTURN,
}
#[doc = r"Register block"]
#[doc = "no description available"]
pub mod static_;
#[doc = "Controls operation of the memory controller\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [control](control) module"]
pub type CONTROL = crate::Reg<u32, _CONTROL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CONTROL;
#[doc = "`read()` method returns [control::R](control::R) reader structure"]
impl crate::Readable for CONTROL {}
#[doc = "`write(|w| ..)` method takes [control::W](control::W) writer structure"]
impl crate::Writable for CONTROL {}
#[doc = "Controls operation of the memory controller"]
pub mod control;
#[doc = "Provides EMC status information\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [status](status) module"]
pub type STATUS = crate::Reg<u32, _STATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _STATUS;
#[doc = "`read()` method returns [status::R](status::R) reader structure"]
impl crate::Readable for STATUS {}
#[doc = "Provides EMC status information"]
pub mod status;
#[doc = "Configures operation of the memory controller\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [config](config) module"]
pub type CONFIG = crate::Reg<u32, _CONFIG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CONFIG;
#[doc = "`read()` method returns [config::R](config::R) reader structure"]
impl crate::Readable for CONFIG {}
#[doc = "`write(|w| ..)` method takes [config::W](config::W) writer structure"]
impl crate::Writable for CONFIG {}
#[doc = "Configures operation of the memory controller"]
pub mod config;
#[doc = "Controls dynamic memory operation\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dynamiccontrol](dynamiccontrol) module"]
pub type DYNAMICCONTROL = crate::Reg<u32, _DYNAMICCONTROL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DYNAMICCONTROL;
#[doc = "`read()` method returns [dynamiccontrol::R](dynamiccontrol::R) reader structure"]
impl crate::Readable for DYNAMICCONTROL {}
#[doc = "`write(|w| ..)` method takes [dynamiccontrol::W](dynamiccontrol::W) writer structure"]
impl crate::Writable for DYNAMICCONTROL {}
#[doc = "Controls dynamic memory operation"]
pub mod dynamiccontrol;
#[doc = "Configures dynamic memory refresh\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dynamicrefresh](dynamicrefresh) module"]
pub type DYNAMICREFRESH = crate::Reg<u32, _DYNAMICREFRESH>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DYNAMICREFRESH;
#[doc = "`read()` method returns [dynamicrefresh::R](dynamicrefresh::R) reader structure"]
impl crate::Readable for DYNAMICREFRESH {}
#[doc = "`write(|w| ..)` method takes [dynamicrefresh::W](dynamicrefresh::W) writer structure"]
impl crate::Writable for DYNAMICREFRESH {}
#[doc = "Configures dynamic memory refresh"]
pub mod dynamicrefresh;
#[doc = "Configures dynamic memory read strategy\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dynamicreadconfig](dynamicreadconfig) module"]
pub type DYNAMICREADCONFIG = crate::Reg<u32, _DYNAMICREADCONFIG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DYNAMICREADCONFIG;
#[doc = "`read()` method returns [dynamicreadconfig::R](dynamicreadconfig::R) reader structure"]
impl crate::Readable for DYNAMICREADCONFIG {}
#[doc = "`write(|w| ..)` method takes [dynamicreadconfig::W](dynamicreadconfig::W) writer structure"]
impl crate::Writable for DYNAMICREADCONFIG {}
#[doc = "Configures dynamic memory read strategy"]
pub mod dynamicreadconfig;
#[doc = "Precharge command period\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dynamicrp](dynamicrp) module"]
pub type DYNAMICRP = crate::Reg<u32, _DYNAMICRP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DYNAMICRP;
#[doc = "`read()` method returns [dynamicrp::R](dynamicrp::R) reader structure"]
impl crate::Readable for DYNAMICRP {}
#[doc = "`write(|w| ..)` method takes [dynamicrp::W](dynamicrp::W) writer structure"]
impl crate::Writable for DYNAMICRP {}
#[doc = "Precharge command period"]
pub mod dynamicrp;
#[doc = "Active to precharge command period\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dynamicras](dynamicras) module"]
pub type DYNAMICRAS = crate::Reg<u32, _DYNAMICRAS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DYNAMICRAS;
#[doc = "`read()` method returns [dynamicras::R](dynamicras::R) reader structure"]
impl crate::Readable for DYNAMICRAS {}
#[doc = "`write(|w| ..)` method takes [dynamicras::W](dynamicras::W) writer structure"]
impl crate::Writable for DYNAMICRAS {}
#[doc = "Active to precharge command period"]
pub mod dynamicras;
#[doc = "Self-refresh exit time\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dynamicsrex](dynamicsrex) module"]
pub type DYNAMICSREX = crate::Reg<u32, _DYNAMICSREX>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DYNAMICSREX;
#[doc = "`read()` method returns [dynamicsrex::R](dynamicsrex::R) reader structure"]
impl crate::Readable for DYNAMICSREX {}
#[doc = "`write(|w| ..)` method takes [dynamicsrex::W](dynamicsrex::W) writer structure"]
impl crate::Writable for DYNAMICSREX {}
#[doc = "Self-refresh exit time"]
pub mod dynamicsrex;
#[doc = "Last-data-out to active command time\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dynamicapr](dynamicapr) module"]
pub type DYNAMICAPR = crate::Reg<u32, _DYNAMICAPR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DYNAMICAPR;
#[doc = "`read()` method returns [dynamicapr::R](dynamicapr::R) reader structure"]
impl crate::Readable for DYNAMICAPR {}
#[doc = "`write(|w| ..)` method takes [dynamicapr::W](dynamicapr::W) writer structure"]
impl crate::Writable for DYNAMICAPR {}
#[doc = "Last-data-out to active command time"]
pub mod dynamicapr;
#[doc = "Data-in to active command time\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dynamicdal](dynamicdal) module"]
pub type DYNAMICDAL = crate::Reg<u32, _DYNAMICDAL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DYNAMICDAL;
#[doc = "`read()` method returns [dynamicdal::R](dynamicdal::R) reader structure"]
impl crate::Readable for DYNAMICDAL {}
#[doc = "`write(|w| ..)` method takes [dynamicdal::W](dynamicdal::W) writer structure"]
impl crate::Writable for DYNAMICDAL {}
#[doc = "Data-in to active command time"]
pub mod dynamicdal;
#[doc = "Write recovery time\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dynamicwr](dynamicwr) module"]
pub type DYNAMICWR = crate::Reg<u32, _DYNAMICWR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DYNAMICWR;
#[doc = "`read()` method returns [dynamicwr::R](dynamicwr::R) reader structure"]
impl crate::Readable for DYNAMICWR {}
#[doc = "`write(|w| ..)` method takes [dynamicwr::W](dynamicwr::W) writer structure"]
impl crate::Writable for DYNAMICWR {}
#[doc = "Write recovery time"]
pub mod dynamicwr;
#[doc = "Selects the active to active command period\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dynamicrc](dynamicrc) module"]
pub type DYNAMICRC = crate::Reg<u32, _DYNAMICRC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DYNAMICRC;
#[doc = "`read()` method returns [dynamicrc::R](dynamicrc::R) reader structure"]
impl crate::Readable for DYNAMICRC {}
#[doc = "`write(|w| ..)` method takes [dynamicrc::W](dynamicrc::W) writer structure"]
impl crate::Writable for DYNAMICRC {}
#[doc = "Selects the active to active command period"]
pub mod dynamicrc;
#[doc = "Selects the auto-refresh period\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dynamicrfc](dynamicrfc) module"]
pub type DYNAMICRFC = crate::Reg<u32, _DYNAMICRFC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DYNAMICRFC;
#[doc = "`read()` method returns [dynamicrfc::R](dynamicrfc::R) reader structure"]
impl crate::Readable for DYNAMICRFC {}
#[doc = "`write(|w| ..)` method takes [dynamicrfc::W](dynamicrfc::W) writer structure"]
impl crate::Writable for DYNAMICRFC {}
#[doc = "Selects the auto-refresh period"]
pub mod dynamicrfc;
#[doc = "Time for exit self-refresh to active command\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dynamicxsr](dynamicxsr) module"]
pub type DYNAMICXSR = crate::Reg<u32, _DYNAMICXSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DYNAMICXSR;
#[doc = "`read()` method returns [dynamicxsr::R](dynamicxsr::R) reader structure"]
impl crate::Readable for DYNAMICXSR {}
#[doc = "`write(|w| ..)` method takes [dynamicxsr::W](dynamicxsr::W) writer structure"]
impl crate::Writable for DYNAMICXSR {}
#[doc = "Time for exit self-refresh to active command"]
pub mod dynamicxsr;
#[doc = "Latency for active bank A to active bank B\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dynamicrrd](dynamicrrd) module"]
pub type DYNAMICRRD = crate::Reg<u32, _DYNAMICRRD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DYNAMICRRD;
#[doc = "`read()` method returns [dynamicrrd::R](dynamicrrd::R) reader structure"]
impl crate::Readable for DYNAMICRRD {}
#[doc = "`write(|w| ..)` method takes [dynamicrrd::W](dynamicrrd::W) writer structure"]
impl crate::Writable for DYNAMICRRD {}
#[doc = "Latency for active bank A to active bank B"]
pub mod dynamicrrd;
#[doc = "Time for load mode register to active command\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dynamicmrd](dynamicmrd) module"]
pub type DYNAMICMRD = crate::Reg<u32, _DYNAMICMRD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DYNAMICMRD;
#[doc = "`read()` method returns [dynamicmrd::R](dynamicmrd::R) reader structure"]
impl crate::Readable for DYNAMICMRD {}
#[doc = "`write(|w| ..)` method takes [dynamicmrd::W](dynamicmrd::W) writer structure"]
impl crate::Writable for DYNAMICMRD {}
#[doc = "Time for load mode register to active command"]
pub mod dynamicmrd;
#[doc = "Time for long static memory read and write transfers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [staticextendedwait](staticextendedwait) module"]
pub type STATICEXTENDEDWAIT = crate::Reg<u32, _STATICEXTENDEDWAIT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _STATICEXTENDEDWAIT;
#[doc = "`read()` method returns [staticextendedwait::R](staticextendedwait::R) reader structure"]
impl crate::Readable for STATICEXTENDEDWAIT {}
#[doc = "`write(|w| ..)` method takes [staticextendedwait::W](staticextendedwait::W) writer structure"]
impl crate::Writable for STATICEXTENDEDWAIT {}
#[doc = "Time for long static memory read and write transfers"]
pub mod staticextendedwait;
