///Register block
#[repr(C)]
pub struct RegisterBlock {
    ///0x00 - SCT configuration register
    pub config: crate::Reg<config::CONFIG_SPEC>,
    ///0x04 - SCT control register
    pub ctrl: crate::Reg<ctrl::CTRL_SPEC>,
    ///0x08 - SCT limit event select register
    pub limit: crate::Reg<limit::LIMIT_SPEC>,
    ///0x0c - SCT halt event select register
    pub halt: crate::Reg<halt::HALT_SPEC>,
    ///0x10 - SCT stop event select register
    pub stop: crate::Reg<stop::STOP_SPEC>,
    ///0x14 - SCT start event select register
    pub start: crate::Reg<start::START_SPEC>,
    _reserved6: [u8; 0x28],
    ///0x40 - SCT counter register
    pub count: crate::Reg<count::COUNT_SPEC>,
    ///0x44 - SCT state register
    pub state: crate::Reg<state::STATE_SPEC>,
    ///0x48 - SCT input register
    pub input: crate::Reg<input::INPUT_SPEC>,
    ///0x4c - SCT match/capture mode register
    pub regmode: crate::Reg<regmode::REGMODE_SPEC>,
    ///0x50 - SCT output register
    pub output: crate::Reg<output::OUTPUT_SPEC>,
    ///0x54 - SCT output counter direction control register
    pub outputdirctrl: crate::Reg<outputdirctrl::OUTPUTDIRCTRL_SPEC>,
    ///0x58 - SCT conflict resolution register
    pub res: crate::Reg<res::RES_SPEC>,
    ///0x5c - SCT DMA request 0 register
    pub dmareq0: crate::Reg<dmareq0::DMAREQ0_SPEC>,
    ///0x60 - SCT DMA request 1 register
    pub dmareq1: crate::Reg<dmareq1::DMAREQ1_SPEC>,
    _reserved15: [u8; 0x8c],
    ///0xf0 - SCT event interrupt enable register
    pub even: crate::Reg<even::EVEN_SPEC>,
    ///0xf4 - SCT event flag register
    pub evflag: crate::Reg<evflag::EVFLAG_SPEC>,
    ///0xf8 - SCT conflict interrupt enable register
    pub conen: crate::Reg<conen::CONEN_SPEC>,
    ///0xfc - SCT conflict flag register
    pub conflag: crate::Reg<conflag::CONFLAG_SPEC>,
    _reserved_19_cap_match: [u8; 0x04],
    _reserved_20_cap_match: [u8; 0x04],
    _reserved_21_cap_match: [u8; 0x04],
    _reserved_22_cap_match: [u8; 0x04],
    _reserved_23_cap_match: [u8; 0x04],
    _reserved_24_cap_match: [u8; 0x04],
    _reserved_25_cap_match: [u8; 0x04],
    _reserved_26_cap_match: [u8; 0x04],
    _reserved_27_cap_match: [u8; 0x04],
    _reserved_28_cap_match: [u8; 0x04],
    _reserved29: [u8; 0xd8],
    _reserved_29_capctrl_matchrel: [u8; 0x04],
    _reserved_30_capctrl_matchrel: [u8; 0x04],
    _reserved_31_capctrl_matchrel: [u8; 0x04],
    _reserved_32_capctrl_matchrel: [u8; 0x04],
    _reserved_33_capctrl_matchrel: [u8; 0x04],
    _reserved_34_capctrl_matchrel: [u8; 0x04],
    _reserved_35_capctrl_matchrel: [u8; 0x04],
    _reserved_36_capctrl_matchrel: [u8; 0x04],
    _reserved_37_capctrl_matchrel: [u8; 0x04],
    _reserved_38_capctrl_matchrel: [u8; 0x04],
    _reserved39: [u8; 0xd8],
    ///0x300..0x350 - no description available
    pub ev: [EV; 10],
    _reserved40: [u8; 0x01b0],
    ///0x500..0x550 - no description available
    pub out: [OUT; 10],
}
impl RegisterBlock {
    ///0x100 - SCT match value register of match channels
    #[inline(always)]
    pub fn cap_match_match0(&self) -> &crate::Reg<cap_match_match0::CAP_MATCH_MATCH0_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(256usize)
                as *const crate::Reg<cap_match_match0::CAP_MATCH_MATCH0_SPEC>)
        }
    }
    ///0x100 - SCT capture register of capture channel
    #[inline(always)]
    pub fn cap_match_cap0(&self) -> &crate::Reg<cap_match_cap0::CAP_MATCH_CAP0_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(256usize)
                as *const crate::Reg<cap_match_cap0::CAP_MATCH_CAP0_SPEC>)
        }
    }
    ///0x104 - SCT match value register of match channels
    #[inline(always)]
    pub fn cap_match_match1(&self) -> &crate::Reg<cap_match_match1::CAP_MATCH_MATCH1_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(260usize)
                as *const crate::Reg<cap_match_match1::CAP_MATCH_MATCH1_SPEC>)
        }
    }
    ///0x104 - SCT capture register of capture channel
    #[inline(always)]
    pub fn cap_match_cap1(&self) -> &crate::Reg<cap_match_cap1::CAP_MATCH_CAP1_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(260usize)
                as *const crate::Reg<cap_match_cap1::CAP_MATCH_CAP1_SPEC>)
        }
    }
    ///0x108 - SCT match value register of match channels
    #[inline(always)]
    pub fn cap_match_match2(&self) -> &crate::Reg<cap_match_match2::CAP_MATCH_MATCH2_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(264usize)
                as *const crate::Reg<cap_match_match2::CAP_MATCH_MATCH2_SPEC>)
        }
    }
    ///0x108 - SCT capture register of capture channel
    #[inline(always)]
    pub fn cap_match_cap2(&self) -> &crate::Reg<cap_match_cap2::CAP_MATCH_CAP2_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(264usize)
                as *const crate::Reg<cap_match_cap2::CAP_MATCH_CAP2_SPEC>)
        }
    }
    ///0x10c - SCT match value register of match channels
    #[inline(always)]
    pub fn cap_match_match3(&self) -> &crate::Reg<cap_match_match3::CAP_MATCH_MATCH3_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(268usize)
                as *const crate::Reg<cap_match_match3::CAP_MATCH_MATCH3_SPEC>)
        }
    }
    ///0x10c - SCT capture register of capture channel
    #[inline(always)]
    pub fn cap_match_cap3(&self) -> &crate::Reg<cap_match_cap3::CAP_MATCH_CAP3_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(268usize)
                as *const crate::Reg<cap_match_cap3::CAP_MATCH_CAP3_SPEC>)
        }
    }
    ///0x110 - SCT match value register of match channels
    #[inline(always)]
    pub fn cap_match_match4(&self) -> &crate::Reg<cap_match_match4::CAP_MATCH_MATCH4_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(272usize)
                as *const crate::Reg<cap_match_match4::CAP_MATCH_MATCH4_SPEC>)
        }
    }
    ///0x110 - SCT capture register of capture channel
    #[inline(always)]
    pub fn cap_match_cap4(&self) -> &crate::Reg<cap_match_cap4::CAP_MATCH_CAP4_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(272usize)
                as *const crate::Reg<cap_match_cap4::CAP_MATCH_CAP4_SPEC>)
        }
    }
    ///0x114 - SCT match value register of match channels
    #[inline(always)]
    pub fn cap_match_match5(&self) -> &crate::Reg<cap_match_match5::CAP_MATCH_MATCH5_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(276usize)
                as *const crate::Reg<cap_match_match5::CAP_MATCH_MATCH5_SPEC>)
        }
    }
    ///0x114 - SCT capture register of capture channel
    #[inline(always)]
    pub fn cap_match_cap5(&self) -> &crate::Reg<cap_match_cap5::CAP_MATCH_CAP5_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(276usize)
                as *const crate::Reg<cap_match_cap5::CAP_MATCH_CAP5_SPEC>)
        }
    }
    ///0x118 - SCT match value register of match channels
    #[inline(always)]
    pub fn cap_match_match6(&self) -> &crate::Reg<cap_match_match6::CAP_MATCH_MATCH6_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(280usize)
                as *const crate::Reg<cap_match_match6::CAP_MATCH_MATCH6_SPEC>)
        }
    }
    ///0x118 - SCT capture register of capture channel
    #[inline(always)]
    pub fn cap_match_cap6(&self) -> &crate::Reg<cap_match_cap6::CAP_MATCH_CAP6_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(280usize)
                as *const crate::Reg<cap_match_cap6::CAP_MATCH_CAP6_SPEC>)
        }
    }
    ///0x11c - SCT match value register of match channels
    #[inline(always)]
    pub fn cap_match_match7(&self) -> &crate::Reg<cap_match_match7::CAP_MATCH_MATCH7_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(284usize)
                as *const crate::Reg<cap_match_match7::CAP_MATCH_MATCH7_SPEC>)
        }
    }
    ///0x11c - SCT capture register of capture channel
    #[inline(always)]
    pub fn cap_match_cap7(&self) -> &crate::Reg<cap_match_cap7::CAP_MATCH_CAP7_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(284usize)
                as *const crate::Reg<cap_match_cap7::CAP_MATCH_CAP7_SPEC>)
        }
    }
    ///0x120 - SCT match value register of match channels
    #[inline(always)]
    pub fn cap_match_match8(&self) -> &crate::Reg<cap_match_match8::CAP_MATCH_MATCH8_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(288usize)
                as *const crate::Reg<cap_match_match8::CAP_MATCH_MATCH8_SPEC>)
        }
    }
    ///0x120 - SCT capture register of capture channel
    #[inline(always)]
    pub fn cap_match_cap8(&self) -> &crate::Reg<cap_match_cap8::CAP_MATCH_CAP8_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(288usize)
                as *const crate::Reg<cap_match_cap8::CAP_MATCH_CAP8_SPEC>)
        }
    }
    ///0x124 - SCT match value register of match channels
    #[inline(always)]
    pub fn cap_match_match9(&self) -> &crate::Reg<cap_match_match9::CAP_MATCH_MATCH9_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(292usize)
                as *const crate::Reg<cap_match_match9::CAP_MATCH_MATCH9_SPEC>)
        }
    }
    ///0x124 - SCT capture register of capture channel
    #[inline(always)]
    pub fn cap_match_cap9(&self) -> &crate::Reg<cap_match_cap9::CAP_MATCH_CAP9_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(292usize)
                as *const crate::Reg<cap_match_cap9::CAP_MATCH_CAP9_SPEC>)
        }
    }
    ///0x200 - SCT match reload value register
    #[inline(always)]
    pub fn capctrl_matchrel_matchrel0(
        &self,
    ) -> &crate::Reg<capctrl_matchrel_matchrel0::CAPCTRL_MATCHREL_MATCHREL0_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(512usize)
                as *const crate::Reg<capctrl_matchrel_matchrel0::CAPCTRL_MATCHREL_MATCHREL0_SPEC>)
        }
    }
    ///0x200 - SCT capture control register
    #[inline(always)]
    pub fn capctrl_matchrel_capctrl0(
        &self,
    ) -> &crate::Reg<capctrl_matchrel_capctrl0::CAPCTRL_MATCHREL_CAPCTRL0_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(512usize)
                as *const crate::Reg<capctrl_matchrel_capctrl0::CAPCTRL_MATCHREL_CAPCTRL0_SPEC>)
        }
    }
    ///0x204 - SCT match reload value register
    #[inline(always)]
    pub fn capctrl_matchrel_matchrel1(
        &self,
    ) -> &crate::Reg<capctrl_matchrel_matchrel1::CAPCTRL_MATCHREL_MATCHREL1_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(516usize)
                as *const crate::Reg<capctrl_matchrel_matchrel1::CAPCTRL_MATCHREL_MATCHREL1_SPEC>)
        }
    }
    ///0x204 - SCT capture control register
    #[inline(always)]
    pub fn capctrl_matchrel_capctrl1(
        &self,
    ) -> &crate::Reg<capctrl_matchrel_capctrl1::CAPCTRL_MATCHREL_CAPCTRL1_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(516usize)
                as *const crate::Reg<capctrl_matchrel_capctrl1::CAPCTRL_MATCHREL_CAPCTRL1_SPEC>)
        }
    }
    ///0x208 - SCT match reload value register
    #[inline(always)]
    pub fn capctrl_matchrel_matchrel2(
        &self,
    ) -> &crate::Reg<capctrl_matchrel_matchrel2::CAPCTRL_MATCHREL_MATCHREL2_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(520usize)
                as *const crate::Reg<capctrl_matchrel_matchrel2::CAPCTRL_MATCHREL_MATCHREL2_SPEC>)
        }
    }
    ///0x208 - SCT capture control register
    #[inline(always)]
    pub fn capctrl_matchrel_capctrl2(
        &self,
    ) -> &crate::Reg<capctrl_matchrel_capctrl2::CAPCTRL_MATCHREL_CAPCTRL2_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(520usize)
                as *const crate::Reg<capctrl_matchrel_capctrl2::CAPCTRL_MATCHREL_CAPCTRL2_SPEC>)
        }
    }
    ///0x20c - SCT match reload value register
    #[inline(always)]
    pub fn capctrl_matchrel_matchrel3(
        &self,
    ) -> &crate::Reg<capctrl_matchrel_matchrel3::CAPCTRL_MATCHREL_MATCHREL3_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(524usize)
                as *const crate::Reg<capctrl_matchrel_matchrel3::CAPCTRL_MATCHREL_MATCHREL3_SPEC>)
        }
    }
    ///0x20c - SCT capture control register
    #[inline(always)]
    pub fn capctrl_matchrel_capctrl3(
        &self,
    ) -> &crate::Reg<capctrl_matchrel_capctrl3::CAPCTRL_MATCHREL_CAPCTRL3_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(524usize)
                as *const crate::Reg<capctrl_matchrel_capctrl3::CAPCTRL_MATCHREL_CAPCTRL3_SPEC>)
        }
    }
    ///0x210 - SCT match reload value register
    #[inline(always)]
    pub fn capctrl_matchrel_matchrel4(
        &self,
    ) -> &crate::Reg<capctrl_matchrel_matchrel4::CAPCTRL_MATCHREL_MATCHREL4_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(528usize)
                as *const crate::Reg<capctrl_matchrel_matchrel4::CAPCTRL_MATCHREL_MATCHREL4_SPEC>)
        }
    }
    ///0x210 - SCT capture control register
    #[inline(always)]
    pub fn capctrl_matchrel_capctrl4(
        &self,
    ) -> &crate::Reg<capctrl_matchrel_capctrl4::CAPCTRL_MATCHREL_CAPCTRL4_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(528usize)
                as *const crate::Reg<capctrl_matchrel_capctrl4::CAPCTRL_MATCHREL_CAPCTRL4_SPEC>)
        }
    }
    ///0x214 - SCT match reload value register
    #[inline(always)]
    pub fn capctrl_matchrel_matchrel5(
        &self,
    ) -> &crate::Reg<capctrl_matchrel_matchrel5::CAPCTRL_MATCHREL_MATCHREL5_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(532usize)
                as *const crate::Reg<capctrl_matchrel_matchrel5::CAPCTRL_MATCHREL_MATCHREL5_SPEC>)
        }
    }
    ///0x214 - SCT capture control register
    #[inline(always)]
    pub fn capctrl_matchrel_capctrl5(
        &self,
    ) -> &crate::Reg<capctrl_matchrel_capctrl5::CAPCTRL_MATCHREL_CAPCTRL5_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(532usize)
                as *const crate::Reg<capctrl_matchrel_capctrl5::CAPCTRL_MATCHREL_CAPCTRL5_SPEC>)
        }
    }
    ///0x218 - SCT match reload value register
    #[inline(always)]
    pub fn capctrl_matchrel_matchrel6(
        &self,
    ) -> &crate::Reg<capctrl_matchrel_matchrel6::CAPCTRL_MATCHREL_MATCHREL6_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(536usize)
                as *const crate::Reg<capctrl_matchrel_matchrel6::CAPCTRL_MATCHREL_MATCHREL6_SPEC>)
        }
    }
    ///0x218 - SCT capture control register
    #[inline(always)]
    pub fn capctrl_matchrel_capctrl6(
        &self,
    ) -> &crate::Reg<capctrl_matchrel_capctrl6::CAPCTRL_MATCHREL_CAPCTRL6_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(536usize)
                as *const crate::Reg<capctrl_matchrel_capctrl6::CAPCTRL_MATCHREL_CAPCTRL6_SPEC>)
        }
    }
    ///0x21c - SCT match reload value register
    #[inline(always)]
    pub fn capctrl_matchrel_matchrel7(
        &self,
    ) -> &crate::Reg<capctrl_matchrel_matchrel7::CAPCTRL_MATCHREL_MATCHREL7_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(540usize)
                as *const crate::Reg<capctrl_matchrel_matchrel7::CAPCTRL_MATCHREL_MATCHREL7_SPEC>)
        }
    }
    ///0x21c - SCT capture control register
    #[inline(always)]
    pub fn capctrl_matchrel_capctrl7(
        &self,
    ) -> &crate::Reg<capctrl_matchrel_capctrl7::CAPCTRL_MATCHREL_CAPCTRL7_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(540usize)
                as *const crate::Reg<capctrl_matchrel_capctrl7::CAPCTRL_MATCHREL_CAPCTRL7_SPEC>)
        }
    }
    ///0x220 - SCT match reload value register
    #[inline(always)]
    pub fn capctrl_matchrel_matchrel8(
        &self,
    ) -> &crate::Reg<capctrl_matchrel_matchrel8::CAPCTRL_MATCHREL_MATCHREL8_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(544usize)
                as *const crate::Reg<capctrl_matchrel_matchrel8::CAPCTRL_MATCHREL_MATCHREL8_SPEC>)
        }
    }
    ///0x220 - SCT capture control register
    #[inline(always)]
    pub fn capctrl_matchrel_capctrl8(
        &self,
    ) -> &crate::Reg<capctrl_matchrel_capctrl8::CAPCTRL_MATCHREL_CAPCTRL8_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(544usize)
                as *const crate::Reg<capctrl_matchrel_capctrl8::CAPCTRL_MATCHREL_CAPCTRL8_SPEC>)
        }
    }
    ///0x224 - SCT match reload value register
    #[inline(always)]
    pub fn capctrl_matchrel_matchrel9(
        &self,
    ) -> &crate::Reg<capctrl_matchrel_matchrel9::CAPCTRL_MATCHREL_MATCHREL9_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(548usize)
                as *const crate::Reg<capctrl_matchrel_matchrel9::CAPCTRL_MATCHREL_MATCHREL9_SPEC>)
        }
    }
    ///0x224 - SCT capture control register
    #[inline(always)]
    pub fn capctrl_matchrel_capctrl9(
        &self,
    ) -> &crate::Reg<capctrl_matchrel_capctrl9::CAPCTRL_MATCHREL_CAPCTRL9_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(548usize)
                as *const crate::Reg<capctrl_matchrel_capctrl9::CAPCTRL_MATCHREL_CAPCTRL9_SPEC>)
        }
    }
}
///Register block
#[repr(C)]
pub struct EV {
    ///0x00 - SCT event state register 0
    pub ev_state: crate::Reg<self::ev::ev_state::EV_STATE_SPEC>,
    ///0x04 - SCT event control register 0
    pub ev_ctrl: crate::Reg<self::ev::ev_ctrl::EV_CTRL_SPEC>,
}
///Register block
///no description available
pub mod ev;
///Register block
#[repr(C)]
pub struct OUT {
    ///0x00 - SCT output 0 set register
    pub out_set: crate::Reg<self::out::out_set::OUT_SET_SPEC>,
    ///0x04 - SCT output 0 clear register
    pub out_clr: crate::Reg<self::out::out_clr::OUT_CLR_SPEC>,
}
///Register block
///no description available
pub mod out;
///CONFIG register accessor: an alias for `Reg<CONFIG_SPEC>`
pub type CONFIG = crate::Reg<config::CONFIG_SPEC>;
///SCT configuration register
pub mod config;
///CTRL register accessor: an alias for `Reg<CTRL_SPEC>`
pub type CTRL = crate::Reg<ctrl::CTRL_SPEC>;
///SCT control register
pub mod ctrl;
///LIMIT register accessor: an alias for `Reg<LIMIT_SPEC>`
pub type LIMIT = crate::Reg<limit::LIMIT_SPEC>;
///SCT limit event select register
pub mod limit;
///HALT register accessor: an alias for `Reg<HALT_SPEC>`
pub type HALT = crate::Reg<halt::HALT_SPEC>;
///SCT halt event select register
pub mod halt;
///STOP register accessor: an alias for `Reg<STOP_SPEC>`
pub type STOP = crate::Reg<stop::STOP_SPEC>;
///SCT stop event select register
pub mod stop;
///START register accessor: an alias for `Reg<START_SPEC>`
pub type START = crate::Reg<start::START_SPEC>;
///SCT start event select register
pub mod start;
///COUNT register accessor: an alias for `Reg<COUNT_SPEC>`
pub type COUNT = crate::Reg<count::COUNT_SPEC>;
///SCT counter register
pub mod count;
///STATE register accessor: an alias for `Reg<STATE_SPEC>`
pub type STATE = crate::Reg<state::STATE_SPEC>;
///SCT state register
pub mod state;
///INPUT register accessor: an alias for `Reg<INPUT_SPEC>`
pub type INPUT = crate::Reg<input::INPUT_SPEC>;
///SCT input register
pub mod input;
///REGMODE register accessor: an alias for `Reg<REGMODE_SPEC>`
pub type REGMODE = crate::Reg<regmode::REGMODE_SPEC>;
///SCT match/capture mode register
pub mod regmode;
///OUTPUT register accessor: an alias for `Reg<OUTPUT_SPEC>`
pub type OUTPUT = crate::Reg<output::OUTPUT_SPEC>;
///SCT output register
pub mod output;
///OUTPUTDIRCTRL register accessor: an alias for `Reg<OUTPUTDIRCTRL_SPEC>`
pub type OUTPUTDIRCTRL = crate::Reg<outputdirctrl::OUTPUTDIRCTRL_SPEC>;
///SCT output counter direction control register
pub mod outputdirctrl;
///RES register accessor: an alias for `Reg<RES_SPEC>`
pub type RES = crate::Reg<res::RES_SPEC>;
///SCT conflict resolution register
pub mod res;
///DMAREQ0 register accessor: an alias for `Reg<DMAREQ0_SPEC>`
pub type DMAREQ0 = crate::Reg<dmareq0::DMAREQ0_SPEC>;
///SCT DMA request 0 register
pub mod dmareq0;
///DMAREQ1 register accessor: an alias for `Reg<DMAREQ1_SPEC>`
pub type DMAREQ1 = crate::Reg<dmareq1::DMAREQ1_SPEC>;
///SCT DMA request 1 register
pub mod dmareq1;
///EVEN register accessor: an alias for `Reg<EVEN_SPEC>`
pub type EVEN = crate::Reg<even::EVEN_SPEC>;
///SCT event interrupt enable register
pub mod even;
///EVFLAG register accessor: an alias for `Reg<EVFLAG_SPEC>`
pub type EVFLAG = crate::Reg<evflag::EVFLAG_SPEC>;
///SCT event flag register
pub mod evflag;
///CONEN register accessor: an alias for `Reg<CONEN_SPEC>`
pub type CONEN = crate::Reg<conen::CONEN_SPEC>;
///SCT conflict interrupt enable register
pub mod conen;
///CONFLAG register accessor: an alias for `Reg<CONFLAG_SPEC>`
pub type CONFLAG = crate::Reg<conflag::CONFLAG_SPEC>;
///SCT conflict flag register
pub mod conflag;
///CAP_MATCH_CAP0 register accessor: an alias for `Reg<CAP_MATCH_CAP0_SPEC>`
pub type CAP_MATCH_CAP0 = crate::Reg<cap_match_cap0::CAP_MATCH_CAP0_SPEC>;
///SCT capture register of capture channel
pub mod cap_match_cap0;
///CAP_MATCH_MATCH0 register accessor: an alias for `Reg<CAP_MATCH_MATCH0_SPEC>`
pub type CAP_MATCH_MATCH0 = crate::Reg<cap_match_match0::CAP_MATCH_MATCH0_SPEC>;
///SCT match value register of match channels
pub mod cap_match_match0;
///CAP_MATCH_CAP1 register accessor: an alias for `Reg<CAP_MATCH_CAP1_SPEC>`
pub type CAP_MATCH_CAP1 = crate::Reg<cap_match_cap1::CAP_MATCH_CAP1_SPEC>;
///SCT capture register of capture channel
pub mod cap_match_cap1;
///CAP_MATCH_MATCH1 register accessor: an alias for `Reg<CAP_MATCH_MATCH1_SPEC>`
pub type CAP_MATCH_MATCH1 = crate::Reg<cap_match_match1::CAP_MATCH_MATCH1_SPEC>;
///SCT match value register of match channels
pub mod cap_match_match1;
///CAP_MATCH_CAP2 register accessor: an alias for `Reg<CAP_MATCH_CAP2_SPEC>`
pub type CAP_MATCH_CAP2 = crate::Reg<cap_match_cap2::CAP_MATCH_CAP2_SPEC>;
///SCT capture register of capture channel
pub mod cap_match_cap2;
///CAP_MATCH_MATCH2 register accessor: an alias for `Reg<CAP_MATCH_MATCH2_SPEC>`
pub type CAP_MATCH_MATCH2 = crate::Reg<cap_match_match2::CAP_MATCH_MATCH2_SPEC>;
///SCT match value register of match channels
pub mod cap_match_match2;
///CAP_MATCH_CAP3 register accessor: an alias for `Reg<CAP_MATCH_CAP3_SPEC>`
pub type CAP_MATCH_CAP3 = crate::Reg<cap_match_cap3::CAP_MATCH_CAP3_SPEC>;
///SCT capture register of capture channel
pub mod cap_match_cap3;
///CAP_MATCH_MATCH3 register accessor: an alias for `Reg<CAP_MATCH_MATCH3_SPEC>`
pub type CAP_MATCH_MATCH3 = crate::Reg<cap_match_match3::CAP_MATCH_MATCH3_SPEC>;
///SCT match value register of match channels
pub mod cap_match_match3;
///CAP_MATCH_CAP4 register accessor: an alias for `Reg<CAP_MATCH_CAP4_SPEC>`
pub type CAP_MATCH_CAP4 = crate::Reg<cap_match_cap4::CAP_MATCH_CAP4_SPEC>;
///SCT capture register of capture channel
pub mod cap_match_cap4;
///CAP_MATCH_MATCH4 register accessor: an alias for `Reg<CAP_MATCH_MATCH4_SPEC>`
pub type CAP_MATCH_MATCH4 = crate::Reg<cap_match_match4::CAP_MATCH_MATCH4_SPEC>;
///SCT match value register of match channels
pub mod cap_match_match4;
///CAP_MATCH_CAP5 register accessor: an alias for `Reg<CAP_MATCH_CAP5_SPEC>`
pub type CAP_MATCH_CAP5 = crate::Reg<cap_match_cap5::CAP_MATCH_CAP5_SPEC>;
///SCT capture register of capture channel
pub mod cap_match_cap5;
///CAP_MATCH_MATCH5 register accessor: an alias for `Reg<CAP_MATCH_MATCH5_SPEC>`
pub type CAP_MATCH_MATCH5 = crate::Reg<cap_match_match5::CAP_MATCH_MATCH5_SPEC>;
///SCT match value register of match channels
pub mod cap_match_match5;
///CAP_MATCH_CAP6 register accessor: an alias for `Reg<CAP_MATCH_CAP6_SPEC>`
pub type CAP_MATCH_CAP6 = crate::Reg<cap_match_cap6::CAP_MATCH_CAP6_SPEC>;
///SCT capture register of capture channel
pub mod cap_match_cap6;
///CAP_MATCH_MATCH6 register accessor: an alias for `Reg<CAP_MATCH_MATCH6_SPEC>`
pub type CAP_MATCH_MATCH6 = crate::Reg<cap_match_match6::CAP_MATCH_MATCH6_SPEC>;
///SCT match value register of match channels
pub mod cap_match_match6;
///CAP_MATCH_CAP7 register accessor: an alias for `Reg<CAP_MATCH_CAP7_SPEC>`
pub type CAP_MATCH_CAP7 = crate::Reg<cap_match_cap7::CAP_MATCH_CAP7_SPEC>;
///SCT capture register of capture channel
pub mod cap_match_cap7;
///CAP_MATCH_MATCH7 register accessor: an alias for `Reg<CAP_MATCH_MATCH7_SPEC>`
pub type CAP_MATCH_MATCH7 = crate::Reg<cap_match_match7::CAP_MATCH_MATCH7_SPEC>;
///SCT match value register of match channels
pub mod cap_match_match7;
///CAP_MATCH_CAP8 register accessor: an alias for `Reg<CAP_MATCH_CAP8_SPEC>`
pub type CAP_MATCH_CAP8 = crate::Reg<cap_match_cap8::CAP_MATCH_CAP8_SPEC>;
///SCT capture register of capture channel
pub mod cap_match_cap8;
///CAP_MATCH_MATCH8 register accessor: an alias for `Reg<CAP_MATCH_MATCH8_SPEC>`
pub type CAP_MATCH_MATCH8 = crate::Reg<cap_match_match8::CAP_MATCH_MATCH8_SPEC>;
///SCT match value register of match channels
pub mod cap_match_match8;
///CAP_MATCH_CAP9 register accessor: an alias for `Reg<CAP_MATCH_CAP9_SPEC>`
pub type CAP_MATCH_CAP9 = crate::Reg<cap_match_cap9::CAP_MATCH_CAP9_SPEC>;
///SCT capture register of capture channel
pub mod cap_match_cap9;
///CAP_MATCH_MATCH9 register accessor: an alias for `Reg<CAP_MATCH_MATCH9_SPEC>`
pub type CAP_MATCH_MATCH9 = crate::Reg<cap_match_match9::CAP_MATCH_MATCH9_SPEC>;
///SCT match value register of match channels
pub mod cap_match_match9;
///CAPCTRL_MATCHREL_CAPCTRL0 register accessor: an alias for `Reg<CAPCTRL_MATCHREL_CAPCTRL0_SPEC>`
pub type CAPCTRL_MATCHREL_CAPCTRL0 =
    crate::Reg<capctrl_matchrel_capctrl0::CAPCTRL_MATCHREL_CAPCTRL0_SPEC>;
