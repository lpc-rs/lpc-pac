#[doc = "Reader of register RST_N"]
pub type R = crate::R<u32, super::RST_N>;
#[doc = "Writer for register RST_N"]
pub type W = crate::W<u32, super::RST_N>;
#[doc = "Register RST_N `reset()`'s with value 0x01"]
impl crate::ResetValue for super::RST_N {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x01
    }
}
#[doc = "Reader of field `CARD_RESET`"]
pub type CARD_RESET_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CARD_RESET`"]
pub struct CARD_RESET_W<'a> {
    w: &'a mut W,
}
impl<'a> CARD_RESET_W<'a> {
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
    #[doc = "Bit 0 - Hardware reset."]
    #[inline(always)]
    pub fn card_reset(&self) -> CARD_RESET_R {
        CARD_RESET_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Hardware reset."]
    #[inline(always)]
    pub fn card_reset(&mut self) -> CARD_RESET_W {
        CARD_RESET_W { w: self }
    }
}
