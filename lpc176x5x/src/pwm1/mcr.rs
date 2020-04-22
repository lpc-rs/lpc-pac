#[doc = "Reader of register MCR"]
pub type R = crate::R<u32, super::MCR>;
#[doc = "Writer for register MCR"]
pub type W = crate::W<u32, super::MCR>;
#[doc = "Register MCR `reset()`'s with value 0"]
impl crate::ResetValue for super::MCR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Interrupt PWM0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PWMMR0I_A {
    #[doc = "0: Disabled."]
    DISABLED_ = 0,
    #[doc = "1: Interrupt on PWMMR0: an interrupt is generated when PWMMR0 matches the value in the PWMTC."]
    INTERRUPT_ON_PWMMR0 = 1,
}
impl From<PWMMR0I_A> for bool {
    #[inline(always)]
    fn from(variant: PWMMR0I_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PWMMR0I`"]
pub type PWMMR0I_R = crate::R<bool, PWMMR0I_A>;
impl PWMMR0I_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PWMMR0I_A {
        match self.bits {
            false => PWMMR0I_A::DISABLED_,
            true => PWMMR0I_A::INTERRUPT_ON_PWMMR0,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED_`"]
    #[inline(always)]
    pub fn is_disabled_(&self) -> bool {
        *self == PWMMR0I_A::DISABLED_
    }
    #[doc = "Checks if the value of the field is `INTERRUPT_ON_PWMMR0`"]
    #[inline(always)]
    pub fn is_interrupt_on_pwmmr0(&self) -> bool {
        *self == PWMMR0I_A::INTERRUPT_ON_PWMMR0
    }
}
#[doc = "Write proxy for field `PWMMR0I`"]
pub struct PWMMR0I_W<'a> {
    w: &'a mut W,
}
impl<'a> PWMMR0I_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PWMMR0I_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disabled."]
    #[inline(always)]
    pub fn disabled_(self) -> &'a mut W {
        self.variant(PWMMR0I_A::DISABLED_)
    }
    #[doc = "Interrupt on PWMMR0: an interrupt is generated when PWMMR0 matches the value in the PWMTC."]
    #[inline(always)]
    pub fn interrupt_on_pwmmr0(self) -> &'a mut W {
        self.variant(PWMMR0I_A::INTERRUPT_ON_PWMMR0)
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
#[doc = "Reset PWM0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PWMMR0R_A {
    #[doc = "0: Disabled."]
    DISABLED_ = 0,
    #[doc = "1: Reset on PWMMR0: the PWMTC will be reset if PWMMR0 matches it."]
    RESET_ON_PWMMR0_THE = 1,
}
impl From<PWMMR0R_A> for bool {
    #[inline(always)]
    fn from(variant: PWMMR0R_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PWMMR0R`"]
pub type PWMMR0R_R = crate::R<bool, PWMMR0R_A>;
impl PWMMR0R_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PWMMR0R_A {
        match self.bits {
            false => PWMMR0R_A::DISABLED_,
            true => PWMMR0R_A::RESET_ON_PWMMR0_THE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED_`"]
    #[inline(always)]
    pub fn is_disabled_(&self) -> bool {
        *self == PWMMR0R_A::DISABLED_
    }
    #[doc = "Checks if the value of the field is `RESET_ON_PWMMR0_THE`"]
    #[inline(always)]
    pub fn is_reset_on_pwmmr0_the(&self) -> bool {
        *self == PWMMR0R_A::RESET_ON_PWMMR0_THE
    }
}
#[doc = "Write proxy for field `PWMMR0R`"]
pub struct PWMMR0R_W<'a> {
    w: &'a mut W,
}
impl<'a> PWMMR0R_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PWMMR0R_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disabled."]
    #[inline(always)]
    pub fn disabled_(self) -> &'a mut W {
        self.variant(PWMMR0R_A::DISABLED_)
    }
    #[doc = "Reset on PWMMR0: the PWMTC will be reset if PWMMR0 matches it."]
    #[inline(always)]
    pub fn reset_on_pwmmr0_the(self) -> &'a mut W {
        self.variant(PWMMR0R_A::RESET_ON_PWMMR0_THE)
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
#[doc = "Stop PWM0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PWMMR0S_A {
    #[doc = "0: Disabled"]
    DISABLED = 0,
    #[doc = "1: Stop on PWMMR0: the PWMTC and PWMPC will be stopped and PWMTCR bit 0 will be set to 0 if PWMMR0 matches the PWMTC."]
    STOP_ON_PWMMR0_THE_ = 1,
}
impl From<PWMMR0S_A> for bool {
    #[inline(always)]
    fn from(variant: PWMMR0S_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PWMMR0S`"]
pub type PWMMR0S_R = crate::R<bool, PWMMR0S_A>;
impl PWMMR0S_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PWMMR0S_A {
        match self.bits {
            false => PWMMR0S_A::DISABLED,
            true => PWMMR0S_A::STOP_ON_PWMMR0_THE_,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PWMMR0S_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `STOP_ON_PWMMR0_THE_`"]
    #[inline(always)]
    pub fn is_stop_on_pwmmr0_the_(&self) -> bool {
        *self == PWMMR0S_A::STOP_ON_PWMMR0_THE_
    }
}
#[doc = "Write proxy for field `PWMMR0S`"]
pub struct PWMMR0S_W<'a> {
    w: &'a mut W,
}
impl<'a> PWMMR0S_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PWMMR0S_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(PWMMR0S_A::DISABLED)
    }
    #[doc = "Stop on PWMMR0: the PWMTC and PWMPC will be stopped and PWMTCR bit 0 will be set to 0 if PWMMR0 matches the PWMTC."]
    #[inline(always)]
    pub fn stop_on_pwmmr0_the_(self) -> &'a mut W {
        self.variant(PWMMR0S_A::STOP_ON_PWMMR0_THE_)
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
#[doc = "Interrupt PWM1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PWMMR1I_A {
    #[doc = "0: Disabled."]
    DISABLED_ = 0,
    #[doc = "1: Interrupt on PWMMR1: an interrupt is generated when PWMMR1 matches the value in the PWMTC."]
    INTERRUPT_ON_PWMMR1 = 1,
}
impl From<PWMMR1I_A> for bool {
    #[inline(always)]
    fn from(variant: PWMMR1I_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PWMMR1I`"]
pub type PWMMR1I_R = crate::R<bool, PWMMR1I_A>;
impl PWMMR1I_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PWMMR1I_A {
        match self.bits {
            false => PWMMR1I_A::DISABLED_,
            true => PWMMR1I_A::INTERRUPT_ON_PWMMR1,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED_`"]
    #[inline(always)]
    pub fn is_disabled_(&self) -> bool {
        *self == PWMMR1I_A::DISABLED_
    }
    #[doc = "Checks if the value of the field is `INTERRUPT_ON_PWMMR1`"]
    #[inline(always)]
    pub fn is_interrupt_on_pwmmr1(&self) -> bool {
        *self == PWMMR1I_A::INTERRUPT_ON_PWMMR1
    }
}
#[doc = "Write proxy for field `PWMMR1I`"]
pub struct PWMMR1I_W<'a> {
    w: &'a mut W,
}
impl<'a> PWMMR1I_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PWMMR1I_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disabled."]
    #[inline(always)]
    pub fn disabled_(self) -> &'a mut W {
        self.variant(PWMMR1I_A::DISABLED_)
    }
    #[doc = "Interrupt on PWMMR1: an interrupt is generated when PWMMR1 matches the value in the PWMTC."]
    #[inline(always)]
    pub fn interrupt_on_pwmmr1(self) -> &'a mut W {
        self.variant(PWMMR1I_A::INTERRUPT_ON_PWMMR1)
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
#[doc = "Reset PWM1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PWMMR1R_A {
    #[doc = "0: Disabled."]
    DISABLED_ = 0,
    #[doc = "1: Reset on PWMMR1: the PWMTC will be reset if PWMMR1 matches it."]
    RESET_ON_PWMMR1_THE = 1,
}
impl From<PWMMR1R_A> for bool {
    #[inline(always)]
    fn from(variant: PWMMR1R_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PWMMR1R`"]
pub type PWMMR1R_R = crate::R<bool, PWMMR1R_A>;
impl PWMMR1R_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PWMMR1R_A {
        match self.bits {
            false => PWMMR1R_A::DISABLED_,
            true => PWMMR1R_A::RESET_ON_PWMMR1_THE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED_`"]
    #[inline(always)]
    pub fn is_disabled_(&self) -> bool {
        *self == PWMMR1R_A::DISABLED_
    }
    #[doc = "Checks if the value of the field is `RESET_ON_PWMMR1_THE`"]
    #[inline(always)]
    pub fn is_reset_on_pwmmr1_the(&self) -> bool {
        *self == PWMMR1R_A::RESET_ON_PWMMR1_THE
    }
}
#[doc = "Write proxy for field `PWMMR1R`"]
pub struct PWMMR1R_W<'a> {
    w: &'a mut W,
}
impl<'a> PWMMR1R_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PWMMR1R_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disabled."]
    #[inline(always)]
    pub fn disabled_(self) -> &'a mut W {
        self.variant(PWMMR1R_A::DISABLED_)
    }
    #[doc = "Reset on PWMMR1: the PWMTC will be reset if PWMMR1 matches it."]
    #[inline(always)]
    pub fn reset_on_pwmmr1_the(self) -> &'a mut W {
        self.variant(PWMMR1R_A::RESET_ON_PWMMR1_THE)
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
#[doc = "Stop PWM1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PWMMR1S_A {
    #[doc = "0: Disabled"]
    DISABLED = 0,
    #[doc = "1: Stop on PWMMR1: the PWMTC and PWMPC will be stopped and PWMTCR bit 0 will be set to 0 if PWMMR1 matches the PWMTC."]
    STOP_ON_PWMMR1_THE_ = 1,
}
impl From<PWMMR1S_A> for bool {
    #[inline(always)]
    fn from(variant: PWMMR1S_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PWMMR1S`"]
pub type PWMMR1S_R = crate::R<bool, PWMMR1S_A>;
impl PWMMR1S_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PWMMR1S_A {
        match self.bits {
            false => PWMMR1S_A::DISABLED,
            true => PWMMR1S_A::STOP_ON_PWMMR1_THE_,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PWMMR1S_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `STOP_ON_PWMMR1_THE_`"]
    #[inline(always)]
    pub fn is_stop_on_pwmmr1_the_(&self) -> bool {
        *self == PWMMR1S_A::STOP_ON_PWMMR1_THE_
    }
}
#[doc = "Write proxy for field `PWMMR1S`"]
pub struct PWMMR1S_W<'a> {
    w: &'a mut W,
}
impl<'a> PWMMR1S_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PWMMR1S_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(PWMMR1S_A::DISABLED)
    }
    #[doc = "Stop on PWMMR1: the PWMTC and PWMPC will be stopped and PWMTCR bit 0 will be set to 0 if PWMMR1 matches the PWMTC."]
    #[inline(always)]
    pub fn stop_on_pwmmr1_the_(self) -> &'a mut W {
        self.variant(PWMMR1S_A::STOP_ON_PWMMR1_THE_)
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
#[doc = "Interrupt PWM0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PWMMR2I_A {
    #[doc = "0: Disabled."]
    DISABLED_ = 0,
    #[doc = "1: Interrupt on PWMMR2: an interrupt is generated when PWMMR2 matches the value in the PWMTC."]
    INTERRUPT_ON_PWMMR2 = 1,
}
impl From<PWMMR2I_A> for bool {
    #[inline(always)]
    fn from(variant: PWMMR2I_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PWMMR2I`"]
pub type PWMMR2I_R = crate::R<bool, PWMMR2I_A>;
impl PWMMR2I_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PWMMR2I_A {
        match self.bits {
            false => PWMMR2I_A::DISABLED_,
            true => PWMMR2I_A::INTERRUPT_ON_PWMMR2,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED_`"]
    #[inline(always)]
    pub fn is_disabled_(&self) -> bool {
        *self == PWMMR2I_A::DISABLED_
    }
    #[doc = "Checks if the value of the field is `INTERRUPT_ON_PWMMR2`"]
    #[inline(always)]
    pub fn is_interrupt_on_pwmmr2(&self) -> bool {
        *self == PWMMR2I_A::INTERRUPT_ON_PWMMR2
    }
}
#[doc = "Write proxy for field `PWMMR2I`"]
pub struct PWMMR2I_W<'a> {
    w: &'a mut W,
}
impl<'a> PWMMR2I_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PWMMR2I_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disabled."]
    #[inline(always)]
    pub fn disabled_(self) -> &'a mut W {
        self.variant(PWMMR2I_A::DISABLED_)
    }
    #[doc = "Interrupt on PWMMR2: an interrupt is generated when PWMMR2 matches the value in the PWMTC."]
    #[inline(always)]
    pub fn interrupt_on_pwmmr2(self) -> &'a mut W {
        self.variant(PWMMR2I_A::INTERRUPT_ON_PWMMR2)
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
#[doc = "Reset PWM0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PWMMR2R_A {
    #[doc = "0: Disabled."]
    DISABLED_ = 0,
    #[doc = "1: Reset on PWMMR2: the PWMTC will be reset if PWMMR2 matches it."]
    RESET_ON_PWMMR2_THE = 1,
}
impl From<PWMMR2R_A> for bool {
    #[inline(always)]
    fn from(variant: PWMMR2R_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PWMMR2R`"]
pub type PWMMR2R_R = crate::R<bool, PWMMR2R_A>;
impl PWMMR2R_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PWMMR2R_A {
        match self.bits {
            false => PWMMR2R_A::DISABLED_,
            true => PWMMR2R_A::RESET_ON_PWMMR2_THE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED_`"]
    #[inline(always)]
    pub fn is_disabled_(&self) -> bool {
        *self == PWMMR2R_A::DISABLED_
    }
    #[doc = "Checks if the value of the field is `RESET_ON_PWMMR2_THE`"]
    #[inline(always)]
    pub fn is_reset_on_pwmmr2_the(&self) -> bool {
        *self == PWMMR2R_A::RESET_ON_PWMMR2_THE
    }
}
#[doc = "Write proxy for field `PWMMR2R`"]
pub struct PWMMR2R_W<'a> {
    w: &'a mut W,
}
impl<'a> PWMMR2R_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PWMMR2R_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disabled."]
    #[inline(always)]
    pub fn disabled_(self) -> &'a mut W {
        self.variant(PWMMR2R_A::DISABLED_)
    }
    #[doc = "Reset on PWMMR2: the PWMTC will be reset if PWMMR2 matches it."]
    #[inline(always)]
    pub fn reset_on_pwmmr2_the(self) -> &'a mut W {
        self.variant(PWMMR2R_A::RESET_ON_PWMMR2_THE)
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
#[doc = "Stop PWM0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PWMMR2S_A {
    #[doc = "0: Disabled"]
    DISABLED = 0,
    #[doc = "1: Stop on PWMMR2: the PWMTC and PWMPC will be stopped and PWMTCR bit 0 will be set to 0 if PWMMR0 matches the PWMTC."]
    STOP_ON_PWMMR2_THE_ = 1,
}
impl From<PWMMR2S_A> for bool {
    #[inline(always)]
    fn from(variant: PWMMR2S_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PWMMR2S`"]
pub type PWMMR2S_R = crate::R<bool, PWMMR2S_A>;
impl PWMMR2S_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PWMMR2S_A {
        match self.bits {
            false => PWMMR2S_A::DISABLED,
            true => PWMMR2S_A::STOP_ON_PWMMR2_THE_,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PWMMR2S_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `STOP_ON_PWMMR2_THE_`"]
    #[inline(always)]
    pub fn is_stop_on_pwmmr2_the_(&self) -> bool {
        *self == PWMMR2S_A::STOP_ON_PWMMR2_THE_
    }
}
#[doc = "Write proxy for field `PWMMR2S`"]
pub struct PWMMR2S_W<'a> {
    w: &'a mut W,
}
impl<'a> PWMMR2S_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PWMMR2S_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(PWMMR2S_A::DISABLED)
    }
    #[doc = "Stop on PWMMR2: the PWMTC and PWMPC will be stopped and PWMTCR bit 0 will be set to 0 if PWMMR0 matches the PWMTC."]
    #[inline(always)]
    pub fn stop_on_pwmmr2_the_(self) -> &'a mut W {
        self.variant(PWMMR2S_A::STOP_ON_PWMMR2_THE_)
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
#[doc = "Interrupt PWM3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PWMMR3I_A {
    #[doc = "0: Disabled."]
    DISABLED_ = 0,
    #[doc = "1: Interrupt on PWMMR3: an interrupt is generated when PWMMR3 matches the value in the PWMTC."]
    INTERRUPT_ON_PWMMR3 = 1,
}
impl From<PWMMR3I_A> for bool {
    #[inline(always)]
    fn from(variant: PWMMR3I_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PWMMR3I`"]
pub type PWMMR3I_R = crate::R<bool, PWMMR3I_A>;
impl PWMMR3I_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PWMMR3I_A {
        match self.bits {
            false => PWMMR3I_A::DISABLED_,
            true => PWMMR3I_A::INTERRUPT_ON_PWMMR3,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED_`"]
    #[inline(always)]
    pub fn is_disabled_(&self) -> bool {
        *self == PWMMR3I_A::DISABLED_
    }
    #[doc = "Checks if the value of the field is `INTERRUPT_ON_PWMMR3`"]
    #[inline(always)]
    pub fn is_interrupt_on_pwmmr3(&self) -> bool {
        *self == PWMMR3I_A::INTERRUPT_ON_PWMMR3
    }
}
#[doc = "Write proxy for field `PWMMR3I`"]
pub struct PWMMR3I_W<'a> {
    w: &'a mut W,
}
impl<'a> PWMMR3I_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PWMMR3I_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disabled."]
    #[inline(always)]
    pub fn disabled_(self) -> &'a mut W {
        self.variant(PWMMR3I_A::DISABLED_)
    }
    #[doc = "Interrupt on PWMMR3: an interrupt is generated when PWMMR3 matches the value in the PWMTC."]
    #[inline(always)]
    pub fn interrupt_on_pwmmr3(self) -> &'a mut W {
        self.variant(PWMMR3I_A::INTERRUPT_ON_PWMMR3)
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
        self.w
    }
}
#[doc = "Reset PWM3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PWMMR3R_A {
    #[doc = "0: Disabled."]
    DISABLED_ = 0,
    #[doc = "1: Reset on PWMMR3: the PWMTC will be reset if PWMMR3 matches it."]
    RESET_ON_PWMMR3_THE = 1,
}
impl From<PWMMR3R_A> for bool {
    #[inline(always)]
    fn from(variant: PWMMR3R_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PWMMR3R`"]
pub type PWMMR3R_R = crate::R<bool, PWMMR3R_A>;
impl PWMMR3R_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PWMMR3R_A {
        match self.bits {
            false => PWMMR3R_A::DISABLED_,
            true => PWMMR3R_A::RESET_ON_PWMMR3_THE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED_`"]
    #[inline(always)]
    pub fn is_disabled_(&self) -> bool {
        *self == PWMMR3R_A::DISABLED_
    }
    #[doc = "Checks if the value of the field is `RESET_ON_PWMMR3_THE`"]
    #[inline(always)]
    pub fn is_reset_on_pwmmr3_the(&self) -> bool {
        *self == PWMMR3R_A::RESET_ON_PWMMR3_THE
    }
}
#[doc = "Write proxy for field `PWMMR3R`"]
pub struct PWMMR3R_W<'a> {
    w: &'a mut W,
}
impl<'a> PWMMR3R_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PWMMR3R_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disabled."]
    #[inline(always)]
    pub fn disabled_(self) -> &'a mut W {
        self.variant(PWMMR3R_A::DISABLED_)
    }
    #[doc = "Reset on PWMMR3: the PWMTC will be reset if PWMMR3 matches it."]
    #[inline(always)]
    pub fn reset_on_pwmmr3_the(self) -> &'a mut W {
        self.variant(PWMMR3R_A::RESET_ON_PWMMR3_THE)
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
        self.w
    }
}
#[doc = "Stop PWM0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PWMMR3S_A {
    #[doc = "0: Disabled"]
    DISABLED = 0,
    #[doc = "1: Stop on PWMMR3: the PWMTC and PWMPC will be stopped and PWMTCR bit 0 will be set to 0 if PWMMR0 matches the PWMTC."]
    STOP_ON_PWMMR3_THE_ = 1,
}
impl From<PWMMR3S_A> for bool {
    #[inline(always)]
    fn from(variant: PWMMR3S_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PWMMR3S`"]
pub type PWMMR3S_R = crate::R<bool, PWMMR3S_A>;
impl PWMMR3S_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PWMMR3S_A {
        match self.bits {
            false => PWMMR3S_A::DISABLED,
            true => PWMMR3S_A::STOP_ON_PWMMR3_THE_,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PWMMR3S_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `STOP_ON_PWMMR3_THE_`"]
    #[inline(always)]
    pub fn is_stop_on_pwmmr3_the_(&self) -> bool {
        *self == PWMMR3S_A::STOP_ON_PWMMR3_THE_
    }
}
#[doc = "Write proxy for field `PWMMR3S`"]
pub struct PWMMR3S_W<'a> {
    w: &'a mut W,
}
impl<'a> PWMMR3S_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PWMMR3S_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(PWMMR3S_A::DISABLED)
    }
    #[doc = "Stop on PWMMR3: the PWMTC and PWMPC will be stopped and PWMTCR bit 0 will be set to 0 if PWMMR0 matches the PWMTC."]
    #[inline(always)]
    pub fn stop_on_pwmmr3_the_(self) -> &'a mut W {
        self.variant(PWMMR3S_A::STOP_ON_PWMMR3_THE_)
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u32) & 0x01) << 11);
        self.w
    }
}
#[doc = "Interrupt PWM4\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PWMMR4I_A {
    #[doc = "0: Disabled."]
    DISABLED_ = 0,
    #[doc = "1: Interrupt on PWMMR4: an interrupt is generated when PWMMR4 matches the value in the PWMTC."]
    INTERRUPT_ON_PWMMR4 = 1,
}
impl From<PWMMR4I_A> for bool {
    #[inline(always)]
    fn from(variant: PWMMR4I_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PWMMR4I`"]
pub type PWMMR4I_R = crate::R<bool, PWMMR4I_A>;
impl PWMMR4I_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PWMMR4I_A {
        match self.bits {
            false => PWMMR4I_A::DISABLED_,
            true => PWMMR4I_A::INTERRUPT_ON_PWMMR4,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED_`"]
    #[inline(always)]
    pub fn is_disabled_(&self) -> bool {
        *self == PWMMR4I_A::DISABLED_
    }
    #[doc = "Checks if the value of the field is `INTERRUPT_ON_PWMMR4`"]
    #[inline(always)]
    pub fn is_interrupt_on_pwmmr4(&self) -> bool {
        *self == PWMMR4I_A::INTERRUPT_ON_PWMMR4
    }
}
#[doc = "Write proxy for field `PWMMR4I`"]
pub struct PWMMR4I_W<'a> {
    w: &'a mut W,
}
impl<'a> PWMMR4I_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PWMMR4I_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disabled."]
    #[inline(always)]
    pub fn disabled_(self) -> &'a mut W {
        self.variant(PWMMR4I_A::DISABLED_)
    }
    #[doc = "Interrupt on PWMMR4: an interrupt is generated when PWMMR4 matches the value in the PWMTC."]
    #[inline(always)]
    pub fn interrupt_on_pwmmr4(self) -> &'a mut W {
        self.variant(PWMMR4I_A::INTERRUPT_ON_PWMMR4)
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u32) & 0x01) << 12);
        self.w
    }
}
#[doc = "Reset PWM4\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PWMMR4R_A {
    #[doc = "0: Disabled."]
    DISABLED_ = 0,
    #[doc = "1: Reset on PWMMR4: the PWMTC will be reset if PWMMR4 matches it."]
    RESET_ON_PWMMR4_THE = 1,
}
impl From<PWMMR4R_A> for bool {
    #[inline(always)]
    fn from(variant: PWMMR4R_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PWMMR4R`"]
pub type PWMMR4R_R = crate::R<bool, PWMMR4R_A>;
impl PWMMR4R_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PWMMR4R_A {
        match self.bits {
            false => PWMMR4R_A::DISABLED_,
            true => PWMMR4R_A::RESET_ON_PWMMR4_THE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED_`"]
    #[inline(always)]
    pub fn is_disabled_(&self) -> bool {
        *self == PWMMR4R_A::DISABLED_
    }
    #[doc = "Checks if the value of the field is `RESET_ON_PWMMR4_THE`"]
    #[inline(always)]
    pub fn is_reset_on_pwmmr4_the(&self) -> bool {
        *self == PWMMR4R_A::RESET_ON_PWMMR4_THE
    }
}
#[doc = "Write proxy for field `PWMMR4R`"]
pub struct PWMMR4R_W<'a> {
    w: &'a mut W,
}
impl<'a> PWMMR4R_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PWMMR4R_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disabled."]
    #[inline(always)]
    pub fn disabled_(self) -> &'a mut W {
        self.variant(PWMMR4R_A::DISABLED_)
    }
    #[doc = "Reset on PWMMR4: the PWMTC will be reset if PWMMR4 matches it."]
    #[inline(always)]
    pub fn reset_on_pwmmr4_the(self) -> &'a mut W {
        self.variant(PWMMR4R_A::RESET_ON_PWMMR4_THE)
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | (((value as u32) & 0x01) << 13);
        self.w
    }
}
#[doc = "Stop PWM4\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PWMMR4S_A {
    #[doc = "0: Disabled"]
    DISABLED = 0,
    #[doc = "1: Stop on PWMMR4: the PWMTC and PWMPC will be stopped and PWMTCR bit 0 will be set to 0 if PWMMR4 matches the PWMTC."]
    STOP_ON_PWMMR4_THE_ = 1,
}
impl From<PWMMR4S_A> for bool {
    #[inline(always)]
    fn from(variant: PWMMR4S_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PWMMR4S`"]
pub type PWMMR4S_R = crate::R<bool, PWMMR4S_A>;
impl PWMMR4S_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PWMMR4S_A {
        match self.bits {
            false => PWMMR4S_A::DISABLED,
            true => PWMMR4S_A::STOP_ON_PWMMR4_THE_,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PWMMR4S_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `STOP_ON_PWMMR4_THE_`"]
    #[inline(always)]
    pub fn is_stop_on_pwmmr4_the_(&self) -> bool {
        *self == PWMMR4S_A::STOP_ON_PWMMR4_THE_
    }
}
#[doc = "Write proxy for field `PWMMR4S`"]
pub struct PWMMR4S_W<'a> {
    w: &'a mut W,
}
impl<'a> PWMMR4S_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PWMMR4S_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(PWMMR4S_A::DISABLED)
    }
    #[doc = "Stop on PWMMR4: the PWMTC and PWMPC will be stopped and PWMTCR bit 0 will be set to 0 if PWMMR4 matches the PWMTC."]
    #[inline(always)]
    pub fn stop_on_pwmmr4_the_(self) -> &'a mut W {
        self.variant(PWMMR4S_A::STOP_ON_PWMMR4_THE_)
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | (((value as u32) & 0x01) << 14);
        self.w
    }
}
#[doc = "Interrupt PWM5\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PWMMR5I_A {
    #[doc = "0: Disabled."]
    DISABLED_ = 0,
    #[doc = "1: Interrupt on PWMMR5: an interrupt is generated when PWMMR5 matches the value in the PWMTC."]
    INTERRUPT_ON_PWMMR5 = 1,
}
impl From<PWMMR5I_A> for bool {
    #[inline(always)]
    fn from(variant: PWMMR5I_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PWMMR5I`"]
pub type PWMMR5I_R = crate::R<bool, PWMMR5I_A>;
impl PWMMR5I_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PWMMR5I_A {
        match self.bits {
            false => PWMMR5I_A::DISABLED_,
            true => PWMMR5I_A::INTERRUPT_ON_PWMMR5,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED_`"]
    #[inline(always)]
    pub fn is_disabled_(&self) -> bool {
        *self == PWMMR5I_A::DISABLED_
    }
    #[doc = "Checks if the value of the field is `INTERRUPT_ON_PWMMR5`"]
    #[inline(always)]
    pub fn is_interrupt_on_pwmmr5(&self) -> bool {
        *self == PWMMR5I_A::INTERRUPT_ON_PWMMR5
    }
}
#[doc = "Write proxy for field `PWMMR5I`"]
pub struct PWMMR5I_W<'a> {
    w: &'a mut W,
}
impl<'a> PWMMR5I_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PWMMR5I_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disabled."]
    #[inline(always)]
    pub fn disabled_(self) -> &'a mut W {
        self.variant(PWMMR5I_A::DISABLED_)
    }
    #[doc = "Interrupt on PWMMR5: an interrupt is generated when PWMMR5 matches the value in the PWMTC."]
    #[inline(always)]
    pub fn interrupt_on_pwmmr5(self) -> &'a mut W {
        self.variant(PWMMR5I_A::INTERRUPT_ON_PWMMR5)
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u32) & 0x01) << 15);
        self.w
    }
}
#[doc = "Reset PWM5\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PWMMR5R_A {
    #[doc = "0: Disabled."]
    DISABLED_ = 0,
    #[doc = "1: Reset on PWMMR5: the PWMTC will be reset if PWMMR5 matches it."]
    RESET_ON_PWMMR5_THE = 1,
}
impl From<PWMMR5R_A> for bool {
    #[inline(always)]
    fn from(variant: PWMMR5R_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PWMMR5R`"]
pub type PWMMR5R_R = crate::R<bool, PWMMR5R_A>;
impl PWMMR5R_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PWMMR5R_A {
        match self.bits {
            false => PWMMR5R_A::DISABLED_,
            true => PWMMR5R_A::RESET_ON_PWMMR5_THE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED_`"]
    #[inline(always)]
    pub fn is_disabled_(&self) -> bool {
        *self == PWMMR5R_A::DISABLED_
    }
    #[doc = "Checks if the value of the field is `RESET_ON_PWMMR5_THE`"]
    #[inline(always)]
    pub fn is_reset_on_pwmmr5_the(&self) -> bool {
        *self == PWMMR5R_A::RESET_ON_PWMMR5_THE
    }
}
#[doc = "Write proxy for field `PWMMR5R`"]
pub struct PWMMR5R_W<'a> {
    w: &'a mut W,
}
impl<'a> PWMMR5R_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PWMMR5R_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disabled."]
    #[inline(always)]
    pub fn disabled_(self) -> &'a mut W {
        self.variant(PWMMR5R_A::DISABLED_)
    }
    #[doc = "Reset on PWMMR5: the PWMTC will be reset if PWMMR5 matches it."]
    #[inline(always)]
    pub fn reset_on_pwmmr5_the(self) -> &'a mut W {
        self.variant(PWMMR5R_A::RESET_ON_PWMMR5_THE)
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
        self.w
    }
}
#[doc = "Stop PWM5\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PWMMR5S_A {
    #[doc = "0: Disabled"]
    DISABLED = 0,
    #[doc = "1: Stop on PWMMR5: the PWMTC and PWMPC will be stopped and PWMTCR bit 0 will be set to 0 if PWMMR5 matches the PWMTC."]
    STOP_ON_PWMMR5_THE_ = 1,
}
impl From<PWMMR5S_A> for bool {
    #[inline(always)]
    fn from(variant: PWMMR5S_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PWMMR5S`"]
pub type PWMMR5S_R = crate::R<bool, PWMMR5S_A>;
impl PWMMR5S_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PWMMR5S_A {
        match self.bits {
            false => PWMMR5S_A::DISABLED,
            true => PWMMR5S_A::STOP_ON_PWMMR5_THE_,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PWMMR5S_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `STOP_ON_PWMMR5_THE_`"]
    #[inline(always)]
    pub fn is_stop_on_pwmmr5_the_(&self) -> bool {
        *self == PWMMR5S_A::STOP_ON_PWMMR5_THE_
    }
}
#[doc = "Write proxy for field `PWMMR5S`"]
pub struct PWMMR5S_W<'a> {
    w: &'a mut W,
}
impl<'a> PWMMR5S_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PWMMR5S_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(PWMMR5S_A::DISABLED)
    }
    #[doc = "Stop on PWMMR5: the PWMTC and PWMPC will be stopped and PWMTCR bit 0 will be set to 0 if PWMMR5 matches the PWMTC."]
    #[inline(always)]
    pub fn stop_on_pwmmr5_the_(self) -> &'a mut W {
        self.variant(PWMMR5S_A::STOP_ON_PWMMR5_THE_)
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | (((value as u32) & 0x01) << 17);
        self.w
    }
}
#[doc = "Interrupt PWM6\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PWMMR6I_A {
    #[doc = "0: Disabled."]
    DISABLED_ = 0,
    #[doc = "1: Interrupt on PWMMR6: an interrupt is generated when PWMMR6 matches the value in the PWMTC."]
    INTERRUPT_ON_PWMMR6 = 1,
}
impl From<PWMMR6I_A> for bool {
    #[inline(always)]
    fn from(variant: PWMMR6I_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PWMMR6I`"]
pub type PWMMR6I_R = crate::R<bool, PWMMR6I_A>;
impl PWMMR6I_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PWMMR6I_A {
        match self.bits {
            false => PWMMR6I_A::DISABLED_,
            true => PWMMR6I_A::INTERRUPT_ON_PWMMR6,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED_`"]
    #[inline(always)]
    pub fn is_disabled_(&self) -> bool {
        *self == PWMMR6I_A::DISABLED_
    }
    #[doc = "Checks if the value of the field is `INTERRUPT_ON_PWMMR6`"]
    #[inline(always)]
    pub fn is_interrupt_on_pwmmr6(&self) -> bool {
        *self == PWMMR6I_A::INTERRUPT_ON_PWMMR6
    }
}
#[doc = "Write proxy for field `PWMMR6I`"]
pub struct PWMMR6I_W<'a> {
    w: &'a mut W,
}
impl<'a> PWMMR6I_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PWMMR6I_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disabled."]
    #[inline(always)]
    pub fn disabled_(self) -> &'a mut W {
        self.variant(PWMMR6I_A::DISABLED_)
    }
    #[doc = "Interrupt on PWMMR6: an interrupt is generated when PWMMR6 matches the value in the PWMTC."]
    #[inline(always)]
    pub fn interrupt_on_pwmmr6(self) -> &'a mut W {
        self.variant(PWMMR6I_A::INTERRUPT_ON_PWMMR6)
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | (((value as u32) & 0x01) << 18);
        self.w
    }
}
#[doc = "Reset PWM6\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PWMMR6R_A {
    #[doc = "0: Disabled."]
    DISABLED_ = 0,
    #[doc = "1: Reset on PWMMR6: the PWMTC will be reset if PWMMR6 matches it."]
    RESET_ON_PWMMR6_THE = 1,
}
impl From<PWMMR6R_A> for bool {
    #[inline(always)]
    fn from(variant: PWMMR6R_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PWMMR6R`"]
pub type PWMMR6R_R = crate::R<bool, PWMMR6R_A>;
impl PWMMR6R_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PWMMR6R_A {
        match self.bits {
            false => PWMMR6R_A::DISABLED_,
            true => PWMMR6R_A::RESET_ON_PWMMR6_THE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED_`"]
    #[inline(always)]
    pub fn is_disabled_(&self) -> bool {
        *self == PWMMR6R_A::DISABLED_
    }
    #[doc = "Checks if the value of the field is `RESET_ON_PWMMR6_THE`"]
    #[inline(always)]
    pub fn is_reset_on_pwmmr6_the(&self) -> bool {
        *self == PWMMR6R_A::RESET_ON_PWMMR6_THE
    }
}
#[doc = "Write proxy for field `PWMMR6R`"]
pub struct PWMMR6R_W<'a> {
    w: &'a mut W,
}
impl<'a> PWMMR6R_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PWMMR6R_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disabled."]
    #[inline(always)]
    pub fn disabled_(self) -> &'a mut W {
        self.variant(PWMMR6R_A::DISABLED_)
    }
    #[doc = "Reset on PWMMR6: the PWMTC will be reset if PWMMR6 matches it."]
    #[inline(always)]
    pub fn reset_on_pwmmr6_the(self) -> &'a mut W {
        self.variant(PWMMR6R_A::RESET_ON_PWMMR6_THE)
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | (((value as u32) & 0x01) << 19);
        self.w
    }
}
#[doc = "Stop PWM6\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PWMMR6S_A {
    #[doc = "0: Disabled"]
    DISABLED = 0,
    #[doc = "1: Stop on PWMMR6: the PWMTC and PWMPC will be stopped and PWMTCR bit 0 will be set to 0 if PWMMR6 matches the PWMTC."]
    STOP_ON_PWMMR6_THE_ = 1,
}
impl From<PWMMR6S_A> for bool {
    #[inline(always)]
    fn from(variant: PWMMR6S_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PWMMR6S`"]
pub type PWMMR6S_R = crate::R<bool, PWMMR6S_A>;
impl PWMMR6S_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PWMMR6S_A {
        match self.bits {
            false => PWMMR6S_A::DISABLED,
            true => PWMMR6S_A::STOP_ON_PWMMR6_THE_,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PWMMR6S_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `STOP_ON_PWMMR6_THE_`"]
    #[inline(always)]
    pub fn is_stop_on_pwmmr6_the_(&self) -> bool {
        *self == PWMMR6S_A::STOP_ON_PWMMR6_THE_
    }
}
#[doc = "Write proxy for field `PWMMR6S`"]
pub struct PWMMR6S_W<'a> {
    w: &'a mut W,
}
impl<'a> PWMMR6S_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PWMMR6S_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(PWMMR6S_A::DISABLED)
    }
    #[doc = "Stop on PWMMR6: the PWMTC and PWMPC will be stopped and PWMTCR bit 0 will be set to 0 if PWMMR6 matches the PWMTC."]
    #[inline(always)]
    pub fn stop_on_pwmmr6_the_(self) -> &'a mut W {
        self.variant(PWMMR6S_A::STOP_ON_PWMMR6_THE_)
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | (((value as u32) & 0x01) << 20);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Interrupt PWM0"]
    #[inline(always)]
    pub fn pwmmr0i(&self) -> PWMMR0I_R {
        PWMMR0I_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Reset PWM0"]
    #[inline(always)]
    pub fn pwmmr0r(&self) -> PWMMR0R_R {
        PWMMR0R_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Stop PWM0"]
    #[inline(always)]
    pub fn pwmmr0s(&self) -> PWMMR0S_R {
        PWMMR0S_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Interrupt PWM1"]
    #[inline(always)]
    pub fn pwmmr1i(&self) -> PWMMR1I_R {
        PWMMR1I_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Reset PWM1"]
    #[inline(always)]
    pub fn pwmmr1r(&self) -> PWMMR1R_R {
        PWMMR1R_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Stop PWM1"]
    #[inline(always)]
    pub fn pwmmr1s(&self) -> PWMMR1S_R {
        PWMMR1S_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Interrupt PWM0"]
    #[inline(always)]
    pub fn pwmmr2i(&self) -> PWMMR2I_R {
        PWMMR2I_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Reset PWM0"]
    #[inline(always)]
    pub fn pwmmr2r(&self) -> PWMMR2R_R {
        PWMMR2R_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Stop PWM0"]
    #[inline(always)]
    pub fn pwmmr2s(&self) -> PWMMR2S_R {
        PWMMR2S_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Interrupt PWM3"]
    #[inline(always)]
    pub fn pwmmr3i(&self) -> PWMMR3I_R {
        PWMMR3I_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Reset PWM3"]
    #[inline(always)]
    pub fn pwmmr3r(&self) -> PWMMR3R_R {
        PWMMR3R_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Stop PWM0"]
    #[inline(always)]
    pub fn pwmmr3s(&self) -> PWMMR3S_R {
        PWMMR3S_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Interrupt PWM4"]
    #[inline(always)]
    pub fn pwmmr4i(&self) -> PWMMR4I_R {
        PWMMR4I_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Reset PWM4"]
    #[inline(always)]
    pub fn pwmmr4r(&self) -> PWMMR4R_R {
        PWMMR4R_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Stop PWM4"]
    #[inline(always)]
    pub fn pwmmr4s(&self) -> PWMMR4S_R {
        PWMMR4S_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Interrupt PWM5"]
    #[inline(always)]
    pub fn pwmmr5i(&self) -> PWMMR5I_R {
        PWMMR5I_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Reset PWM5"]
    #[inline(always)]
    pub fn pwmmr5r(&self) -> PWMMR5R_R {
        PWMMR5R_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Stop PWM5"]
    #[inline(always)]
    pub fn pwmmr5s(&self) -> PWMMR5S_R {
        PWMMR5S_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Interrupt PWM6"]
    #[inline(always)]
    pub fn pwmmr6i(&self) -> PWMMR6I_R {
        PWMMR6I_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Reset PWM6"]
    #[inline(always)]
    pub fn pwmmr6r(&self) -> PWMMR6R_R {
        PWMMR6R_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Stop PWM6"]
    #[inline(always)]
    pub fn pwmmr6s(&self) -> PWMMR6S_R {
        PWMMR6S_R::new(((self.bits >> 20) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Interrupt PWM0"]
    #[inline(always)]
    pub fn pwmmr0i(&mut self) -> PWMMR0I_W {
        PWMMR0I_W { w: self }
    }
    #[doc = "Bit 1 - Reset PWM0"]
    #[inline(always)]
    pub fn pwmmr0r(&mut self) -> PWMMR0R_W {
        PWMMR0R_W { w: self }
    }
    #[doc = "Bit 2 - Stop PWM0"]
    #[inline(always)]
    pub fn pwmmr0s(&mut self) -> PWMMR0S_W {
        PWMMR0S_W { w: self }
    }
    #[doc = "Bit 3 - Interrupt PWM1"]
    #[inline(always)]
    pub fn pwmmr1i(&mut self) -> PWMMR1I_W {
        PWMMR1I_W { w: self }
    }
    #[doc = "Bit 4 - Reset PWM1"]
    #[inline(always)]
    pub fn pwmmr1r(&mut self) -> PWMMR1R_W {
        PWMMR1R_W { w: self }
    }
    #[doc = "Bit 5 - Stop PWM1"]
    #[inline(always)]
    pub fn pwmmr1s(&mut self) -> PWMMR1S_W {
        PWMMR1S_W { w: self }
    }
    #[doc = "Bit 6 - Interrupt PWM0"]
    #[inline(always)]
    pub fn pwmmr2i(&mut self) -> PWMMR2I_W {
        PWMMR2I_W { w: self }
    }
    #[doc = "Bit 7 - Reset PWM0"]
    #[inline(always)]
    pub fn pwmmr2r(&mut self) -> PWMMR2R_W {
        PWMMR2R_W { w: self }
    }
    #[doc = "Bit 8 - Stop PWM0"]
    #[inline(always)]
    pub fn pwmmr2s(&mut self) -> PWMMR2S_W {
        PWMMR2S_W { w: self }
    }
    #[doc = "Bit 9 - Interrupt PWM3"]
    #[inline(always)]
    pub fn pwmmr3i(&mut self) -> PWMMR3I_W {
        PWMMR3I_W { w: self }
    }
    #[doc = "Bit 10 - Reset PWM3"]
    #[inline(always)]
    pub fn pwmmr3r(&mut self) -> PWMMR3R_W {
        PWMMR3R_W { w: self }
    }
    #[doc = "Bit 11 - Stop PWM0"]
    #[inline(always)]
    pub fn pwmmr3s(&mut self) -> PWMMR3S_W {
        PWMMR3S_W { w: self }
    }
    #[doc = "Bit 12 - Interrupt PWM4"]
    #[inline(always)]
    pub fn pwmmr4i(&mut self) -> PWMMR4I_W {
        PWMMR4I_W { w: self }
    }
    #[doc = "Bit 13 - Reset PWM4"]
    #[inline(always)]
    pub fn pwmmr4r(&mut self) -> PWMMR4R_W {
        PWMMR4R_W { w: self }
    }
    #[doc = "Bit 14 - Stop PWM4"]
    #[inline(always)]
    pub fn pwmmr4s(&mut self) -> PWMMR4S_W {
        PWMMR4S_W { w: self }
    }
    #[doc = "Bit 15 - Interrupt PWM5"]
    #[inline(always)]
    pub fn pwmmr5i(&mut self) -> PWMMR5I_W {
        PWMMR5I_W { w: self }
    }
    #[doc = "Bit 16 - Reset PWM5"]
    #[inline(always)]
    pub fn pwmmr5r(&mut self) -> PWMMR5R_W {
        PWMMR5R_W { w: self }
    }
    #[doc = "Bit 17 - Stop PWM5"]
    #[inline(always)]
    pub fn pwmmr5s(&mut self) -> PWMMR5S_W {
        PWMMR5S_W { w: self }
    }
    #[doc = "Bit 18 - Interrupt PWM6"]
    #[inline(always)]
    pub fn pwmmr6i(&mut self) -> PWMMR6I_W {
        PWMMR6I_W { w: self }
    }
    #[doc = "Bit 19 - Reset PWM6"]
    #[inline(always)]
    pub fn pwmmr6r(&mut self) -> PWMMR6R_W {
        PWMMR6R_W { w: self }
    }
    #[doc = "Bit 20 - Stop PWM6"]
    #[inline(always)]
    pub fn pwmmr6s(&mut self) -> PWMMR6S_W {
        PWMMR6S_W { w: self }
    }
}
