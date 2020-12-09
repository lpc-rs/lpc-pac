#[doc = "Reader of register INTMSK"]
pub type R = crate::R<u32, super::INTMSK>;
#[doc = "Writer for register INTMSK"]
pub type W = crate::W<u32, super::INTMSK>;
#[doc = "Register INTMSK `reset()`'s with value 0"]
impl crate::ResetValue for super::INTMSK {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `FUFIM`"]
pub type FUFIM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FUFIM`"]
pub struct FUFIM_W<'a> {
    w: &'a mut W,
}
impl<'a> FUFIM_W<'a> {
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
#[doc = "Reader of field `LNBUIM`"]
pub type LNBUIM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LNBUIM`"]
pub struct LNBUIM_W<'a> {
    w: &'a mut W,
}
impl<'a> LNBUIM_W<'a> {
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
#[doc = "Reader of field `VCOMPIM`"]
pub type VCOMPIM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `VCOMPIM`"]
pub struct VCOMPIM_W<'a> {
    w: &'a mut W,
}
impl<'a> VCOMPIM_W<'a> {
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
#[doc = "Reader of field `BERIM`"]
pub type BERIM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BERIM`"]
pub struct BERIM_W<'a> {
    w: &'a mut W,
}
impl<'a> BERIM_W<'a> {
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
    #[doc = "Bit 1 - FIFO underflow interrupt enable."]
    #[inline(always)]
    pub fn fufim(&self) -> FUFIM_R {
        FUFIM_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - LCD next base address update interrupt enable."]
    #[inline(always)]
    pub fn lnbuim(&self) -> LNBUIM_R {
        LNBUIM_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Vertical compare interrupt enable."]
    #[inline(always)]
    pub fn vcompim(&self) -> VCOMPIM_R {
        VCOMPIM_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - AHB master error interrupt enable."]
    #[inline(always)]
    pub fn berim(&self) -> BERIM_R {
        BERIM_R::new(((self.bits >> 4) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - FIFO underflow interrupt enable."]
    #[inline(always)]
    pub fn fufim(&mut self) -> FUFIM_W {
        FUFIM_W { w: self }
    }
    #[doc = "Bit 2 - LCD next base address update interrupt enable."]
    #[inline(always)]
    pub fn lnbuim(&mut self) -> LNBUIM_W {
        LNBUIM_W { w: self }
    }
    #[doc = "Bit 3 - Vertical compare interrupt enable."]
    #[inline(always)]
    pub fn vcompim(&mut self) -> VCOMPIM_W {
        VCOMPIM_W { w: self }
    }
    #[doc = "Bit 4 - AHB master error interrupt enable."]
    #[inline(always)]
    pub fn berim(&mut self) -> BERIM_W {
        BERIM_W { w: self }
    }
}
