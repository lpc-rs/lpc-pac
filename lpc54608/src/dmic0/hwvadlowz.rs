#[doc = "Register `HWVADLOWZ` reader"]
pub struct R(crate::R<HWVADLOWZ_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HWVADLOWZ_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HWVADLOWZ_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HWVADLOWZ_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `LOWZ` reader - Noise envelope estimator value."]
pub struct LOWZ_R(crate::FieldReader<u16, u16>);
impl LOWZ_R {
    pub(crate) fn new(bits: u16) -> Self {
        LOWZ_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LOWZ_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:15 - Noise envelope estimator value."]
    #[inline(always)]
    pub fn lowz(&self) -> LOWZ_R {
        LOWZ_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "HWVAD noise envelope estimator register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hwvadlowz](index.html) module"]
pub struct HWVADLOWZ_SPEC;
impl crate::RegisterSpec for HWVADLOWZ_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hwvadlowz::R](R) reader structure"]
impl crate::Readable for HWVADLOWZ_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets HWVADLOWZ to value 0"]
impl crate::Resettable for HWVADLOWZ_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
