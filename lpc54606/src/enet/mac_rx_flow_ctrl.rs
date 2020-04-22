#[doc = "Reader of register MAC_RX_FLOW_CTRL"]
pub type R = crate::R<u32, super::MAC_RX_FLOW_CTRL>;
#[doc = "Writer for register MAC_RX_FLOW_CTRL"]
pub type W = crate::W<u32, super::MAC_RX_FLOW_CTRL>;
#[doc = "Register MAC_RX_FLOW_CTRL `reset()`'s with value 0"]
impl crate::ResetValue for super::MAC_RX_FLOW_CTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RFE`"]
pub type RFE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RFE`"]
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
#[doc = "Reader of field `UP`"]
pub type UP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UP`"]
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
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
}
