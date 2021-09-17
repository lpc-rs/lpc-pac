#[doc = "Register `MIS` reader"]
pub struct R(crate::R<MIS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MIS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MIS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MIS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RORMIS` reader - This bit is 1 if another frame was completely received while the RxFIFO was full, and this interrupt is enabled."]
pub struct RORMIS_R(crate::FieldReader<bool, bool>);
impl RORMIS_R {
    pub(crate) fn new(bits: bool) -> Self {
        RORMIS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RORMIS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RTMIS` reader - This bit is 1 if the Rx FIFO is not empty, has not been read for a time-out period, and this interrupt is enabled. The time-out period is the same for master and slave modes and is determined by the SSP bit rate: 32 bits at PCLK / (CPSDVSR X \\[SCR+1\\])."]
pub struct RTMIS_R(crate::FieldReader<bool, bool>);
impl RTMIS_R {
    pub(crate) fn new(bits: bool) -> Self {
        RTMIS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RTMIS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXMIS` reader - This bit is 1 if the Rx FIFO is at least half full, and this interrupt is enabled."]
pub struct RXMIS_R(crate::FieldReader<bool, bool>);
impl RXMIS_R {
    pub(crate) fn new(bits: bool) -> Self {
        RXMIS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXMIS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXMIS` reader - This bit is 1 if the Tx FIFO is at least half empty, and this interrupt is enabled."]
pub struct TXMIS_R(crate::FieldReader<bool, bool>);
impl TXMIS_R {
    pub(crate) fn new(bits: bool) -> Self {
        TXMIS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXMIS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - This bit is 1 if another frame was completely received while the RxFIFO was full, and this interrupt is enabled."]
    #[inline(always)]
    pub fn rormis(&self) -> RORMIS_R {
        RORMIS_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - This bit is 1 if the Rx FIFO is not empty, has not been read for a time-out period, and this interrupt is enabled. The time-out period is the same for master and slave modes and is determined by the SSP bit rate: 32 bits at PCLK / (CPSDVSR X \\[SCR+1\\])."]
    #[inline(always)]
    pub fn rtmis(&self) -> RTMIS_R {
        RTMIS_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - This bit is 1 if the Rx FIFO is at least half full, and this interrupt is enabled."]
    #[inline(always)]
    pub fn rxmis(&self) -> RXMIS_R {
        RXMIS_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - This bit is 1 if the Tx FIFO is at least half empty, and this interrupt is enabled."]
    #[inline(always)]
    pub fn txmis(&self) -> TXMIS_R {
        TXMIS_R::new(((self.bits >> 3) & 0x01) != 0)
    }
}
#[doc = "Masked Interrupt Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mis](index.html) module"]
pub struct MIS_SPEC;
impl crate::RegisterSpec for MIS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mis::R](R) reader structure"]
impl crate::Readable for MIS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets MIS to value 0"]
impl crate::Resettable for MIS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
