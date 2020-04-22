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
    _reserved_19_sctcap0: [u8; 4usize],
    _reserved_20_sctcap1: [u8; 4usize],
    _reserved_21_sctcap2: [u8; 4usize],
    _reserved_22_sctcap3: [u8; 4usize],
    _reserved_23_sctcap4: [u8; 4usize],
    _reserved_24_sctcap5: [u8; 4usize],
    _reserved_25_sctcap6: [u8; 4usize],
    _reserved_26_sctcap7: [u8; 4usize],
    _reserved27: [u8; 224usize],
    _reserved_27_sctcapctrl0: [u8; 4usize],
    _reserved_28_sctcapctrl1: [u8; 4usize],
    _reserved_29_sctcapctrl2: [u8; 4usize],
    _reserved_30_sctcapctrl3: [u8; 4usize],
    _reserved_31_sctcapctrl4: [u8; 4usize],
    _reserved_32_sctcapctrl5: [u8; 4usize],
    _reserved_33_sctcapctrl6: [u8; 4usize],
    _reserved_34_sctcapctrl7: [u8; 4usize],
    _reserved35: [u8; 224usize],
    #[doc = "0x300 - no description available"]
    pub event: [EVENT; 8],
    _reserved36: [u8; 448usize],
    #[doc = "0x500 - no description available"]
    pub out: [OUT; 6],
}
impl RegisterBlock {
    #[doc = "0x100 - SCT match value register of match channels"]
    #[inline(always)]
    pub fn sctmatch0(&self) -> &SCTMATCH0 {
        unsafe { &*(((self as *const Self) as *const u8).add(256usize) as *const SCTMATCH0) }
    }
    #[doc = "0x100 - SCT match value register of match channels"]
    #[inline(always)]
    pub fn sctmatch0_mut(&self) -> &mut SCTMATCH0 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(256usize) as *mut SCTMATCH0) }
    }
    #[doc = "0x100 - SCT capture register of capture channel"]
    #[inline(always)]
    pub fn sctcap0(&self) -> &SCTCAP0 {
        unsafe { &*(((self as *const Self) as *const u8).add(256usize) as *const SCTCAP0) }
    }
    #[doc = "0x100 - SCT capture register of capture channel"]
    #[inline(always)]
    pub fn sctcap0_mut(&self) -> &mut SCTCAP0 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(256usize) as *mut SCTCAP0) }
    }
    #[doc = "0x104 - SCT match value register of match channels"]
    #[inline(always)]
    pub fn sctmatch1(&self) -> &SCTMATCH1 {
        unsafe { &*(((self as *const Self) as *const u8).add(260usize) as *const SCTMATCH1) }
    }
    #[doc = "0x104 - SCT match value register of match channels"]
    #[inline(always)]
    pub fn sctmatch1_mut(&self) -> &mut SCTMATCH1 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(260usize) as *mut SCTMATCH1) }
    }
    #[doc = "0x104 - SCT capture register of capture channel"]
    #[inline(always)]
    pub fn sctcap1(&self) -> &SCTCAP1 {
        unsafe { &*(((self as *const Self) as *const u8).add(260usize) as *const SCTCAP1) }
    }
    #[doc = "0x104 - SCT capture register of capture channel"]
    #[inline(always)]
    pub fn sctcap1_mut(&self) -> &mut SCTCAP1 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(260usize) as *mut SCTCAP1) }
    }
    #[doc = "0x108 - SCT match value register of match channels"]
    #[inline(always)]
    pub fn sctmatch2(&self) -> &SCTMATCH2 {
        unsafe { &*(((self as *const Self) as *const u8).add(264usize) as *const SCTMATCH2) }
    }
    #[doc = "0x108 - SCT match value register of match channels"]
    #[inline(always)]
    pub fn sctmatch2_mut(&self) -> &mut SCTMATCH2 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(264usize) as *mut SCTMATCH2) }
    }
    #[doc = "0x108 - SCT capture register of capture channel"]
    #[inline(always)]
    pub fn sctcap2(&self) -> &SCTCAP2 {
        unsafe { &*(((self as *const Self) as *const u8).add(264usize) as *const SCTCAP2) }
    }
    #[doc = "0x108 - SCT capture register of capture channel"]
    #[inline(always)]
    pub fn sctcap2_mut(&self) -> &mut SCTCAP2 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(264usize) as *mut SCTCAP2) }
    }
    #[doc = "0x10c - SCT match value register of match channels"]
    #[inline(always)]
    pub fn sctmatch3(&self) -> &SCTMATCH3 {
        unsafe { &*(((self as *const Self) as *const u8).add(268usize) as *const SCTMATCH3) }
    }
    #[doc = "0x10c - SCT match value register of match channels"]
    #[inline(always)]
    pub fn sctmatch3_mut(&self) -> &mut SCTMATCH3 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(268usize) as *mut SCTMATCH3) }
    }
    #[doc = "0x10c - SCT capture register of capture channel"]
    #[inline(always)]
    pub fn sctcap3(&self) -> &SCTCAP3 {
        unsafe { &*(((self as *const Self) as *const u8).add(268usize) as *const SCTCAP3) }
    }
    #[doc = "0x10c - SCT capture register of capture channel"]
    #[inline(always)]
    pub fn sctcap3_mut(&self) -> &mut SCTCAP3 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(268usize) as *mut SCTCAP3) }
    }
    #[doc = "0x110 - SCT match value register of match channels"]
    #[inline(always)]
    pub fn sctmatch4(&self) -> &SCTMATCH4 {
        unsafe { &*(((self as *const Self) as *const u8).add(272usize) as *const SCTMATCH4) }
    }
    #[doc = "0x110 - SCT match value register of match channels"]
    #[inline(always)]
    pub fn sctmatch4_mut(&self) -> &mut SCTMATCH4 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(272usize) as *mut SCTMATCH4) }
    }
    #[doc = "0x110 - SCT capture register of capture channel"]
    #[inline(always)]
    pub fn sctcap4(&self) -> &SCTCAP4 {
        unsafe { &*(((self as *const Self) as *const u8).add(272usize) as *const SCTCAP4) }
    }
    #[doc = "0x110 - SCT capture register of capture channel"]
    #[inline(always)]
    pub fn sctcap4_mut(&self) -> &mut SCTCAP4 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(272usize) as *mut SCTCAP4) }
    }
    #[doc = "0x114 - SCT match value register of match channels"]
    #[inline(always)]
    pub fn sctmatch5(&self) -> &SCTMATCH5 {
        unsafe { &*(((self as *const Self) as *const u8).add(276usize) as *const SCTMATCH5) }
    }
    #[doc = "0x114 - SCT match value register of match channels"]
    #[inline(always)]
    pub fn sctmatch5_mut(&self) -> &mut SCTMATCH5 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(276usize) as *mut SCTMATCH5) }
    }
    #[doc = "0x114 - SCT capture register of capture channel"]
    #[inline(always)]
    pub fn sctcap5(&self) -> &SCTCAP5 {
        unsafe { &*(((self as *const Self) as *const u8).add(276usize) as *const SCTCAP5) }
    }
    #[doc = "0x114 - SCT capture register of capture channel"]
    #[inline(always)]
    pub fn sctcap5_mut(&self) -> &mut SCTCAP5 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(276usize) as *mut SCTCAP5) }
    }
    #[doc = "0x118 - SCT match value register of match channels"]
    #[inline(always)]
    pub fn sctmatch6(&self) -> &SCTMATCH6 {
        unsafe { &*(((self as *const Self) as *const u8).add(280usize) as *const SCTMATCH6) }
    }
    #[doc = "0x118 - SCT match value register of match channels"]
    #[inline(always)]
    pub fn sctmatch6_mut(&self) -> &mut SCTMATCH6 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(280usize) as *mut SCTMATCH6) }
    }
    #[doc = "0x118 - SCT capture register of capture channel"]
    #[inline(always)]
    pub fn sctcap6(&self) -> &SCTCAP6 {
        unsafe { &*(((self as *const Self) as *const u8).add(280usize) as *const SCTCAP6) }
    }
    #[doc = "0x118 - SCT capture register of capture channel"]
    #[inline(always)]
    pub fn sctcap6_mut(&self) -> &mut SCTCAP6 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(280usize) as *mut SCTCAP6) }
    }
    #[doc = "0x11c - SCT match value register of match channels"]
    #[inline(always)]
    pub fn sctmatch7(&self) -> &SCTMATCH7 {
        unsafe { &*(((self as *const Self) as *const u8).add(284usize) as *const SCTMATCH7) }
    }
    #[doc = "0x11c - SCT match value register of match channels"]
    #[inline(always)]
    pub fn sctmatch7_mut(&self) -> &mut SCTMATCH7 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(284usize) as *mut SCTMATCH7) }
    }
    #[doc = "0x11c - SCT capture register of capture channel"]
    #[inline(always)]
    pub fn sctcap7(&self) -> &SCTCAP7 {
        unsafe { &*(((self as *const Self) as *const u8).add(284usize) as *const SCTCAP7) }
    }
    #[doc = "0x11c - SCT capture register of capture channel"]
    #[inline(always)]
    pub fn sctcap7_mut(&self) -> &mut SCTCAP7 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(284usize) as *mut SCTCAP7) }
    }
    #[doc = "0x200 - SCT match reload value register"]
    #[inline(always)]
    pub fn sctmatchrel0(&self) -> &SCTMATCHREL0 {
        unsafe { &*(((self as *const Self) as *const u8).add(512usize) as *const SCTMATCHREL0) }
    }
    #[doc = "0x200 - SCT match reload value register"]
    #[inline(always)]
    pub fn sctmatchrel0_mut(&self) -> &mut SCTMATCHREL0 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(512usize) as *mut SCTMATCHREL0) }
    }
    #[doc = "0x200 - SCT capture control register"]
    #[inline(always)]
    pub fn sctcapctrl0(&self) -> &SCTCAPCTRL0 {
        unsafe { &*(((self as *const Self) as *const u8).add(512usize) as *const SCTCAPCTRL0) }
    }
    #[doc = "0x200 - SCT capture control register"]
    #[inline(always)]
    pub fn sctcapctrl0_mut(&self) -> &mut SCTCAPCTRL0 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(512usize) as *mut SCTCAPCTRL0) }
    }
    #[doc = "0x204 - SCT match reload value register"]
    #[inline(always)]
    pub fn sctmatchrel1(&self) -> &SCTMATCHREL1 {
        unsafe { &*(((self as *const Self) as *const u8).add(516usize) as *const SCTMATCHREL1) }
    }
    #[doc = "0x204 - SCT match reload value register"]
    #[inline(always)]
    pub fn sctmatchrel1_mut(&self) -> &mut SCTMATCHREL1 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(516usize) as *mut SCTMATCHREL1) }
    }
    #[doc = "0x204 - SCT capture control register"]
    #[inline(always)]
    pub fn sctcapctrl1(&self) -> &SCTCAPCTRL1 {
        unsafe { &*(((self as *const Self) as *const u8).add(516usize) as *const SCTCAPCTRL1) }
    }
    #[doc = "0x204 - SCT capture control register"]
    #[inline(always)]
    pub fn sctcapctrl1_mut(&self) -> &mut SCTCAPCTRL1 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(516usize) as *mut SCTCAPCTRL1) }
    }
    #[doc = "0x208 - SCT match reload value register"]
    #[inline(always)]
    pub fn sctmatchrel2(&self) -> &SCTMATCHREL2 {
        unsafe { &*(((self as *const Self) as *const u8).add(520usize) as *const SCTMATCHREL2) }
    }
    #[doc = "0x208 - SCT match reload value register"]
    #[inline(always)]
    pub fn sctmatchrel2_mut(&self) -> &mut SCTMATCHREL2 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(520usize) as *mut SCTMATCHREL2) }
    }
    #[doc = "0x208 - SCT capture control register"]
    #[inline(always)]
    pub fn sctcapctrl2(&self) -> &SCTCAPCTRL2 {
        unsafe { &*(((self as *const Self) as *const u8).add(520usize) as *const SCTCAPCTRL2) }
    }
    #[doc = "0x208 - SCT capture control register"]
    #[inline(always)]
    pub fn sctcapctrl2_mut(&self) -> &mut SCTCAPCTRL2 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(520usize) as *mut SCTCAPCTRL2) }
    }
    #[doc = "0x20c - SCT match reload value register"]
    #[inline(always)]
    pub fn sctmatchrel3(&self) -> &SCTMATCHREL3 {
        unsafe { &*(((self as *const Self) as *const u8).add(524usize) as *const SCTMATCHREL3) }
    }
    #[doc = "0x20c - SCT match reload value register"]
    #[inline(always)]
    pub fn sctmatchrel3_mut(&self) -> &mut SCTMATCHREL3 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(524usize) as *mut SCTMATCHREL3) }
    }
    #[doc = "0x20c - SCT capture control register"]
    #[inline(always)]
    pub fn sctcapctrl3(&self) -> &SCTCAPCTRL3 {
        unsafe { &*(((self as *const Self) as *const u8).add(524usize) as *const SCTCAPCTRL3) }
    }
    #[doc = "0x20c - SCT capture control register"]
    #[inline(always)]
    pub fn sctcapctrl3_mut(&self) -> &mut SCTCAPCTRL3 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(524usize) as *mut SCTCAPCTRL3) }
    }
    #[doc = "0x210 - SCT match reload value register"]
    #[inline(always)]
    pub fn sctmatchrel4(&self) -> &SCTMATCHREL4 {
        unsafe { &*(((self as *const Self) as *const u8).add(528usize) as *const SCTMATCHREL4) }
    }
    #[doc = "0x210 - SCT match reload value register"]
    #[inline(always)]
    pub fn sctmatchrel4_mut(&self) -> &mut SCTMATCHREL4 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(528usize) as *mut SCTMATCHREL4) }
    }
    #[doc = "0x210 - SCT capture control register"]
    #[inline(always)]
    pub fn sctcapctrl4(&self) -> &SCTCAPCTRL4 {
        unsafe { &*(((self as *const Self) as *const u8).add(528usize) as *const SCTCAPCTRL4) }
    }
    #[doc = "0x210 - SCT capture control register"]
    #[inline(always)]
    pub fn sctcapctrl4_mut(&self) -> &mut SCTCAPCTRL4 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(528usize) as *mut SCTCAPCTRL4) }
    }
    #[doc = "0x214 - SCT match reload value register"]
    #[inline(always)]
    pub fn sctmatchrel5(&self) -> &SCTMATCHREL5 {
        unsafe { &*(((self as *const Self) as *const u8).add(532usize) as *const SCTMATCHREL5) }
    }
    #[doc = "0x214 - SCT match reload value register"]
    #[inline(always)]
    pub fn sctmatchrel5_mut(&self) -> &mut SCTMATCHREL5 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(532usize) as *mut SCTMATCHREL5) }
    }
    #[doc = "0x214 - SCT capture control register"]
    #[inline(always)]
    pub fn sctcapctrl5(&self) -> &SCTCAPCTRL5 {
        unsafe { &*(((self as *const Self) as *const u8).add(532usize) as *const SCTCAPCTRL5) }
    }
    #[doc = "0x214 - SCT capture control register"]
    #[inline(always)]
    pub fn sctcapctrl5_mut(&self) -> &mut SCTCAPCTRL5 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(532usize) as *mut SCTCAPCTRL5) }
    }
    #[doc = "0x218 - SCT match reload value register"]
    #[inline(always)]
    pub fn sctmatchrel6(&self) -> &SCTMATCHREL6 {
        unsafe { &*(((self as *const Self) as *const u8).add(536usize) as *const SCTMATCHREL6) }
    }
    #[doc = "0x218 - SCT match reload value register"]
    #[inline(always)]
    pub fn sctmatchrel6_mut(&self) -> &mut SCTMATCHREL6 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(536usize) as *mut SCTMATCHREL6) }
    }
    #[doc = "0x218 - SCT capture control register"]
    #[inline(always)]
    pub fn sctcapctrl6(&self) -> &SCTCAPCTRL6 {
        unsafe { &*(((self as *const Self) as *const u8).add(536usize) as *const SCTCAPCTRL6) }
    }
    #[doc = "0x218 - SCT capture control register"]
    #[inline(always)]
    pub fn sctcapctrl6_mut(&self) -> &mut SCTCAPCTRL6 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(536usize) as *mut SCTCAPCTRL6) }
    }
    #[doc = "0x21c - SCT match reload value register"]
    #[inline(always)]
    pub fn sctmatchrel7(&self) -> &SCTMATCHREL7 {
        unsafe { &*(((self as *const Self) as *const u8).add(540usize) as *const SCTMATCHREL7) }
    }
    #[doc = "0x21c - SCT match reload value register"]
    #[inline(always)]
    pub fn sctmatchrel7_mut(&self) -> &mut SCTMATCHREL7 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(540usize) as *mut SCTMATCHREL7) }
    }
    #[doc = "0x21c - SCT capture control register"]
    #[inline(always)]
    pub fn sctcapctrl7(&self) -> &SCTCAPCTRL7 {
        unsafe { &*(((self as *const Self) as *const u8).add(540usize) as *const SCTCAPCTRL7) }
    }
    #[doc = "0x21c - SCT capture control register"]
    #[inline(always)]
    pub fn sctcapctrl7_mut(&self) -> &mut SCTCAPCTRL7 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(540usize) as *mut SCTCAPCTRL7) }
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
#[doc = "SCT capture register of capture channel\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sctcap0](sctcap0) module"]
pub type SCTCAP0 = crate::Reg<u32, _SCTCAP0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SCTCAP0;
#[doc = "`read()` method returns [sctcap0::R](sctcap0::R) reader structure"]
impl crate::Readable for SCTCAP0 {}
#[doc = "`write(|w| ..)` method takes [sctcap0::W](sctcap0::W) writer structure"]
impl crate::Writable for SCTCAP0 {}
#[doc = "SCT capture register of capture channel"]
pub mod sctcap0;
#[doc = "SCT match value register of match channels\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sctmatch0](sctmatch0) module"]
pub type SCTMATCH0 = crate::Reg<u32, _SCTMATCH0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SCTMATCH0;
#[doc = "`read()` method returns [sctmatch0::R](sctmatch0::R) reader structure"]
impl crate::Readable for SCTMATCH0 {}
#[doc = "`write(|w| ..)` method takes [sctmatch0::W](sctmatch0::W) writer structure"]
impl crate::Writable for SCTMATCH0 {}
#[doc = "SCT match value register of match channels"]
pub mod sctmatch0;
#[doc = "SCT capture register of capture channel\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sctcap1](sctcap1) module"]
pub type SCTCAP1 = crate::Reg<u32, _SCTCAP1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SCTCAP1;
#[doc = "`read()` method returns [sctcap1::R](sctcap1::R) reader structure"]
impl crate::Readable for SCTCAP1 {}
#[doc = "`write(|w| ..)` method takes [sctcap1::W](sctcap1::W) writer structure"]
impl crate::Writable for SCTCAP1 {}
#[doc = "SCT capture register of capture channel"]
pub mod sctcap1;
#[doc = "SCT match value register of match channels\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sctmatch1](sctmatch1) module"]
pub type SCTMATCH1 = crate::Reg<u32, _SCTMATCH1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SCTMATCH1;
#[doc = "`read()` method returns [sctmatch1::R](sctmatch1::R) reader structure"]
impl crate::Readable for SCTMATCH1 {}
#[doc = "`write(|w| ..)` method takes [sctmatch1::W](sctmatch1::W) writer structure"]
impl crate::Writable for SCTMATCH1 {}
#[doc = "SCT match value register of match channels"]
pub mod sctmatch1;
#[doc = "SCT capture register of capture channel\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sctcap2](sctcap2) module"]
pub type SCTCAP2 = crate::Reg<u32, _SCTCAP2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SCTCAP2;
#[doc = "`read()` method returns [sctcap2::R](sctcap2::R) reader structure"]
impl crate::Readable for SCTCAP2 {}
#[doc = "`write(|w| ..)` method takes [sctcap2::W](sctcap2::W) writer structure"]
impl crate::Writable for SCTCAP2 {}
#[doc = "SCT capture register of capture channel"]
pub mod sctcap2;
#[doc = "SCT match value register of match channels\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sctmatch2](sctmatch2) module"]
pub type SCTMATCH2 = crate::Reg<u32, _SCTMATCH2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SCTMATCH2;
#[doc = "`read()` method returns [sctmatch2::R](sctmatch2::R) reader structure"]
impl crate::Readable for SCTMATCH2 {}
#[doc = "`write(|w| ..)` method takes [sctmatch2::W](sctmatch2::W) writer structure"]
impl crate::Writable for SCTMATCH2 {}
#[doc = "SCT match value register of match channels"]
pub mod sctmatch2;
#[doc = "SCT capture register of capture channel\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sctcap3](sctcap3) module"]
pub type SCTCAP3 = crate::Reg<u32, _SCTCAP3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SCTCAP3;
#[doc = "`read()` method returns [sctcap3::R](sctcap3::R) reader structure"]
impl crate::Readable for SCTCAP3 {}
#[doc = "`write(|w| ..)` method takes [sctcap3::W](sctcap3::W) writer structure"]
impl crate::Writable for SCTCAP3 {}
#[doc = "SCT capture register of capture channel"]
pub mod sctcap3;
#[doc = "SCT match value register of match channels\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sctmatch3](sctmatch3) module"]
pub type SCTMATCH3 = crate::Reg<u32, _SCTMATCH3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SCTMATCH3;
#[doc = "`read()` method returns [sctmatch3::R](sctmatch3::R) reader structure"]
impl crate::Readable for SCTMATCH3 {}
#[doc = "`write(|w| ..)` method takes [sctmatch3::W](sctmatch3::W) writer structure"]
impl crate::Writable for SCTMATCH3 {}
#[doc = "SCT match value register of match channels"]
pub mod sctmatch3;
#[doc = "SCT capture register of capture channel\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sctcap4](sctcap4) module"]
pub type SCTCAP4 = crate::Reg<u32, _SCTCAP4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SCTCAP4;
#[doc = "`read()` method returns [sctcap4::R](sctcap4::R) reader structure"]
impl crate::Readable for SCTCAP4 {}
#[doc = "`write(|w| ..)` method takes [sctcap4::W](sctcap4::W) writer structure"]
impl crate::Writable for SCTCAP4 {}
#[doc = "SCT capture register of capture channel"]
pub mod sctcap4;
#[doc = "SCT match value register of match channels\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sctmatch4](sctmatch4) module"]
pub type SCTMATCH4 = crate::Reg<u32, _SCTMATCH4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SCTMATCH4;
#[doc = "`read()` method returns [sctmatch4::R](sctmatch4::R) reader structure"]
impl crate::Readable for SCTMATCH4 {}
#[doc = "`write(|w| ..)` method takes [sctmatch4::W](sctmatch4::W) writer structure"]
impl crate::Writable for SCTMATCH4 {}
#[doc = "SCT match value register of match channels"]
pub mod sctmatch4;
#[doc = "SCT capture register of capture channel\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sctcap5](sctcap5) module"]
pub type SCTCAP5 = crate::Reg<u32, _SCTCAP5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SCTCAP5;
#[doc = "`read()` method returns [sctcap5::R](sctcap5::R) reader structure"]
impl crate::Readable for SCTCAP5 {}
#[doc = "`write(|w| ..)` method takes [sctcap5::W](sctcap5::W) writer structure"]
impl crate::Writable for SCTCAP5 {}
#[doc = "SCT capture register of capture channel"]
pub mod sctcap5;
#[doc = "SCT match value register of match channels\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sctmatch5](sctmatch5) module"]
pub type SCTMATCH5 = crate::Reg<u32, _SCTMATCH5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SCTMATCH5;
#[doc = "`read()` method returns [sctmatch5::R](sctmatch5::R) reader structure"]
impl crate::Readable for SCTMATCH5 {}
#[doc = "`write(|w| ..)` method takes [sctmatch5::W](sctmatch5::W) writer structure"]
impl crate::Writable for SCTMATCH5 {}
#[doc = "SCT match value register of match channels"]
pub mod sctmatch5;
#[doc = "SCT capture register of capture channel\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sctcap6](sctcap6) module"]
pub type SCTCAP6 = crate::Reg<u32, _SCTCAP6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SCTCAP6;
#[doc = "`read()` method returns [sctcap6::R](sctcap6::R) reader structure"]
impl crate::Readable for SCTCAP6 {}
#[doc = "`write(|w| ..)` method takes [sctcap6::W](sctcap6::W) writer structure"]
impl crate::Writable for SCTCAP6 {}
#[doc = "SCT capture register of capture channel"]
pub mod sctcap6;
#[doc = "SCT match value register of match channels\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sctmatch6](sctmatch6) module"]
pub type SCTMATCH6 = crate::Reg<u32, _SCTMATCH6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SCTMATCH6;
#[doc = "`read()` method returns [sctmatch6::R](sctmatch6::R) reader structure"]
impl crate::Readable for SCTMATCH6 {}
#[doc = "`write(|w| ..)` method takes [sctmatch6::W](sctmatch6::W) writer structure"]
impl crate::Writable for SCTMATCH6 {}
#[doc = "SCT match value register of match channels"]
pub mod sctmatch6;
#[doc = "SCT capture register of capture channel\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sctcap7](sctcap7) module"]
pub type SCTCAP7 = crate::Reg<u32, _SCTCAP7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SCTCAP7;
#[doc = "`read()` method returns [sctcap7::R](sctcap7::R) reader structure"]
impl crate::Readable for SCTCAP7 {}
#[doc = "`write(|w| ..)` method takes [sctcap7::W](sctcap7::W) writer structure"]
impl crate::Writable for SCTCAP7 {}
#[doc = "SCT capture register of capture channel"]
pub mod sctcap7;
#[doc = "SCT match value register of match channels\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sctmatch7](sctmatch7) module"]
pub type SCTMATCH7 = crate::Reg<u32, _SCTMATCH7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SCTMATCH7;
#[doc = "`read()` method returns [sctmatch7::R](sctmatch7::R) reader structure"]
impl crate::Readable for SCTMATCH7 {}
#[doc = "`write(|w| ..)` method takes [sctmatch7::W](sctmatch7::W) writer structure"]
impl crate::Writable for SCTMATCH7 {}
#[doc = "SCT match value register of match channels"]
pub mod sctmatch7;
#[doc = "SCT capture control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sctcapctrl0](sctcapctrl0) module"]
pub type SCTCAPCTRL0 = crate::Reg<u32, _SCTCAPCTRL0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SCTCAPCTRL0;
#[doc = "`read()` method returns [sctcapctrl0::R](sctcapctrl0::R) reader structure"]
impl crate::Readable for SCTCAPCTRL0 {}
#[doc = "`write(|w| ..)` method takes [sctcapctrl0::W](sctcapctrl0::W) writer structure"]
impl crate::Writable for SCTCAPCTRL0 {}
#[doc = "SCT capture control register"]
pub mod sctcapctrl0;
#[doc = "SCT match reload value register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sctmatchrel0](sctmatchrel0) module"]
pub type SCTMATCHREL0 = crate::Reg<u32, _SCTMATCHREL0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SCTMATCHREL0;
#[doc = "`read()` method returns [sctmatchrel0::R](sctmatchrel0::R) reader structure"]
impl crate::Readable for SCTMATCHREL0 {}
#[doc = "`write(|w| ..)` method takes [sctmatchrel0::W](sctmatchrel0::W) writer structure"]
impl crate::Writable for SCTMATCHREL0 {}
#[doc = "SCT match reload value register"]
pub mod sctmatchrel0;
#[doc = "SCT capture control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sctcapctrl1](sctcapctrl1) module"]
pub type SCTCAPCTRL1 = crate::Reg<u32, _SCTCAPCTRL1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SCTCAPCTRL1;
#[doc = "`read()` method returns [sctcapctrl1::R](sctcapctrl1::R) reader structure"]
impl crate::Readable for SCTCAPCTRL1 {}
#[doc = "`write(|w| ..)` method takes [sctcapctrl1::W](sctcapctrl1::W) writer structure"]
impl crate::Writable for SCTCAPCTRL1 {}
#[doc = "SCT capture control register"]
pub mod sctcapctrl1;
#[doc = "SCT match reload value register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sctmatchrel1](sctmatchrel1) module"]
pub type SCTMATCHREL1 = crate::Reg<u32, _SCTMATCHREL1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SCTMATCHREL1;
#[doc = "`read()` method returns [sctmatchrel1::R](sctmatchrel1::R) reader structure"]
impl crate::Readable for SCTMATCHREL1 {}
#[doc = "`write(|w| ..)` method takes [sctmatchrel1::W](sctmatchrel1::W) writer structure"]
impl crate::Writable for SCTMATCHREL1 {}
#[doc = "SCT match reload value register"]
pub mod sctmatchrel1;
#[doc = "SCT capture control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sctcapctrl2](sctcapctrl2) module"]
pub type SCTCAPCTRL2 = crate::Reg<u32, _SCTCAPCTRL2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SCTCAPCTRL2;
#[doc = "`read()` method returns [sctcapctrl2::R](sctcapctrl2::R) reader structure"]
impl crate::Readable for SCTCAPCTRL2 {}
#[doc = "`write(|w| ..)` method takes [sctcapctrl2::W](sctcapctrl2::W) writer structure"]
impl crate::Writable for SCTCAPCTRL2 {}
#[doc = "SCT capture control register"]
pub mod sctcapctrl2;
#[doc = "SCT match reload value register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sctmatchrel2](sctmatchrel2) module"]
pub type SCTMATCHREL2 = crate::Reg<u32, _SCTMATCHREL2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SCTMATCHREL2;
#[doc = "`read()` method returns [sctmatchrel2::R](sctmatchrel2::R) reader structure"]
impl crate::Readable for SCTMATCHREL2 {}
#[doc = "`write(|w| ..)` method takes [sctmatchrel2::W](sctmatchrel2::W) writer structure"]
impl crate::Writable for SCTMATCHREL2 {}
#[doc = "SCT match reload value register"]
pub mod sctmatchrel2;
#[doc = "SCT capture control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sctcapctrl3](sctcapctrl3) module"]
pub type SCTCAPCTRL3 = crate::Reg<u32, _SCTCAPCTRL3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SCTCAPCTRL3;
#[doc = "`read()` method returns [sctcapctrl3::R](sctcapctrl3::R) reader structure"]
impl crate::Readable for SCTCAPCTRL3 {}
#[doc = "`write(|w| ..)` method takes [sctcapctrl3::W](sctcapctrl3::W) writer structure"]
impl crate::Writable for SCTCAPCTRL3 {}
#[doc = "SCT capture control register"]
pub mod sctcapctrl3;
#[doc = "SCT match reload value register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sctmatchrel3](sctmatchrel3) module"]
pub type SCTMATCHREL3 = crate::Reg<u32, _SCTMATCHREL3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SCTMATCHREL3;
#[doc = "`read()` method returns [sctmatchrel3::R](sctmatchrel3::R) reader structure"]
impl crate::Readable for SCTMATCHREL3 {}
#[doc = "`write(|w| ..)` method takes [sctmatchrel3::W](sctmatchrel3::W) writer structure"]
impl crate::Writable for SCTMATCHREL3 {}
#[doc = "SCT match reload value register"]
pub mod sctmatchrel3;
#[doc = "SCT capture control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sctcapctrl4](sctcapctrl4) module"]
pub type SCTCAPCTRL4 = crate::Reg<u32, _SCTCAPCTRL4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SCTCAPCTRL4;
#[doc = "`read()` method returns [sctcapctrl4::R](sctcapctrl4::R) reader structure"]
impl crate::Readable for SCTCAPCTRL4 {}
#[doc = "`write(|w| ..)` method takes [sctcapctrl4::W](sctcapctrl4::W) writer structure"]
impl crate::Writable for SCTCAPCTRL4 {}
#[doc = "SCT capture control register"]
pub mod sctcapctrl4;
#[doc = "SCT match reload value register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sctmatchrel4](sctmatchrel4) module"]
pub type SCTMATCHREL4 = crate::Reg<u32, _SCTMATCHREL4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SCTMATCHREL4;
#[doc = "`read()` method returns [sctmatchrel4::R](sctmatchrel4::R) reader structure"]
impl crate::Readable for SCTMATCHREL4 {}
#[doc = "`write(|w| ..)` method takes [sctmatchrel4::W](sctmatchrel4::W) writer structure"]
impl crate::Writable for SCTMATCHREL4 {}
#[doc = "SCT match reload value register"]
pub mod sctmatchrel4;
#[doc = "SCT capture control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sctcapctrl5](sctcapctrl5) module"]
pub type SCTCAPCTRL5 = crate::Reg<u32, _SCTCAPCTRL5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SCTCAPCTRL5;
#[doc = "`read()` method returns [sctcapctrl5::R](sctcapctrl5::R) reader structure"]
impl crate::Readable for SCTCAPCTRL5 {}
#[doc = "`write(|w| ..)` method takes [sctcapctrl5::W](sctcapctrl5::W) writer structure"]
impl crate::Writable for SCTCAPCTRL5 {}
#[doc = "SCT capture control register"]
pub mod sctcapctrl5;
#[doc = "SCT match reload value register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sctmatchrel5](sctmatchrel5) module"]
pub type SCTMATCHREL5 = crate::Reg<u32, _SCTMATCHREL5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SCTMATCHREL5;
#[doc = "`read()` method returns [sctmatchrel5::R](sctmatchrel5::R) reader structure"]
impl crate::Readable for SCTMATCHREL5 {}
#[doc = "`write(|w| ..)` method takes [sctmatchrel5::W](sctmatchrel5::W) writer structure"]
impl crate::Writable for SCTMATCHREL5 {}
#[doc = "SCT match reload value register"]
pub mod sctmatchrel5;
#[doc = "SCT capture control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sctcapctrl6](sctcapctrl6) module"]
pub type SCTCAPCTRL6 = crate::Reg<u32, _SCTCAPCTRL6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SCTCAPCTRL6;
#[doc = "`read()` method returns [sctcapctrl6::R](sctcapctrl6::R) reader structure"]
impl crate::Readable for SCTCAPCTRL6 {}
#[doc = "`write(|w| ..)` method takes [sctcapctrl6::W](sctcapctrl6::W) writer structure"]
impl crate::Writable for SCTCAPCTRL6 {}
#[doc = "SCT capture control register"]
pub mod sctcapctrl6;
#[doc = "SCT match reload value register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sctmatchrel6](sctmatchrel6) module"]
pub type SCTMATCHREL6 = crate::Reg<u32, _SCTMATCHREL6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SCTMATCHREL6;
#[doc = "`read()` method returns [sctmatchrel6::R](sctmatchrel6::R) reader structure"]
impl crate::Readable for SCTMATCHREL6 {}
#[doc = "`write(|w| ..)` method takes [sctmatchrel6::W](sctmatchrel6::W) writer structure"]
impl crate::Writable for SCTMATCHREL6 {}
#[doc = "SCT match reload value register"]
pub mod sctmatchrel6;
#[doc = "SCT capture control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sctcapctrl7](sctcapctrl7) module"]
pub type SCTCAPCTRL7 = crate::Reg<u32, _SCTCAPCTRL7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SCTCAPCTRL7;
#[doc = "`read()` method returns [sctcapctrl7::R](sctcapctrl7::R) reader structure"]
impl crate::Readable for SCTCAPCTRL7 {}
#[doc = "`write(|w| ..)` method takes [sctcapctrl7::W](sctcapctrl7::W) writer structure"]
impl crate::Writable for SCTCAPCTRL7 {}
#[doc = "SCT capture control register"]
pub mod sctcapctrl7;
#[doc = "SCT match reload value register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sctmatchrel7](sctmatchrel7) module"]
pub type SCTMATCHREL7 = crate::Reg<u32, _SCTMATCHREL7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SCTMATCHREL7;
#[doc = "`read()` method returns [sctmatchrel7::R](sctmatchrel7::R) reader structure"]
impl crate::Readable for SCTMATCHREL7 {}
#[doc = "`write(|w| ..)` method takes [sctmatchrel7::W](sctmatchrel7::W) writer structure"]
impl crate::Writable for SCTMATCHREL7 {}
#[doc = "SCT match reload value register"]
pub mod sctmatchrel7;