///SCT capture control register
pub mod capctrl_matchrel_capctrl0;
///CAPCTRL_MATCHREL_MATCHREL0 register accessor: an alias for `Reg<CAPCTRL_MATCHREL_MATCHREL0_SPEC>`
pub type CAPCTRL_MATCHREL_MATCHREL0 =
    crate::Reg<capctrl_matchrel_matchrel0::CAPCTRL_MATCHREL_MATCHREL0_SPEC>;
///SCT match reload value register
pub mod capctrl_matchrel_matchrel0;
///CAPCTRL_MATCHREL_CAPCTRL1 register accessor: an alias for `Reg<CAPCTRL_MATCHREL_CAPCTRL1_SPEC>`
pub type CAPCTRL_MATCHREL_CAPCTRL1 =
    crate::Reg<capctrl_matchrel_capctrl1::CAPCTRL_MATCHREL_CAPCTRL1_SPEC>;
///SCT capture control register
pub mod capctrl_matchrel_capctrl1;
///CAPCTRL_MATCHREL_MATCHREL1 register accessor: an alias for `Reg<CAPCTRL_MATCHREL_MATCHREL1_SPEC>`
pub type CAPCTRL_MATCHREL_MATCHREL1 =
    crate::Reg<capctrl_matchrel_matchrel1::CAPCTRL_MATCHREL_MATCHREL1_SPEC>;
