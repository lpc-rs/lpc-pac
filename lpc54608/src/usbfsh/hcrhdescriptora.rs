#[doc = "Reader of register HCRHDESCRIPTORA"]
pub type R = crate::R<u32, super::HCRHDESCRIPTORA>;
#[doc = "Writer for register HCRHDESCRIPTORA"]
pub type W = crate::W<u32, super::HCRHDESCRIPTORA>;
#[doc = "Register HCRHDESCRIPTORA `reset()`'s with value 0xff00_0902"]
impl crate::ResetValue for super::HCRHDESCRIPTORA {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0xff00_0902
    }
}
#[doc = "Reader of field `NDP`"]
pub type NDP_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `NDP`"]
pub struct NDP_W<'a> {
    w: &'a mut W,
}
impl<'a> NDP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
#[doc = "Reader of field `PSM`"]
pub type PSM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PSM`"]
pub struct PSM_W<'a> {
    w: &'a mut W,
}
impl<'a> PSM_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
#[doc = "Reader of field `NPS`"]
pub type NPS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `NPS`"]
pub struct NPS_W<'a> {
    w: &'a mut W,
}
impl<'a> NPS_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
        self.w
    }
}
#[doc = "Reader of field `DT`"]
pub type DT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DT`"]
pub struct DT_W<'a> {
    w: &'a mut W,
}
impl<'a> DT_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
        self.w
    }
}
#[doc = "Reader of field `OCPM`"]
pub type OCPM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OCPM`"]
pub struct OCPM_W<'a> {
    w: &'a mut W,
}
impl<'a> OCPM_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u32) & 0x01) << 11);
        self.w
    }
}
#[doc = "Reader of field `NOCP`"]
pub type NOCP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `NOCP`"]
pub struct NOCP_W<'a> {
    w: &'a mut W,
}
impl<'a> NOCP_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u32) & 0x01) << 12);
        self.w
    }
}
#[doc = "Reader of field `POTPGT`"]
pub type POTPGT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `POTPGT`"]
pub struct POTPGT_W<'a> {
    w: &'a mut W,
}
impl<'a> POTPGT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 24)) | (((value as u32) & 0xff) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - NumberDownstreamPorts These bits specify the number of downstream ports supported by the root hub."]
    #[inline(always)]
    pub fn ndp(&self) -> NDP_R {
        NDP_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 8 - PowerSwitchingMode This bit is used to specify how the power switching of the root hub ports is controlled."]
    #[inline(always)]
    pub fn psm(&self) -> PSM_R {
        PSM_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - NoPowerSwitching These bits are used to specify whether power switching is supported or port are always powered."]
    #[inline(always)]
    pub fn nps(&self) -> NPS_R {
        NPS_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - DeviceType This bit specifies that the root hub is not a compound device."]
    #[inline(always)]
    pub fn dt(&self) -> DT_R {
        DT_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - OverCurrentProtectionMode This bit describes how the overcurrent status for the root hub ports are reported."]
    #[inline(always)]
    pub fn ocpm(&self) -> OCPM_R {
        OCPM_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - NoOverCurrentProtection This bit describes how the overcurrent status for the root hub ports are reported."]
    #[inline(always)]
    pub fn nocp(&self) -> NOCP_R {
        NOCP_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bits 24:31 - PowerOnToPowerGoodTime This byte specifies the duration the HCD has to wait before accessing a powered-on port of the root hub."]
    #[inline(always)]
    pub fn potpgt(&self) -> POTPGT_R {
        POTPGT_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - NumberDownstreamPorts These bits specify the number of downstream ports supported by the root hub."]
    #[inline(always)]
    pub fn ndp(&mut self) -> NDP_W {
        NDP_W { w: self }
    }
    #[doc = "Bit 8 - PowerSwitchingMode This bit is used to specify how the power switching of the root hub ports is controlled."]
    #[inline(always)]
    pub fn psm(&mut self) -> PSM_W {
        PSM_W { w: self }
    }
    #[doc = "Bit 9 - NoPowerSwitching These bits are used to specify whether power switching is supported or port are always powered."]
    #[inline(always)]
    pub fn nps(&mut self) -> NPS_W {
        NPS_W { w: self }
    }
    #[doc = "Bit 10 - DeviceType This bit specifies that the root hub is not a compound device."]
    #[inline(always)]
    pub fn dt(&mut self) -> DT_W {
        DT_W { w: self }
    }
    #[doc = "Bit 11 - OverCurrentProtectionMode This bit describes how the overcurrent status for the root hub ports are reported."]
    #[inline(always)]
    pub fn ocpm(&mut self) -> OCPM_W {
        OCPM_W { w: self }
    }
    #[doc = "Bit 12 - NoOverCurrentProtection This bit describes how the overcurrent status for the root hub ports are reported."]
    #[inline(always)]
    pub fn nocp(&mut self) -> NOCP_W {
        NOCP_W { w: self }
    }
    #[doc = "Bits 24:31 - PowerOnToPowerGoodTime This byte specifies the duration the HCD has to wait before accessing a powered-on port of the root hub."]
    #[inline(always)]
    pub fn potpgt(&mut self) -> POTPGT_W {
        POTPGT_W { w: self }
    }
}
