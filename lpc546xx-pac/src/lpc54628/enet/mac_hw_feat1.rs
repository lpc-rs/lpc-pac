#[doc = "Register `MAC_HW_FEAT1` reader"]
pub struct R(crate::R<MAC_HW_FEAT1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MAC_HW_FEAT1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MAC_HW_FEAT1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MAC_HW_FEAT1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RXFIFOSIZE` reader - MTL Receive FIFO Size."]
pub struct RXFIFOSIZE_R(crate::FieldReader<u8, u8>);
impl RXFIFOSIZE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        RXFIFOSIZE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXFIFOSIZE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXFIFOSIZE` reader - MTL Transmit FIFO Size."]
pub struct TXFIFOSIZE_R(crate::FieldReader<u8, u8>);
impl TXFIFOSIZE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        TXFIFOSIZE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXFIFOSIZE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OSTEN` reader - One-Step Timestamping Feature."]
pub struct OSTEN_R(crate::FieldReader<bool, bool>);
impl OSTEN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        OSTEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OSTEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PTOEN` reader - PTP OffLoad Feature."]
pub struct PTOEN_R(crate::FieldReader<bool, bool>);
impl PTOEN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PTOEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PTOEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADVTHWORD` reader - IEEE 1588 High Word Register Feature."]
pub struct ADVTHWORD_R(crate::FieldReader<bool, bool>);
impl ADVTHWORD_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ADVTHWORD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADVTHWORD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADDR64` reader - Address width."]
pub struct ADDR64_R(crate::FieldReader<u8, u8>);
impl ADDR64_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        ADDR64_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADDR64_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DCBEN` reader - Data Center Bridging feature."]
pub struct DCBEN_R(crate::FieldReader<bool, bool>);
impl DCBEN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DCBEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DCBEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SPEN` reader - Split Header Structure feature."]
pub struct SPEN_R(crate::FieldReader<bool, bool>);
impl SPEN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SPEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SPEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TSOEN` reader - TCP Segment Offload Feature."]
pub struct TSOEN_R(crate::FieldReader<bool, bool>);
impl TSOEN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TSOEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TSOEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DBGMEMA` reader - DMA Debug Register Feature."]
pub struct DBGMEMA_R(crate::FieldReader<bool, bool>);
impl DBGMEMA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DBGMEMA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DBGMEMA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AVSEL` reader - Audio Video Bridging Feature."]
pub struct AVSEL_R(crate::FieldReader<bool, bool>);
impl AVSEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        AVSEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AVSEL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LPMODEEN` reader - Low Power Mode Feature Support ."]
pub struct LPMODEEN_R(crate::FieldReader<bool, bool>);
impl LPMODEEN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        LPMODEEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LPMODEEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HASHTBLSZ` reader - Hash Table Size."]
pub struct HASHTBLSZ_R(crate::FieldReader<u8, u8>);
impl HASHTBLSZ_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        HASHTBLSZ_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HASHTBLSZ_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `L3_L4_FILTER` reader - Total Number of L3 and L4 Filters ."]
pub struct L3_L4_FILTER_R(crate::FieldReader<u8, u8>);
impl L3_L4_FILTER_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        L3_L4_FILTER_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for L3_L4_FILTER_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:4 - MTL Receive FIFO Size."]
    #[inline(always)]
    pub fn rxfifosize(&self) -> RXFIFOSIZE_R {
        RXFIFOSIZE_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 6:10 - MTL Transmit FIFO Size."]
    #[inline(always)]
    pub fn txfifosize(&self) -> TXFIFOSIZE_R {
        TXFIFOSIZE_R::new(((self.bits >> 6) & 0x1f) as u8)
    }
    #[doc = "Bit 11 - One-Step Timestamping Feature."]
    #[inline(always)]
    pub fn osten(&self) -> OSTEN_R {
        OSTEN_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - PTP OffLoad Feature."]
    #[inline(always)]
    pub fn ptoen(&self) -> PTOEN_R {
        PTOEN_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - IEEE 1588 High Word Register Feature."]
    #[inline(always)]
    pub fn advthword(&self) -> ADVTHWORD_R {
        ADVTHWORD_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bits 14:15 - Address width."]
    #[inline(always)]
    pub fn addr64(&self) -> ADDR64_R {
        ADDR64_R::new(((self.bits >> 14) & 0x03) as u8)
    }
    #[doc = "Bit 16 - Data Center Bridging feature."]
    #[inline(always)]
    pub fn dcben(&self) -> DCBEN_R {
        DCBEN_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Split Header Structure feature."]
    #[inline(always)]
    pub fn spen(&self) -> SPEN_R {
        SPEN_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - TCP Segment Offload Feature."]
    #[inline(always)]
    pub fn tsoen(&self) -> TSOEN_R {
        TSOEN_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - DMA Debug Register Feature."]
    #[inline(always)]
    pub fn dbgmema(&self) -> DBGMEMA_R {
        DBGMEMA_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Audio Video Bridging Feature."]
    #[inline(always)]
    pub fn avsel(&self) -> AVSEL_R {
        AVSEL_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 23 - Low Power Mode Feature Support ."]
    #[inline(always)]
    pub fn lpmodeen(&self) -> LPMODEEN_R {
        LPMODEEN_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bits 24:25 - Hash Table Size."]
    #[inline(always)]
    pub fn hashtblsz(&self) -> HASHTBLSZ_R {
        HASHTBLSZ_R::new(((self.bits >> 24) & 0x03) as u8)
    }
    #[doc = "Bits 27:30 - Total Number of L3 and L4 Filters ."]
    #[inline(always)]
    pub fn l3_l4_filter(&self) -> L3_L4_FILTER_R {
        L3_L4_FILTER_R::new(((self.bits >> 27) & 0x0f) as u8)
    }
}
#[doc = "MAC hardware feature register 0x0201\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mac_hw_feat1](index.html) module"]
pub struct MAC_HW_FEAT1_SPEC;
impl crate::RegisterSpec for MAC_HW_FEAT1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mac_hw_feat1::R](R) reader structure"]
impl crate::Readable for MAC_HW_FEAT1_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets MAC_HW_FEAT1 to value 0"]
impl crate::Resettable for MAC_HW_FEAT1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
