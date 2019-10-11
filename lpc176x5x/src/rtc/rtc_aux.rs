#[doc = "Reader of register RTC_AUX"]
pub type R = crate::R<u32, super::RTC_AUX>;
#[doc = "Writer for register RTC_AUX"]
pub type W = crate::W<u32, super::RTC_AUX>;
#[doc = "Register RTC_AUX `reset()`'s with value 0x10"]
impl crate::ResetValue for super::RTC_AUX {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x10
    }
}
#[doc = "Reader of field `RTC_OSCF`"]
pub type RTC_OSCF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTC_OSCF`"]
pub struct RTC_OSCF_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_OSCF_W<'a> {
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
#[doc = "Reader of field `RTC_PDOUT`"]
pub type RTC_PDOUT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTC_PDOUT`"]
pub struct RTC_PDOUT_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_PDOUT_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
impl R {
    #[doc = "Bit 4 - RTC Oscillator Fail detect flag. Read: this bit is set if the RTC oscillator stops, and when RTC power is first turned on. An interrupt will occur when this bit is set, the RTC_OSCFEN bit in RTC_AUXEN is a 1, and the RTC interrupt is enabled in the NVIC. Write: writing a 1 to this bit clears the flag."]
    #[inline(always)]
    pub fn rtc_oscf(&self) -> RTC_OSCF_R {
        RTC_OSCF_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 6 - When 0: the RTC_ALARM pin reflects the RTC alarm status. When 1: the RTC_ALARM pin indicates Deep Power-down mode."]
    #[inline(always)]
    pub fn rtc_pdout(&self) -> RTC_PDOUT_R {
        RTC_PDOUT_R::new(((self.bits >> 6) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 4 - RTC Oscillator Fail detect flag. Read: this bit is set if the RTC oscillator stops, and when RTC power is first turned on. An interrupt will occur when this bit is set, the RTC_OSCFEN bit in RTC_AUXEN is a 1, and the RTC interrupt is enabled in the NVIC. Write: writing a 1 to this bit clears the flag."]
    #[inline(always)]
    pub fn rtc_oscf(&mut self) -> RTC_OSCF_W {
        RTC_OSCF_W { w: self }
    }
    #[doc = "Bit 6 - When 0: the RTC_ALARM pin reflects the RTC alarm status. When 1: the RTC_ALARM pin indicates Deep Power-down mode."]
    #[inline(always)]
    pub fn rtc_pdout(&mut self) -> RTC_PDOUT_W {
        RTC_PDOUT_W { w: self }
    }
}
