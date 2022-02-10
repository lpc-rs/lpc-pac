///Register `SUM` reader
pub struct R(crate::R<SUM_WR_DATA_SUM_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SUM_WR_DATA_SUM_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SUM_WR_DATA_SUM_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SUM_WR_DATA_SUM_SPEC>) -> Self {
        R(reader)
    }
}
///Field `CRC_SUM` reader - The most recent CRC sum can be read through this register with selected bit order and 1's complement post-processes.
pub struct CRC_SUM_R(crate::FieldReader<u32, u32>);
impl CRC_SUM_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        CRC_SUM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CRC_SUM_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    ///Bits 0:31 - The most recent CRC sum can be read through this register with selected bit order and 1's complement post-processes.
    #[inline(always)]
    pub fn crc_sum(&self) -> CRC_SUM_R {
        CRC_SUM_R::new(self.bits)
    }
}
///CRC checksum register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [sum_wr_data_sum](index.html) module
pub struct SUM_WR_DATA_SUM_SPEC;
impl crate::RegisterSpec for SUM_WR_DATA_SUM_SPEC {
    type Ux = u32;
}
///`read()` method returns [sum_wr_data_sum::R](R) reader structure
impl crate::Readable for SUM_WR_DATA_SUM_SPEC {
    type Reader = R;
}
///`reset()` method sets SUM to value 0xffff
impl crate::Resettable for SUM_WR_DATA_SUM_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xffff
    }
}
