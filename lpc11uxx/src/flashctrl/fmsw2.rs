#[doc = "Register `FMSW2` reader"]
pub struct R(crate::R<FMSW2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FMSW2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FMSW2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FMSW2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `SW2_95_64` reader - Word 2 of 128-bit signature (bits 95 to 64)."]
pub struct SW2_95_64_R(crate::FieldReader<u32, u32>);
impl SW2_95_64_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        SW2_95_64_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SW2_95_64_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:31 - Word 2 of 128-bit signature (bits 95 to 64)."]
    #[inline(always)]
    pub fn sw2_95_64(&self) -> SW2_95_64_R {
        SW2_95_64_R::new(self.bits)
    }
}
#[doc = "Word 2 \\[95:64\\]\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fmsw2](index.html) module"]
pub struct FMSW2_SPEC;
impl crate::RegisterSpec for FMSW2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fmsw2::R](R) reader structure"]
impl crate::Readable for FMSW2_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets FMSW2 to value 0"]
impl crate::Resettable for FMSW2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
