#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00..0x40 - no description available"]
    pub channel: [CHANNEL; 4],
    _reserved1: [u8; 0xb0],
    #[doc = "0xf0 - Module Configuration register. This register provides information about this particular MRT instance, and allows choosing an overall mode for the idle channel feature."]
    pub modcfg: crate::Reg<modcfg::MODCFG_SPEC>,
    #[doc = "0xf4 - Idle channel register. This register returns the number of the first idle channel."]
    pub idle_ch: crate::Reg<idle_ch::IDLE_CH_SPEC>,
    #[doc = "0xf8 - Global interrupt flag register"]
    pub irq_flag: crate::Reg<irq_flag::IRQ_FLAG_SPEC>,
}
#[doc = r"Register block"]
#[repr(C)]
pub struct CHANNEL {
    #[doc = "0x00 - MRT Time interval value register. This value is loaded into the TIMER register."]
    pub intval: crate::Reg<self::channel::intval::INTVAL_SPEC>,
    #[doc = "0x04 - MRT Timer register. This register reads the value of the down-counter."]
    pub timer: crate::Reg<self::channel::timer::TIMER_SPEC>,
    #[doc = "0x08 - MRT Control register. This register controls the MRT modes."]
    pub ctrl: crate::Reg<self::channel::ctrl::CTRL_SPEC>,
    #[doc = "0x0c - MRT Status register."]
    pub stat: crate::Reg<self::channel::stat::STAT_SPEC>,
}
#[doc = r"Register block"]
#[doc = "no description available"]
pub mod channel;
#[doc = "MODCFG register accessor: an alias for `Reg<MODCFG_SPEC>`"]
pub type MODCFG = crate::Reg<modcfg::MODCFG_SPEC>;
#[doc = "Module Configuration register. This register provides information about this particular MRT instance, and allows choosing an overall mode for the idle channel feature."]
pub mod modcfg;
#[doc = "IDLE_CH register accessor: an alias for `Reg<IDLE_CH_SPEC>`"]
pub type IDLE_CH = crate::Reg<idle_ch::IDLE_CH_SPEC>;
#[doc = "Idle channel register. This register returns the number of the first idle channel."]
pub mod idle_ch;
#[doc = "IRQ_FLAG register accessor: an alias for `Reg<IRQ_FLAG_SPEC>`"]
pub type IRQ_FLAG = crate::Reg<irq_flag::IRQ_FLAG_SPEC>;
#[doc = "Global interrupt flag register"]
pub mod irq_flag;
