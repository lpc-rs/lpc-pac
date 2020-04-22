#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 4088usize],
    #[doc = "0xff8 - Peripheral Select and Flexcomm ID register."]
    pub pselid: PSELID,
    #[doc = "0xffc - Peripheral identification register."]
    pub pid: PID,
}
#[doc = "Peripheral Select and Flexcomm ID register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pselid](pselid) module"]
pub type PSELID = crate::Reg<u32, _PSELID>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PSELID;
#[doc = "`read()` method returns [pselid::R](pselid::R) reader structure"]
impl crate::Readable for PSELID {}
#[doc = "`write(|w| ..)` method takes [pselid::W](pselid::W) writer structure"]
impl crate::Writable for PSELID {}
#[doc = "Peripheral Select and Flexcomm ID register."]
pub mod pselid;
#[doc = "Peripheral identification register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pid](pid) module"]
pub type PID = crate::Reg<u32, _PID>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PID;
#[doc = "`read()` method returns [pid::R](pid::R) reader structure"]
impl crate::Readable for PID {}
#[doc = "`write(|w| ..)` method takes [pid::W](pid::W) writer structure"]
impl crate::Writable for PID {}
#[doc = "Peripheral identification register."]
pub mod pid;
