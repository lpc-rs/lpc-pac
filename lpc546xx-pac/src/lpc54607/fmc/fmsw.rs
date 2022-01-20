#[doc = "Register `FMSW[%s]` reader"]
pub struct R(crate::R<FMSW_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FMSW_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FMSW_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FMSW_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `SW` reader - Words of 128-bit signature (bits)."]
pub struct SW_R(crate::FieldReader<u32, u32>);
impl SW_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        SW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SW_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:31 - Words of 128-bit signature (bits)."]
    #[inline(always)]
    pub fn sw(&self) -> SW_R {
        SW_R::new(self.bits)
    }
}
#[doc = "Words of 128-bit signature word\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fmsw](index.html) module"]
pub struct FMSW_SPEC;
impl crate::RegisterSpec for FMSW_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fmsw::R](R) reader structure"]
impl crate::Readable for FMSW_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets FMSW[%s]
to value 0"]
impl crate::Resettable for FMSW_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
