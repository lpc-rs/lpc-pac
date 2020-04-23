#[doc = "Reader of register CTRL"]
pub type R = crate::R<u32, super::CTRL>;
#[doc = "Writer for register CTRL"]
pub type W = crate::W<u32, super::CTRL>;
#[doc = "Register CTRL `reset()`'s with value 0x0c"]
impl crate::ResetValue for super::CTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0c
    }
}
#[doc = "Reader of field `RITINT`"]
pub type RITINT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RITINT`"]
pub struct RITINT_W<'a> {
    w: &'a mut W,
}
impl<'a> RITINT_W<'a> {
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
#[doc = "Reader of field `RITENCLR`"]
pub type RITENCLR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RITENCLR`"]
pub struct RITENCLR_W<'a> {
    w: &'a mut W,
}
impl<'a> RITENCLR_W<'a> {
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
#[doc = "Reader of field `RITENBR`"]
pub type RITENBR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RITENBR`"]
pub struct RITENBR_W<'a> {
    w: &'a mut W,
}
impl<'a> RITENBR_W<'a> {
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
#[doc = "Reader of field `RITEN`"]
pub type RITEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RITEN`"]
pub struct RITEN_W<'a> {
    w: &'a mut W,
}
impl<'a> RITEN_W<'a> {
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
    #[doc = "Bit 0 - Interrupt flag."]
    #[inline(always)]
    pub fn ritint(&self) -> RITINT_R {
        RITINT_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Timer enable clear."]
    #[inline(always)]
    pub fn ritenclr(&self) -> RITENCLR_R {
        RITENCLR_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Timer enable for debug."]
    #[inline(always)]
    pub fn ritenbr(&self) -> RITENBR_R {
        RITENBR_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Timer enable."]
    #[inline(always)]
    pub fn riten(&self) -> RITEN_R {
        RITEN_R::new(((self.bits >> 3) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Interrupt flag."]
    #[inline(always)]
    pub fn ritint(&mut self) -> RITINT_W {
        RITINT_W { w: self }
    }
    #[doc = "Bit 1 - Timer enable clear."]
    #[inline(always)]
    pub fn ritenclr(&mut self) -> RITENCLR_W {
        RITENCLR_W { w: self }
    }
    #[doc = "Bit 2 - Timer enable for debug."]
    #[inline(always)]
    pub fn ritenbr(&mut self) -> RITENBR_W {
        RITENBR_W { w: self }
    }
    #[doc = "Bit 3 - Timer enable."]
    #[inline(always)]
    pub fn riten(&mut self) -> RITEN_W {
        RITEN_W { w: self }
    }
}
