#[doc = "Reader of register CRSR_INTMSK"]
pub type R = crate::R<u32, super::CRSR_INTMSK>;
#[doc = "Writer for register CRSR_INTMSK"]
pub type W = crate::W<u32, super::CRSR_INTMSK>;
#[doc = "Register CRSR_INTMSK `reset()`'s with value 0"]
impl crate::ResetValue for super::CRSR_INTMSK {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CRSRIM`"]
pub type CRSRIM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CRSRIM`"]
pub struct CRSRIM_W<'a> {
    w: &'a mut W,
}
impl<'a> CRSRIM_W<'a> {
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
    #[doc = "Bit 0 - Cursor interrupt mask."]
    #[inline(always)]
    pub fn crsrim(&self) -> CRSRIM_R {
        CRSRIM_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Cursor interrupt mask."]
    #[inline(always)]
    pub fn crsrim(&mut self) -> CRSRIM_W {
        CRSRIM_W { w: self }
    }
}
