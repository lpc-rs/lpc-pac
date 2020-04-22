#[doc = "Reader of register _ITATBCTR0"]
pub type R = crate::R<u32, super::_ITATBCTR0>;
#[doc = "Writer for register _ITATBCTR0"]
pub type W = crate::W<u32, super::_ITATBCTR0>;
#[doc = "Register _ITATBCTR0 `reset()`'s with value 0"]
impl crate::ResetValue for super::_ITATBCTR0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `ATVALID`"]
pub struct ATVALID_W<'a> {
    w: &'a mut W,
}
impl<'a> ATVALID_W<'a> {
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
    #[doc = "Bit 0 - A write to this bit sets the value of the ETM ATVALID output."]
    #[inline(always)]
    pub fn atvalid(&mut self) -> ATVALID_W {
        ATVALID_W { w: self }
    }
}
