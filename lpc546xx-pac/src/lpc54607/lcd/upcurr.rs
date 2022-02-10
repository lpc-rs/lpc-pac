///Register `UPCURR` reader
pub struct R(crate::R<UPCURR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UPCURR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UPCURR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UPCURR_SPEC>) -> Self {
        R(reader)
    }
}
///Field `LCDUPCURR` reader - LCD Upper Panel Current Address.
pub struct LCDUPCURR_R(crate::FieldReader<u32, u32>);
impl LCDUPCURR_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        LCDUPCURR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LCDUPCURR_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    ///Bits 0:31 - LCD Upper Panel Current Address.
    #[inline(always)]
    pub fn lcdupcurr(&self) -> LCDUPCURR_R {
        LCDUPCURR_R::new(self.bits)
    }
}
///Upper Panel Current Address Value register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [upcurr](index.html) module
pub struct UPCURR_SPEC;
impl crate::RegisterSpec for UPCURR_SPEC {
    type Ux = u32;
}
///`read()` method returns [upcurr::R](R) reader structure
impl crate::Readable for UPCURR_SPEC {
    type Reader = R;
}
///`reset()` method sets UPCURR to value 0
impl crate::Resettable for UPCURR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
