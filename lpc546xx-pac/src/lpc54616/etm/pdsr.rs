///Register `PDSR` reader
pub struct R(crate::R<PDSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PDSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PDSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PDSR_SPEC>) -> Self {
        R(reader)
    }
}
///Field `ETMpoweredup` reader - The value of this bit indicates whether you can access the ETM Trace Registers. The value of this bit is always 1, indicating that the ETM Trace Registers can be accessed.
pub struct ETMPOWEREDUP_R(crate::FieldReader<bool, bool>);
impl ETMPOWEREDUP_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ETMPOWEREDUP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ETMPOWEREDUP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    ///Bit 0 - The value of this bit indicates whether you can access the ETM Trace Registers. The value of this bit is always 1, indicating that the ETM Trace Registers can be accessed.
    #[inline(always)]
    pub fn etmpoweredup(&self) -> ETMPOWEREDUP_R {
        ETMPOWEREDUP_R::new((self.bits & 0x01) != 0)
    }
}
///Device Power-Down Status Register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [pdsr](index.html) module
pub struct PDSR_SPEC;
impl crate::RegisterSpec for PDSR_SPEC {
    type Ux = u32;
}
///`read()` method returns [pdsr::R](R) reader structure
impl crate::Readable for PDSR_SPEC {
    type Reader = R;
}
///`reset()` method sets PDSR to value 0x01
impl crate::Resettable for PDSR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x01
    }
}
