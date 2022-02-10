///Register `MAC_Tx_TIMESTAMP_STATUS_SECONDS` reader
pub struct R(crate::R<MAC_TX_TIMESTAMP_STATUS_SECONDS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MAC_TX_TIMESTAMP_STATUS_SECONDS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MAC_TX_TIMESTAMP_STATUS_SECONDS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MAC_TX_TIMESTAMP_STATUS_SECONDS_SPEC>) -> Self {
        R(reader)
    }
}
///Field `TXTSSTSHI` reader - Transmit timestamp status high.
pub struct TXTSSTSHI_R(crate::FieldReader<u32, u32>);
impl TXTSSTSHI_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        TXTSSTSHI_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXTSSTSHI_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    ///Bits 0:31 - Transmit timestamp status high.
    #[inline(always)]
    pub fn txtsstshi(&self) -> TXTSSTSHI_R {
        TXTSSTSHI_R::new(self.bits)
    }
}
///Tx timestamp status seconds
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [mac_tx_timestamp_status_seconds](index.html) module
pub struct MAC_TX_TIMESTAMP_STATUS_SECONDS_SPEC;
impl crate::RegisterSpec for MAC_TX_TIMESTAMP_STATUS_SECONDS_SPEC {
    type Ux = u32;
}
///`read()` method returns [mac_tx_timestamp_status_seconds::R](R) reader structure
impl crate::Readable for MAC_TX_TIMESTAMP_STATUS_SECONDS_SPEC {
    type Reader = R;
}
///`reset()` method sets MAC_Tx_TIMESTAMP_STATUS_SECONDS to value 0
impl crate::Resettable for MAC_TX_TIMESTAMP_STATUS_SECONDS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
