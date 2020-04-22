#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Acceptance Filter Register"]
    pub afmr: AFMR,
    #[doc = "0x04 - Standard Frame Individual Start Address Register"]
    pub sff_sa: SFF_SA,
    #[doc = "0x08 - Standard Frame Group Start Address Register"]
    pub sff_grp_sa: SFF_GRP_SA,
    #[doc = "0x0c - Extended Frame Start Address Register"]
    pub eff_sa: EFF_SA,
    #[doc = "0x10 - Extended Frame Group Start Address Register"]
    pub eff_grp_sa: EFF_GRP_SA,
    #[doc = "0x14 - End of AF Tables register"]
    pub endoftable: ENDOFTABLE,
    #[doc = "0x18 - LUT Error Address register"]
    pub luterrad: LUTERRAD,
    #[doc = "0x1c - LUT Error Register"]
    pub luterr: LUTERR,
    #[doc = "0x20 - FullCAN interrupt enable register"]
    pub fcanie: FCANIE,
    #[doc = "0x24 - FullCAN interrupt and capture register0"]
    pub fcanic0: FCANIC0,
    #[doc = "0x28 - FullCAN interrupt and capture register1"]
    pub fcanic1: FCANIC1,
}
#[doc = "Acceptance Filter Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [afmr](afmr) module"]
pub type AFMR = crate::Reg<u32, _AFMR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AFMR;
#[doc = "`read()` method returns [afmr::R](afmr::R) reader structure"]
impl crate::Readable for AFMR {}
#[doc = "`write(|w| ..)` method takes [afmr::W](afmr::W) writer structure"]
impl crate::Writable for AFMR {}
#[doc = "Acceptance Filter Register"]
pub mod afmr;
#[doc = "Standard Frame Individual Start Address Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sff_sa](sff_sa) module"]
pub type SFF_SA = crate::Reg<u32, _SFF_SA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SFF_SA;
#[doc = "`read()` method returns [sff_sa::R](sff_sa::R) reader structure"]
impl crate::Readable for SFF_SA {}
#[doc = "`write(|w| ..)` method takes [sff_sa::W](sff_sa::W) writer structure"]
impl crate::Writable for SFF_SA {}
#[doc = "Standard Frame Individual Start Address Register"]
pub mod sff_sa;
#[doc = "Standard Frame Group Start Address Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sff_grp_sa](sff_grp_sa) module"]
pub type SFF_GRP_SA = crate::Reg<u32, _SFF_GRP_SA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SFF_GRP_SA;
#[doc = "`read()` method returns [sff_grp_sa::R](sff_grp_sa::R) reader structure"]
impl crate::Readable for SFF_GRP_SA {}
#[doc = "`write(|w| ..)` method takes [sff_grp_sa::W](sff_grp_sa::W) writer structure"]
impl crate::Writable for SFF_GRP_SA {}
#[doc = "Standard Frame Group Start Address Register"]
pub mod sff_grp_sa;
#[doc = "Extended Frame Start Address Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eff_sa](eff_sa) module"]
pub type EFF_SA = crate::Reg<u32, _EFF_SA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EFF_SA;
#[doc = "`read()` method returns [eff_sa::R](eff_sa::R) reader structure"]
impl crate::Readable for EFF_SA {}
#[doc = "`write(|w| ..)` method takes [eff_sa::W](eff_sa::W) writer structure"]
impl crate::Writable for EFF_SA {}
#[doc = "Extended Frame Start Address Register"]
pub mod eff_sa;
#[doc = "Extended Frame Group Start Address Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eff_grp_sa](eff_grp_sa) module"]
pub type EFF_GRP_SA = crate::Reg<u32, _EFF_GRP_SA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EFF_GRP_SA;
#[doc = "`read()` method returns [eff_grp_sa::R](eff_grp_sa::R) reader structure"]
impl crate::Readable for EFF_GRP_SA {}
#[doc = "`write(|w| ..)` method takes [eff_grp_sa::W](eff_grp_sa::W) writer structure"]
impl crate::Writable for EFF_GRP_SA {}
#[doc = "Extended Frame Group Start Address Register"]
pub mod eff_grp_sa;
#[doc = "End of AF Tables register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [endoftable](endoftable) module"]
pub type ENDOFTABLE = crate::Reg<u32, _ENDOFTABLE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ENDOFTABLE;
#[doc = "`read()` method returns [endoftable::R](endoftable::R) reader structure"]
impl crate::Readable for ENDOFTABLE {}
#[doc = "`write(|w| ..)` method takes [endoftable::W](endoftable::W) writer structure"]
impl crate::Writable for ENDOFTABLE {}
#[doc = "End of AF Tables register"]
pub mod endoftable;
#[doc = "LUT Error Address register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [luterrad](luterrad) module"]
pub type LUTERRAD = crate::Reg<u32, _LUTERRAD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LUTERRAD;
#[doc = "`read()` method returns [luterrad::R](luterrad::R) reader structure"]
impl crate::Readable for LUTERRAD {}
#[doc = "LUT Error Address register"]
pub mod luterrad;
#[doc = "LUT Error Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [luterr](luterr) module"]
pub type LUTERR = crate::Reg<u32, _LUTERR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LUTERR;
#[doc = "`read()` method returns [luterr::R](luterr::R) reader structure"]
impl crate::Readable for LUTERR {}
#[doc = "LUT Error Register"]
pub mod luterr;
#[doc = "FullCAN interrupt enable register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fcanie](fcanie) module"]
pub type FCANIE = crate::Reg<u32, _FCANIE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FCANIE;
#[doc = "`read()` method returns [fcanie::R](fcanie::R) reader structure"]
impl crate::Readable for FCANIE {}
#[doc = "`write(|w| ..)` method takes [fcanie::W](fcanie::W) writer structure"]
impl crate::Writable for FCANIE {}
#[doc = "FullCAN interrupt enable register"]
pub mod fcanie;
#[doc = "FullCAN interrupt and capture register0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fcanic0](fcanic0) module"]
pub type FCANIC0 = crate::Reg<u32, _FCANIC0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FCANIC0;
#[doc = "`read()` method returns [fcanic0::R](fcanic0::R) reader structure"]
impl crate::Readable for FCANIC0 {}
#[doc = "`write(|w| ..)` method takes [fcanic0::W](fcanic0::W) writer structure"]
impl crate::Writable for FCANIC0 {}
#[doc = "FullCAN interrupt and capture register0"]
pub mod fcanic0;
#[doc = "FullCAN interrupt and capture register1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fcanic1](fcanic1) module"]
pub type FCANIC1 = crate::Reg<u32, _FCANIC1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FCANIC1;
#[doc = "`read()` method returns [fcanic1::R](fcanic1::R) reader structure"]
impl crate::Readable for FCANIC1 {}
#[doc = "`write(|w| ..)` method takes [fcanic1::W](fcanic1::W) writer structure"]
impl crate::Writable for FCANIC1 {}
#[doc = "FullCAN interrupt and capture register1"]
pub mod fcanic1;
