#[doc = "Register `MAC_RX_FLOW_CTRL` reader"]
pub struct R(crate::R<MAC_RX_FLOW_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MAC_RX_FLOW_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MAC_RX_FLOW_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MAC_RX_FLOW_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MAC_RX_FLOW_CTRL` writer"]
pub struct W(crate::W<MAC_RX_FLOW_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MAC_RX_FLOW_CTRL_SPEC>;
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
impl From<crate::W<MAC_RX_FLOW_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MAC_RX_FLOW_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RFE` reader - Receive Flow Control Enable When this bit is set and the MAC is operating in full-duplex mode, the MAC decodes the received Pause packet and disables its transmitter for a specified (Pause) time."]
pub struct RFE_R(crate::FieldReader<bool, bool>);
impl RFE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RFE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RFE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RFE` writer - Receive Flow Control Enable When this bit is set and the MAC is operating in full-duplex mode, the MAC decodes the received Pause packet and disables its transmitter for a specified (Pause) time."]
pub struct RFE_W<'a> {
    w: &'a mut W,
}
impl<'a> RFE_W<'a> {
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
#[doc = "Field `UP` reader - Unicast Pause Packet Detect A pause packet is processed when it has the unique multicast address specified in the IEEE 802."]
pub struct UP_R(crate::FieldReader<bool, bool>);
impl UP_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        UP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UP` writer - Unicast Pause Packet Detect A pause packet is processed when it has the unique multicast address specified in the IEEE 802."]
pub struct UP_W<'a> {
    w: &'a mut W,
}
impl<'a> UP_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Receive Flow Control Enable When this bit is set and the MAC is operating in full-duplex mode, the MAC decodes the received Pause packet and disables its transmitter for a specified (Pause) time."]
    #[inline(always)]
    pub fn rfe(&self) -> RFE_R {
        RFE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Unicast Pause Packet Detect A pause packet is processed when it has the unique multicast address specified in the IEEE 802."]
    #[inline(always)]
    pub fn up(&self) -> UP_R {
        UP_R::new(((self.bits >> 1) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Receive Flow Control Enable When this bit is set and the MAC is operating in full-duplex mode, the MAC decodes the received Pause packet and disables its transmitter for a specified (Pause) time."]
    #[inline(always)]
    pub fn rfe(&mut self) -> RFE_W {
        RFE_W { w: self }
    }
    #[doc = "Bit 1 - Unicast Pause Packet Detect A pause packet is processed when it has the unique multicast address specified in the IEEE 802."]
    #[inline(always)]
    pub fn up(&mut self) -> UP_W {
        UP_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Receive flow control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mac_rx_flow_ctrl](index.html) module"]
pub struct MAC_RX_FLOW_CTRL_SPEC;
impl crate::RegisterSpec for MAC_RX_FLOW_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mac_rx_flow_ctrl::R](R) reader structure"]
impl crate::Readable for MAC_RX_FLOW_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mac_rx_flow_ctrl::W](W) writer structure"]
impl crate::Writable for MAC_RX_FLOW_CTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MAC_RX_FLOW_CTRL to value 0"]
impl crate::Resettable for MAC_RX_FLOW_CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
