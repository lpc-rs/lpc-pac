#[doc = "Reader of register PWRDWN"]
pub type R = crate::R<u32, super::PWRDWN>;
#[doc = "Writer for register PWRDWN"]
pub type W = crate::W<u32, super::PWRDWN>;
#[doc = "Register PWRDWN `reset()`'s with value 0"]
impl crate::ResetValue for super::PWRDWN {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `PWRDWN`"]
pub type PWRDWN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PWRDWN`"]
pub struct PWRDWN_W<'a> {
    w: &'a mut W,
}
impl<'a> PWRDWN_W<'a> {
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
    #[doc = "Bit 0 - Power down mode bit."]
    #[inline(always)]
    pub fn pwrdwn(&self) -> PWRDWN_R {
        PWRDWN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Power down mode bit."]
    #[inline(always)]
    pub fn pwrdwn(&mut self) -> PWRDWN_W {
        PWRDWN_W { w: self }
    }
}
