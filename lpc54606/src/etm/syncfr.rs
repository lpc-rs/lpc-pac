#[doc = "Register `SYNCFR` reader"]
pub struct R(crate::R<SYNCFR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SYNCFR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SYNCFR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SYNCFR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `SyncFrequency` reader - Synchronization frequency. Default value is 1024."]
pub struct SYNCFREQUENCY_R(crate::FieldReader<u16, u16>);
impl SYNCFREQUENCY_R {
    pub(crate) fn new(bits: u16) -> Self {
        SYNCFREQUENCY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SYNCFREQUENCY_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:11 - Synchronization frequency. Default value is 1024."]
    #[inline(always)]
    pub fn sync_frequency(&self) -> SYNCFREQUENCY_R {
        SYNCFREQUENCY_R::new((self.bits & 0x0fff) as u16)
    }
}
#[doc = "Synchronization Frequency Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [syncfr](index.html) module"]
pub struct SYNCFR_SPEC;
impl crate::RegisterSpec for SYNCFR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [syncfr::R](R) reader structure"]
impl crate::Readable for SYNCFR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SYNCFR to value 0x0400"]
impl crate::Resettable for SYNCFR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0400
    }
}
