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
    _reserved6: [u8; 0x28],
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
    _reserved15: [u8; 0x8c],
    #[doc = "0xf0 - SCT event interrupt enable register"]
    pub even: crate::Reg<even::EVEN_SPEC>,
    #[doc = "0xf4 - SCT event flag register"]
    pub evflag: crate::Reg<evflag::EVFLAG_SPEC>,
    #[doc = "0xf8 - SCT conflict interrupt enable register"]
    pub conen: crate::Reg<conen::CONEN_SPEC>,
    #[doc = "0xfc - SCT conflict flag register"]
    pub conflag: crate::Reg<conflag::CONFLAG_SPEC>,
    _reserved_19_cap_match: [u8; 0x04],
    _reserved_20_cap_match: [u8; 0x04],
    _reserved_21_cap_match: [u8; 0x04],
    _reserved_22_cap_match: [u8; 0x04],
    _reserved_23_cap_match: [u8; 0x04],
    _reserved_24_cap_match: [u8; 0x04],
    _reserved_25_cap_match: [u8; 0x04],
    _reserved_26_cap_match: [u8; 0x04],
    _reserved27: [u8; 0xe0],
    _reserved_27_capctrl_matchrel: [u8; 0x04],
    _reserved_28_capctrl_matchrel: [u8; 0x04],
    _reserved_29_capctrl_matchrel: [u8; 0x04],
    _reserved_30_capctrl_matchrel: [u8; 0x04],
    _reserved_31_capctrl_matchrel: [u8; 0x04],
    _reserved_32_capctrl_matchrel: [u8; 0x04],
    _reserved_33_capctrl_matchrel: [u8; 0x04],
    _reserved_34_capctrl_matchrel: [u8; 0x04],
    _reserved35: [u8; 0xe0],
    #[doc = "0x300..0x340 - no description available"]
    pub event: [EVENT; 8],
    _reserved36: [u8; 0x01c0],
    #[doc = "0x500..0x530 - no description available"]
    pub out: [OUT; 6],
}
impl RegisterBlock {
    #[doc = "0x100 - SCT match value register of match channels"]
    #[inline(always)]
    pub fn cap_match_sctmatch0(
        &self,
    ) -> &crate::Reg<cap_match_sctmatch0::CAP_MATCH_SCTMATCH0_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(256usize)
                as *const crate::Reg<cap_match_sctmatch0::CAP_MATCH_SCTMATCH0_SPEC>)
        }
    }
    #[doc = "0x100 - SCT capture register of capture channel"]
    #[inline(always)]
    pub fn cap_match_sctcap0(&self) -> &crate::Reg<cap_match_sctcap0::CAP_MATCH_SCTCAP0_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(256usize)
                as *const crate::Reg<cap_match_sctcap0::CAP_MATCH_SCTCAP0_SPEC>)
        }
    }
    #[doc = "0x104 - SCT match value register of match channels"]
    #[inline(always)]
    pub fn cap_match_sctmatch1(
        &self,
    ) -> &crate::Reg<cap_match_sctmatch1::CAP_MATCH_SCTMATCH1_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(260usize)
                as *const crate::Reg<cap_match_sctmatch1::CAP_MATCH_SCTMATCH1_SPEC>)
        }
    }
    #[doc = "0x104 - SCT capture register of capture channel"]
    #[inline(always)]
    pub fn cap_match_sctcap1(&self) -> &crate::Reg<cap_match_sctcap1::CAP_MATCH_SCTCAP1_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(260usize)
                as *const crate::Reg<cap_match_sctcap1::CAP_MATCH_SCTCAP1_SPEC>)
        }
    }
    #[doc = "0x108 - SCT match value register of match channels"]
    #[inline(always)]
    pub fn cap_match_sctmatch2(
        &self,
    ) -> &crate::Reg<cap_match_sctmatch2::CAP_MATCH_SCTMATCH2_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(264usize)
                as *const crate::Reg<cap_match_sctmatch2::CAP_MATCH_SCTMATCH2_SPEC>)
        }
    }
    #[doc = "0x108 - SCT capture register of capture channel"]
    #[inline(always)]
    pub fn cap_match_sctcap2(&self) -> &crate::Reg<cap_match_sctcap2::CAP_MATCH_SCTCAP2_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(264usize)
                as *const crate::Reg<cap_match_sctcap2::CAP_MATCH_SCTCAP2_SPEC>)
        }
    }
    #[doc = "0x10c - SCT match value register of match channels"]
    #[inline(always)]
    pub fn cap_match_sctmatch3(
        &self,
    ) -> &crate::Reg<cap_match_sctmatch3::CAP_MATCH_SCTMATCH3_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(268usize)
                as *const crate::Reg<cap_match_sctmatch3::CAP_MATCH_SCTMATCH3_SPEC>)
        }
    }
    #[doc = "0x10c - SCT capture register of capture channel"]
    #[inline(always)]
    pub fn cap_match_sctcap3(&self) -> &crate::Reg<cap_match_sctcap3::CAP_MATCH_SCTCAP3_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(268usize)
                as *const crate::Reg<cap_match_sctcap3::CAP_MATCH_SCTCAP3_SPEC>)
        }
    }
    #[doc = "0x110 - SCT match value register of match channels"]
    #[inline(always)]
    pub fn cap_match_sctmatch4(
        &self,
    ) -> &crate::Reg<cap_match_sctmatch4::CAP_MATCH_SCTMATCH4_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(272usize)
                as *const crate::Reg<cap_match_sctmatch4::CAP_MATCH_SCTMATCH4_SPEC>)
        }
    }
    #[doc = "0x110 - SCT capture register of capture channel"]
    #[inline(always)]
    pub fn cap_match_sctcap4(&self) -> &crate::Reg<cap_match_sctcap4::CAP_MATCH_SCTCAP4_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(272usize)
                as *const crate::Reg<cap_match_sctcap4::CAP_MATCH_SCTCAP4_SPEC>)
        }
    }
    #[doc = "0x114 - SCT match value register of match channels"]
    #[inline(always)]
    pub fn cap_match_sctmatch5(
        &self,
    ) -> &crate::Reg<cap_match_sctmatch5::CAP_MATCH_SCTMATCH5_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(276usize)
                as *const crate::Reg<cap_match_sctmatch5::CAP_MATCH_SCTMATCH5_SPEC>)
        }
    }
    #[doc = "0x114 - SCT capture register of capture channel"]
    #[inline(always)]
    pub fn cap_match_sctcap5(&self) -> &crate::Reg<cap_match_sctcap5::CAP_MATCH_SCTCAP5_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(276usize)
                as *const crate::Reg<cap_match_sctcap5::CAP_MATCH_SCTCAP5_SPEC>)
        }
    }
    #[doc = "0x118 - SCT match value register of match channels"]
    #[inline(always)]
    pub fn cap_match_sctmatch6(
        &self,
    ) -> &crate::Reg<cap_match_sctmatch6::CAP_MATCH_SCTMATCH6_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(280usize)
                as *const crate::Reg<cap_match_sctmatch6::CAP_MATCH_SCTMATCH6_SPEC>)
        }
    }
    #[doc = "0x118 - SCT capture register of capture channel"]
    #[inline(always)]
    pub fn cap_match_sctcap6(&self) -> &crate::Reg<cap_match_sctcap6::CAP_MATCH_SCTCAP6_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(280usize)
                as *const crate::Reg<cap_match_sctcap6::CAP_MATCH_SCTCAP6_SPEC>)
        }
    }
    #[doc = "0x11c - SCT match value register of match channels"]
    #[inline(always)]
    pub fn cap_match_sctmatch7(
        &self,
    ) -> &crate::Reg<cap_match_sctmatch7::CAP_MATCH_SCTMATCH7_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(284usize)
                as *const crate::Reg<cap_match_sctmatch7::CAP_MATCH_SCTMATCH7_SPEC>)
        }
    }
    #[doc = "0x11c - SCT capture register of capture channel"]
    #[inline(always)]
    pub fn cap_match_sctcap7(&self) -> &crate::Reg<cap_match_sctcap7::CAP_MATCH_SCTCAP7_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(284usize)
                as *const crate::Reg<cap_match_sctcap7::CAP_MATCH_SCTCAP7_SPEC>)
        }
    }
    #[doc = "0x200 - SCT match reload value register"]
    #[inline(always)]
    pub fn capctrl_matchrel_sctmatchrel0(
        &self,
    ) -> &crate::Reg<capctrl_matchrel_sctmatchrel0::CAPCTRL_MATCHREL_SCTMATCHREL0_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(512usize)
                as *const crate::Reg<
                    capctrl_matchrel_sctmatchrel0::CAPCTRL_MATCHREL_SCTMATCHREL0_SPEC,
                >)
        }
    }
    #[doc = "0x200 - SCT capture control register"]
    #[inline(always)]
    pub fn capctrl_matchrel_sctcapctrl0(
        &self,
    ) -> &crate::Reg<capctrl_matchrel_sctcapctrl0::CAPCTRL_MATCHREL_SCTCAPCTRL0_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(512usize)
                as *const crate::Reg<
                    capctrl_matchrel_sctcapctrl0::CAPCTRL_MATCHREL_SCTCAPCTRL0_SPEC,
                >)
        }
    }
    #[doc = "0x204 - SCT match reload value register"]
    #[inline(always)]
    pub fn capctrl_matchrel_sctmatchrel1(
        &self,
    ) -> &crate::Reg<capctrl_matchrel_sctmatchrel1::CAPCTRL_MATCHREL_SCTMATCHREL1_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(516usize)
                as *const crate::Reg<
                    capctrl_matchrel_sctmatchrel1::CAPCTRL_MATCHREL_SCTMATCHREL1_SPEC,
                >)
        }
    }
    #[doc = "0x204 - SCT capture control register"]
    #[inline(always)]
    pub fn capctrl_matchrel_sctcapctrl1(
        &self,
    ) -> &crate::Reg<capctrl_matchrel_sctcapctrl1::CAPCTRL_MATCHREL_SCTCAPCTRL1_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(516usize)
                as *const crate::Reg<
                    capctrl_matchrel_sctcapctrl1::CAPCTRL_MATCHREL_SCTCAPCTRL1_SPEC,
                >)
        }
    }
    #[doc = "0x208 - SCT match reload value register"]
    #[inline(always)]
    pub fn capctrl_matchrel_sctmatchrel2(
        &self,
    ) -> &crate::Reg<capctrl_matchrel_sctmatchrel2::CAPCTRL_MATCHREL_SCTMATCHREL2_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(520usize)
                as *const crate::Reg<
                    capctrl_matchrel_sctmatchrel2::CAPCTRL_MATCHREL_SCTMATCHREL2_SPEC,
                >)
        }
    }
    #[doc = "0x208 - SCT capture control register"]
    #[inline(always)]
    pub fn capctrl_matchrel_sctcapctrl2(
        &self,
    ) -> &crate::Reg<capctrl_matchrel_sctcapctrl2::CAPCTRL_MATCHREL_SCTCAPCTRL2_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(520usize)
                as *const crate::Reg<
                    capctrl_matchrel_sctcapctrl2::CAPCTRL_MATCHREL_SCTCAPCTRL2_SPEC,
                >)
        }
    }
    #[doc = "0x20c - SCT match reload value register"]
    #[inline(always)]
    pub fn capctrl_matchrel_sctmatchrel3(
        &self,
    ) -> &crate::Reg<capctrl_matchrel_sctmatchrel3::CAPCTRL_MATCHREL_SCTMATCHREL3_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(524usize)
                as *const crate::Reg<
                    capctrl_matchrel_sctmatchrel3::CAPCTRL_MATCHREL_SCTMATCHREL3_SPEC,
                >)
        }
    }
    #[doc = "0x20c - SCT capture control register"]
    #[inline(always)]
    pub fn capctrl_matchrel_sctcapctrl3(
        &self,
    ) -> &crate::Reg<capctrl_matchrel_sctcapctrl3::CAPCTRL_MATCHREL_SCTCAPCTRL3_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(524usize)
                as *const crate::Reg<
                    capctrl_matchrel_sctcapctrl3::CAPCTRL_MATCHREL_SCTCAPCTRL3_SPEC,
                >)
        }
    }
    #[doc = "0x210 - SCT match reload value register"]
    #[inline(always)]
    pub fn capctrl_matchrel_sctmatchrel4(
        &self,
    ) -> &crate::Reg<capctrl_matchrel_sctmatchrel4::CAPCTRL_MATCHREL_SCTMATCHREL4_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(528usize)
                as *const crate::Reg<
                    capctrl_matchrel_sctmatchrel4::CAPCTRL_MATCHREL_SCTMATCHREL4_SPEC,
                >)
        }
    }
    #[doc = "0x210 - SCT capture control register"]
    #[inline(always)]
    pub fn capctrl_matchrel_sctcapctrl4(
        &self,
    ) -> &crate::Reg<capctrl_matchrel_sctcapctrl4::CAPCTRL_MATCHREL_SCTCAPCTRL4_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(528usize)
                as *const crate::Reg<
                    capctrl_matchrel_sctcapctrl4::CAPCTRL_MATCHREL_SCTCAPCTRL4_SPEC,
                >)
        }
    }
    #[doc = "0x214 - SCT match reload value register"]
    #[inline(always)]
    pub fn capctrl_matchrel_sctmatchrel5(
        &self,
    ) -> &crate::Reg<capctrl_matchrel_sctmatchrel5::CAPCTRL_MATCHREL_SCTMATCHREL5_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(532usize)
                as *const crate::Reg<
                    capctrl_matchrel_sctmatchrel5::CAPCTRL_MATCHREL_SCTMATCHREL5_SPEC,
                >)
        }
    }
    #[doc = "0x214 - SCT capture control register"]
    #[inline(always)]
    pub fn capctrl_matchrel_sctcapctrl5(
        &self,
    ) -> &crate::Reg<capctrl_matchrel_sctcapctrl5::CAPCTRL_MATCHREL_SCTCAPCTRL5_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(532usize)
                as *const crate::Reg<
                    capctrl_matchrel_sctcapctrl5::CAPCTRL_MATCHREL_SCTCAPCTRL5_SPEC,
                >)
        }
    }
    #[doc = "0x218 - SCT match reload value register"]
    #[inline(always)]
    pub fn capctrl_matchrel_sctmatchrel6(
        &self,
    ) -> &crate::Reg<capctrl_matchrel_sctmatchrel6::CAPCTRL_MATCHREL_SCTMATCHREL6_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(536usize)
                as *const crate::Reg<
                    capctrl_matchrel_sctmatchrel6::CAPCTRL_MATCHREL_SCTMATCHREL6_SPEC,
                >)
        }
    }
    #[doc = "0x218 - SCT capture control register"]
    #[inline(always)]
    pub fn capctrl_matchrel_sctcapctrl6(
        &self,
    ) -> &crate::Reg<capctrl_matchrel_sctcapctrl6::CAPCTRL_MATCHREL_SCTCAPCTRL6_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(536usize)
                as *const crate::Reg<
                    capctrl_matchrel_sctcapctrl6::CAPCTRL_MATCHREL_SCTCAPCTRL6_SPEC,
                >)
        }
    }
    #[doc = "0x21c - SCT match reload value register"]
    #[inline(always)]
    pub fn capctrl_matchrel_sctmatchrel7(
        &self,
    ) -> &crate::Reg<capctrl_matchrel_sctmatchrel7::CAPCTRL_MATCHREL_SCTMATCHREL7_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(540usize)
                as *const crate::Reg<
                    capctrl_matchrel_sctmatchrel7::CAPCTRL_MATCHREL_SCTMATCHREL7_SPEC,
                >)
        }
    }
    #[doc = "0x21c - SCT capture control register"]
    #[inline(always)]
    pub fn capctrl_matchrel_sctcapctrl7(
        &self,
    ) -> &crate::Reg<capctrl_matchrel_sctcapctrl7::CAPCTRL_MATCHREL_SCTCAPCTRL7_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(540usize)
                as *const crate::Reg<
                    capctrl_matchrel_sctcapctrl7::CAPCTRL_MATCHREL_SCTCAPCTRL7_SPEC,
                >)
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
#[doc = "CAP_MATCH_SCTCAP0 register accessor: an alias for `Reg<CAP_MATCH_SCTCAP0_SPEC>`"]
pub type CAP_MATCH_SCTCAP0 = crate::Reg<cap_match_sctcap0::CAP_MATCH_SCTCAP0_SPEC>;
#[doc = "SCT capture register of capture channel"]
pub mod cap_match_sctcap0;
#[doc = "CAP_MATCH_SCTMATCH0 register accessor: an alias for `Reg<CAP_MATCH_SCTMATCH0_SPEC>`"]
pub type CAP_MATCH_SCTMATCH0 = crate::Reg<cap_match_sctmatch0::CAP_MATCH_SCTMATCH0_SPEC>;
#[doc = "SCT match value register of match channels"]
pub mod cap_match_sctmatch0;
#[doc = "CAP_MATCH_SCTCAP1 register accessor: an alias for `Reg<CAP_MATCH_SCTCAP1_SPEC>`"]
pub type CAP_MATCH_SCTCAP1 = crate::Reg<cap_match_sctcap1::CAP_MATCH_SCTCAP1_SPEC>;
#[doc = "SCT capture register of capture channel"]
pub mod cap_match_sctcap1;
#[doc = "CAP_MATCH_SCTMATCH1 register accessor: an alias for `Reg<CAP_MATCH_SCTMATCH1_SPEC>`"]
pub type CAP_MATCH_SCTMATCH1 = crate::Reg<cap_match_sctmatch1::CAP_MATCH_SCTMATCH1_SPEC>;
#[doc = "SCT match value register of match channels"]
pub mod cap_match_sctmatch1;
#[doc = "CAP_MATCH_SCTCAP2 register accessor: an alias for `Reg<CAP_MATCH_SCTCAP2_SPEC>`"]
pub type CAP_MATCH_SCTCAP2 = crate::Reg<cap_match_sctcap2::CAP_MATCH_SCTCAP2_SPEC>;
#[doc = "SCT capture register of capture channel"]
pub mod cap_match_sctcap2;
#[doc = "CAP_MATCH_SCTMATCH2 register accessor: an alias for `Reg<CAP_MATCH_SCTMATCH2_SPEC>`"]
pub type CAP_MATCH_SCTMATCH2 = crate::Reg<cap_match_sctmatch2::CAP_MATCH_SCTMATCH2_SPEC>;
#[doc = "SCT match value register of match channels"]
pub mod cap_match_sctmatch2;
#[doc = "CAP_MATCH_SCTCAP3 register accessor: an alias for `Reg<CAP_MATCH_SCTCAP3_SPEC>`"]
pub type CAP_MATCH_SCTCAP3 = crate::Reg<cap_match_sctcap3::CAP_MATCH_SCTCAP3_SPEC>;
#[doc = "SCT capture register of capture channel"]
pub mod cap_match_sctcap3;
#[doc = "CAP_MATCH_SCTMATCH3 register accessor: an alias for `Reg<CAP_MATCH_SCTMATCH3_SPEC>`"]
pub type CAP_MATCH_SCTMATCH3 = crate::Reg<cap_match_sctmatch3::CAP_MATCH_SCTMATCH3_SPEC>;
#[doc = "SCT match value register of match channels"]
pub mod cap_match_sctmatch3;
#[doc = "CAP_MATCH_SCTCAP4 register accessor: an alias for `Reg<CAP_MATCH_SCTCAP4_SPEC>`"]
pub type CAP_MATCH_SCTCAP4 = crate::Reg<cap_match_sctcap4::CAP_MATCH_SCTCAP4_SPEC>;
#[doc = "SCT capture register of capture channel"]
pub mod cap_match_sctcap4;
#[doc = "CAP_MATCH_SCTMATCH4 register accessor: an alias for `Reg<CAP_MATCH_SCTMATCH4_SPEC>`"]
pub type CAP_MATCH_SCTMATCH4 = crate::Reg<cap_match_sctmatch4::CAP_MATCH_SCTMATCH4_SPEC>;
#[doc = "SCT match value register of match channels"]
pub mod cap_match_sctmatch4;
#[doc = "CAP_MATCH_SCTCAP5 register accessor: an alias for `Reg<CAP_MATCH_SCTCAP5_SPEC>`"]
pub type CAP_MATCH_SCTCAP5 = crate::Reg<cap_match_sctcap5::CAP_MATCH_SCTCAP5_SPEC>;
#[doc = "SCT capture register of capture channel"]
pub mod cap_match_sctcap5;
#[doc = "CAP_MATCH_SCTMATCH5 register accessor: an alias for `Reg<CAP_MATCH_SCTMATCH5_SPEC>`"]
pub type CAP_MATCH_SCTMATCH5 = crate::Reg<cap_match_sctmatch5::CAP_MATCH_SCTMATCH5_SPEC>;
#[doc = "SCT match value register of match channels"]
pub mod cap_match_sctmatch5;
#[doc = "CAP_MATCH_SCTCAP6 register accessor: an alias for `Reg<CAP_MATCH_SCTCAP6_SPEC>`"]
pub type CAP_MATCH_SCTCAP6 = crate::Reg<cap_match_sctcap6::CAP_MATCH_SCTCAP6_SPEC>;
#[doc = "SCT capture register of capture channel"]
pub mod cap_match_sctcap6;
#[doc = "CAP_MATCH_SCTMATCH6 register accessor: an alias for `Reg<CAP_MATCH_SCTMATCH6_SPEC>`"]
pub type CAP_MATCH_SCTMATCH6 = crate::Reg<cap_match_sctmatch6::CAP_MATCH_SCTMATCH6_SPEC>;
#[doc = "SCT match value register of match channels"]
pub mod cap_match_sctmatch6;
#[doc = "CAP_MATCH_SCTCAP7 register accessor: an alias for `Reg<CAP_MATCH_SCTCAP7_SPEC>`"]
pub type CAP_MATCH_SCTCAP7 = crate::Reg<cap_match_sctcap7::CAP_MATCH_SCTCAP7_SPEC>;
#[doc = "SCT capture register of capture channel"]
pub mod cap_match_sctcap7;
#[doc = "CAP_MATCH_SCTMATCH7 register accessor: an alias for `Reg<CAP_MATCH_SCTMATCH7_SPEC>`"]
pub type CAP_MATCH_SCTMATCH7 = crate::Reg<cap_match_sctmatch7::CAP_MATCH_SCTMATCH7_SPEC>;
#[doc = "SCT match value register of match channels"]
pub mod cap_match_sctmatch7;
#[doc = "CAPCTRL_MATCHREL_SCTCAPCTRL0 register accessor: an alias for `Reg<CAPCTRL_MATCHREL_SCTCAPCTRL0_SPEC>`"]
pub type CAPCTRL_MATCHREL_SCTCAPCTRL0 =
    crate::Reg<capctrl_matchrel_sctcapctrl0::CAPCTRL_MATCHREL_SCTCAPCTRL0_SPEC>;
