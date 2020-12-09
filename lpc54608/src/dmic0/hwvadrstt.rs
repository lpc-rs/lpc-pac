#[doc = "Reader of register HWVADRSTT"]
pub type R = crate::R<u32, super::HWVADRSTT>;
#[doc = "Writer for register HWVADRSTT"]
pub type W = crate::W<u32, super::HWVADRSTT>;
#[doc = "Register HWVADRSTT `reset()`'s with value 0"]
impl crate::ResetValue for super::HWVADRSTT {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RSTT`"]
pub type RSTT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RSTT`"]
pub struct RSTT_W<'a> {
    w: &'a mut W,
}
impl<'a> RSTT_W<'a> {
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
    #[doc = "Bit 0 - Writing a 1 resets all filter values"]
    #[inline(always)]
    pub fn rstt(&self) -> RSTT_R {
        RSTT_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Writing a 1 resets all filter values"]
    #[inline(always)]
    pub fn rstt(&mut self) -> RSTT_W {
        RSTT_W { w: self }
    }
}