///SCT match reload value register
pub mod capctrl_matchrel_matchrel1;
///CAPCTRL_MATCHREL_CAPCTRL2 register accessor: an alias for `Reg<CAPCTRL_MATCHREL_CAPCTRL2_SPEC>`
pub type CAPCTRL_MATCHREL_CAPCTRL2 =
    crate::Reg<capctrl_matchrel_capctrl2::CAPCTRL_MATCHREL_CAPCTRL2_SPEC>;
///SCT capture control register
pub mod capctrl_matchrel_capctrl2;
///CAPCTRL_MATCHREL_MATCHREL2 register accessor: an alias for `Reg<CAPCTRL_MATCHREL_MATCHREL2_SPEC>`
pub type CAPCTRL_MATCHREL_MATCHREL2 =
    crate::Reg<capctrl_matchrel_matchrel2::CAPCTRL_MATCHREL_MATCHREL2_SPEC>;
///SCT match reload value register
pub mod capctrl_matchrel_matchrel2;
///CAPCTRL_MATCHREL_CAPCTRL3 register accessor: an alias for `Reg<CAPCTRL_MATCHREL_CAPCTRL3_SPEC>`
pub type CAPCTRL_MATCHREL_CAPCTRL3 =
    crate::Reg<capctrl_matchrel_capctrl3::CAPCTRL_MATCHREL_CAPCTRL3_SPEC>;
