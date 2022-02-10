///Register `MTL_RXQx_MISSPKT_OVRFLW_CNT` reader
pub struct R(crate::R<MTL_RXQX_MISSPKT_OVRFLW_CNT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MTL_RXQX_MISSPKT_OVRFLW_CNT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MTL_RXQX_MISSPKT_OVRFLW_CNT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MTL_RXQX_MISSPKT_OVRFLW_CNT_SPEC>) -> Self {
        R(reader)
    }
}
///Register `MTL_RXQx_MISSPKT_OVRFLW_CNT` writer
pub struct W(crate::W<MTL_RXQX_MISSPKT_OVRFLW_CNT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MTL_RXQX_MISSPKT_OVRFLW_CNT_SPEC>;
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
impl From<crate::W<MTL_RXQX_MISSPKT_OVRFLW_CNT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MTL_RXQX_MISSPKT_OVRFLW_CNT_SPEC>) -> Self {
        W(writer)
    }
}
///Field `OVFPKTCNT` reader - Overflow Packet Counter This field indicates the number of packets discarded by the Ethernet block because of Receive queue overflow.
pub struct OVFPKTCNT_R(crate::FieldReader<u16, u16>);
impl OVFPKTCNT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        OVFPKTCNT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OVFPKTCNT_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `OVFPKTCNT` writer - Overflow Packet Counter This field indicates the number of packets discarded by the Ethernet block because of Receive queue overflow.
pub struct OVFPKTCNT_W<'a> {
    w: &'a mut W,
}
impl<'a> OVFPKTCNT_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07ff) | (value as u32 & 0x07ff);
        self.w
    }
}
///Field `OVFCNTOVF` reader - Overflow Counter Overflow Bit When set, this bit indicates that the Rx Queue Overflow Packet Counter field crossed the maximum limit.
pub struct OVFCNTOVF_R(crate::FieldReader<bool, bool>);
impl OVFCNTOVF_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        OVFCNTOVF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OVFCNTOVF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    ///Bits 0:10 - Overflow Packet Counter This field indicates the number of packets discarded by the Ethernet block because of Receive queue overflow.
    #[inline(always)]
    pub fn ovfpktcnt(&self) -> OVFPKTCNT_R {
        OVFPKTCNT_R::new((self.bits & 0x07ff) as u16)
    }
    ///Bit 11 - Overflow Counter Overflow Bit When set, this bit indicates that the Rx Queue Overflow Packet Counter field crossed the maximum limit.
    #[inline(always)]
    pub fn ovfcntovf(&self) -> OVFCNTOVF_R {
        OVFCNTOVF_R::new(((self.bits >> 11) & 0x01) != 0)
    }
}
impl W {
    ///Bits 0:10 - Overflow Packet Counter This field indicates the number of packets discarded by the Ethernet block because of Receive queue overflow.
    #[inline(always)]
    pub fn ovfpktcnt(&mut self) -> OVFPKTCNT_W {
        OVFPKTCNT_W { w: self }
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///MTL RxQx Missed Packet Overflow Counter register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [mtl_rxqx_misspkt_ovrflw_cnt](index.html) module
pub struct MTL_RXQX_MISSPKT_OVRFLW_CNT_SPEC;
impl crate::RegisterSpec for MTL_RXQX_MISSPKT_OVRFLW_CNT_SPEC {
    type Ux = u32;
}
///`read()` method returns [mtl_rxqx_misspkt_ovrflw_cnt::R](R) reader structure
impl crate::Readable for MTL_RXQX_MISSPKT_OVRFLW_CNT_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [mtl_rxqx_misspkt_ovrflw_cnt::W](W) writer structure
impl crate::Writable for MTL_RXQX_MISSPKT_OVRFLW_CNT_SPEC {
    type Writer = W;
}
///`reset()` method sets MTL_RXQx_MISSPKT_OVRFLW_CNT to value 0
impl crate::Resettable for MTL_RXQX_MISSPKT_OVRFLW_CNT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
