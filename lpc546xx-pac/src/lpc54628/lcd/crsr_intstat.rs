#[doc = "Register `CRSR_INTSTAT` reader"]
pub struct R(crate::R<CRSR_INTSTAT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CRSR_INTSTAT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CRSR_INTSTAT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CRSR_INTSTAT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CRSRMIS` reader - Cursor masked interrupt status."]
pub struct CRSRMIS_R(crate::FieldReader<bool, bool>);
impl CRSRMIS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CRSRMIS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CRSRMIS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - Cursor masked interrupt status."]
    #[inline(always)]
    pub fn crsrmis(&self) -> CRSRMIS_R {
        CRSRMIS_R::new((self.bits & 0x01) != 0)
    }
}
#[doc = "Cursor Masked Interrupt Status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [crsr_intstat](index.html) module"]
pub struct CRSR_INTSTAT_SPEC;
impl crate::RegisterSpec for CRSR_INTSTAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [crsr_intstat::R](R) reader structure"]
impl crate::Readable for CRSR_INTSTAT_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CRSR_INTSTAT to value 0"]
impl crate::Resettable for CRSR_INTSTAT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