#[doc = "SCT capture control register"]
pub mod capctrl_matchrel_sctcapctrl0;
#[doc = "CAPCTRL_MATCHREL_SCTMATCHREL0 register accessor: an alias for `Reg<CAPCTRL_MATCHREL_SCTMATCHREL0_SPEC>`"]
pub type CAPCTRL_MATCHREL_SCTMATCHREL0 =
    crate::Reg<capctrl_matchrel_sctmatchrel0::CAPCTRL_MATCHREL_SCTMATCHREL0_SPEC>;
#[doc = "SCT match reload value register"]
pub mod capctrl_matchrel_sctmatchrel0;
#[doc = "CAPCTRL_MATCHREL_SCTCAPCTRL1 register accessor: an alias for `Reg<CAPCTRL_MATCHREL_SCTCAPCTRL1_SPEC>`"]
pub type CAPCTRL_MATCHREL_SCTCAPCTRL1 =
    crate::Reg<capctrl_matchrel_sctcapctrl1::CAPCTRL_MATCHREL_SCTCAPCTRL1_SPEC>;
#[doc = "SCT capture control register"]
pub mod capctrl_matchrel_sctcapctrl1;
#[doc = "CAPCTRL_MATCHREL_SCTMATCHREL1 register accessor: an alias for `Reg<CAPCTRL_MATCHREL_SCTMATCHREL1_SPEC>`"]
pub type CAPCTRL_MATCHREL_SCTMATCHREL1 =
    crate::Reg<capctrl_matchrel_sctmatchrel1::CAPCTRL_MATCHREL_SCTMATCHREL1_SPEC>;
