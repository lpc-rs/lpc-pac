#[doc = "Register `FMSW0` reader"]
pub struct R(crate::R<FMSW0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FMSW0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FMSW0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FMSW0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `SIG` reader - 32-bit signature."]
pub struct SIG_R(crate::FieldReader<u32, u32>);
impl SIG_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        SIG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SIG_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:31 - 32-bit signature."]
    #[inline(always)]
    pub fn sig(&self) -> SIG_R {
        SIG_R::new(self.bits)
    }
}
#[doc = "Flash signature generation result register returns the flash signature produced by the embedded signature generator..\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fmsw0](index.html) module"]
pub struct FMSW0_SPEC;
impl crate::RegisterSpec for FMSW0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fmsw0::R](R) reader structure"]
impl crate::Readable for FMSW0_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets FMSW0 to value 0"]
impl crate::Resettable for FMSW0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
