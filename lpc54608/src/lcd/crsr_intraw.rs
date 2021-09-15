#[doc = "Register `CRSR_INTRAW` reader"]
pub struct R(crate::R<CRSR_INTRAW_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CRSR_INTRAW_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CRSR_INTRAW_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CRSR_INTRAW_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CRSRRIS` reader - Cursor raw interrupt status."]
pub struct CRSRRIS_R(crate::FieldReader<bool, bool>);
impl CRSRRIS_R {
    pub(crate) fn new(bits: bool) -> Self {
        CRSRRIS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CRSRRIS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - Cursor raw interrupt status."]
    #[inline(always)]
    pub fn crsrris(&self) -> CRSRRIS_R {
        CRSRRIS_R::new((self.bits & 0x01) != 0)
    }
}
#[doc = "Cursor Raw Interrupt Status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [crsr_intraw](index.html) module"]
pub struct CRSR_INTRAW_SPEC;
impl crate::RegisterSpec for CRSR_INTRAW_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [crsr_intraw::R](R) reader structure"]
impl crate::Readable for CRSR_INTRAW_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CRSR_INTRAW to value 0"]
impl crate::Resettable for CRSR_INTRAW_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
