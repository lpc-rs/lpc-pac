#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - GPIO overall Interrupt Status."]
    pub status: STATUS,
    #[doc = "0x04 - GPIO Interrupt Status for Rising edge for Port 0."]
    pub statr0: STATR0,
    #[doc = "0x08 - GPIO Interrupt Status for Falling edge for Port 0."]
    pub statf0: STATF0,
    #[doc = "0x0c - GPIO Interrupt Clear."]
    pub clr0: CLR0,
    #[doc = "0x10 - GPIO Interrupt Enable for Rising edge for Port 0."]
    pub enr0: ENR0,
    #[doc = "0x14 - GPIO Interrupt Enable for Falling edge for Port 0."]
    pub enf0: ENF0,
    _reserved6: [u8; 12usize],
    #[doc = "0x24 - GPIO Interrupt Status for Rising edge for Port 0."]
    pub statr2: STATR2,
    #[doc = "0x28 - GPIO Interrupt Status for Falling edge for Port 0."]
    pub statf2: STATF2,
    #[doc = "0x2c - GPIO Interrupt Clear."]
    pub clr2: CLR2,
    #[doc = "0x30 - GPIO Interrupt Enable for Rising edge for Port 0."]
    pub enr2: ENR2,
    #[doc = "0x34 - GPIO Interrupt Enable for Falling edge for Port 0."]
    pub enf2: ENF2,
}
#[doc = "GPIO overall Interrupt Status.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [status](status) module"]
pub type STATUS = crate::Reg<u32, _STATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _STATUS;
#[doc = "`read()` method returns [status::R](status::R) reader structure"]
impl crate::Readable for STATUS {}
#[doc = "GPIO overall Interrupt Status."]
pub mod status;
#[doc = "GPIO Interrupt Status for Rising edge for Port 0.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [statr0](statr0) module"]
pub type STATR0 = crate::Reg<u32, _STATR0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _STATR0;
#[doc = "`read()` method returns [statr0::R](statr0::R) reader structure"]
impl crate::Readable for STATR0 {}
#[doc = "GPIO Interrupt Status for Rising edge for Port 0."]
pub mod statr0;
#[doc = "GPIO Interrupt Status for Falling edge for Port 0.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [statf0](statf0) module"]
pub type STATF0 = crate::Reg<u32, _STATF0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _STATF0;
#[doc = "`read()` method returns [statf0::R](statf0::R) reader structure"]
impl crate::Readable for STATF0 {}
#[doc = "GPIO Interrupt Status for Falling edge for Port 0."]
pub mod statf0;
#[doc = "GPIO Interrupt Clear.\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clr0](clr0) module"]
pub type CLR0 = crate::Reg<u32, _CLR0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CLR0;
#[doc = "`write(|w| ..)` method takes [clr0::W](clr0::W) writer structure"]
impl crate::Writable for CLR0 {}
#[doc = "GPIO Interrupt Clear."]
pub mod clr0;
#[doc = "GPIO Interrupt Enable for Rising edge for Port 0.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [enr0](enr0) module"]
pub type ENR0 = crate::Reg<u32, _ENR0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ENR0;
#[doc = "`read()` method returns [enr0::R](enr0::R) reader structure"]
impl crate::Readable for ENR0 {}
#[doc = "`write(|w| ..)` method takes [enr0::W](enr0::W) writer structure"]
impl crate::Writable for ENR0 {}
#[doc = "GPIO Interrupt Enable for Rising edge for Port 0."]
pub mod enr0;
#[doc = "GPIO Interrupt Enable for Falling edge for Port 0.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [enf0](enf0) module"]
pub type ENF0 = crate::Reg<u32, _ENF0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ENF0;
#[doc = "`read()` method returns [enf0::R](enf0::R) reader structure"]
impl crate::Readable for ENF0 {}
#[doc = "`write(|w| ..)` method takes [enf0::W](enf0::W) writer structure"]
impl crate::Writable for ENF0 {}
#[doc = "GPIO Interrupt Enable for Falling edge for Port 0."]
pub mod enf0;
#[doc = "GPIO Interrupt Status for Rising edge for Port 0.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [statr2](statr2) module"]
pub type STATR2 = crate::Reg<u32, _STATR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _STATR2;
#[doc = "`read()` method returns [statr2::R](statr2::R) reader structure"]
impl crate::Readable for STATR2 {}
#[doc = "GPIO Interrupt Status for Rising edge for Port 0."]
pub mod statr2;
#[doc = "GPIO Interrupt Status for Falling edge for Port 0.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [statf2](statf2) module"]
pub type STATF2 = crate::Reg<u32, _STATF2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _STATF2;
#[doc = "`read()` method returns [statf2::R](statf2::R) reader structure"]
impl crate::Readable for STATF2 {}
#[doc = "GPIO Interrupt Status for Falling edge for Port 0."]
pub mod statf2;
#[doc = "GPIO Interrupt Clear.\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clr2](clr2) module"]
pub type CLR2 = crate::Reg<u32, _CLR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CLR2;
#[doc = "`write(|w| ..)` method takes [clr2::W](clr2::W) writer structure"]
impl crate::Writable for CLR2 {}
#[doc = "GPIO Interrupt Clear."]
pub mod clr2;
#[doc = "GPIO Interrupt Enable for Rising edge for Port 0.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [enr2](enr2) module"]
pub type ENR2 = crate::Reg<u32, _ENR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ENR2;
#[doc = "`read()` method returns [enr2::R](enr2::R) reader structure"]
impl crate::Readable for ENR2 {}
#[doc = "`write(|w| ..)` method takes [enr2::W](enr2::W) writer structure"]
impl crate::Writable for ENR2 {}
#[doc = "GPIO Interrupt Enable for Rising edge for Port 0."]
pub mod enr2;
#[doc = "GPIO Interrupt Enable for Falling edge for Port 0.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [enf2](enf2) module"]
pub type ENF2 = crate::Reg<u32, _ENF2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ENF2;
#[doc = "`read()` method returns [enf2::R](enf2::R) reader structure"]
impl crate::Readable for ENF2 {}
#[doc = "`write(|w| ..)` method takes [enf2::W](enf2::W) writer structure"]
impl crate::Writable for ENF2 {}
#[doc = "GPIO Interrupt Enable for Falling edge for Port 0."]
pub mod enf2;
