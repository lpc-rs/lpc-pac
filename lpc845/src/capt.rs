#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Configuration and control to setup the functional clock, the rules, and the pin selections and rules."]
    pub ctrl: crate::Reg<ctrl::CTRL_SPEC>,
    #[doc = "0x04 - Status from triggers and time-outs including if in a poll now. Some are used for interrupts."]
    pub status: crate::Reg<status::STATUS_SPEC>,
    #[doc = "0x08 - This sets up the polling counter and measurement counter rules."]
    pub poll_tcnt: crate::Reg<poll_tcnt::POLL_TCNT_SPEC>,
    _reserved3: [u8; 0x04],
    #[doc = "0x10 - Interrupt enable"]
    pub intenset: crate::Reg<intenset::INTENSET_SPEC>,
    #[doc = "0x14 - Interrupt enable clear"]
    pub intenclr: crate::Reg<intenclr::INTENCLR_SPEC>,
    #[doc = "0x18 - Interrupt status (mask of STATUS and INTEN)"]
    pub intstat: crate::Reg<intstat::INTSTAT_SPEC>,
    _reserved6: [u8; 0x04],
    #[doc = "0x20 - Last touch event (touch or no-touch) in context."]
    pub touch: crate::Reg<touch::TOUCH_SPEC>,
    _reserved7: [u8; 0x0fd8],
    #[doc = "0xffc - Block ID"]
    pub id: crate::Reg<id::ID_SPEC>,
}
#[doc = "CTRL register accessor: an alias for `Reg<CTRL_SPEC>`"]
pub type CTRL = crate::Reg<ctrl::CTRL_SPEC>;
#[doc = "Configuration and control to setup the functional clock, the rules, and the pin selections and rules."]
pub mod ctrl;
#[doc = "STATUS register accessor: an alias for `Reg<STATUS_SPEC>`"]
pub type STATUS = crate::Reg<status::STATUS_SPEC>;
#[doc = "Status from triggers and time-outs including if in a poll now. Some are used for interrupts."]
pub mod status;
#[doc = "POLL_TCNT register accessor: an alias for `Reg<POLL_TCNT_SPEC>`"]
pub type POLL_TCNT = crate::Reg<poll_tcnt::POLL_TCNT_SPEC>;
#[doc = "This sets up the polling counter and measurement counter rules."]
pub mod poll_tcnt;
#[doc = "INTENSET register accessor: an alias for `Reg<INTENSET_SPEC>`"]
pub type INTENSET = crate::Reg<intenset::INTENSET_SPEC>;
#[doc = "Interrupt enable"]
pub mod intenset;
#[doc = "INTENCLR register accessor: an alias for `Reg<INTENCLR_SPEC>`"]
pub type INTENCLR = crate::Reg<intenclr::INTENCLR_SPEC>;
#[doc = "Interrupt enable clear"]
pub mod intenclr;
#[doc = "INTSTAT register accessor: an alias for `Reg<INTSTAT_SPEC>`"]
pub type INTSTAT = crate::Reg<intstat::INTSTAT_SPEC>;
#[doc = "Interrupt status (mask of STATUS and INTEN)"]
pub mod intstat;
#[doc = "TOUCH register accessor: an alias for `Reg<TOUCH_SPEC>`"]
pub type TOUCH = crate::Reg<touch::TOUCH_SPEC>;
#[doc = "Last touch event (touch or no-touch) in context."]
pub mod touch;
#[doc = "ID register accessor: an alias for `Reg<ID_SPEC>`"]
pub type ID = crate::Reg<id::ID_SPEC>;
#[doc = "Block ID"]
pub mod id;
