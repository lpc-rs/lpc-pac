///Register `MTL_RXQ_DMA_MAP` reader
pub struct R(crate::R<MTL_RXQ_DMA_MAP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MTL_RXQ_DMA_MAP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MTL_RXQ_DMA_MAP_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MTL_RXQ_DMA_MAP_SPEC>) -> Self {
        R(reader)
    }
}
///Register `MTL_RXQ_DMA_MAP` writer
pub struct W(crate::W<MTL_RXQ_DMA_MAP_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MTL_RXQ_DMA_MAP_SPEC>;
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
impl From<crate::W<MTL_RXQ_DMA_MAP_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MTL_RXQ_DMA_MAP_SPEC>) -> Self {
        W(writer)
    }
}
///Field `Q0MDMACH` reader - Queue 0 Mapped to DMA Channel This field controls the routing of the packet received in Queue 0 to the DMA channel: 0: DMA Channel 0 1: DMA Channel 1 This field is valid when the Q0DDMACH field is reset.
pub struct Q0MDMACH_R(crate::FieldReader<bool, bool>);
impl Q0MDMACH_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        Q0MDMACH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for Q0MDMACH_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `Q0MDMACH` writer - Queue 0 Mapped to DMA Channel This field controls the routing of the packet received in Queue 0 to the DMA channel: 0: DMA Channel 0 1: DMA Channel 1 This field is valid when the Q0DDMACH field is reset.
pub struct Q0MDMACH_W<'a> {
    w: &'a mut W,
}
impl<'a> Q0MDMACH_W<'a> {
    ///Sets the field bit
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    ///Clears the field bit
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
///Field `Q0DDMACH` reader - Queue 0 Enabled for DA-based DMA Channel Selection When set, this bit indicates that the packets received in Queue 0 are routed to a particular DMA channel as decided in the MAC Receiver based on the DMA channel number programmed in the L3-L4 filter registers, or the Ethernet DA address.
pub struct Q0DDMACH_R(crate::FieldReader<bool, bool>);
impl Q0DDMACH_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        Q0DDMACH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for Q0DDMACH_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `Q0DDMACH` writer - Queue 0 Enabled for DA-based DMA Channel Selection When set, this bit indicates that the packets received in Queue 0 are routed to a particular DMA channel as decided in the MAC Receiver based on the DMA channel number programmed in the L3-L4 filter registers, or the Ethernet DA address.
pub struct Q0DDMACH_W<'a> {
    w: &'a mut W,
}
impl<'a> Q0DDMACH_W<'a> {
    ///Sets the field bit
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    ///Clears the field bit
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 4)) | ((value as u32 & 0x01) << 4);
        self.w
    }
}
///Field `Q1MDMACH` reader - Queue 1 Mapped to DMA Channel This field controls the routing of the received packet in Queue 1 to the DMA channel: 0: DMA Channel 0 1: DMA Channel 1 This field is valid when the Q1DDMACH field is reset.
pub struct Q1MDMACH_R(crate::FieldReader<bool, bool>);
impl Q1MDMACH_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        Q1MDMACH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for Q1MDMACH_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `Q1MDMACH` writer - Queue 1 Mapped to DMA Channel This field controls the routing of the received packet in Queue 1 to the DMA channel: 0: DMA Channel 0 1: DMA Channel 1 This field is valid when the Q1DDMACH field is reset.
pub struct Q1MDMACH_W<'a> {
    w: &'a mut W,
}
impl<'a> Q1MDMACH_W<'a> {
    ///Sets the field bit
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    ///Clears the field bit
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 8)) | ((value as u32 & 0x01) << 8);
        self.w
    }
}
///Field `Q1DDMACH` reader - Queue 1 Enabled for DA-based DMA Channel Selection When set, this bit indicates that the packets received in Queue 1 are routed to a particular DMA channel as decided in the MAC Receiver based on the DMA channel number programmed in the L3-L4 filter registers, or the Ethernet DA address.
pub struct Q1DDMACH_R(crate::FieldReader<bool, bool>);
impl Q1DDMACH_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        Q1DDMACH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for Q1DDMACH_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `Q1DDMACH` writer - Queue 1 Enabled for DA-based DMA Channel Selection When set, this bit indicates that the packets received in Queue 1 are routed to a particular DMA channel as decided in the MAC Receiver based on the DMA channel number programmed in the L3-L4 filter registers, or the Ethernet DA address.
pub struct Q1DDMACH_W<'a> {
    w: &'a mut W,
}
impl<'a> Q1DDMACH_W<'a> {
    ///Sets the field bit
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    ///Clears the field bit
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 12)) | ((value as u32 & 0x01) << 12);
        self.w
    }
}
impl R {
    ///Bit 0 - Queue 0 Mapped to DMA Channel This field controls the routing of the packet received in Queue 0 to the DMA channel: 0: DMA Channel 0 1: DMA Channel 1 This field is valid when the Q0DDMACH field is reset.
    #[inline(always)]
    pub fn q0mdmach(&self) -> Q0MDMACH_R {
        Q0MDMACH_R::new((self.bits & 0x01) != 0)
    }
    ///Bit 4 - Queue 0 Enabled for DA-based DMA Channel Selection When set, this bit indicates that the packets received in Queue 0 are routed to a particular DMA channel as decided in the MAC Receiver based on the DMA channel number programmed in the L3-L4 filter registers, or the Ethernet DA address.
    #[inline(always)]
    pub fn q0ddmach(&self) -> Q0DDMACH_R {
        Q0DDMACH_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    ///Bit 8 - Queue 1 Mapped to DMA Channel This field controls the routing of the received packet in Queue 1 to the DMA channel: 0: DMA Channel 0 1: DMA Channel 1 This field is valid when the Q1DDMACH field is reset.
    #[inline(always)]
    pub fn q1mdmach(&self) -> Q1MDMACH_R {
        Q1MDMACH_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    ///Bit 12 - Queue 1 Enabled for DA-based DMA Channel Selection When set, this bit indicates that the packets received in Queue 1 are routed to a particular DMA channel as decided in the MAC Receiver based on the DMA channel number programmed in the L3-L4 filter registers, or the Ethernet DA address.
    #[inline(always)]
    pub fn q1ddmach(&self) -> Q1DDMACH_R {
        Q1DDMACH_R::new(((self.bits >> 12) & 0x01) != 0)
    }
}
impl W {
    ///Bit 0 - Queue 0 Mapped to DMA Channel This field controls the routing of the packet received in Queue 0 to the DMA channel: 0: DMA Channel 0 1: DMA Channel 1 This field is valid when the Q0DDMACH field is reset.
    #[inline(always)]
    pub fn q0mdmach(&mut self) -> Q0MDMACH_W {
        Q0MDMACH_W { w: self }
    }
    ///Bit 4 - Queue 0 Enabled for DA-based DMA Channel Selection When set, this bit indicates that the packets received in Queue 0 are routed to a particular DMA channel as decided in the MAC Receiver based on the DMA channel number programmed in the L3-L4 filter registers, or the Ethernet DA address.
    #[inline(always)]
    pub fn q0ddmach(&mut self) -> Q0DDMACH_W {
        Q0DDMACH_W { w: self }
    }
    ///Bit 8 - Queue 1 Mapped to DMA Channel This field controls the routing of the received packet in Queue 1 to the DMA channel: 0: DMA Channel 0 1: DMA Channel 1 This field is valid when the Q1DDMACH field is reset.
    #[inline(always)]
    pub fn q1mdmach(&mut self) -> Q1MDMACH_W {
        Q1MDMACH_W { w: self }
    }
    ///Bit 12 - Queue 1 Enabled for DA-based DMA Channel Selection When set, this bit indicates that the packets received in Queue 1 are routed to a particular DMA channel as decided in the MAC Receiver based on the DMA channel number programmed in the L3-L4 filter registers, or the Ethernet DA address.
    #[inline(always)]
    pub fn q1ddmach(&mut self) -> Q1DDMACH_W {
        Q1DDMACH_W { w: self }
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///MTL Receive Queue and DMA Channel Mapping register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [mtl_rxq_dma_map](index.html) module
pub struct MTL_RXQ_DMA_MAP_SPEC;
impl crate::RegisterSpec for MTL_RXQ_DMA_MAP_SPEC {
    type Ux = u32;
}
///`read()` method returns [mtl_rxq_dma_map::R](R) reader structure
impl crate::Readable for MTL_RXQ_DMA_MAP_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [mtl_rxq_dma_map::W](W) writer structure
impl crate::Writable for MTL_RXQ_DMA_MAP_SPEC {
    type Writer = W;
}
///`reset()` method sets MTL_RXQ_DMA_MAP to value 0
impl crate::Resettable for MTL_RXQ_DMA_MAP_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
