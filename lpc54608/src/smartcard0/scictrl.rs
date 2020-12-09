#[doc = "Reader of register SCICTRL"]
pub type R = crate::R<u32, super::SCICTRL>;
#[doc = "Writer for register SCICTRL"]
pub type W = crate::W<u32, super::SCICTRL>;
#[doc = "Register SCICTRL `reset()`'s with value 0"]
impl crate::ResetValue for super::SCICTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SCIEN`"]
pub type SCIEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SCIEN`"]
pub struct SCIEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SCIEN_W<'a> {
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
#[doc = "Reader of field `NACKDIS`"]
pub type NACKDIS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `NACKDIS`"]
pub struct NACKDIS_W<'a> {
    w: &'a mut W,
}
impl<'a> NACKDIS_W<'a> {
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
#[doc = "Reader of field `PROTSEL`"]
pub type PROTSEL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PROTSEL`"]
pub struct PROTSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> PROTSEL_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "Reader of field `TXRETRY`"]
pub type TXRETRY_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TXRETRY`"]
pub struct TXRETRY_W<'a> {
    w: &'a mut W,
}
impl<'a> TXRETRY_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 5)) | (((value as u32) & 0x07) << 5);
        self.w
    }
}
#[doc = "Reader of field `GUARDTIME`"]
pub type GUARDTIME_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `GUARDTIME`"]
pub struct GUARDTIME_W<'a> {
    w: &'a mut W,
}
impl<'a> GUARDTIME_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Smart Card Interface Enable."]
    #[inline(always)]
    pub fn scien(&self) -> SCIEN_R {
        SCIEN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - NACK response disable."]
    #[inline(always)]
    pub fn nackdis(&self) -> NACKDIS_R {
        NACKDIS_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Protocol selection as defined in the ISO7816-3 standard."]
    #[inline(always)]
    pub fn protsel(&self) -> PROTSEL_R {
        PROTSEL_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bits 5:7 - Maximum number of retransmissions in case of a negative acknowledge (protocol T=0)."]
    #[inline(always)]
    pub fn txretry(&self) -> TXRETRY_R {
        TXRETRY_R::new(((self.bits >> 5) & 0x07) as u8)
    }
    #[doc = "Bits 8:15 - Extra guard time."]
    #[inline(always)]
    pub fn guardtime(&self) -> GUARDTIME_R {
        GUARDTIME_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Smart Card Interface Enable."]
    #[inline(always)]
    pub fn scien(&mut self) -> SCIEN_W {
        SCIEN_W { w: self }
    }
    #[doc = "Bit 1 - NACK response disable."]
    #[inline(always)]
    pub fn nackdis(&mut self) -> NACKDIS_W {
        NACKDIS_W { w: self }
    }
    #[doc = "Bit 2 - Protocol selection as defined in the ISO7816-3 standard."]
    #[inline(always)]
    pub fn protsel(&mut self) -> PROTSEL_W {
        PROTSEL_W { w: self }
    }
    #[doc = "Bits 5:7 - Maximum number of retransmissions in case of a negative acknowledge (protocol T=0)."]
    #[inline(always)]
    pub fn txretry(&mut self) -> TXRETRY_W {
        TXRETRY_W { w: self }
    }
    #[doc = "Bits 8:15 - Extra guard time."]
    #[inline(always)]
    pub fn guardtime(&mut self) -> GUARDTIME_W {
        GUARDTIME_W { w: self }
    }
}