#[doc = "SCT match reload value register"]
pub mod capctrl_matchrel_sctmatchrel1;
#[doc = "CAPCTRL_MATCHREL_SCTCAPCTRL2 register accessor: an alias for `Reg<CAPCTRL_MATCHREL_SCTCAPCTRL2_SPEC>`"]
pub type CAPCTRL_MATCHREL_SCTCAPCTRL2 =
    crate::Reg<capctrl_matchrel_sctcapctrl2::CAPCTRL_MATCHREL_SCTCAPCTRL2_SPEC>;
#[doc = "SCT capture control register"]
pub mod capctrl_matchrel_sctcapctrl2;
#[doc = "CAPCTRL_MATCHREL_SCTMATCHREL2 register accessor: an alias for `Reg<CAPCTRL_MATCHREL_SCTMATCHREL2_SPEC>`"]
pub type CAPCTRL_MATCHREL_SCTMATCHREL2 =
    crate::Reg<capctrl_matchrel_sctmatchrel2::CAPCTRL_MATCHREL_SCTMATCHREL2_SPEC>;
#[doc = "SCT match reload value register"]
pub mod capctrl_matchrel_sctmatchrel2;
#[doc = "CAPCTRL_MATCHREL_SCTCAPCTRL3 register accessor: an alias for `Reg<CAPCTRL_MATCHREL_SCTCAPCTRL3_SPEC>`"]
pub type CAPCTRL_MATCHREL_SCTCAPCTRL3 =
    crate::Reg<capctrl_matchrel_sctcapctrl3::CAPCTRL_MATCHREL_SCTCAPCTRL3_SPEC>;