///SCT capture control register
pub mod capctrl_matchrel_capctrl3;
///CAPCTRL_MATCHREL_MATCHREL3 register accessor: an alias for `Reg<CAPCTRL_MATCHREL_MATCHREL3_SPEC>`
pub type CAPCTRL_MATCHREL_MATCHREL3 =
    crate::Reg<capctrl_matchrel_matchrel3::CAPCTRL_MATCHREL_MATCHREL3_SPEC>;
///SCT match reload value register
pub mod capctrl_matchrel_matchrel3;
///CAPCTRL_MATCHREL_CAPCTRL4 register accessor: an alias for `Reg<CAPCTRL_MATCHREL_CAPCTRL4_SPEC>`
pub type CAPCTRL_MATCHREL_CAPCTRL4 =
    crate::Reg<capctrl_matchrel_capctrl4::CAPCTRL_MATCHREL_CAPCTRL4_SPEC>;
///SCT capture control register
pub mod capctrl_matchrel_capctrl4;
///CAPCTRL_MATCHREL_MATCHREL4 register accessor: an alias for `Reg<CAPCTRL_MATCHREL_MATCHREL4_SPEC>`
pub type CAPCTRL_MATCHREL_MATCHREL4 =
    crate::Reg<capctrl_matchrel_matchrel4::CAPCTRL_MATCHREL_MATCHREL4_SPEC>;
