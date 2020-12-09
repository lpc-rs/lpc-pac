#[doc = "Reader of register USBCMD"]
pub type R = crate::R<u32, super::USBCMD>;
#[doc = "Writer for register USBCMD"]
pub type W = crate::W<u32, super::USBCMD>;
#[doc = "Register USBCMD `reset()`'s with value 0"]
impl crate::ResetValue for super::USBCMD {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RS`"]
pub type RS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RS`"]
pub struct RS_W<'a> {
    w: &'a mut W,
}
impl<'a> RS_W<'a> {
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
#[doc = "Reader of field `HCRESET`"]
pub type HCRESET_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `HCRESET`"]
pub struct HCRESET_W<'a> {
    w: &'a mut W,
}
impl<'a> HCRESET_W<'a> {
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
#[doc = "Reader of field `FLS`"]
pub type FLS_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `FLS`"]
pub struct FLS_W<'a> {
    w: &'a mut W,
}
impl<'a> FLS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | (((value as u32) & 0x03) << 2);
        self.w
    }
}
#[doc = "Reader of field `LHCR`"]
pub type LHCR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LHCR`"]
pub struct LHCR_W<'a> {
    w: &'a mut W,
}
impl<'a> LHCR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
#[doc = "Reader of field `ATL_EN`"]
pub type ATL_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ATL_EN`"]
pub struct ATL_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> ATL_EN_W<'a> {
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
#[doc = "Reader of field `ISO_EN`"]
pub type ISO_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ISO_EN`"]
pub struct ISO_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> ISO_EN_W<'a> {
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
#[doc = "Reader of field `INT_EN`"]
pub type INT_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `INT_EN`"]
pub struct INT_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> INT_EN_W<'a> {
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
#[doc = "Reader of field `HIRD`"]
pub type HIRD_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `HIRD`"]
pub struct HIRD_W<'a> {
    w: &'a mut W,
}
impl<'a> HIRD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 24)) | (((value as u32) & 0x0f) << 24);
        self.w
    }
}
#[doc = "Reader of field `LPM_RWU`"]
pub type LPM_RWU_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LPM_RWU`"]
pub struct LPM_RWU_W<'a> {
    w: &'a mut W,
}
impl<'a> LPM_RWU_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 28)) | (((value as u32) & 0x01) << 28);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Run/Stop: 1b = Run."]
    #[inline(always)]
    pub fn rs(&self) -> RS_R {
        RS_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Host Controller Reset: This control bit is used by the software to reset the host controller."]
    #[inline(always)]
    pub fn hcreset(&self) -> HCRESET_R {
        HCRESET_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bits 2:3 - Frame List Size: This field specifies the size of the frame list."]
    #[inline(always)]
    pub fn fls(&self) -> FLS_R {
        FLS_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bit 7 - Light Host Controller Reset: This bit allows the driver software to reset the host controller without affecting the state of the ports."]
    #[inline(always)]
    pub fn lhcr(&self) -> LHCR_R {
        LHCR_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - ATL List enabled."]
    #[inline(always)]
    pub fn atl_en(&self) -> ATL_EN_R {
        ATL_EN_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - ISO List enabled."]
    #[inline(always)]
    pub fn iso_en(&self) -> ISO_EN_R {
        ISO_EN_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - INT List enabled."]
    #[inline(always)]
    pub fn int_en(&self) -> INT_EN_R {
        INT_EN_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bits 24:27 - Host-Initiated Resume Duration."]
    #[inline(always)]
    pub fn hird(&self) -> HIRD_R {
        HIRD_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bit 28 - bRemoteWake field."]
    #[inline(always)]
    pub fn lpm_rwu(&self) -> LPM_RWU_R {
        LPM_RWU_R::new(((self.bits >> 28) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Run/Stop: 1b = Run."]
    #[inline(always)]
    pub fn rs(&mut self) -> RS_W {
        RS_W { w: self }
    }
    #[doc = "Bit 1 - Host Controller Reset: This control bit is used by the software to reset the host controller."]
    #[inline(always)]
    pub fn hcreset(&mut self) -> HCRESET_W {
        HCRESET_W { w: self }
    }
    #[doc = "Bits 2:3 - Frame List Size: This field specifies the size of the frame list."]
    #[inline(always)]
    pub fn fls(&mut self) -> FLS_W {
        FLS_W { w: self }
    }
    #[doc = "Bit 7 - Light Host Controller Reset: This bit allows the driver software to reset the host controller without affecting the state of the ports."]
    #[inline(always)]
    pub fn lhcr(&mut self) -> LHCR_W {
        LHCR_W { w: self }
    }
    #[doc = "Bit 8 - ATL List enabled."]
    #[inline(always)]
    pub fn atl_en(&mut self) -> ATL_EN_W {
        ATL_EN_W { w: self }
    }
    #[doc = "Bit 9 - ISO List enabled."]
    #[inline(always)]
    pub fn iso_en(&mut self) -> ISO_EN_W {
        ISO_EN_W { w: self }
    }
    #[doc = "Bit 10 - INT List enabled."]
    #[inline(always)]
    pub fn int_en(&mut self) -> INT_EN_W {
        INT_EN_W { w: self }
    }
    #[doc = "Bits 24:27 - Host-Initiated Resume Duration."]
    #[inline(always)]
    pub fn hird(&mut self) -> HIRD_W {
        HIRD_W { w: self }
    }
    #[doc = "Bit 28 - bRemoteWake field."]
    #[inline(always)]
    pub fn lpm_rwu(&mut self) -> LPM_RWU_W {
        LPM_RWU_W { w: self }
    }
}
