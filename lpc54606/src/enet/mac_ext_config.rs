#[doc = "Reader of register MAC_EXT_CONFIG"]
pub type R = crate::R<u32, super::MAC_EXT_CONFIG>;
#[doc = "Writer for register MAC_EXT_CONFIG"]
pub type W = crate::W<u32, super::MAC_EXT_CONFIG>;
#[doc = "Register MAC_EXT_CONFIG `reset()`'s with value 0"]
impl crate::ResetValue for super::MAC_EXT_CONFIG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `GPSL`"]
pub type GPSL_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `GPSL`"]
pub struct GPSL_W<'a> {
    w: &'a mut W,
}
impl<'a> GPSL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3fff) | ((value as u32) & 0x3fff);
        self.w
    }
}
#[doc = "Reader of field `DCRCC`"]
pub type DCRCC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DCRCC`"]
pub struct DCRCC_W<'a> {
    w: &'a mut W,
}
impl<'a> DCRCC_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
        self.w
    }
}
#[doc = "Reader of field `SPEN`"]
pub type SPEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SPEN`"]
pub struct SPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SPEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | (((value as u32) & 0x01) << 17);
        self.w
    }
}
#[doc = "Reader of field `USP`"]
pub type USP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `USP`"]
pub struct USP_W<'a> {
    w: &'a mut W,
}
impl<'a> USP_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | (((value as u32) & 0x01) << 18);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:13 - Giant Packet Size Limit If the received packet size is greater than the value programmed in this field in units of bytes, the MAC declares the received packet as Giant packet."]
    #[inline(always)]
    pub fn gpsl(&self) -> GPSL_R {
        GPSL_R::new((self.bits & 0x3fff) as u16)
    }
    #[doc = "Bit 16 - Disable CRC Checking for Received Packets When this bit is set, the MAC receiver does not check the CRC field in the received packets."]
    #[inline(always)]
    pub fn dcrcc(&self) -> DCRCC_R {
        DCRCC_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Slow Protocol Detection Enable When this bit is set, MAC processes the Slow Protocol packets (Ether Type 0x8809) and provides the Rx status."]
    #[inline(always)]
    pub fn spen(&self) -> SPEN_R {
        SPEN_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Unicast Slow Protocol Packet Detect When this bit is set, the MAC detects the Slow Protocol packets with unicast address of the station specified in the MAC Address High Table 747 and MAC Address Low Table 748 registers."]
    #[inline(always)]
    pub fn usp(&self) -> USP_R {
        USP_R::new(((self.bits >> 18) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:13 - Giant Packet Size Limit If the received packet size is greater than the value programmed in this field in units of bytes, the MAC declares the received packet as Giant packet."]
    #[inline(always)]
    pub fn gpsl(&mut self) -> GPSL_W {
        GPSL_W { w: self }
    }
    #[doc = "Bit 16 - Disable CRC Checking for Received Packets When this bit is set, the MAC receiver does not check the CRC field in the received packets."]
    #[inline(always)]
    pub fn dcrcc(&mut self) -> DCRCC_W {
        DCRCC_W { w: self }
    }
    #[doc = "Bit 17 - Slow Protocol Detection Enable When this bit is set, MAC processes the Slow Protocol packets (Ether Type 0x8809) and provides the Rx status."]
    #[inline(always)]
    pub fn spen(&mut self) -> SPEN_W {
        SPEN_W { w: self }
    }
    #[doc = "Bit 18 - Unicast Slow Protocol Packet Detect When this bit is set, the MAC detects the Slow Protocol packets with unicast address of the station specified in the MAC Address High Table 747 and MAC Address Low Table 748 registers."]
    #[inline(always)]
    pub fn usp(&mut self) -> USP_W {
        USP_W { w: self }
    }
}
