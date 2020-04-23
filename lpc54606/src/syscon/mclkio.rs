#[doc = "Reader of register MCLKIO"]
pub type R = crate::R<u32, super::MCLKIO>;
#[doc = "Writer for register MCLKIO"]
pub type W = crate::W<u32, super::MCLKIO>;
#[doc = "Register MCLKIO `reset()`'s with value 0"]
impl crate::ResetValue for super::MCLKIO {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DIR`"]
pub type DIR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DIR`"]
pub struct DIR_W<'a> {
    w: &'a mut W,
}
impl<'a> DIR_W<'a> {
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
impl R {
    #[doc = "Bit 0 - MCLK direction control."]
    #[inline(always)]
    pub fn dir(&self) -> DIR_R {
        DIR_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - MCLK direction control."]
    #[inline(always)]
    pub fn dir(&mut self) -> DIR_W {
        DIR_W { w: self }
    }
}
