#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Configuration and control to setup the functional clock, the rules, and the pin selections and rules."]
    pub ctrl: CTRL,
    #[doc = "0x04 - Status from triggers and time-outs including if in a poll now. Some are used for interrupts."]
    pub status: STATUS,
    #[doc = "0x08 - This sets up the polling counter and measurement counter rules."]
    pub poll_tcnt: POLL_TCNT,
    _reserved3: [u8; 4usize],
    #[doc = "0x10 - Interrupt enable"]
    pub intenset: INTENSET,
    #[doc = "0x14 - Interrupt enable clear"]
    pub intenclr: INTENCLR,
    #[doc = "0x18 - Interrupt status (mask of STATUS and INTEN)"]
    pub intstat: INTSTAT,
    _reserved6: [u8; 4usize],
    #[doc = "0x20 - Last touch event (touch or no-touch) in context."]
    pub touch: TOUCH,
    _reserved7: [u8; 4056usize],
    #[doc = "0xffc - Block ID"]
    pub id: ID,
}
#[doc = "Configuration and control to setup the functional clock, the rules, and the pin selections and rules.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrl](ctrl) module"]
pub type CTRL = crate::Reg<u32, _CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CTRL;
#[doc = "`read()` method returns [ctrl::R](ctrl::R) reader structure"]
impl crate::Readable for CTRL {}
#[doc = "`write(|w| ..)` method takes [ctrl::W](ctrl::W) writer structure"]
impl crate::Writable for CTRL {}
#[doc = "Configuration and control to setup the functional clock, the rules, and the pin selections and rules."]
pub mod ctrl;
#[doc = "Status from triggers and time-outs including if in a poll now. Some are used for interrupts.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [status](status) module"]
pub type STATUS = crate::Reg<u32, _STATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _STATUS;
#[doc = "`read()` method returns [status::R](status::R) reader structure"]
impl crate::Readable for STATUS {}
#[doc = "`write(|w| ..)` method takes [status::W](status::W) writer structure"]
impl crate::Writable for STATUS {}
#[doc = "Status from triggers and time-outs including if in a poll now. Some are used for interrupts."]
pub mod status;
#[doc = "This sets up the polling counter and measurement counter rules.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [poll_tcnt](poll_tcnt) module"]
pub type POLL_TCNT = crate::Reg<u32, _POLL_TCNT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _POLL_TCNT;
#[doc = "`read()` method returns [poll_tcnt::R](poll_tcnt::R) reader structure"]
impl crate::Readable for POLL_TCNT {}
#[doc = "`write(|w| ..)` method takes [poll_tcnt::W](poll_tcnt::W) writer structure"]
impl crate::Writable for POLL_TCNT {}
#[doc = "This sets up the polling counter and measurement counter rules."]
pub mod poll_tcnt;
#[doc = "Interrupt enable\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intenset](intenset) module"]
pub type INTENSET = crate::Reg<u32, _INTENSET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTENSET;
#[doc = "`read()` method returns [intenset::R](intenset::R) reader structure"]
impl crate::Readable for INTENSET {}
#[doc = "`write(|w| ..)` method takes [intenset::W](intenset::W) writer structure"]
impl crate::Writable for INTENSET {}
#[doc = "Interrupt enable"]
pub mod intenset;
#[doc = "Interrupt enable clear\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intenclr](intenclr) module"]
pub type INTENCLR = crate::Reg<u32, _INTENCLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTENCLR;
#[doc = "`read()` method returns [intenclr::R](intenclr::R) reader structure"]
impl crate::Readable for INTENCLR {}
#[doc = "`write(|w| ..)` method takes [intenclr::W](intenclr::W) writer structure"]
impl crate::Writable for INTENCLR {}
#[doc = "Interrupt enable clear"]
pub mod intenclr;
#[doc = "Interrupt status (mask of STATUS and INTEN)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intstat](intstat) module"]
pub type INTSTAT = crate::Reg<u32, _INTSTAT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTSTAT;
#[doc = "`read()` method returns [intstat::R](intstat::R) reader structure"]
impl crate::Readable for INTSTAT {}
#[doc = "`write(|w| ..)` method takes [intstat::W](intstat::W) writer structure"]
impl crate::Writable for INTSTAT {}
#[doc = "Interrupt status (mask of STATUS and INTEN)"]
pub mod intstat;
#[doc = "Last touch event (touch or no-touch) in context.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [touch](touch) module"]
pub type TOUCH = crate::Reg<u32, _TOUCH>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TOUCH;
#[doc = "`read()` method returns [touch::R](touch::R) reader structure"]
impl crate::Readable for TOUCH {}
#[doc = "`write(|w| ..)` method takes [touch::W](touch::W) writer structure"]
impl crate::Writable for TOUCH {}
#[doc = "Last touch event (touch or no-touch) in context."]
pub mod touch;
#[doc = "Block ID\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [id](id) module"]
pub type ID = crate::Reg<u32, _ID>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ID;
#[doc = "`read()` method returns [id::R](id::R) reader structure"]
impl crate::Readable for ID {}
#[doc = "Block ID"]
pub mod id;
