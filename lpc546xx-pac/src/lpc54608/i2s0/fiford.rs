///Register `FIFORD` reader
pub struct R(crate::R<FIFORD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FIFORD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FIFORD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FIFORD_SPEC>) -> Self {
        R(reader)
    }
}
///Field `RXDATA` reader - Received data from the FIFO. The number of bits used depends on configuration details.
pub struct RXDATA_R(crate::FieldReader<u32, u32>);
impl RXDATA_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        RXDATA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXDATA_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    ///Bits 0:31 - Received data from the FIFO. The number of bits used depends on configuration details.
    #[inline(always)]
    pub fn rxdata(&self) -> RXDATA_R {
        RXDATA_R::new(self.bits)
    }
}
///FIFO read data.
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [fiford](index.html) module
pub struct FIFORD_SPEC;
impl crate::RegisterSpec for FIFORD_SPEC {
    type Ux = u32;
}
///`read()` method returns [fiford::R](R) reader structure
impl crate::Readable for FIFORD_SPEC {
    type Reader = R;
}
///`reset()` method sets FIFORD to value 0
impl crate::Resettable for FIFORD_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
