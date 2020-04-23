#[doc = "Reader of register BACKENDPWR"]
pub type R = crate::R<u32, super::BACKENDPWR>;
#[doc = "Writer for register BACKENDPWR"]
pub type W = crate::W<u32, super::BACKENDPWR>;
#[doc = "Register BACKENDPWR `reset()`'s with value 0"]
impl crate::ResetValue for super::BACKENDPWR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `BACKENDPWR`"]
pub type BACKENDPWR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BACKENDPWR`"]
pub struct BACKENDPWR_W<'a> {
    w: &'a mut W,
}
impl<'a> BACKENDPWR_W<'a> {
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
    #[doc = "Bit 0 - Back-end Power control for card application."]
    #[inline(always)]
    pub fn backendpwr(&self) -> BACKENDPWR_R {
        BACKENDPWR_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Back-end Power control for card application."]
    #[inline(always)]
    pub fn backendpwr(&mut self) -> BACKENDPWR_W {
        BACKENDPWR_W { w: self }
    }
}
