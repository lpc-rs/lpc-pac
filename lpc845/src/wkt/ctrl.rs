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
#[doc = "Select the self wake-up timer clock source. Remark: This bit only has an effect if the SEL_EXTCLK bit is not set.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CLKSEL_A {
    #[doc = "0: Divided FRO clock. This clock runs at 750 kHz and provides time-out periods of up to approximately 95 minutes in 1.33 us increments. Remark: This clock is not available in not available in Deep-sleep, power-down, deep power-down modes. Do not select this option if the timer is to be used to wake up from one of these modes."]
    DIVIDED_FRO_CLOCK = 0,
    #[doc = "1: This is the (nominally) 10 kHz clock and provides time-out periods of up to approximately 119 hours in 100 us increments. The accuracy of this clock is limited to +/- 40 % over temperature and processing. Remark: This clock is available in all power modes. Prior to use, the low-power oscillator must be enabled. The oscillator must also be set to remain active in Deep power-down if needed."]
    LOW_POWER_CLOCK = 1,
}
impl From<CLKSEL_A> for bool {
    #[inline(always)]
    fn from(variant: CLKSEL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CLKSEL`"]
pub type CLKSEL_R = crate::R<bool, CLKSEL_A>;
impl CLKSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CLKSEL_A {
        match self.bits {
            false => CLKSEL_A::DIVIDED_FRO_CLOCK,
            true => CLKSEL_A::LOW_POWER_CLOCK,
        }
    }
    #[doc = "Checks if the value of the field is `DIVIDED_FRO_CLOCK`"]
    #[inline(always)]
    pub fn is_divided_fro_clock(&self) -> bool {
        *self == CLKSEL_A::DIVIDED_FRO_CLOCK
    }
    #[doc = "Checks if the value of the field is `LOW_POWER_CLOCK`"]
    #[inline(always)]
    pub fn is_low_power_clock(&self) -> bool {
        *self == CLKSEL_A::LOW_POWER_CLOCK
    }
}
#[doc = "Write proxy for field `CLKSEL`"]
pub struct CLKSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> CLKSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CLKSEL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Divided FRO clock. This clock runs at 750 kHz and provides time-out periods of up to approximately 95 minutes in 1.33 us increments. Remark: This clock is not available in not available in Deep-sleep, power-down, deep power-down modes. Do not select this option if the timer is to be used to wake up from one of these modes."]
    #[inline(always)]
    pub fn divided_fro_clock(self) -> &'a mut W {
        self.variant(CLKSEL_A::DIVIDED_FRO_CLOCK)
    }
    #[doc = "This is the (nominally) 10 kHz clock and provides time-out periods of up to approximately 119 hours in 100 us increments. The accuracy of this clock is limited to +/- 40 % over temperature and processing. Remark: This clock is available in all power modes. Prior to use, the low-power oscillator must be enabled. The oscillator must also be set to remain active in Deep power-down if needed."]
    #[inline(always)]
    pub fn low_power_clock(self) -> &'a mut W {
        self.variant(CLKSEL_A::LOW_POWER_CLOCK)
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
#[doc = "Wake-up or alarm timer flag.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ALARMFLAG_A {
    #[doc = "0: No time-out. The self wake-up timer has not timed out. Writing a 0 to has no effect."]
    NO_TIME_OUT = 0,
    #[doc = "1: Time-out. The self wake-up timer has timed out. This flag generates an interrupt request which can wake up the part from any reduced power mode including Deep power-down if the clock source is the low power oscillator. Writing a 1 clears this status bit."]
    TIME_OUT = 1,
}
impl From<ALARMFLAG_A> for bool {
    #[inline(always)]
    fn from(variant: ALARMFLAG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ALARMFLAG`"]
pub type ALARMFLAG_R = crate::R<bool, ALARMFLAG_A>;
impl ALARMFLAG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ALARMFLAG_A {
        match self.bits {
            false => ALARMFLAG_A::NO_TIME_OUT,
            true => ALARMFLAG_A::TIME_OUT,
        }
    }
    #[doc = "Checks if the value of the field is `NO_TIME_OUT`"]
    #[inline(always)]
    pub fn is_no_time_out(&self) -> bool {
        *self == ALARMFLAG_A::NO_TIME_OUT
    }
    #[doc = "Checks if the value of the field is `TIME_OUT`"]
    #[inline(always)]
    pub fn is_time_out(&self) -> bool {
        *self == ALARMFLAG_A::TIME_OUT
    }
}
#[doc = "Write proxy for field `ALARMFLAG`"]
pub struct ALARMFLAG_W<'a> {
    w: &'a mut W,
}
impl<'a> ALARMFLAG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ALARMFLAG_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No time-out. The self wake-up timer has not timed out. Writing a 0 to has no effect."]
    #[inline(always)]
    pub fn no_time_out(self) -> &'a mut W {
        self.variant(ALARMFLAG_A::NO_TIME_OUT)
    }
    #[doc = "Time-out. The self wake-up timer has timed out. This flag generates an interrupt request which can wake up the part from any reduced power mode including Deep power-down if the clock source is the low power oscillator. Writing a 1 clears this status bit."]
    #[inline(always)]
    pub fn time_out(self) -> &'a mut W {
        self.variant(ALARMFLAG_A::TIME_OUT)
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
#[doc = "Clears the self wake-up timer.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CLEARCTR_A {
    #[doc = "0: No effect. Reading this bit always returns 0."]
    NO_EFFECT = 0,
    #[doc = "1: Clear the counter. Counting is halted until a new count value is loaded."]
    CLEAR_THE_COUNTER = 1,
}
impl From<CLEARCTR_A> for bool {
    #[inline(always)]
    fn from(variant: CLEARCTR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CLEARCTR`"]
pub type CLEARCTR_R = crate::R<bool, CLEARCTR_A>;
impl CLEARCTR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CLEARCTR_A {
        match self.bits {
            false => CLEARCTR_A::NO_EFFECT,
            true => CLEARCTR_A::CLEAR_THE_COUNTER,
        }
    }
    #[doc = "Checks if the value of the field is `NO_EFFECT`"]
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        *self == CLEARCTR_A::NO_EFFECT
    }
    #[doc = "Checks if the value of the field is `CLEAR_THE_COUNTER`"]
    #[inline(always)]
    pub fn is_clear_the_counter(&self) -> bool {
        *self == CLEARCTR_A::CLEAR_THE_COUNTER
    }
}
#[doc = "Write proxy for field `CLEARCTR`"]
pub struct CLEARCTR_W<'a> {
    w: &'a mut W,
}
impl<'a> CLEARCTR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CLEARCTR_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect. Reading this bit always returns 0."]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(CLEARCTR_A::NO_EFFECT)
    }
    #[doc = "Clear the counter. Counting is halted until a new count value is loaded."]
    #[inline(always)]
    pub fn clear_the_counter(self) -> &'a mut W {
        self.variant(CLEARCTR_A::CLEAR_THE_COUNTER)
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "Select external or internal clock source for the self wake-up timer. The internal clock source is selected by the CLKSEL bit in this register if SET_EXTCLK is set to internal.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SEL_EXTCLK_A {
    #[doc = "0: Internal. The clock source is the internal clock selected by the CLKSEL bit."]
    INTERNAL = 0,
    #[doc = "1: External. The self wake-up timer uses the external WKTCLKIN pin."]
    EXTERNAL = 1,
}
impl From<SEL_EXTCLK_A> for bool {
    #[inline(always)]
    fn from(variant: SEL_EXTCLK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SEL_EXTCLK`"]
pub type SEL_EXTCLK_R = crate::R<bool, SEL_EXTCLK_A>;
impl SEL_EXTCLK_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SEL_EXTCLK_A {
        match self.bits {
            false => SEL_EXTCLK_A::INTERNAL,
            true => SEL_EXTCLK_A::EXTERNAL,
        }
    }
    #[doc = "Checks if the value of the field is `INTERNAL`"]
    #[inline(always)]
    pub fn is_internal(&self) -> bool {
        *self == SEL_EXTCLK_A::INTERNAL
    }
    #[doc = "Checks if the value of the field is `EXTERNAL`"]
    #[inline(always)]
    pub fn is_external(&self) -> bool {
        *self == SEL_EXTCLK_A::EXTERNAL
    }
}
#[doc = "Write proxy for field `SEL_EXTCLK`"]
pub struct SEL_EXTCLK_W<'a> {
    w: &'a mut W,
}
impl<'a> SEL_EXTCLK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SEL_EXTCLK_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Internal. The clock source is the internal clock selected by the CLKSEL bit."]
    #[inline(always)]
    pub fn internal(self) -> &'a mut W {
        self.variant(SEL_EXTCLK_A::INTERNAL)
    }
    #[doc = "External. The self wake-up timer uses the external WKTCLKIN pin."]
    #[inline(always)]
    pub fn external(self) -> &'a mut W {
        self.variant(SEL_EXTCLK_A::EXTERNAL)
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Select the self wake-up timer clock source. Remark: This bit only has an effect if the SEL_EXTCLK bit is not set."]
    #[inline(always)]
    pub fn clksel(&self) -> CLKSEL_R {
        CLKSEL_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Wake-up or alarm timer flag."]
    #[inline(always)]
    pub fn alarmflag(&self) -> ALARMFLAG_R {
        ALARMFLAG_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Clears the self wake-up timer."]
    #[inline(always)]
    pub fn clearctr(&self) -> CLEARCTR_R {
        CLEARCTR_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Select external or internal clock source for the self wake-up timer. The internal clock source is selected by the CLKSEL bit in this register if SET_EXTCLK is set to internal."]
    #[inline(always)]
    pub fn sel_extclk(&self) -> SEL_EXTCLK_R {
        SEL_EXTCLK_R::new(((self.bits >> 3) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Select the self wake-up timer clock source. Remark: This bit only has an effect if the SEL_EXTCLK bit is not set."]
    #[inline(always)]
    pub fn clksel(&mut self) -> CLKSEL_W {
        CLKSEL_W { w: self }
    }
    #[doc = "Bit 1 - Wake-up or alarm timer flag."]
    #[inline(always)]
    pub fn alarmflag(&mut self) -> ALARMFLAG_W {
        ALARMFLAG_W { w: self }
    }
    #[doc = "Bit 2 - Clears the self wake-up timer."]
    #[inline(always)]
    pub fn clearctr(&mut self) -> CLEARCTR_W {
        CLEARCTR_W { w: self }
    }
    #[doc = "Bit 3 - Select external or internal clock source for the self wake-up timer. The internal clock source is selected by the CLKSEL bit in this register if SET_EXTCLK is set to internal."]
    #[inline(always)]
    pub fn sel_extclk(&mut self) -> SEL_EXTCLK_W {
        SEL_EXTCLK_W { w: self }
    }
}
