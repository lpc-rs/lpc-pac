#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - no description available"]
    pub channel: [CHANNEL; 4],
    _reserved0: [u8; 180usize],
    #[doc = "0xf4 - Idle channel register. This register returns the number of the first idle channel."]
    pub idle_ch: IDLE_CH,
    #[doc = "0xf8 - Global interrupt flag register"]
    pub irq_flag: IRQ_FLAG,
}
#[doc = r" Register block"]
#[repr(C)]
pub struct CHANNEL {
    #[doc = "0x00 - MRT Time interval value register. This value is loaded into the TIMER register."]
    pub intval: self::channel::INTVAL,
    #[doc = "0x04 - MRT Timer register. This register reads the value of the down-counter."]
    pub timer: self::channel::TIMER,
    #[doc = "0x08 - MRT Control register. This register controls the MRT modes."]
    pub ctrl: self::channel::CTRL,
    #[doc = "0x0c - MRT Status register."]
    pub stat: self::channel::STAT,
}
#[doc = r" Register block"]
#[doc = "no description available"]
pub mod channel;
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
