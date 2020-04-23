#[doc = "Reader of register FIFOINTENCLR"]
pub type R = crate::R<u32, super::FIFOINTENCLR>;
#[doc = "Writer for register FIFOINTENCLR"]
pub type W = crate::W<u32, super::FIFOINTENCLR>;
#[doc = "Register FIFOINTENCLR `reset()`'s with value 0"]
impl crate::ResetValue for super::FIFOINTENCLR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `TXERR`"]
pub type TXERR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TXERR`"]
pub struct TXERR_W<'a> {
    w: &'a mut W,
}
impl<'a> TXERR_W<'a> {
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
#[doc = "Reader of field `RXERR`"]
pub type RXERR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RXERR`"]
pub struct RXERR_W<'a> {
    w: &'a mut W,
}
impl<'a> RXERR_W<'a> {
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
#[doc = "Reader of field `TXLVL`"]
pub type TXLVL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TXLVL`"]
pub struct TXLVL_W<'a> {
    w: &'a mut W,
}
impl<'a> TXLVL_W<'a> {
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
#[doc = "Reader of field `RXLVL`"]
pub type RXLVL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RXLVL`"]
pub struct RXLVL_W<'a> {
    w: &'a mut W,
}
impl<'a> RXLVL_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Writing one clears the corresponding bits in the FIFOINTENSET register."]
    #[inline(always)]
    pub fn txerr(&self) -> TXERR_R {
        TXERR_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Writing one clears the corresponding bits in the FIFOINTENSET register."]
    #[inline(always)]
    pub fn rxerr(&self) -> RXERR_R {
        RXERR_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Writing one clears the corresponding bits in the FIFOINTENSET register."]
    #[inline(always)]
    pub fn txlvl(&self) -> TXLVL_R {
        TXLVL_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Writing one clears the corresponding bits in the FIFOINTENSET register."]
    #[inline(always)]
    pub fn rxlvl(&self) -> RXLVL_R {
        RXLVL_R::new(((self.bits >> 3) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Writing one clears the corresponding bits in the FIFOINTENSET register."]
    #[inline(always)]
    pub fn txerr(&mut self) -> TXERR_W {
        TXERR_W { w: self }
    }
    #[doc = "Bit 1 - Writing one clears the corresponding bits in the FIFOINTENSET register."]
    #[inline(always)]
    pub fn rxerr(&mut self) -> RXERR_W {
        RXERR_W { w: self }
    }
    #[doc = "Bit 2 - Writing one clears the corresponding bits in the FIFOINTENSET register."]
    #[inline(always)]
    pub fn txlvl(&mut self) -> TXLVL_W {
        TXLVL_W { w: self }
    }
    #[doc = "Bit 3 - Writing one clears the corresponding bits in the FIFOINTENSET register."]
    #[inline(always)]
    pub fn rxlvl(&mut self) -> RXLVL_W {
        RXLVL_W { w: self }
    }
}
