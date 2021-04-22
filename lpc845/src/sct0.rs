#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - SCT configuration register"]
    pub config: crate::Reg<config::CONFIG_SPEC>,
    #[doc = "0x04 - SCT control register"]
    pub ctrl: crate::Reg<ctrl::CTRL_SPEC>,
    #[doc = "0x08 - SCT limit event select register"]
    pub limit: crate::Reg<limit::LIMIT_SPEC>,
    #[doc = "0x0c - SCT halt event select register"]
    pub halt: crate::Reg<halt::HALT_SPEC>,
    #[doc = "0x10 - SCT stop event select register"]
    pub stop: crate::Reg<stop::STOP_SPEC>,
    #[doc = "0x14 - SCT start event select register"]
    pub start: crate::Reg<start::START_SPEC>,
    _reserved6: [u8; 40usize],
    #[doc = "0x40 - SCT counter register"]
    pub count: crate::Reg<count::COUNT_SPEC>,
    #[doc = "0x44 - SCT state register"]
    pub state: crate::Reg<state::STATE_SPEC>,
    #[doc = "0x48 - SCT input register"]
    pub input: crate::Reg<input::INPUT_SPEC>,
    #[doc = "0x4c - SCT match/capture mode register"]
    pub regmode: crate::Reg<regmode::REGMODE_SPEC>,
    #[doc = "0x50 - SCT output register"]
    pub output: crate::Reg<output::OUTPUT_SPEC>,
    #[doc = "0x54 - SCT output counter direction control register"]
    pub outputdirctrl: crate::Reg<outputdirctrl::OUTPUTDIRCTRL_SPEC>,
    #[doc = "0x58 - SCT conflict resolution register"]
    pub res: crate::Reg<res::RES_SPEC>,
    #[doc = "0x5c - SCT DMA request 0 register"]
    pub dma0request: crate::Reg<dma0request::DMA0REQUEST_SPEC>,
    #[doc = "0x60 - SCT DMA request 1 register"]
    pub dma1request: crate::Reg<dma1request::DMA1REQUEST_SPEC>,
    _reserved15: [u8; 140usize],
    #[doc = "0xf0 - SCT event interrupt enable register"]
    pub even: crate::Reg<even::EVEN_SPEC>,
    #[doc = "0xf4 - SCT event flag register"]
    pub evflag: crate::Reg<evflag::EVFLAG_SPEC>,
    #[doc = "0xf8 - SCT conflict interrupt enable register"]
    pub conen: crate::Reg<conen::CONEN_SPEC>,
    #[doc = "0xfc - SCT conflict flag register"]
    pub conflag: crate::Reg<conflag::CONFLAG_SPEC>,
    _reserved_19_cap_match: [u8; 32usize],
    _reserved20: [u8; 224usize],
    _reserved_20_capctrl_matchrel: [u8; 32usize],
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
    pub fn cap_match_sctmatch(
        &self,
    ) -> &[crate::Reg<cap_match_sctmatch::CAP_MATCH_SCTMATCH_SPEC>; 8] {
        unsafe {
            &*(((self as *const Self) as *const u8).add(256usize)
                as *const [crate::Reg<cap_match_sctmatch::CAP_MATCH_SCTMATCH_SPEC>; 8])
        }
    }
    #[doc = "0x100 - SCT capture register of capture channel"]
    #[inline(always)]
    pub fn cap_match_sctcap(&self) -> &[crate::Reg<cap_match_sctcap::CAP_MATCH_SCTCAP_SPEC>; 8] {
        unsafe {
            &*(((self as *const Self) as *const u8).add(256usize)
                as *const [crate::Reg<cap_match_sctcap::CAP_MATCH_SCTCAP_SPEC>; 8])
        }
    }
    #[doc = "0x200 - SCT match reload value register"]
    #[inline(always)]
    pub fn capctrl_matchrel_sctmatchrel(
        &self,
    ) -> &[crate::Reg<capctrl_matchrel_sctmatchrel::CAPCTRL_MATCHREL_SCTMATCHREL_SPEC>; 8] {
        unsafe {
            &*(((self as *const Self) as *const u8).add(512usize)
                as *const [crate::Reg<
                    capctrl_matchrel_sctmatchrel::CAPCTRL_MATCHREL_SCTMATCHREL_SPEC,
                >; 8])
        }
    }
    #[doc = "0x200 - SCT capture control register"]
    #[inline(always)]
    pub fn capctrl_matchrel_sctcapctrl(
        &self,
    ) -> &[crate::Reg<capctrl_matchrel_sctcapctrl::CAPCTRL_MATCHREL_SCTCAPCTRL_SPEC>; 8] {
        unsafe {
            &*(((self as *const Self) as *const u8).add(512usize)
                as *const [crate::Reg<
                    capctrl_matchrel_sctcapctrl::CAPCTRL_MATCHREL_SCTCAPCTRL_SPEC,
                >; 8])
        }
    }
}
#[doc = r"Register block"]
#[repr(C)]
pub struct EVENT {
    #[doc = "0x00 - SCT event state register 0"]
    pub state: crate::Reg<self::event::state::STATE_SPEC>,
    #[doc = "0x04 - SCT event control register 0"]
    pub ctrl: crate::Reg<self::event::ctrl::CTRL_SPEC>,
}
#[doc = r"Register block"]
#[doc = "no description available"]
pub mod event;
#[doc = r"Register block"]
#[repr(C)]
pub struct OUT {
    #[doc = "0x00 - SCT output 0 set register"]
    pub set: crate::Reg<self::out::set::SET_SPEC>,
    #[doc = "0x04 - SCT output 0 clear register"]
    pub clr: crate::Reg<self::out::clr::CLR_SPEC>,
}
#[doc = r"Register block"]
#[doc = "no description available"]
pub mod out;
#[doc = "CONFIG register accessor: an alias for `Reg<CONFIG_SPEC>`"]
pub type CONFIG = crate::Reg<config::CONFIG_SPEC>;
#[doc = "SCT configuration register"]
pub mod config;
#[doc = "CTRL register accessor: an alias for `Reg<CTRL_SPEC>`"]
pub type CTRL = crate::Reg<ctrl::CTRL_SPEC>;
#[doc = "SCT control register"]
pub mod ctrl;
#[doc = "LIMIT register accessor: an alias for `Reg<LIMIT_SPEC>`"]
pub type LIMIT = crate::Reg<limit::LIMIT_SPEC>;
#[doc = "SCT limit event select register"]
pub mod limit;
#[doc = "HALT register accessor: an alias for `Reg<HALT_SPEC>`"]
pub type HALT = crate::Reg<halt::HALT_SPEC>;
#[doc = "SCT halt event select register"]
pub mod halt;
#[doc = "STOP register accessor: an alias for `Reg<STOP_SPEC>`"]
pub type STOP = crate::Reg<stop::STOP_SPEC>;
#[doc = "SCT stop event select register"]
pub mod stop;
#[doc = "START register accessor: an alias for `Reg<START_SPEC>`"]
pub type START = crate::Reg<start::START_SPEC>;
#[doc = "SCT start event select register"]
pub mod start;
#[doc = "COUNT register accessor: an alias for `Reg<COUNT_SPEC>`"]
pub type COUNT = crate::Reg<count::COUNT_SPEC>;
#[doc = "SCT counter register"]
pub mod count;
#[doc = "STATE register accessor: an alias for `Reg<STATE_SPEC>`"]
pub type STATE = crate::Reg<state::STATE_SPEC>;
#[doc = "SCT state register"]
pub mod state;
#[doc = "INPUT register accessor: an alias for `Reg<INPUT_SPEC>`"]
pub type INPUT = crate::Reg<input::INPUT_SPEC>;
#[doc = "SCT input register"]
pub mod input;
#[doc = "REGMODE register accessor: an alias for `Reg<REGMODE_SPEC>`"]
pub type REGMODE = crate::Reg<regmode::REGMODE_SPEC>;
#[doc = "SCT match/capture mode register"]
pub mod regmode;
#[doc = "OUTPUT register accessor: an alias for `Reg<OUTPUT_SPEC>`"]
pub type OUTPUT = crate::Reg<output::OUTPUT_SPEC>;
#[doc = "SCT output register"]
pub mod output;
#[doc = "OUTPUTDIRCTRL register accessor: an alias for `Reg<OUTPUTDIRCTRL_SPEC>`"]
pub type OUTPUTDIRCTRL = crate::Reg<outputdirctrl::OUTPUTDIRCTRL_SPEC>;
#[doc = "SCT output counter direction control register"]
pub mod outputdirctrl;
#[doc = "RES register accessor: an alias for `Reg<RES_SPEC>`"]
pub type RES = crate::Reg<res::RES_SPEC>;
#[doc = "SCT conflict resolution register"]
pub mod res;
#[doc = "DMA0REQUEST register accessor: an alias for `Reg<DMA0REQUEST_SPEC>`"]
pub type DMA0REQUEST = crate::Reg<dma0request::DMA0REQUEST_SPEC>;
#[doc = "SCT DMA request 0 register"]
pub mod dma0request;
#[doc = "DMA1REQUEST register accessor: an alias for `Reg<DMA1REQUEST_SPEC>`"]
pub type DMA1REQUEST = crate::Reg<dma1request::DMA1REQUEST_SPEC>;
#[doc = "SCT DMA request 1 register"]
pub mod dma1request;
#[doc = "EVEN register accessor: an alias for `Reg<EVEN_SPEC>`"]
pub type EVEN = crate::Reg<even::EVEN_SPEC>;
#[doc = "SCT event interrupt enable register"]
pub mod even;
#[doc = "EVFLAG register accessor: an alias for `Reg<EVFLAG_SPEC>`"]
pub type EVFLAG = crate::Reg<evflag::EVFLAG_SPEC>;
#[doc = "SCT event flag register"]
pub mod evflag;
#[doc = "CONEN register accessor: an alias for `Reg<CONEN_SPEC>`"]
pub type CONEN = crate::Reg<conen::CONEN_SPEC>;
#[doc = "SCT conflict interrupt enable register"]
pub mod conen;
#[doc = "CONFLAG register accessor: an alias for `Reg<CONFLAG_SPEC>`"]
pub type CONFLAG = crate::Reg<conflag::CONFLAG_SPEC>;
#[doc = "SCT conflict flag register"]
pub mod conflag;
#[doc = "CAP_MATCH_SCTCAP register accessor: an alias for `Reg<CAP_MATCH_SCTCAP_SPEC>`"]
pub type CAP_MATCH_SCTCAP = crate::Reg<cap_match_sctcap::CAP_MATCH_SCTCAP_SPEC>;
#[doc = "SCT capture register of capture channel"]
pub mod cap_match_sctcap;
#[doc = "CAP_MATCH_SCTMATCH register accessor: an alias for `Reg<CAP_MATCH_SCTMATCH_SPEC>`"]
pub type CAP_MATCH_SCTMATCH = crate::Reg<cap_match_sctmatch::CAP_MATCH_SCTMATCH_SPEC>;
#[doc = "SCT match value register of match channels"]
pub mod cap_match_sctmatch;
#[doc = "CAPCTRL_MATCHREL_SCTCAPCTRL register accessor: an alias for `Reg<CAPCTRL_MATCHREL_SCTCAPCTRL_SPEC>`"]
pub type CAPCTRL_MATCHREL_SCTCAPCTRL =
    crate::Reg<capctrl_matchrel_sctcapctrl::CAPCTRL_MATCHREL_SCTCAPCTRL_SPEC>;
#[doc = "SCT capture control register"]
pub mod capctrl_matchrel_sctcapctrl;
#[doc = "CAPCTRL_MATCHREL_SCTMATCHREL register accessor: an alias for `Reg<CAPCTRL_MATCHREL_SCTMATCHREL_SPEC>`"]
pub type CAPCTRL_MATCHREL_SCTMATCHREL =
    crate::Reg<capctrl_matchrel_sctmatchrel::CAPCTRL_MATCHREL_SCTMATCHREL_SPEC>;
#[doc = "SCT match reload value register"]
pub mod capctrl_matchrel_sctmatchrel;
