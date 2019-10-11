#[doc = "Reader of register RTC_AUXEN"]
pub type R = crate::R<u32, super::RTC_AUXEN>;
#[doc = "Writer for register RTC_AUXEN"]
pub type W = crate::W<u32, super::RTC_AUXEN>;
#[doc = "Register RTC_AUXEN `reset()`'s with value 0"]
impl crate::ResetValue for super::RTC_AUXEN {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RTC_OSCFEN`"]
pub type RTC_OSCFEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTC_OSCFEN`"]
pub struct RTC_OSCFEN_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_OSCFEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
impl R {
    #[doc = "Bit 4 - Oscillator Fail Detect interrupt enable. When 0: the RTC Oscillator Fail detect interrupt is disabled. When 1: the RTC Oscillator Fail detect interrupt is enabled. See Section 30.6.2.5."]
    #[inline(always)]
    pub fn rtc_oscfen(&self) -> RTC_OSCFEN_R {
        RTC_OSCFEN_R::new(((self.bits >> 4) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 4 - Oscillator Fail Detect interrupt enable. When 0: the RTC Oscillator Fail detect interrupt is disabled. When 1: the RTC Oscillator Fail detect interrupt is enabled. See Section 30.6.2.5."]
    #[inline(always)]
    pub fn rtc_oscfen(&mut self) -> RTC_OSCFEN_W {
        RTC_OSCFEN_W { w: self }
    }
}
