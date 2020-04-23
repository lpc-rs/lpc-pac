#[doc = "Reader of register FROCTRL"]
pub type R = crate::R<u32, super::FROCTRL>;
#[doc = "Writer for register FROCTRL"]
pub type W = crate::W<u32, super::FROCTRL>;
#[doc = "Register FROCTRL `reset()`'s with value 0x4000"]
impl crate::ResetValue for super::FROCTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x4000
    }
}
#[doc = "Reader of field `TRIM`"]
pub type TRIM_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `TRIM`"]
pub struct TRIM_W<'a> {
    w: &'a mut W,
}
impl<'a> TRIM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3fff) | ((value as u32) & 0x3fff);
        self.w
    }
}
#[doc = "Reader of field `SEL`"]
pub type SEL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SEL`"]
pub struct SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> SEL_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | (((value as u32) & 0x01) << 14);
        self.w
    }
}
#[doc = "Reader of field `FREQTRIM`"]
pub type FREQTRIM_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `FREQTRIM`"]
pub struct FREQTRIM_W<'a> {
    w: &'a mut W,
}
impl<'a> FREQTRIM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | (((value as u32) & 0xff) << 16);
        self.w
    }
}
#[doc = "Reader of field `USBCLKADJ`"]
pub type USBCLKADJ_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `USBCLKADJ`"]
pub struct USBCLKADJ_W<'a> {
    w: &'a mut W,
}
impl<'a> USBCLKADJ_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | (((value as u32) & 0x01) << 24);
        self.w
    }
}
#[doc = "Reader of field `USBMODCHG`"]
pub type USBMODCHG_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `USBMODCHG`"]
pub struct USBMODCHG_W<'a> {
    w: &'a mut W,
}
impl<'a> USBMODCHG_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 25)) | (((value as u32) & 0x01) << 25);
        self.w
    }
}
#[doc = "Reader of field `HSPDCLK`"]
pub type HSPDCLK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `HSPDCLK`"]
pub struct HSPDCLK_W<'a> {
    w: &'a mut W,
}
impl<'a> HSPDCLK_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 30)) | (((value as u32) & 0x01) << 30);
        self.w
    }
}
#[doc = "Reader of field `WRTRIM`"]
pub type WRTRIM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WRTRIM`"]
pub struct WRTRIM_W<'a> {
    w: &'a mut W,
}
impl<'a> WRTRIM_W<'a> {
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
    #[doc = "Bits 0:13 - This value is factory trimmed to account for bias and temperature compensation."]
    #[inline(always)]
    pub fn trim(&self) -> TRIM_R {
        TRIM_R::new((self.bits & 0x3fff) as u16)
    }
    #[doc = "Bit 14 - Select the FRO HF output frequency."]
    #[inline(always)]
    pub fn sel(&self) -> SEL_R {
        SEL_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bits 16:23 - Frequency trim."]
    #[inline(always)]
    pub fn freqtrim(&self) -> FREQTRIM_R {
        FREQTRIM_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bit 24 - USB clock adjust mode."]
    #[inline(always)]
    pub fn usbclkadj(&self) -> USBCLKADJ_R {
        USBCLKADJ_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - USB Mode value Change flag."]
    #[inline(always)]
    pub fn usbmodchg(&self) -> USBMODCHG_R {
        USBMODCHG_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 30 - High speed clock enable."]
    #[inline(always)]
    pub fn hspdclk(&self) -> HSPDCLK_R {
        HSPDCLK_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Write Trim value."]
    #[inline(always)]
    pub fn wrtrim(&self) -> WRTRIM_R {
        WRTRIM_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:13 - This value is factory trimmed to account for bias and temperature compensation."]
    #[inline(always)]
    pub fn trim(&mut self) -> TRIM_W {
        TRIM_W { w: self }
    }
    #[doc = "Bit 14 - Select the FRO HF output frequency."]
    #[inline(always)]
    pub fn sel(&mut self) -> SEL_W {
        SEL_W { w: self }
    }
    #[doc = "Bits 16:23 - Frequency trim."]
    #[inline(always)]
    pub fn freqtrim(&mut self) -> FREQTRIM_W {
        FREQTRIM_W { w: self }
    }
    #[doc = "Bit 24 - USB clock adjust mode."]
    #[inline(always)]
    pub fn usbclkadj(&mut self) -> USBCLKADJ_W {
        USBCLKADJ_W { w: self }
    }
    #[doc = "Bit 25 - USB Mode value Change flag."]
    #[inline(always)]
    pub fn usbmodchg(&mut self) -> USBMODCHG_W {
        USBMODCHG_W { w: self }
    }
    #[doc = "Bit 30 - High speed clock enable."]
    #[inline(always)]
    pub fn hspdclk(&mut self) -> HSPDCLK_W {
        HSPDCLK_W { w: self }
    }
    #[doc = "Bit 31 - Write Trim value."]
    #[inline(always)]
    pub fn wrtrim(&mut self) -> WRTRIM_W {
        WRTRIM_W { w: self }
    }
}
