#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Configuration and control to setup the functional clock, the rules, and the pin selections and rules."]
    pub ctrl: CTRL,
    #[doc = "0x04 - Status from triggers and time-outs including if in a poll now. Some are used for interrupts."]
    pub status: STATUS,
    #[doc = "0x08 - This sets up the polling counter and measurement counter rules."]
    pub poll_tcnt: POLL_TCNT,
    _reserved0: [u8; 4usize],
    #[doc = "0x10 - Interrupt enable"]
    pub intenset: INTENSET,
    #[doc = "0x14 - Interrupt enable clear"]
    pub intenclr: INTENCLR,
    #[doc = "0x18 - Interrupt status (mask of STATUS and INTEN)"]
    pub intstat: INTSTAT,
    _reserved1: [u8; 4usize],
    #[doc = "0x20 - Last touch event (touch or no-touch) in context."]
    pub touch: TOUCH,
    _reserved2: [u8; 4056usize],
    #[doc = "0xffc - Block ID"]
    pub id: ID,
}
#[doc = "Configuration and control to setup the functional clock, the rules, and the pin selections and rules."]
pub struct CTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Configuration and control to setup the functional clock, the rules, and the pin selections and rules."]
pub mod ctrl;
#[doc = "Status from triggers and time-outs including if in a poll now. Some are used for interrupts."]
pub struct STATUS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Status from triggers and time-outs including if in a poll now. Some are used for interrupts."]
pub mod status;
#[doc = "This sets up the polling counter and measurement counter rules."]
pub struct POLL_TCNT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "This sets up the polling counter and measurement counter rules."]
pub mod poll_tcnt;
#[doc = "Interrupt enable"]
pub struct INTENSET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt enable"]
pub mod intenset;
#[doc = "Interrupt enable clear"]
pub struct INTENCLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt enable clear"]
pub mod intenclr;
#[doc = "Interrupt status (mask of STATUS and INTEN)"]
pub struct INTSTAT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt status (mask of STATUS and INTEN)"]
pub mod intstat;
#[doc = "Last touch event (touch or no-touch) in context."]
pub struct TOUCH {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Last touch event (touch or no-touch) in context."]
pub mod touch;
#[doc = "Block ID"]
pub struct ID {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Block ID"]
pub mod id;
