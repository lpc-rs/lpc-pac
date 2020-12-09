#[doc = "Reader of register HCRHSTATUS"]
pub type R = crate::R<u32, super::HCRHSTATUS>;
#[doc = "Writer for register HCRHSTATUS"]
pub type W = crate::W<u32, super::HCRHSTATUS>;
#[doc = "Register HCRHSTATUS `reset()`'s with value 0"]
impl crate::ResetValue for super::HCRHSTATUS {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `LPS`"]
pub type LPS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LPS`"]
pub struct LPS_W<'a> {
    w: &'a mut W,
}
impl<'a> LPS_W<'a> {
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
#[doc = "Reader of field `OCI`"]
pub type OCI_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OCI`"]
pub struct OCI_W<'a> {
    w: &'a mut W,
}
impl<'a> OCI_W<'a> {
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
#[doc = "Reader of field `DRWE`"]
pub type DRWE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DRWE`"]
pub struct DRWE_W<'a> {
    w: &'a mut W,
}
impl<'a> DRWE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u32) & 0x01) << 15);
        self.w
    }
}
#[doc = "Reader of field `LPSC`"]
pub type LPSC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LPSC`"]
pub struct LPSC_W<'a> {
    w: &'a mut W,
}
impl<'a> LPSC_W<'a> {
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
#[doc = "Reader of field `OCIC`"]
pub type OCIC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OCIC`"]
pub struct OCIC_W<'a> {
    w: &'a mut W,
}
impl<'a> OCIC_W<'a> {
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
#[doc = "Reader of field `CRWE`"]
pub type CRWE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CRWE`"]
pub struct CRWE_W<'a> {
    w: &'a mut W,
}
impl<'a> CRWE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | (((value as u32) & 0x01) << 31);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - (read) LocalPowerStatus The Root Hub does not support the local power status feature; thus, this bit is always read as 0."]
    #[inline(always)]
    pub fn lps(&self) -> LPS_R {
        LPS_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - OverCurrentIndicator This bit reports overcurrent conditions when the global reporting is implemented."]
    #[inline(always)]
    pub fn oci(&self) -> OCI_R {
        OCI_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 15 - (read) DeviceRemoteWakeupEnable This bit enables a ConnectStatusChange bit as a resume event, causing a USBSUSPEND to USBRESUME state transition and setting the ResumeDetected interrupt."]
    #[inline(always)]
    pub fn drwe(&self) -> DRWE_R {
        DRWE_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - (read) LocalPowerStatusChange The root hub does not support the local power status feature."]
    #[inline(always)]
    pub fn lpsc(&self) -> LPSC_R {
        LPSC_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - OverCurrentIndicatorChange This bit is set by hardware when a change has occurred to the OCI field of this register."]
    #[inline(always)]
    pub fn ocic(&self) -> OCIC_R {
        OCIC_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 31 - (write) ClearRemoteWakeupEnable Writing a 1 clears DeviceRemoveWakeupEnable."]
    #[inline(always)]
    pub fn crwe(&self) -> CRWE_R {
        CRWE_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - (read) LocalPowerStatus The Root Hub does not support the local power status feature; thus, this bit is always read as 0."]
    #[inline(always)]
    pub fn lps(&mut self) -> LPS_W {
        LPS_W { w: self }
    }
    #[doc = "Bit 1 - OverCurrentIndicator This bit reports overcurrent conditions when the global reporting is implemented."]
    #[inline(always)]
    pub fn oci(&mut self) -> OCI_W {
        OCI_W { w: self }
    }
    #[doc = "Bit 15 - (read) DeviceRemoteWakeupEnable This bit enables a ConnectStatusChange bit as a resume event, causing a USBSUSPEND to USBRESUME state transition and setting the ResumeDetected interrupt."]
    #[inline(always)]
    pub fn drwe(&mut self) -> DRWE_W {
        DRWE_W { w: self }
    }
    #[doc = "Bit 16 - (read) LocalPowerStatusChange The root hub does not support the local power status feature."]
    #[inline(always)]
    pub fn lpsc(&mut self) -> LPSC_W {
        LPSC_W { w: self }
    }
    #[doc = "Bit 17 - OverCurrentIndicatorChange This bit is set by hardware when a change has occurred to the OCI field of this register."]
    #[inline(always)]
    pub fn ocic(&mut self) -> OCIC_W {
        OCIC_W { w: self }
    }
    #[doc = "Bit 31 - (write) ClearRemoteWakeupEnable Writing a 1 clears DeviceRemoveWakeupEnable."]
    #[inline(always)]
    pub fn crwe(&mut self) -> CRWE_W {
        CRWE_W { w: self }
    }
}