///SCT match reload value register
pub mod capctrl_matchrel_matchrel4;
///CAPCTRL_MATCHREL_CAPCTRL5 register accessor: an alias for `Reg<CAPCTRL_MATCHREL_CAPCTRL5_SPEC>`
pub type CAPCTRL_MATCHREL_CAPCTRL5 =
    crate::Reg<capctrl_matchrel_capctrl5::CAPCTRL_MATCHREL_CAPCTRL5_SPEC>;
///SCT capture control register
pub mod capctrl_matchrel_capctrl5;
///CAPCTRL_MATCHREL_MATCHREL5 register accessor: an alias for `Reg<CAPCTRL_MATCHREL_MATCHREL5_SPEC>`
pub type CAPCTRL_MATCHREL_MATCHREL5 =
    crate::Reg<capctrl_matchrel_matchrel5::CAPCTRL_MATCHREL_MATCHREL5_SPEC>;
///SCT match reload value register
pub mod capctrl_matchrel_matchrel5;
///CAPCTRL_MATCHREL_CAPCTRL6 register accessor: an alias for `Reg<CAPCTRL_MATCHREL_CAPCTRL6_SPEC>`
pub type CAPCTRL_MATCHREL_CAPCTRL6 =
    crate::Reg<capctrl_matchrel_capctrl6::CAPCTRL_MATCHREL_CAPCTRL6_SPEC>;
