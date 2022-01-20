#[doc = "Register `MAC_EXT_CONFIG` reader"]
pub struct R(crate::R<MAC_EXT_CONFIG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MAC_EXT_CONFIG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MAC_EXT_CONFIG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MAC_EXT_CONFIG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MAC_EXT_CONFIG` writer"]
pub struct W(crate::W<MAC_EXT_CONFIG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MAC_EXT_CONFIG_SPEC>;
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
impl From<crate::W<MAC_EXT_CONFIG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MAC_EXT_CONFIG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `GPSL` reader - Giant Packet Size Limit If the received packet size is greater than the value programmed in this field in units of bytes, the MAC declares the received packet as Giant packet."]
pub struct GPSL_R(crate::FieldReader<u16, u16>);
impl GPSL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        GPSL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPSL_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPSL` writer - Giant Packet Size Limit If the received packet size is greater than the value programmed in this field in units of bytes, the MAC declares the received packet as Giant packet."]
pub struct GPSL_W<'a> {
    w: &'a mut W,
}
impl<'a> GPSL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3fff) | (value as u32 & 0x3fff);
        self.w
    }
}
#[doc = "Field `DCRCC` reader - Disable CRC Checking for Received Packets When this bit is set, the MAC receiver does not check the CRC field in the received packets."]
pub struct DCRCC_R(crate::FieldReader<bool, bool>);
impl DCRCC_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DCRCC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DCRCC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DCRCC` writer - Disable CRC Checking for Received Packets When this bit is set, the MAC receiver does not check the CRC field in the received packets."]
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | ((value as u32 & 0x01) << 16);
        self.w
    }
}
#[doc = "Field `SPEN` reader - Slow Protocol Detection Enable When this bit is set, MAC processes the Slow Protocol packets (Ether Type 0x8809) and provides the Rx status."]
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
#[doc = "Field `SPEN` writer - Slow Protocol Detection Enable When this bit is set, MAC processes the Slow Protocol packets (Ether Type 0x8809) and provides the Rx status."]
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | ((value as u32 & 0x01) << 17);
        self.w
    }
}
#[doc = "Field `USP` reader - Unicast Slow Protocol Packet Detect When this bit is set, the MAC detects the Slow Protocol packets with unicast address of the station specified in the MAC Address High Table 747 and MAC Address Low Table 748 registers."]
pub struct USP_R(crate::FieldReader<bool, bool>);
impl USP_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        USP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for USP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `USP` writer - Unicast Slow Protocol Packet Detect When this bit is set, the MAC detects the Slow Protocol packets with unicast address of the station specified in the MAC Address High Table 747 and MAC Address Low Table 748 registers."]
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | ((value as u32 & 0x01) << 18);
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
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "no description available\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mac_ext_config](index.html) module"]
pub struct MAC_EXT_CONFIG_SPEC;
impl crate::RegisterSpec for MAC_EXT_CONFIG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mac_ext_config::R](R) reader structure"]
impl crate::Readable for MAC_EXT_CONFIG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mac_ext_config::W](W) writer structure"]
impl crate::Writable for MAC_EXT_CONFIG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MAC_EXT_CONFIG to value 0"]
impl crate::Resettable for MAC_EXT_CONFIG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
