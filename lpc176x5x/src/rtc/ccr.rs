#[doc = "Reader of register CCR"]
pub type R = crate::R<u32, super::CCR>;
#[doc = "Writer for register CCR"]
pub type W = crate::W<u32, super::CCR>;
#[doc = "Register CCR `reset()`'s with value 0"]
impl crate::ResetValue for super::CCR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Clock Enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CLKEN_A {
    #[doc = "1: The time counters are enabled."]
    THE_TIME_COUNTERS_AR = 1,
    #[doc = "0: The time counters are disabled so that they may be initialized."]
    THE_TIME_COUNTERS_AR = 0,
}
impl From<CLKEN_A> for bool {
    #[inline(always)]
    fn from(variant: CLKEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CLKEN`"]
pub type CLKEN_R = crate::R<bool, CLKEN_A>;
impl CLKEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CLKEN_A {
        match self.bits {
            true => CLKEN_A::THE_TIME_COUNTERS_AR,
            false => CLKEN_A::THE_TIME_COUNTERS_AR,
        }
    }
    #[doc = "Checks if the value of the field is `THE_TIME_COUNTERS_AR`"]
    #[inline(always)]
    pub fn is_the_time_counters_ar(&self) -> bool {
        *self == CLKEN_A::THE_TIME_COUNTERS_AR
    }
    #[doc = "Checks if the value of the field is `THE_TIME_COUNTERS_AR`"]
    #[inline(always)]
    pub fn is_the_time_counters_ar(&self) -> bool {
        *self == CLKEN_A::THE_TIME_COUNTERS_AR
    }
}
#[doc = "Write proxy for field `CLKEN`"]
pub struct CLKEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CLKEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CLKEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The time counters are enabled."]
    #[inline(always)]
    pub fn the_time_counters_ar(self) -> &'a mut W {
        self.variant(CLKEN_A::THE_TIME_COUNTERS_AR)
    }
    #[doc = "The time counters are disabled so that they may be initialized."]
    #[inline(always)]
    pub fn the_time_counters_ar(self) -> &'a mut W {
        self.variant(CLKEN_A::THE_TIME_COUNTERS_AR)
    }
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
#[doc = "CTC Reset.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CTCRST_A {
    #[doc = "1: When one, the elements in the internal oscillator divider are reset, and remain reset until CCR\\[1\\]
is changed to zero. This is the divider that generates the 1 Hz clock from the 32.768 kHz crystal. The state of the divider is not visible to software."]
    RESET = 1,
    #[doc = "0: No effect."]
    NO_EFFECT_ = 0,
}
impl From<CTCRST_A> for bool {
    #[inline(always)]
    fn from(variant: CTCRST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CTCRST`"]
pub type CTCRST_R = crate::R<bool, CTCRST_A>;
impl CTCRST_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CTCRST_A {
        match self.bits {
            true => CTCRST_A::RESET,
            false => CTCRST_A::NO_EFFECT_,
        }
    }
    #[doc = "Checks if the value of the field is `RESET`"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == CTCRST_A::RESET
    }
    #[doc = "Checks if the value of the field is `NO_EFFECT_`"]
    #[inline(always)]
    pub fn is_no_effect_(&self) -> bool {
        *self == CTCRST_A::NO_EFFECT_
    }
}
#[doc = "Write proxy for field `CTCRST`"]
pub struct CTCRST_W<'a> {
    w: &'a mut W,
}
impl<'a> CTCRST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CTCRST_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "When one, the elements in the internal oscillator divider are reset, and remain reset until CCR\\[1\\]
is changed to zero. This is the divider that generates the 1 Hz clock from the 32.768 kHz crystal. The state of the divider is not visible to software."]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(CTCRST_A::RESET)
    }
    #[doc = "No effect."]
    #[inline(always)]
    pub fn no_effect_(self) -> &'a mut W {
        self.variant(CTCRST_A::NO_EFFECT_)
    }
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "Calibration counter enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CCALEN_A {
    #[doc = "1: The calibration counter is disabled and reset to zero."]
    THE_CALIBRATION_COUN = 1,
    #[doc = "0: The calibration counter is enabled and counting, using the 1 Hz clock. When the calibration counter is equal to the value of the CALIBRATION register, the counter resets and repeats counting up to the value of the CALIBRATION register. See Section 30.6.4.2 and  Section 30.6.5."]
    THE_CALIBRATION_COUN = 0,
}
impl From<CCALEN_A> for bool {
    #[inline(always)]
    fn from(variant: CCALEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CCALEN`"]
pub type CCALEN_R = crate::R<bool, CCALEN_A>;
impl CCALEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CCALEN_A {
        match self.bits {
            true => CCALEN_A::THE_CALIBRATION_COUN,
            false => CCALEN_A::THE_CALIBRATION_COUN,
        }
    }
    #[doc = "Checks if the value of the field is `THE_CALIBRATION_COUN`"]
    #[inline(always)]
    pub fn is_the_calibration_coun(&self) -> bool {
        *self == CCALEN_A::THE_CALIBRATION_COUN
    }
    #[doc = "Checks if the value of the field is `THE_CALIBRATION_COUN`"]
    #[inline(always)]
    pub fn is_the_calibration_coun(&self) -> bool {
        *self == CCALEN_A::THE_CALIBRATION_COUN
    }
}
#[doc = "Write proxy for field `CCALEN`"]
pub struct CCALEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CCALEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CCALEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The calibration counter is disabled and reset to zero."]
    #[inline(always)]
    pub fn the_calibration_coun(self) -> &'a mut W {
        self.variant(CCALEN_A::THE_CALIBRATION_COUN)
    }
    #[doc = "The calibration counter is enabled and counting, using the 1 Hz clock. When the calibration counter is equal to the value of the CALIBRATION register, the counter resets and repeats counting up to the value of the CALIBRATION register. See Section 30.6.4.2 and Section 30.6.5."]
    #[inline(always)]
    pub fn the_calibration_coun(self) -> &'a mut W {
        self.variant(CCALEN_A::THE_CALIBRATION_COUN)
    }
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
    #[doc = "Bit 0 - Clock Enable."]
    #[inline(always)]
    pub fn clken(&self) -> CLKEN_R {
        CLKEN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - CTC Reset."]
    #[inline(always)]
    pub fn ctcrst(&self) -> CTCRST_R {
        CTCRST_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Calibration counter enable."]
    #[inline(always)]
    pub fn ccalen(&self) -> CCALEN_R {
        CCALEN_R::new(((self.bits >> 4) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Clock Enable."]
    #[inline(always)]
    pub fn clken(&mut self) -> CLKEN_W {
        CLKEN_W { w: self }
    }
    #[doc = "Bit 1 - CTC Reset."]
    #[inline(always)]
    pub fn ctcrst(&mut self) -> CTCRST_W {
        CTCRST_W { w: self }
    }
    #[doc = "Bit 4 - Calibration counter enable."]
    #[inline(always)]
    pub fn ccalen(&mut self) -> CCALEN_W {
        CCALEN_W { w: self }
    }
}
