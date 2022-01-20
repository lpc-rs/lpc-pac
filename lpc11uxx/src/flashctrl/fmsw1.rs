#[doc = "Register `FMSW1` reader"]
pub struct R(crate::R<FMSW1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FMSW1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FMSW1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FMSW1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `SW1_63_32` reader - Word 1 of 128-bit signature (bits 63 to 32)."]
pub struct SW1_63_32_R(crate::FieldReader<u32, u32>);
impl SW1_63_32_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        SW1_63_32_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SW1_63_32_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:31 - Word 1 of 128-bit signature (bits 63 to 32)."]
    #[inline(always)]
    pub fn sw1_63_32(&self) -> SW1_63_32_R {
        SW1_63_32_R::new(self.bits)
    }
}
#[doc = "Word 1 \\[63:32\\]\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fmsw1](index.html) module"]
pub struct FMSW1_SPEC;
impl crate::RegisterSpec for FMSW1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fmsw1::R](R) reader structure"]
impl crate::Readable for FMSW1_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets FMSW1 to value 0"]
impl crate::Resettable for FMSW1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
