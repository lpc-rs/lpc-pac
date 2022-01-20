#[doc = "Register `HCREVISION` reader"]
pub struct R(crate::R<HCREVISION_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HCREVISION_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HCREVISION_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HCREVISION_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `REV` reader - Revision."]
pub struct REV_R(crate::FieldReader<u8, u8>);
impl REV_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        REV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for REV_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:7 - Revision."]
    #[inline(always)]
    pub fn rev(&self) -> REV_R {
        REV_R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "BCD representation of the version of the HCI specification that is implemented by the Host Controller (HC)\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hcrevision](index.html) module"]
pub struct HCREVISION_SPEC;
impl crate::RegisterSpec for HCREVISION_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hcrevision::R](R) reader structure"]
impl crate::Readable for HCREVISION_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets HCREVISION to value 0x10"]
impl crate::Resettable for HCREVISION_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x10
    }
}
