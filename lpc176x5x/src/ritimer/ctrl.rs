#[doc = "Reader of register CTRL"]
pub type R = crate::R<u32, super::CTRL>;
#[doc = "Writer for register CTRL"]
pub type W = crate::W<u32, super::CTRL>;
#[doc = "Register CTRL `reset()`'s with value 0x0c"]
impl crate::ResetValue for super::CTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0c
    }
}
#[doc = "Interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RITINT_A {
    #[doc = "1: This bit is set to 1 by hardware whenever the counter value equals the masked compare value specified by the contents of RICOMPVAL and RIMASK registers. Writing a 1 to this bit will clear it to 0. Writing a 0 has no effect."]
    THIS_BIT_IS_SET_TO_1 = 1,
    #[doc = "0: The counter value does not equal the masked compare value."]
    THE_COUNTER_VALUE_DO = 0,
}
impl From<RITINT_A> for bool {
    #[inline(always)]
    fn from(variant: RITINT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `RITINT`"]
pub type RITINT_R = crate::R<bool, RITINT_A>;
impl RITINT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RITINT_A {
        match self.bits {
            true => RITINT_A::THIS_BIT_IS_SET_TO_1,
            false => RITINT_A::THE_COUNTER_VALUE_DO,
        }
    }
    #[doc = "Checks if the value of the field is `THIS_BIT_IS_SET_TO_1`"]
    #[inline(always)]
    pub fn is_this_bit_is_set_to_1(&self) -> bool {
        *self == RITINT_A::THIS_BIT_IS_SET_TO_1
    }
    #[doc = "Checks if the value of the field is `THE_COUNTER_VALUE_DO`"]
    #[inline(always)]
    pub fn is_the_counter_value_do(&self) -> bool {
        *self == RITINT_A::THE_COUNTER_VALUE_DO
    }
}
#[doc = "Write proxy for field `RITINT`"]
pub struct RITINT_W<'a> {
    w: &'a mut W,
}
impl<'a> RITINT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RITINT_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "This bit is set to 1 by hardware whenever the counter value equals the masked compare value specified by the contents of RICOMPVAL and RIMASK registers. Writing a 1 to this bit will clear it to 0. Writing a 0 has no effect."]
    #[inline(always)]
    pub fn this_bit_is_set_to_1(self) -> &'a mut W {
        self.variant(RITINT_A::THIS_BIT_IS_SET_TO_1)
    }
    #[doc = "The counter value does not equal the masked compare value."]
    #[inline(always)]
    pub fn the_counter_value_do(self) -> &'a mut W {
        self.variant(RITINT_A::THE_COUNTER_VALUE_DO)
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
#[doc = "Timer enable clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RITENCLR_A {
    #[doc = "1: The timer will be cleared to 0 whenever the counter value equals the masked compare value specified by the contents of RICOMPVAL and RIMASK registers. This will occur on the same clock that sets the interrupt flag."]
    THE_TIMER_WILL_BE_CL = 1,
    #[doc = "0: The timer will not be cleared to 0."]
    THE_TIMER_WILL_NOT_B = 0,
}
impl From<RITENCLR_A> for bool {
    #[inline(always)]
    fn from(variant: RITENCLR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `RITENCLR`"]
pub type RITENCLR_R = crate::R<bool, RITENCLR_A>;
impl RITENCLR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RITENCLR_A {
        match self.bits {
            true => RITENCLR_A::THE_TIMER_WILL_BE_CL,
            false => RITENCLR_A::THE_TIMER_WILL_NOT_B,
        }
    }
    #[doc = "Checks if the value of the field is `THE_TIMER_WILL_BE_CL`"]
    #[inline(always)]
    pub fn is_the_timer_will_be_cl(&self) -> bool {
        *self == RITENCLR_A::THE_TIMER_WILL_BE_CL
    }
    #[doc = "Checks if the value of the field is `THE_TIMER_WILL_NOT_B`"]
    #[inline(always)]
    pub fn is_the_timer_will_not_b(&self) -> bool {
        *self == RITENCLR_A::THE_TIMER_WILL_NOT_B
    }
}
#[doc = "Write proxy for field `RITENCLR`"]
pub struct RITENCLR_W<'a> {
    w: &'a mut W,
}
impl<'a> RITENCLR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RITENCLR_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The timer will be cleared to 0 whenever the counter value equals the masked compare value specified by the contents of RICOMPVAL and RIMASK registers. This will occur on the same clock that sets the interrupt flag."]
    #[inline(always)]
    pub fn the_timer_will_be_cl(self) -> &'a mut W {
        self.variant(RITENCLR_A::THE_TIMER_WILL_BE_CL)
    }
    #[doc = "The timer will not be cleared to 0."]
    #[inline(always)]
    pub fn the_timer_will_not_b(self) -> &'a mut W {
        self.variant(RITENCLR_A::THE_TIMER_WILL_NOT_B)
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
#[doc = "Timer enable for debug\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RITENBR_A {
    #[doc = "1: The timer is halted when the processor is halted for debugging."]
    THE_TIMER_IS_HALTED_ = 1,
    #[doc = "0: Debug has no effect on the timer operation."]
    DEBUG_HAS_NO_EFFECT_ = 0,
}
impl From<RITENBR_A> for bool {
    #[inline(always)]
    fn from(variant: RITENBR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `RITENBR`"]
pub type RITENBR_R = crate::R<bool, RITENBR_A>;
impl RITENBR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RITENBR_A {
        match self.bits {
            true => RITENBR_A::THE_TIMER_IS_HALTED_,
            false => RITENBR_A::DEBUG_HAS_NO_EFFECT_,
        }
    }
    #[doc = "Checks if the value of the field is `THE_TIMER_IS_HALTED_`"]
    #[inline(always)]
    pub fn is_the_timer_is_halted_(&self) -> bool {
        *self == RITENBR_A::THE_TIMER_IS_HALTED_
    }
    #[doc = "Checks if the value of the field is `DEBUG_HAS_NO_EFFECT_`"]
    #[inline(always)]
    pub fn is_debug_has_no_effect_(&self) -> bool {
        *self == RITENBR_A::DEBUG_HAS_NO_EFFECT_
    }
}
#[doc = "Write proxy for field `RITENBR`"]
pub struct RITENBR_W<'a> {
    w: &'a mut W,
}
impl<'a> RITENBR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RITENBR_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The timer is halted when the processor is halted for debugging."]
    #[inline(always)]
    pub fn the_timer_is_halted_(self) -> &'a mut W {
        self.variant(RITENBR_A::THE_TIMER_IS_HALTED_)
    }
    #[doc = "Debug has no effect on the timer operation."]
    #[inline(always)]
    pub fn debug_has_no_effect_(self) -> &'a mut W {
        self.variant(RITENBR_A::DEBUG_HAS_NO_EFFECT_)
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
#[doc = "Timer enable.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RITEN_A {
    #[doc = "1: Timer enabled. This can be overruled by a debug halt if enabled in bit 2."]
    TIMER_ENABLED_THIS_ = 1,
    #[doc = "0: Timer disabled."]
    TIMER_DISABLED_ = 0,
}
impl From<RITEN_A> for bool {
    #[inline(always)]
    fn from(variant: RITEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `RITEN`"]
pub type RITEN_R = crate::R<bool, RITEN_A>;
impl RITEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RITEN_A {
        match self.bits {
            true => RITEN_A::TIMER_ENABLED_THIS_,
            false => RITEN_A::TIMER_DISABLED_,
        }
    }
    #[doc = "Checks if the value of the field is `TIMER_ENABLED_THIS_`"]
    #[inline(always)]
    pub fn is_timer_enabled_this_(&self) -> bool {
        *self == RITEN_A::TIMER_ENABLED_THIS_
    }
    #[doc = "Checks if the value of the field is `TIMER_DISABLED_`"]
    #[inline(always)]
    pub fn is_timer_disabled_(&self) -> bool {
        *self == RITEN_A::TIMER_DISABLED_
    }
}
#[doc = "Write proxy for field `RITEN`"]
pub struct RITEN_W<'a> {
    w: &'a mut W,
}
impl<'a> RITEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RITEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Timer enabled. This can be overruled by a debug halt if enabled in bit 2."]
    #[inline(always)]
    pub fn timer_enabled_this_(self) -> &'a mut W {
        self.variant(RITEN_A::TIMER_ENABLED_THIS_)
    }
    #[doc = "Timer disabled."]
    #[inline(always)]
    pub fn timer_disabled_(self) -> &'a mut W {
        self.variant(RITEN_A::TIMER_DISABLED_)
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
    #[doc = "Bit 0 - Interrupt flag"]
    #[inline(always)]
    pub fn ritint(&self) -> RITINT_R {
        RITINT_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Timer enable clear"]
    #[inline(always)]
    pub fn ritenclr(&self) -> RITENCLR_R {
        RITENCLR_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Timer enable for debug"]
    #[inline(always)]
    pub fn ritenbr(&self) -> RITENBR_R {
        RITENBR_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Timer enable."]
    #[inline(always)]
    pub fn riten(&self) -> RITEN_R {
        RITEN_R::new(((self.bits >> 3) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Interrupt flag"]
    #[inline(always)]
    pub fn ritint(&mut self) -> RITINT_W {
        RITINT_W { w: self }
    }
    #[doc = "Bit 1 - Timer enable clear"]
    #[inline(always)]
    pub fn ritenclr(&mut self) -> RITENCLR_W {
        RITENCLR_W { w: self }
    }
    #[doc = "Bit 2 - Timer enable for debug"]
    #[inline(always)]
    pub fn ritenbr(&mut self) -> RITENBR_W {
        RITENBR_W { w: self }
    }
    #[doc = "Bit 3 - Timer enable."]
    #[inline(always)]
    pub fn riten(&mut self) -> RITEN_W {
        RITEN_W { w: self }
    }
}