///SCT capture control register
pub mod capctrl_matchrel_capctrl6;
///CAPCTRL_MATCHREL_MATCHREL6 register accessor: an alias for `Reg<CAPCTRL_MATCHREL_MATCHREL6_SPEC>`
pub type CAPCTRL_MATCHREL_MATCHREL6 =
    crate::Reg<capctrl_matchrel_matchrel6::CAPCTRL_MATCHREL_MATCHREL6_SPEC>;
///SCT match reload value register
pub mod capctrl_matchrel_matchrel6;
///CAPCTRL_MATCHREL_CAPCTRL7 register accessor: an alias for `Reg<CAPCTRL_MATCHREL_CAPCTRL7_SPEC>`
pub type CAPCTRL_MATCHREL_CAPCTRL7 =
    crate::Reg<capctrl_matchrel_capctrl7::CAPCTRL_MATCHREL_CAPCTRL7_SPEC>;
///SCT capture control register
pub mod capctrl_matchrel_capctrl7;
///CAPCTRL_MATCHREL_MATCHREL7 register accessor: an alias for `Reg<CAPCTRL_MATCHREL_MATCHREL7_SPEC>`
pub type CAPCTRL_MATCHREL_MATCHREL7 =
    crate::Reg<capctrl_matchrel_matchrel7::CAPCTRL_MATCHREL_MATCHREL7_SPEC>;