#[doc = "SCT capture control register"]
pub mod capctrl_matchrel_sctcapctrl3;
#[doc = "CAPCTRL_MATCHREL_SCTMATCHREL3 register accessor: an alias for `Reg<CAPCTRL_MATCHREL_SCTMATCHREL3_SPEC>`"]
pub type CAPCTRL_MATCHREL_SCTMATCHREL3 =
    crate::Reg<capctrl_matchrel_sctmatchrel3::CAPCTRL_MATCHREL_SCTMATCHREL3_SPEC>;
#[doc = "SCT match reload value register"]
pub mod capctrl_matchrel_sctmatchrel3;
#[doc = "CAPCTRL_MATCHREL_SCTCAPCTRL4 register accessor: an alias for `Reg<CAPCTRL_MATCHREL_SCTCAPCTRL4_SPEC>`"]
pub type CAPCTRL_MATCHREL_SCTCAPCTRL4 =
    crate::Reg<capctrl_matchrel_sctcapctrl4::CAPCTRL_MATCHREL_SCTCAPCTRL4_SPEC>;
#[doc = "SCT capture control register"]
pub mod capctrl_matchrel_sctcapctrl4;
#[doc = "CAPCTRL_MATCHREL_SCTMATCHREL4 register accessor: an alias for `Reg<CAPCTRL_MATCHREL_SCTMATCHREL4_SPEC>`"]
pub type CAPCTRL_MATCHREL_SCTMATCHREL4 =
    crate::Reg<capctrl_matchrel_sctmatchrel4::CAPCTRL_MATCHREL_SCTMATCHREL4_SPEC>;
