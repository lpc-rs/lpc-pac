#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - SCT configuration register"]
    pub config: CONFIG,
    #[doc = "0x04 - SCT control register"]
    pub ctrl: CTRL,
    #[doc = "0x08 - SCT limit event select register"]
    pub limit: LIMIT,
    #[doc = "0x0c - SCT halt event select register"]
    pub halt: HALT,
    #[doc = "0x10 - SCT stop event select register"]
    pub stop: STOP,
    #[doc = "0x14 - SCT start event select register"]
    pub start: START,
    _reserved6: [u8; 40usize],
    #[doc = "0x40 - SCT counter register"]
    pub count: COUNT,
    #[doc = "0x44 - SCT state register"]
    pub state: STATE,
    #[doc = "0x48 - SCT input register"]
    pub input: INPUT,
    #[doc = "0x4c - SCT match/capture mode register"]
    pub regmode: REGMODE,
    #[doc = "0x50 - SCT output register"]
    pub output: OUTPUT,
    #[doc = "0x54 - SCT output counter direction control register"]
    pub outputdirctrl: OUTPUTDIRCTRL,
    #[doc = "0x58 - SCT conflict resolution register"]
    pub res: RES,
    #[doc = "0x5c - SCT DMA request 0 register"]
    pub dma0request: DMA0REQUEST,
    #[doc = "0x60 - SCT DMA request 1 register"]
    pub dma1request: DMA1REQUEST,
    _reserved15: [u8; 140usize],
    #[doc = "0xf0 - SCT event interrupt enable register"]
    pub even: EVEN,
    #[doc = "0xf4 - SCT event flag register"]
    pub evflag: EVFLAG,
    #[doc = "0xf8 - SCT conflict interrupt enable register"]
    pub conen: CONEN,
    #[doc = "0xfc - SCT conflict flag register"]
    pub conflag: CONFLAG,
    _reserved_19_sctcap: [u8; 32usize],
    _reserved20: [u8; 224usize],
    _reserved_20_sctcapctrl: [u8; 32usize],
    _reserved21: [u8; 224usize],
    #[doc = "0x300 - no description available"]
    pub event: [EVENT; 8],
    _reserved22: [u8; 448usize],
    #[doc = "0x500 - no description available"]
    pub out: [OUT; 7],
}
impl RegisterBlock {
    #[doc = "0x100 - SCT match value register of match channels"]
    #[inline(always)]
    pub fn sctmatch(&self) -> &[SCTMATCH; 8] {
        unsafe { &*(((self as *const Self) as *const u8).add(256usize) as *const [SCTMATCH; 8]) }
    }
    #[doc = "0x100 - SCT match value register of match channels"]
    #[inline(always)]
    pub fn sctmatch_mut(&self) -> &mut [SCTMATCH; 8] {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(256usize) as *mut [SCTMATCH; 8]) }
    }
    #[doc = "0x100 - SCT capture register of capture channel"]
    #[inline(always)]
    pub fn sctcap(&self) -> &[SCTCAP; 8] {
        unsafe { &*(((self as *const Self) as *const u8).add(256usize) as *const [SCTCAP; 8]) }
    }
    #[doc = "0x100 - SCT capture register of capture channel"]
    #[inline(always)]
    pub fn sctcap_mut(&self) -> &mut [SCTCAP; 8] {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(256usize) as *mut [SCTCAP; 8]) }
    }
    #[doc = "0x200 - SCT match reload value register"]
    #[inline(always)]
    pub fn sctmatchrel(&self) -> &[SCTMATCHREL; 8] {
        unsafe { &*(((self as *const Self) as *const u8).add(512usize) as *const [SCTMATCHREL; 8]) }
    }
    #[doc = "0x200 - SCT match reload value register"]
    #[inline(always)]
    pub fn sctmatchrel_mut(&self) -> &mut [SCTMATCHREL; 8] {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(512usize) as *mut [SCTMATCHREL; 8]) }
    }
    #[doc = "0x200 - SCT capture control register"]
    #[inline(always)]
    pub fn sctcapctrl(&self) -> &[SCTCAPCTRL; 8] {
        unsafe { &*(((self as *const Self) as *const u8).add(512usize) as *const [SCTCAPCTRL; 8]) }
    }
    #[doc = "0x200 - SCT capture control register"]
    #[inline(always)]
    pub fn sctcapctrl_mut(&self) -> &mut [SCTCAPCTRL; 8] {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(512usize) as *mut [SCTCAPCTRL; 8]) }
    }
}
#[doc = r"Register block"]
#[repr(C)]
pub struct EVENT {
    #[doc = "0x00 - SCT event state register 0"]
    pub state: self::event::STATE,
    #[doc = "0x04 - SCT event control register 0"]
    pub ctrl: self::event::CTRL,
}
#[doc = r"Register block"]
#[doc = "no description available"]
pub mod event;
#[doc = r"Register block"]
#[repr(C)]
pub struct OUT {
    #[doc = "0x00 - SCT output 0 set register"]
    pub set: self::out::SET,
    #[doc = "0x04 - SCT output 0 clear register"]
    pub clr: self::out::CLR,
}
#[doc = r"Register block"]
#[doc = "no description available"]
pub mod out;
#[doc = "SCT configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [config](config) module"]
pub type CONFIG = crate::Reg<u32, _CONFIG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CONFIG;
#[doc = "`read()` method returns [config::R](config::R) reader structure"]
impl crate::Readable for CONFIG {}
#[doc = "`write(|w| ..)` method takes [config::W](config::W) writer structure"]
impl crate::Writable for CONFIG {}
#[doc = "SCT configuration register"]
pub mod config;
#[doc = "SCT control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrl](ctrl) module"]
pub type CTRL = crate::Reg<u32, _CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CTRL;
#[doc = "`read()` method returns [ctrl::R](ctrl::R) reader structure"]
impl crate::Readable for CTRL {}
#[doc = "`write(|w| ..)` method takes [ctrl::W](ctrl::W) writer structure"]
impl crate::Writable for CTRL {}
#[doc = "SCT control register"]
pub mod ctrl;
#[doc = "SCT limit event select register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [limit](limit) module"]
pub type LIMIT = crate::Reg<u32, _LIMIT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LIMIT;
#[doc = "`read()` method returns [limit::R](limit::R) reader structure"]
impl crate::Readable for LIMIT {}
#[doc = "`write(|w| ..)` method takes [limit::W](limit::W) writer structure"]
impl crate::Writable for LIMIT {}
#[doc = "SCT limit event select register"]
pub mod limit;
#[doc = "SCT halt event select register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [halt](halt) module"]
pub type HALT = crate::Reg<u32, _HALT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HALT;
#[doc = "`read()` method returns [halt::R](halt::R) reader structure"]
impl crate::Readable for HALT {}
#[doc = "`write(|w| ..)` method takes [halt::W](halt::W) writer structure"]
impl crate::Writable for HALT {}
#[doc = "SCT halt event select register"]
pub mod halt;
#[doc = "SCT stop event select register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [stop](stop) module"]
pub type STOP = crate::Reg<u32, _STOP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _STOP;
#[doc = "`read()` method returns [stop::R](stop::R) reader structure"]
impl crate::Readable for STOP {}
#[doc = "`write(|w| ..)` method takes [stop::W](stop::W) writer structure"]
impl crate::Writable for STOP {}
#[doc = "SCT stop event select register"]
pub mod stop;
#[doc = "SCT start event select register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [start](start) module"]
pub type START = crate::Reg<u32, _START>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _START;
#[doc = "`read()` method returns [start::R](start::R) reader structure"]
impl crate::Readable for START {}
#[doc = "`write(|w| ..)` method takes [start::W](start::W) writer structure"]
impl crate::Writable for START {}
#[doc = "SCT start event select register"]
pub mod start;
#[doc = "SCT counter register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [count](count) module"]
pub type COUNT = crate::Reg<u32, _COUNT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _COUNT;
#[doc = "`read()` method returns [count::R](count::R) reader structure"]
impl crate::Readable for COUNT {}
#[doc = "`write(|w| ..)` method takes [count::W](count::W) writer structure"]
impl crate::Writable for COUNT {}
#[doc = "SCT counter register"]
pub mod count;
#[doc = "SCT state register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [state](state) module"]
pub type STATE = crate::Reg<u32, _STATE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _STATE;
#[doc = "`read()` method returns [state::R](state::R) reader structure"]
impl crate::Readable for STATE {}
#[doc = "`write(|w| ..)` method takes [state::W](state::W) writer structure"]
impl crate::Writable for STATE {}
#[doc = "SCT state register"]
pub mod state;
#[doc = "SCT input register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [input](input) module"]
pub type INPUT = crate::Reg<u32, _INPUT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INPUT;
#[doc = "`read()` method returns [input::R](input::R) reader structure"]
impl crate::Readable for INPUT {}
#[doc = "`write(|w| ..)` method takes [input::W](input::W) writer structure"]
impl crate::Writable for INPUT {}
#[doc = "SCT input register"]
pub mod input;
#[doc = "SCT match/capture mode register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [regmode](regmode) module"]
pub type REGMODE = crate::Reg<u32, _REGMODE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _REGMODE;
#[doc = "`read()` method returns [regmode::R](regmode::R) reader structure"]
impl crate::Readable for REGMODE {}
#[doc = "`write(|w| ..)` method takes [regmode::W](regmode::W) writer structure"]
impl crate::Writable for REGMODE {}
#[doc = "SCT match/capture mode register"]
pub mod regmode;
#[doc = "SCT output register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [output](output) module"]
pub type OUTPUT = crate::Reg<u32, _OUTPUT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OUTPUT;
#[doc = "`read()` method returns [output::R](output::R) reader structure"]
impl crate::Readable for OUTPUT {}
#[doc = "`write(|w| ..)` method takes [output::W](output::W) writer structure"]
impl crate::Writable for OUTPUT {}
#[doc = "SCT output register"]
pub mod output;
#[doc = "SCT output counter direction control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [outputdirctrl](outputdirctrl) module"]
pub type OUTPUTDIRCTRL = crate::Reg<u32, _OUTPUTDIRCTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OUTPUTDIRCTRL;
#[doc = "`read()` method returns [outputdirctrl::R](outputdirctrl::R) reader structure"]
impl crate::Readable for OUTPUTDIRCTRL {}
#[doc = "`write(|w| ..)` method takes [outputdirctrl::W](outputdirctrl::W) writer structure"]
impl crate::Writable for OUTPUTDIRCTRL {}
#[doc = "SCT output counter direction control register"]
pub mod outputdirctrl;
#[doc = "SCT conflict resolution register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [res](res) module"]
pub type RES = crate::Reg<u32, _RES>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RES;
#[doc = "`read()` method returns [res::R](res::R) reader structure"]
impl crate::Readable for RES {}
#[doc = "`write(|w| ..)` method takes [res::W](res::W) writer structure"]
impl crate::Writable for RES {}
#[doc = "SCT conflict resolution register"]
pub mod res;
#[doc = "SCT DMA request 0 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma0request](dma0request) module"]
pub type DMA0REQUEST = crate::Reg<u32, _DMA0REQUEST>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMA0REQUEST;
#[doc = "`read()` method returns [dma0request::R](dma0request::R) reader structure"]
impl crate::Readable for DMA0REQUEST {}
#[doc = "`write(|w| ..)` method takes [dma0request::W](dma0request::W) writer structure"]
impl crate::Writable for DMA0REQUEST {}
#[doc = "SCT DMA request 0 register"]
pub mod dma0request;
#[doc = "SCT DMA request 1 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma1request](dma1request) module"]
pub type DMA1REQUEST = crate::Reg<u32, _DMA1REQUEST>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMA1REQUEST;
#[doc = "`read()` method returns [dma1request::R](dma1request::R) reader structure"]
impl crate::Readable for DMA1REQUEST {}
#[doc = "`write(|w| ..)` method takes [dma1request::W](dma1request::W) writer structure"]
impl crate::Writable for DMA1REQUEST {}
#[doc = "SCT DMA request 1 register"]
pub mod dma1request;
#[doc = "SCT event interrupt enable register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [even](even) module"]
pub type EVEN = crate::Reg<u32, _EVEN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EVEN;
#[doc = "`read()` method returns [even::R](even::R) reader structure"]
impl crate::Readable for EVEN {}
#[doc = "`write(|w| ..)` method takes [even::W](even::W) writer structure"]
impl crate::Writable for EVEN {}
#[doc = "SCT event interrupt enable register"]
pub mod even;
#[doc = "SCT event flag register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [evflag](evflag) module"]
pub type EVFLAG = crate::Reg<u32, _EVFLAG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EVFLAG;
#[doc = "`read()` method returns [evflag::R](evflag::R) reader structure"]
impl crate::Readable for EVFLAG {}
#[doc = "`write(|w| ..)` method takes [evflag::W](evflag::W) writer structure"]
impl crate::Writable for EVFLAG {}
#[doc = "SCT event flag register"]
pub mod evflag;
#[doc = "SCT conflict interrupt enable register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [conen](conen) module"]
pub type CONEN = crate::Reg<u32, _CONEN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CONEN;
#[doc = "`read()` method returns [conen::R](conen::R) reader structure"]
impl crate::Readable for CONEN {}
#[doc = "`write(|w| ..)` method takes [conen::W](conen::W) writer structure"]
impl crate::Writable for CONEN {}
#[doc = "SCT conflict interrupt enable register"]
pub mod conen;
#[doc = "SCT conflict flag register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [conflag](conflag) module"]
pub type CONFLAG = crate::Reg<u32, _CONFLAG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CONFLAG;
#[doc = "`read()` method returns [conflag::R](conflag::R) reader structure"]
impl crate::Readable for CONFLAG {}
#[doc = "`write(|w| ..)` method takes [conflag::W](conflag::W) writer structure"]
impl crate::Writable for CONFLAG {}
#[doc = "SCT conflict flag register"]
pub mod conflag;
#[doc = "SCT capture register of capture channel\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sctcap](sctcap) module"]
pub type SCTCAP = crate::Reg<u32, _SCTCAP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SCTCAP;
#[doc = "`read()` method returns [sctcap::R](sctcap::R) reader structure"]
impl crate::Readable for SCTCAP {}
#[doc = "`write(|w| ..)` method takes [sctcap::W](sctcap::W) writer structure"]
impl crate::Writable for SCTCAP {}
#[doc = "SCT capture register of capture channel"]
pub mod sctcap;
#[doc = "SCT match value register of match channels\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sctmatch](sctmatch) module"]
pub type SCTMATCH = crate::Reg<u32, _SCTMATCH>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SCTMATCH;
#[doc = "`read()` method returns [sctmatch::R](sctmatch::R) reader structure"]
impl crate::Readable for SCTMATCH {}
#[doc = "`write(|w| ..)` method takes [sctmatch::W](sctmatch::W) writer structure"]
impl crate::Writable for SCTMATCH {}
#[doc = "SCT match value register of match channels"]
pub mod sctmatch;
#[doc = "SCT capture control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sctcapctrl](sctcapctrl) module"]
pub type SCTCAPCTRL = crate::Reg<u32, _SCTCAPCTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SCTCAPCTRL;
#[doc = "`read()` method returns [sctcapctrl::R](sctcapctrl::R) reader structure"]
impl crate::Readable for SCTCAPCTRL {}
#[doc = "`write(|w| ..)` method takes [sctcapctrl::W](sctcapctrl::W) writer structure"]
impl crate::Writable for SCTCAPCTRL {}
#[doc = "SCT capture control register"]
pub mod sctcapctrl;
#[doc = "SCT match reload value register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sctmatchrel](sctmatchrel) module"]
pub type SCTMATCHREL = crate::Reg<u32, _SCTMATCHREL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SCTMATCHREL;
#[doc = "`read()` method returns [sctmatchrel::R](sctmatchrel::R) reader structure"]
impl crate::Readable for SCTMATCHREL {}
#[doc = "`write(|w| ..)` method takes [sctmatchrel::W](sctmatchrel::W) writer structure"]
impl crate::Writable for SCTMATCHREL {}
#[doc = "SCT match reload value register"]
pub mod sctmatchrel;
