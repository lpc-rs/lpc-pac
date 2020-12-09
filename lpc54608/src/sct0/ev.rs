#[doc = "SCT event state register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ev_state](ev_state) module"]
pub type EV_STATE = crate::Reg<u32, _EV_STATE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EV_STATE;
#[doc = "`read()` method returns [ev_state::R](ev_state::R) reader structure"]
impl crate::Readable for EV_STATE {}
#[doc = "`write(|w| ..)` method takes [ev_state::W](ev_state::W) writer structure"]
impl crate::Writable for EV_STATE {}
#[doc = "SCT event state register 0"]
pub mod ev_state;
#[doc = "SCT event control register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ev_ctrl](ev_ctrl) module"]
pub type EV_CTRL = crate::Reg<u32, _EV_CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EV_CTRL;
#[doc = "`read()` method returns [ev_ctrl::R](ev_ctrl::R) reader structure"]
impl crate::Readable for EV_CTRL {}
#[doc = "`write(|w| ..)` method takes [ev_ctrl::W](ev_ctrl::W) writer structure"]
impl crate::Writable for EV_CTRL {}
#[doc = "SCT event control register 0"]
pub mod ev_ctrl;
