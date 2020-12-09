#[doc = "Reader of register INTCLR"]
pub type R = crate::R<u32, super::INTCLR>;
#[doc = "Writer for register INTCLR"]
pub type W = crate::W<u32, super::INTCLR>;
#[doc = "Register INTCLR `reset()`'s with value 0"]
impl crate::ResetValue for super::INTCLR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `FUFIC`"]
pub struct FUFIC_W<'a> {
    w: &'a mut W,
}
impl<'a> FUFIC_W<'a> {
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
#[doc = "Write proxy for field `LNBUIC`"]
pub struct LNBUIC_W<'a> {
    w: &'a mut W,
}
impl<'a> LNBUIC_W<'a> {
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
#[doc = "Write proxy for field `VCOMPIC`"]
pub struct VCOMPIC_W<'a> {
    w: &'a mut W,
}
impl<'a> VCOMPIC_W<'a> {
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
#[doc = "Reader of field `BERIC`"]
pub type BERIC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BERIC`"]
pub struct BERIC_W<'a> {
    w: &'a mut W,
}
impl<'a> BERIC_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
impl R {
    #[doc = "Bit 4 - AHB master error interrupt clear."]
    #[inline(always)]
    pub fn beric(&self) -> BERIC_R {
        BERIC_R::new(((self.bits >> 4) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - FIFO underflow interrupt clear."]
    #[inline(always)]
    pub fn fufic(&mut self) -> FUFIC_W {
        FUFIC_W { w: self }
    }
    #[doc = "Bit 2 - LCD next address base update interrupt clear."]
    #[inline(always)]
    pub fn lnbuic(&mut self) -> LNBUIC_W {
        LNBUIC_W { w: self }
    }
    #[doc = "Bit 3 - Vertical compare interrupt clear."]
    #[inline(always)]
    pub fn vcompic(&mut self) -> VCOMPIC_W {
        VCOMPIC_W { w: self }
    }
    #[doc = "Bit 4 - AHB master error interrupt clear."]
    #[inline(always)]
    pub fn beric(&mut self) -> BERIC_W {
        BERIC_W { w: self }
    }
}
