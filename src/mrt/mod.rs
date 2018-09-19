#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - MRT0 Time interval value register. This value is loaded into the TIMER0 register."]
    pub intval0: INTVAL,
    #[doc = "0x04 - MRT0 Timer register. This register reads the value of the down-counter."]
    pub timer0: TIMER,
    #[doc = "0x08 - MRT0 Control register. This register controls the MRT0 modes."]
    pub ctrl0: CTRL,
    #[doc = "0x0c - MRT0 Status register."]
    pub stat0: STAT,
    #[doc = "0x10 - MRT0 Time interval value register. This value is loaded into the TIMER0 register."]
    pub intval1: INTVAL,
    #[doc = "0x14 - MRT0 Timer register. This register reads the value of the down-counter."]
    pub timer1: TIMER,
    #[doc = "0x18 - MRT0 Control register. This register controls the MRT0 modes."]
    pub ctrl1: CTRL,
    #[doc = "0x1c - MRT0 Status register."]
    pub stat1: STAT,
    #[doc = "0x20 - MRT0 Time interval value register. This value is loaded into the TIMER0 register."]
    pub intval2: INTVAL,
    #[doc = "0x24 - MRT0 Timer register. This register reads the value of the down-counter."]
    pub timer2: TIMER,
    #[doc = "0x28 - MRT0 Control register. This register controls the MRT0 modes."]
    pub ctrl2: CTRL,
    #[doc = "0x2c - MRT0 Status register."]
    pub stat2: STAT,
    #[doc = "0x30 - MRT0 Time interval value register. This value is loaded into the TIMER0 register."]
    pub intval3: INTVAL,
    #[doc = "0x34 - MRT0 Timer register. This register reads the value of the down-counter."]
    pub timer3: TIMER,
    #[doc = "0x38 - MRT0 Control register. This register controls the MRT0 modes."]
    pub ctrl3: CTRL,
    #[doc = "0x3c - MRT0 Status register."]
    pub stat3: STAT,
    _reserved0: [u8; 180usize],
    #[doc = "0xf4 - Idle channel register. This register returns the number of the first idle channel."]
    pub idle_ch: IDLE_CH,
    #[doc = "0xf8 - Global interrupt flag register"]
    pub irq_flag: IRQ_FLAG,
}
#[doc = "MRT0 Time interval value register. This value is loaded into the TIMER0 register."]
pub struct INTVAL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "MRT0 Time interval value register. This value is loaded into the TIMER0 register."]
pub mod intval;
#[doc = "MRT0 Timer register. This register reads the value of the down-counter."]
pub struct TIMER {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "MRT0 Timer register. This register reads the value of the down-counter."]
pub mod timer;
#[doc = "MRT0 Control register. This register controls the MRT0 modes."]
pub struct CTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "MRT0 Control register. This register controls the MRT0 modes."]
pub mod ctrl;
#[doc = "MRT0 Status register."]
pub struct STAT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "MRT0 Status register."]
pub mod stat;
#[doc = "Idle channel register. This register returns the number of the first idle channel."]
pub struct IDLE_CH {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Idle channel register. This register returns the number of the first idle channel."]
pub mod idle_ch;
#[doc = "Global interrupt flag register"]
pub struct IRQ_FLAG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Global interrupt flag register"]
pub mod irq_flag;