#[doc = "SCT match reload value register"]
pub mod capctrl_matchrel_sctmatchrel4;
#[doc = "CAPCTRL_MATCHREL_SCTCAPCTRL5 register accessor: an alias for `Reg<CAPCTRL_MATCHREL_SCTCAPCTRL5_SPEC>`"]
pub type CAPCTRL_MATCHREL_SCTCAPCTRL5 =
    crate::Reg<capctrl_matchrel_sctcapctrl5::CAPCTRL_MATCHREL_SCTCAPCTRL5_SPEC>;
#[doc = "SCT capture control register"]
pub mod capctrl_matchrel_sctcapctrl5;
#[doc = "CAPCTRL_MATCHREL_SCTMATCHREL5 register accessor: an alias for `Reg<CAPCTRL_MATCHREL_SCTMATCHREL5_SPEC>`"]
pub type CAPCTRL_MATCHREL_SCTMATCHREL5 =
    crate::Reg<capctrl_matchrel_sctmatchrel5::CAPCTRL_MATCHREL_SCTMATCHREL5_SPEC>;
#[doc = "SCT match reload value register"]
pub mod capctrl_matchrel_sctmatchrel5;
#[doc = "CAPCTRL_MATCHREL_SCTCAPCTRL6 register accessor: an alias for `Reg<CAPCTRL_MATCHREL_SCTCAPCTRL6_SPEC>`"]
pub type CAPCTRL_MATCHREL_SCTCAPCTRL6 =
    crate::Reg<capctrl_matchrel_sctcapctrl6::CAPCTRL_MATCHREL_SCTCAPCTRL6_SPEC>;
