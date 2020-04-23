#[doc = "Reader of register CDETECT"]
pub type R = crate::R<u32, super::CDETECT>;
#[doc = "Writer for register CDETECT"]
pub type W = crate::W<u32, super::CDETECT>;
#[doc = "Register CDETECT `reset()`'s with value 0"]
impl crate::ResetValue for super::CDETECT {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CARD_DETECT`"]
pub type CARD_DETECT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CARD_DETECT`"]
pub struct CARD_DETECT_W<'a> {
    w: &'a mut W,
}
impl<'a> CARD_DETECT_W<'a> {
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
    #[doc = "Bit 0 - Card detect."]
    #[inline(always)]
    pub fn card_detect(&self) -> CARD_DETECT_R {
        CARD_DETECT_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Card detect."]
    #[inline(always)]
    pub fn card_detect(&mut self) -> CARD_DETECT_W {
        CARD_DETECT_W { w: self }
    }
}
