#[doc = "Register `MAC_Tx_TIMESTAMP_STATUS_NANOSECONDS` reader"]
pub struct R(crate::R<MAC_TX_TIMESTAMP_STATUS_NANOSECONDS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MAC_TX_TIMESTAMP_STATUS_NANOSECONDS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MAC_TX_TIMESTAMP_STATUS_NANOSECONDS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MAC_TX_TIMESTAMP_STATUS_NANOSECONDS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `TXTSSTSLO` reader - Transmit timestamp status low."]
pub struct TXTSSTSLO_R(crate::FieldReader<u32, u32>);
impl TXTSSTSLO_R {
    pub(crate) fn new(bits: u32) -> Self {
        TXTSSTSLO_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXTSSTSLO_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXTSSTSMIS` reader - Transmit timestamp status missed."]
pub struct TXTSSTSMIS_R(crate::FieldReader<bool, bool>);
impl TXTSSTSMIS_R {
    pub(crate) fn new(bits: bool) -> Self {
        TXTSSTSMIS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXTSSTSMIS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:30 - Transmit timestamp status low."]
    #[inline(always)]
    pub fn txtsstslo(&self) -> TXTSSTSLO_R {
        TXTSSTSLO_R::new((self.bits & 0x7fff_ffff) as u32)
    }
    #[doc = "Bit 31 - Transmit timestamp status missed."]
    #[inline(always)]
    pub fn txtsstsmis(&self) -> TXTSSTSMIS_R {
        TXTSSTSMIS_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
#[doc = "Tx timestamp status nanoseconds\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mac_tx_timestamp_status_nanoseconds](index.html) module"]
pub struct MAC_TX_TIMESTAMP_STATUS_NANOSECONDS_SPEC;
impl crate::RegisterSpec for MAC_TX_TIMESTAMP_STATUS_NANOSECONDS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mac_tx_timestamp_status_nanoseconds::R](R) reader structure"]
impl crate::Readable for MAC_TX_TIMESTAMP_STATUS_NANOSECONDS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets MAC_Tx_TIMESTAMP_STATUS_NANOSECONDS to value 0"]
impl crate::Resettable for MAC_TX_TIMESTAMP_STATUS_NANOSECONDS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