#[doc = "SCT capture control register"]
pub mod capctrl_matchrel_sctcapctrl6;
#[doc = "CAPCTRL_MATCHREL_SCTMATCHREL6 register accessor: an alias for `Reg<CAPCTRL_MATCHREL_SCTMATCHREL6_SPEC>`"]
pub type CAPCTRL_MATCHREL_SCTMATCHREL6 =
    crate::Reg<capctrl_matchrel_sctmatchrel6::CAPCTRL_MATCHREL_SCTMATCHREL6_SPEC>;
#[doc = "SCT match reload value register"]
pub mod capctrl_matchrel_sctmatchrel6;
#[doc = "CAPCTRL_MATCHREL_SCTCAPCTRL7 register accessor: an alias for `Reg<CAPCTRL_MATCHREL_SCTCAPCTRL7_SPEC>`"]
pub type CAPCTRL_MATCHREL_SCTCAPCTRL7 =
    crate::Reg<capctrl_matchrel_sctcapctrl7::CAPCTRL_MATCHREL_SCTCAPCTRL7_SPEC>;
#[doc = "SCT capture control register"]
pub mod capctrl_matchrel_sctcapctrl7;
#[doc = "CAPCTRL_MATCHREL_SCTMATCHREL7 register accessor: an alias for `Reg<CAPCTRL_MATCHREL_SCTMATCHREL7_SPEC>`"]
pub type CAPCTRL_MATCHREL_SCTMATCHREL7 =
    crate::Reg<capctrl_matchrel_sctmatchrel7::CAPCTRL_MATCHREL_SCTMATCHREL7_SPEC>;
#[doc = "SCT match reload value register"]
pub mod capctrl_matchrel_sctmatchrel7;
