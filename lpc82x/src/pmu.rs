#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Power control register"]
    pub pcon: PCON,
    #[doc = "0x04 - General purpose register N"]
    pub gpreg: [GPREG; 4],
    #[doc = "0x14 - Deep power-down control register. Also includes bits for general purpose storage."]
    pub dpdctrl: DPDCTRL,
}
#[doc = "Power control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pcon](pcon) module"]
pub type PCON = crate::Reg<u32, _PCON>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PCON;
#[doc = "`read()` method returns [pcon::R](pcon::R) reader structure"]
impl crate::Readable for PCON {}
#[doc = "`write(|w| ..)` method takes [pcon::W](pcon::W) writer structure"]
impl crate::Writable for PCON {}
#[doc = "Power control register"]
pub mod pcon;
#[doc = "General purpose register N\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpreg](gpreg) module"]
pub type GPREG = crate::Reg<u32, _GPREG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPREG;
#[doc = "`read()` method returns [gpreg::R](gpreg::R) reader structure"]
impl crate::Readable for GPREG {}
#[doc = "`write(|w| ..)` method takes [gpreg::W](gpreg::W) writer structure"]
impl crate::Writable for GPREG {}
#[doc = "General purpose register N"]
pub mod gpreg;
#[doc = "Deep power-down control register. Also includes bits for general purpose storage.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dpdctrl](dpdctrl) module"]
pub type DPDCTRL = crate::Reg<u32, _DPDCTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DPDCTRL;
#[doc = "`read()` method returns [dpdctrl::R](dpdctrl::R) reader structure"]
impl crate::Readable for DPDCTRL {}
#[doc = "`write(|w| ..)` method takes [dpdctrl::W](dpdctrl::W) writer structure"]
impl crate::Writable for DPDCTRL {}
#[doc = "Deep power-down control register. Also includes bits for general purpose storage."]
pub mod dpdctrl;
