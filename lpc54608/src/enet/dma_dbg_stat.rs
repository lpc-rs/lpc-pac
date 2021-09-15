#[doc = "Register `DMA_DBG_STAT` reader"]
pub struct R(crate::R<DMA_DBG_STAT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMA_DBG_STAT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMA_DBG_STAT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMA_DBG_STAT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DMA_DBG_STAT` writer"]
pub struct W(crate::W<DMA_DBG_STAT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DMA_DBG_STAT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<DMA_DBG_STAT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DMA_DBG_STAT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `AHSTS` reader - AHB Master Status When high, this bit indicates that the AHB master FSMs are in the non-idle state."]
pub struct AHSTS_R(crate::FieldReader<bool, bool>);
impl AHSTS_R {
    pub(crate) fn new(bits: bool) -> Self {
        AHSTS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AHSTS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AHSTS` writer - AHB Master Status When high, this bit indicates that the AHB master FSMs are in the non-idle state."]
pub struct AHSTS_W<'a> {
    w: &'a mut W,
}
impl<'a> AHSTS_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
#[doc = "Field `RPS0` reader - DMA Channel 0 Receive Process State This field indicates the Rx DMA FSM state for Channel 0: 0x0: Stopped (Reset or Stop Receive Command issued) 0x1: Running (Fetching Rx Transfer ) 0x2: Reserved 0x3: Running (Waiting for Rx packet) 0x4: Suspended (Rx Unavailable) 0x5: Running (Closing the Rx) 0x6: Timestamp write state 0x7: Running (Transferring the received packet data from the Rx buffer to the system memory) This field does not generate an interrupt."]
pub struct RPS0_R(crate::FieldReader<u8, u8>);
impl RPS0_R {
    pub(crate) fn new(bits: u8) -> Self {
        RPS0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RPS0_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TPS0` reader - DMA Channel 0 Transmit Process State This field indicates the Tx DMA FSM state for Channel 0: 000: Stopped (Reset or Stop Transmit Command issued) 0x1: Running (Fetching Tx Transfer) 0x2: Running (Waiting for status) 0x3: Running (Reading Data from system memory buffer and queuing it to the Tx buffer (Tx FIFO)) 0x4: Timestamp write state 0x5: Reserved for future use 0x6: Suspended (Tx Unavailable or Tx Buffer Underflow) 0x7: Running (Closing Tx ) This field does not generate an interrupt."]
pub struct TPS0_R(crate::FieldReader<u8, u8>);
impl TPS0_R {
    pub(crate) fn new(bits: u8) -> Self {
        TPS0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TPS0_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RPS1` reader - DMA Channel 1 Receive Process State This field indicates the Rx DMA FSM state for Channel 1."]
pub struct RPS1_R(crate::FieldReader<u8, u8>);
impl RPS1_R {
    pub(crate) fn new(bits: u8) -> Self {
        RPS1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RPS1_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TPS1` reader - DMA Channel 1 Transmit Process State This field indicates the Tx DMA FSM state for Channel 1."]
pub struct TPS1_R(crate::FieldReader<u8, u8>);
impl TPS1_R {
    pub(crate) fn new(bits: u8) -> Self {
        TPS1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TPS1_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - AHB Master Status When high, this bit indicates that the AHB master FSMs are in the non-idle state."]
    #[inline(always)]
    pub fn ahsts(&self) -> AHSTS_R {
        AHSTS_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bits 8:11 - DMA Channel 0 Receive Process State This field indicates the Rx DMA FSM state for Channel 0: 0x0: Stopped (Reset or Stop Receive Command issued) 0x1: Running (Fetching Rx Transfer ) 0x2: Reserved 0x3: Running (Waiting for Rx packet) 0x4: Suspended (Rx Unavailable) 0x5: Running (Closing the Rx) 0x6: Timestamp write state 0x7: Running (Transferring the received packet data from the Rx buffer to the system memory) This field does not generate an interrupt."]
    #[inline(always)]
    pub fn rps0(&self) -> RPS0_R {
        RPS0_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - DMA Channel 0 Transmit Process State This field indicates the Tx DMA FSM state for Channel 0: 000: Stopped (Reset or Stop Transmit Command issued) 0x1: Running (Fetching Tx Transfer) 0x2: Running (Waiting for status) 0x3: Running (Reading Data from system memory buffer and queuing it to the Tx buffer (Tx FIFO)) 0x4: Timestamp write state 0x5: Reserved for future use 0x6: Suspended (Tx Unavailable or Tx Buffer Underflow) 0x7: Running (Closing Tx ) This field does not generate an interrupt."]
    #[inline(always)]
    pub fn tps0(&self) -> TPS0_R {
        TPS0_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - DMA Channel 1 Receive Process State This field indicates the Rx DMA FSM state for Channel 1."]
    #[inline(always)]
    pub fn rps1(&self) -> RPS1_R {
        RPS1_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - DMA Channel 1 Transmit Process State This field indicates the Tx DMA FSM state for Channel 1."]
    #[inline(always)]
    pub fn tps1(&self) -> TPS1_R {
        TPS1_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - AHB Master Status When high, this bit indicates that the AHB master FSMs are in the non-idle state."]
    #[inline(always)]
    pub fn ahsts(&mut self) -> AHSTS_W {
        AHSTS_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DMA Debug Status\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_dbg_stat](index.html) module"]
pub struct DMA_DBG_STAT_SPEC;
impl crate::RegisterSpec for DMA_DBG_STAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dma_dbg_stat::R](R) reader structure"]
impl crate::Readable for DMA_DBG_STAT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dma_dbg_stat::W](W) writer structure"]
impl crate::Writable for DMA_DBG_STAT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DMA_DBG_STAT to value 0"]
impl crate::Resettable for DMA_DBG_STAT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
