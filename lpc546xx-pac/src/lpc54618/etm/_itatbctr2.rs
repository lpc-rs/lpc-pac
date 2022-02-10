///Register `_ITATBCTR2` reader
pub struct R(crate::R<_ITATBCTR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<_ITATBCTR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<_ITATBCTR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<_ITATBCTR2_SPEC>) -> Self {
        R(reader)
    }
}
///Field `ATREADY` reader - A read of this bit returns the value of the ETM ATREADY input.
pub struct ATREADY_R(crate::FieldReader<bool, bool>);
impl ATREADY_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ATREADY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ATREADY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    ///Bit 0 - A read of this bit returns the value of the ETM ATREADY input.
    #[inline(always)]
    pub fn atready(&self) -> ATREADY_R {
        ATREADY_R::new((self.bits & 0x01) != 0)
    }
}
///ETM Integration Test ATB Control 2 Register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [_itatbctr2](index.html) module
pub struct _ITATBCTR2_SPEC;
impl crate::RegisterSpec for _ITATBCTR2_SPEC {
    type Ux = u32;
}
///`read()` method returns [_itatbctr2::R](R) reader structure
impl crate::Readable for _ITATBCTR2_SPEC {
    type Reader = R;
}
///`reset()` method sets _ITATBCTR2 to value 0
impl crate::Resettable for _ITATBCTR2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
