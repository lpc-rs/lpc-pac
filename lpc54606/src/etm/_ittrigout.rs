#[doc = "Reader of register _ITTRIGOUT"]
pub type R = crate::R<u32, super::_ITTRIGOUT>;
#[doc = "Writer for register _ITTRIGOUT"]
pub type W = crate::W<u32, super::_ITTRIGOUT>;
#[doc = "Register _ITTRIGOUT `reset()`'s with value 0"]
impl crate::ResetValue for super::_ITTRIGOUT {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `TRIGGER`"]
pub struct TRIGGER_W<'a> {
    w: &'a mut W,
}
impl<'a> TRIGGER_W<'a> {
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
impl R {}
impl W {
    #[doc = "Bit 0 - A write to this bit sets the TRIGGER output."]
    #[inline(always)]
    pub fn trigger(&mut self) -> TRIGGER_W {
        TRIGGER_W { w: self }
    }
}
