#[doc = "Reader of register CTRL"]
pub type R = crate::R<u32, super::CTRL>;
#[doc = "Writer for register CTRL"]
pub type W = crate::W<u32, super::CTRL>;
#[doc = "Register CTRL `reset()`'s with value 0"]
impl crate::ResetValue for super::CTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DELAYVAL`"]
pub type DELAYVAL_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `DELAYVAL`"]
pub struct DELAYVAL_W<'a> {
    w: &'a mut W,
}
impl<'a> DELAYVAL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x7fff_ffff) | ((value as u32) & 0x7fff_ffff);
        self.w
    }
}
#[doc = "Reader of field `REPEAT`"]
pub type REPEAT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `REPEAT`"]
pub struct REPEAT_W<'a> {
    w: &'a mut W,
}
impl<'a> REPEAT_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | (((value as u32) & 0x01) << 31);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:30 - Tick interval value. The delay will be equal to DELAYVAL + 1 periods of the timer clock. The minimum usable value is 1, for a delay of 2 timer clocks. A value of 0 stops the timer."]
    #[inline(always)]
    pub fn delayval(&self) -> DELAYVAL_R {
        DELAYVAL_R::new((self.bits & 0x7fff_ffff) as u32)
    }
    #[doc = "Bit 31 - Repeat delay. 0 = One-time delay. 1 = Delay repeats continuously."]
    #[inline(always)]
    pub fn repeat(&self) -> REPEAT_R {
        REPEAT_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:30 - Tick interval value. The delay will be equal to DELAYVAL + 1 periods of the timer clock. The minimum usable value is 1, for a delay of 2 timer clocks. A value of 0 stops the timer."]
    #[inline(always)]
    pub fn delayval(&mut self) -> DELAYVAL_W {
        DELAYVAL_W { w: self }
    }
    #[doc = "Bit 31 - Repeat delay. 0 = One-time delay. 1 = Delay repeats continuously."]
    #[inline(always)]
    pub fn repeat(&mut self) -> REPEAT_W {
        REPEAT_W { w: self }
    }
}