///SCT match reload value register
pub mod capctrl_matchrel_matchrel7;
///CAPCTRL_MATCHREL_CAPCTRL8 register accessor: an alias for `Reg<CAPCTRL_MATCHREL_CAPCTRL8_SPEC>`
pub type CAPCTRL_MATCHREL_CAPCTRL8 =
    crate::Reg<capctrl_matchrel_capctrl8::CAPCTRL_MATCHREL_CAPCTRL8_SPEC>;
///SCT capture control register
pub mod capctrl_matchrel_capctrl8;
///CAPCTRL_MATCHREL_MATCHREL8 register accessor: an alias for `Reg<CAPCTRL_MATCHREL_MATCHREL8_SPEC>`
pub type CAPCTRL_MATCHREL_MATCHREL8 =
    crate::Reg<capctrl_matchrel_matchrel8::CAPCTRL_MATCHREL_MATCHREL8_SPEC>;
///SCT match reload value register
pub mod capctrl_matchrel_matchrel8;
///CAPCTRL_MATCHREL_CAPCTRL9 register accessor: an alias for `Reg<CAPCTRL_MATCHREL_CAPCTRL9_SPEC>`
pub type CAPCTRL_MATCHREL_CAPCTRL9 =
    crate::Reg<capctrl_matchrel_capctrl9::CAPCTRL_MATCHREL_CAPCTRL9_SPEC>;
///SCT capture control register
pub mod capctrl_matchrel_capctrl9;
///CAPCTRL_MATCHREL_MATCHREL9 register accessor: an alias for `Reg<CAPCTRL_MATCHREL_MATCHREL9_SPEC>`
pub type CAPCTRL_MATCHREL_MATCHREL9 =
    crate::Reg<capctrl_matchrel_matchrel9::CAPCTRL_MATCHREL_MATCHREL9_SPEC>;
///SCT match reload value register
pub mod capctrl_matchrel_matchrel9;
