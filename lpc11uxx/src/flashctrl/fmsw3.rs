#[doc = "Register `FMSW3` reader"]
pub struct R(crate::R<FMSW3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FMSW3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FMSW3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FMSW3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `SW3_127_96` reader - Word 3 of 128-bit signature (bits 127 to 96)."]
pub struct SW3_127_96_R(crate::FieldReader<u32, u32>);
impl SW3_127_96_R {
    pub(crate) fn new(bits: u32) -> Self {
        SW3_127_96_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SW3_127_96_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:31 - Word 3 of 128-bit signature (bits 127 to 96)."]
    #[inline(always)]
    pub fn sw3_127_96(&self) -> SW3_127_96_R {
        SW3_127_96_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
#[doc = "Word 3 \\[127:96\\]\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fmsw3](index.html) module"]
pub struct FMSW3_SPEC;
impl crate::RegisterSpec for FMSW3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fmsw3::R](R) reader structure"]
impl crate::Readable for FMSW3_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets FMSW3 to value 0"]
impl crate::Resettable for FMSW3_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
