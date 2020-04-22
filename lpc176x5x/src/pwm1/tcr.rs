#[doc = "Reader of register TCR"]
pub type R = crate::R<u32, super::TCR>;
#[doc = "Writer for register TCR"]
pub type W = crate::W<u32, super::TCR>;
#[doc = "Register TCR `reset()`'s with value 0"]
impl crate::ResetValue for super::TCR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Counter Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CE_A {
    #[doc = "1: The PWM Timer Counter and PWM Prescale Counter are enabled for counting."]
    THE_PWM_TIMER_COUNTE = 1,
    #[doc = "0: The counters are disabled."]
    THE_COUNTERS_ARE_DIS = 0,
}
impl From<CE_A> for bool {
    #[inline(always)]
    fn from(variant: CE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CE`"]
pub type CE_R = crate::R<bool, CE_A>;
impl CE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CE_A {
        match self.bits {
            true => CE_A::THE_PWM_TIMER_COUNTE,
            false => CE_A::THE_COUNTERS_ARE_DIS,
        }
    }
    #[doc = "Checks if the value of the field is `THE_PWM_TIMER_COUNTE`"]
    #[inline(always)]
    pub fn is_the_pwm_timer_counte(&self) -> bool {
        *self == CE_A::THE_PWM_TIMER_COUNTE
    }
    #[doc = "Checks if the value of the field is `THE_COUNTERS_ARE_DIS`"]
    #[inline(always)]
    pub fn is_the_counters_are_dis(&self) -> bool {
        *self == CE_A::THE_COUNTERS_ARE_DIS
    }
}
#[doc = "Write proxy for field `CE`"]
pub struct CE_W<'a> {
    w: &'a mut W,
}
impl<'a> CE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The PWM Timer Counter and PWM Prescale Counter are enabled for counting."]
    #[inline(always)]
    pub fn the_pwm_timer_counte(self) -> &'a mut W {
        self.variant(CE_A::THE_PWM_TIMER_COUNTE)
    }
    #[doc = "The counters are disabled."]
    #[inline(always)]
    pub fn the_counters_are_dis(self) -> &'a mut W {
        self.variant(CE_A::THE_COUNTERS_ARE_DIS)
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
#[doc = "Counter Reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CR_A {
    #[doc = "1: The PWM Timer Counter and the PWM Prescale Counter are synchronously reset on the next positive edge of PCLK. The counters remain reset until this bit is returned to zero."]
    THE_PWM_TIMER_COUNTE = 1,
    #[doc = "0: Clear reset."]
    CLEAR_RESET_ = 0,
}
impl From<CR_A> for bool {
    #[inline(always)]
    fn from(variant: CR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CR`"]
pub type CR_R = crate::R<bool, CR_A>;
impl CR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CR_A {
        match self.bits {
            true => CR_A::THE_PWM_TIMER_COUNTE,
            false => CR_A::CLEAR_RESET_,
        }
    }
    #[doc = "Checks if the value of the field is `THE_PWM_TIMER_COUNTE`"]
    #[inline(always)]
    pub fn is_the_pwm_timer_counte(&self) -> bool {
        *self == CR_A::THE_PWM_TIMER_COUNTE
    }
    #[doc = "Checks if the value of the field is `CLEAR_RESET_`"]
    #[inline(always)]
    pub fn is_clear_reset_(&self) -> bool {
        *self == CR_A::CLEAR_RESET_
    }
}
#[doc = "Write proxy for field `CR`"]
pub struct CR_W<'a> {
    w: &'a mut W,
}
impl<'a> CR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CR_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The PWM Timer Counter and the PWM Prescale Counter are synchronously reset on the next positive edge of PCLK. The counters remain reset until this bit is returned to zero."]
    #[inline(always)]
    pub fn the_pwm_timer_counte(self) -> &'a mut W {
        self.variant(CR_A::THE_PWM_TIMER_COUNTE)
    }
    #[doc = "Clear reset."]
    #[inline(always)]
    pub fn clear_reset_(self) -> &'a mut W {
        self.variant(CR_A::CLEAR_RESET_)
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
#[doc = "PWM Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PWMEN_A {
    #[doc = "1: PWM mode is enabled (counter resets to 1). PWM mode causes the shadow registers to operate in connection with the Match registers. A program write to a Match register will not have an effect on the Match result until the corresponding bit in PWMLER has been set, followed by the occurrence of a PWM Match 0 event. Note that the PWM Match register that determines the PWM rate (PWM Match Register 0 - MR0) must be set up prior to the PWM being enabled. Otherwise a Match event will not occur to cause shadow register contents to become effective."]
    PWM_MODE_IS_ENABLED_ = 1,
    #[doc = "0: Timer mode is enabled (counter resets to 0)."]
    TIMER_MODE_IS_ENABLE = 0,
}
impl From<PWMEN_A> for bool {
    #[inline(always)]
    fn from(variant: PWMEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PWMEN`"]
pub type PWMEN_R = crate::R<bool, PWMEN_A>;
impl PWMEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PWMEN_A {
        match self.bits {
            true => PWMEN_A::PWM_MODE_IS_ENABLED_,
            false => PWMEN_A::TIMER_MODE_IS_ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `PWM_MODE_IS_ENABLED_`"]
    #[inline(always)]
    pub fn is_pwm_mode_is_enabled_(&self) -> bool {
        *self == PWMEN_A::PWM_MODE_IS_ENABLED_
    }
    #[doc = "Checks if the value of the field is `TIMER_MODE_IS_ENABLE`"]
    #[inline(always)]
    pub fn is_timer_mode_is_enable(&self) -> bool {
        *self == PWMEN_A::TIMER_MODE_IS_ENABLE
    }
}
#[doc = "Write proxy for field `PWMEN`"]
pub struct PWMEN_W<'a> {
    w: &'a mut W,
}
impl<'a> PWMEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PWMEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "PWM mode is enabled (counter resets to 1). PWM mode causes the shadow registers to operate in connection with the Match registers. A program write to a Match register will not have an effect on the Match result until the corresponding bit in PWMLER has been set, followed by the occurrence of a PWM Match 0 event. Note that the PWM Match register that determines the PWM rate (PWM Match Register 0 - MR0) must be set up prior to the PWM being enabled. Otherwise a Match event will not occur to cause shadow register contents to become effective."]
    #[inline(always)]
    pub fn pwm_mode_is_enabled_(self) -> &'a mut W {
        self.variant(PWMEN_A::PWM_MODE_IS_ENABLED_)
    }
    #[doc = "Timer mode is enabled (counter resets to 0)."]
    #[inline(always)]
    pub fn timer_mode_is_enable(self) -> &'a mut W {
        self.variant(PWMEN_A::TIMER_MODE_IS_ENABLE)
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
#[doc = "Master Disable (PWM0 only). The two PWMs may be synchronized using the Master Disable control bit. The Master disable bit of the Master PWM (PWM0 module) controls a secondary enable input to both PWMs, as shown in Figure 141. This bit has no function in the Slave PWM (PWM1).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MDIS_A {
    #[doc = "1: Master use. PWM0 is the master, and both PWMs are enabled for counting."]
    MASTER_USE_PWM0_IS_ = 1,
    #[doc = "0: Individual use. The PWMs are used independently, and the individual Counter Enable bits are used to control the PWMs."]
    INDIVIDUAL_USE_THE_ = 0,
}
impl From<MDIS_A> for bool {
    #[inline(always)]
    fn from(variant: MDIS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `MDIS`"]
pub type MDIS_R = crate::R<bool, MDIS_A>;
impl MDIS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MDIS_A {
        match self.bits {
            true => MDIS_A::MASTER_USE_PWM0_IS_,
            false => MDIS_A::INDIVIDUAL_USE_THE_,
        }
    }
    #[doc = "Checks if the value of the field is `MASTER_USE_PWM0_IS_`"]
    #[inline(always)]
    pub fn is_master_use_pwm0_is_(&self) -> bool {
        *self == MDIS_A::MASTER_USE_PWM0_IS_
    }
    #[doc = "Checks if the value of the field is `INDIVIDUAL_USE_THE_`"]
    #[inline(always)]
    pub fn is_individual_use_the_(&self) -> bool {
        *self == MDIS_A::INDIVIDUAL_USE_THE_
    }
}
#[doc = "Write proxy for field `MDIS`"]
pub struct MDIS_W<'a> {
    w: &'a mut W,
}
impl<'a> MDIS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MDIS_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Master use. PWM0 is the master, and both PWMs are enabled for counting."]
    #[inline(always)]
    pub fn master_use_pwm0_is_(self) -> &'a mut W {
        self.variant(MDIS_A::MASTER_USE_PWM0_IS_)
    }
    #[doc = "Individual use. The PWMs are used independently, and the individual Counter Enable bits are used to control the PWMs."]
    #[inline(always)]
    pub fn individual_use_the_(self) -> &'a mut W {
        self.variant(MDIS_A::INDIVIDUAL_USE_THE_)
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
    #[doc = "Bit 0 - Counter Enable"]
    #[inline(always)]
    pub fn ce(&self) -> CE_R {
        CE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Counter Reset"]
    #[inline(always)]
    pub fn cr(&self) -> CR_R {
        CR_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 3 - PWM Enable"]
    #[inline(always)]
    pub fn pwmen(&self) -> PWMEN_R {
        PWMEN_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Master Disable (PWM0 only). The two PWMs may be synchronized using the Master Disable control bit. The Master disable bit of the Master PWM (PWM0 module) controls a secondary enable input to both PWMs, as shown in Figure 141. This bit has no function in the Slave PWM (PWM1)."]
    #[inline(always)]
    pub fn mdis(&self) -> MDIS_R {
        MDIS_R::new(((self.bits >> 4) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Counter Enable"]
    #[inline(always)]
    pub fn ce(&mut self) -> CE_W {
        CE_W { w: self }
    }
    #[doc = "Bit 1 - Counter Reset"]
    #[inline(always)]
    pub fn cr(&mut self) -> CR_W {
        CR_W { w: self }
    }
    #[doc = "Bit 3 - PWM Enable"]
    #[inline(always)]
    pub fn pwmen(&mut self) -> PWMEN_W {
        PWMEN_W { w: self }
    }
    #[doc = "Bit 4 - Master Disable (PWM0 only). The two PWMs may be synchronized using the Master Disable control bit. The Master disable bit of the Master PWM (PWM0 module) controls a secondary enable input to both PWMs, as shown in Figure 141. This bit has no function in the Slave PWM (PWM1)."]
    #[inline(always)]
    pub fn mdis(&mut self) -> MDIS_W {
        MDIS_W { w: self }
    }
}
