#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CTRL {
    #[doc = r" Modifies the contents of the register"]
    #[inline]
    pub fn modify<F>(&self, f: F)
    where
        for<'w> F: FnOnce(&R, &'w mut W) -> &'w mut W,
    {
        let bits = self.register.get();
        let r = R { bits: bits };
        let mut w = W { bits: bits };
        f(&r, &mut w);
        self.register.set(w.bits);
    }
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
    #[doc = r" Writes to the register"]
    #[inline]
    pub fn write<F>(&self, f: F)
    where
        F: FnOnce(&mut W) -> &mut W,
    {
        let mut w = W::reset_value();
        f(&mut w);
        self.register.set(w.bits);
    }
    #[doc = r" Writes the reset value to the register"]
    #[inline]
    pub fn reset(&self) {
        self.write(|w| w)
    }
}
#[doc = "Possible values of the field `RITINT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RITINTR {
    #[doc = "This bit is set to 1 by hardware whenever the counter value equals the masked compare value specified by the contents of RICOMPVAL and RIMASK registers. Writing a 1 to this bit will clear it to 0. Writing a 0 has no effect."]
    THIS_BIT_IS_SET_TO_1,
    #[doc = "The counter value does not equal the masked compare value."]
    THE_COUNTER_VALUE_DO,
}
impl RITINTR {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            RITINTR::THIS_BIT_IS_SET_TO_1 => true,
            RITINTR::THE_COUNTER_VALUE_DO => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RITINTR {
        match value {
            true => RITINTR::THIS_BIT_IS_SET_TO_1,
            false => RITINTR::THE_COUNTER_VALUE_DO,
        }
    }
    #[doc = "Checks if the value of the field is `THIS_BIT_IS_SET_TO_1`"]
    #[inline]
    pub fn is_this_bit_is_set_to_1(&self) -> bool {
        *self == RITINTR::THIS_BIT_IS_SET_TO_1
    }
    #[doc = "Checks if the value of the field is `THE_COUNTER_VALUE_DO`"]
    #[inline]
    pub fn is_the_counter_value_do(&self) -> bool {
        *self == RITINTR::THE_COUNTER_VALUE_DO
    }
}
#[doc = "Possible values of the field `RITENCLR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RITENCLRR {
    #[doc = "The timer will be cleared to 0 whenever the counter value equals the masked compare value specified by the contents of RICOMPVAL and RIMASK registers. This will occur on the same clock that sets the interrupt flag."]
    THE_TIMER_WILL_BE_CL,
    #[doc = "The timer will not be cleared to 0."]
    THE_TIMER_WILL_NOT_B,
}
impl RITENCLRR {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            RITENCLRR::THE_TIMER_WILL_BE_CL => true,
            RITENCLRR::THE_TIMER_WILL_NOT_B => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RITENCLRR {
        match value {
            true => RITENCLRR::THE_TIMER_WILL_BE_CL,
            false => RITENCLRR::THE_TIMER_WILL_NOT_B,
        }
    }
    #[doc = "Checks if the value of the field is `THE_TIMER_WILL_BE_CL`"]
    #[inline]
    pub fn is_the_timer_will_be_cl(&self) -> bool {
        *self == RITENCLRR::THE_TIMER_WILL_BE_CL
    }
    #[doc = "Checks if the value of the field is `THE_TIMER_WILL_NOT_B`"]
    #[inline]
    pub fn is_the_timer_will_not_b(&self) -> bool {
        *self == RITENCLRR::THE_TIMER_WILL_NOT_B
    }
}
#[doc = "Possible values of the field `RITENBR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RITENBRR {
    #[doc = "The timer is halted when the processor is halted for debugging."]
    THE_TIMER_IS_HALTED_,
    #[doc = "Debug has no effect on the timer operation."]
    DEBUG_HAS_NO_EFFECT_,
}
impl RITENBRR {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            RITENBRR::THE_TIMER_IS_HALTED_ => true,
            RITENBRR::DEBUG_HAS_NO_EFFECT_ => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RITENBRR {
        match value {
            true => RITENBRR::THE_TIMER_IS_HALTED_,
            false => RITENBRR::DEBUG_HAS_NO_EFFECT_,
        }
    }
    #[doc = "Checks if the value of the field is `THE_TIMER_IS_HALTED_`"]
    #[inline]
    pub fn is_the_timer_is_halted_(&self) -> bool {
        *self == RITENBRR::THE_TIMER_IS_HALTED_
    }
    #[doc = "Checks if the value of the field is `DEBUG_HAS_NO_EFFECT_`"]
    #[inline]
    pub fn is_debug_has_no_effect_(&self) -> bool {
        *self == RITENBRR::DEBUG_HAS_NO_EFFECT_
    }
}
#[doc = "Possible values of the field `RITEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RITENR {
    #[doc = "Timer enabled. This can be overruled by a debug halt if enabled in bit 2."]
    TIMER_ENABLED_THIS_,
    #[doc = "Timer disabled."]
    TIMER_DISABLED_,
}
impl RITENR {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            RITENR::TIMER_ENABLED_THIS_ => true,
            RITENR::TIMER_DISABLED_ => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RITENR {
        match value {
            true => RITENR::TIMER_ENABLED_THIS_,
            false => RITENR::TIMER_DISABLED_,
        }
    }
    #[doc = "Checks if the value of the field is `TIMER_ENABLED_THIS_`"]
    #[inline]
    pub fn is_timer_enabled_this_(&self) -> bool {
        *self == RITENR::TIMER_ENABLED_THIS_
    }
    #[doc = "Checks if the value of the field is `TIMER_DISABLED_`"]
    #[inline]
    pub fn is_timer_disabled_(&self) -> bool {
        *self == RITENR::TIMER_DISABLED_
    }
}
#[doc = "Values that can be written to the field `RITINT`"]
pub enum RITINTW {
    #[doc = "This bit is set to 1 by hardware whenever the counter value equals the masked compare value specified by the contents of RICOMPVAL and RIMASK registers. Writing a 1 to this bit will clear it to 0. Writing a 0 has no effect."]
    THIS_BIT_IS_SET_TO_1,
    #[doc = "The counter value does not equal the masked compare value."]
    THE_COUNTER_VALUE_DO,
}
impl RITINTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RITINTW::THIS_BIT_IS_SET_TO_1 => true,
            RITINTW::THE_COUNTER_VALUE_DO => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RITINTW<'a> {
    w: &'a mut W,
}
impl<'a> _RITINTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RITINTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "This bit is set to 1 by hardware whenever the counter value equals the masked compare value specified by the contents of RICOMPVAL and RIMASK registers. Writing a 1 to this bit will clear it to 0. Writing a 0 has no effect."]
    #[inline]
    pub fn this_bit_is_set_to_1(self) -> &'a mut W {
        self.variant(RITINTW::THIS_BIT_IS_SET_TO_1)
    }
    #[doc = "The counter value does not equal the masked compare value."]
    #[inline]
    pub fn the_counter_value_do(self) -> &'a mut W {
        self.variant(RITINTW::THE_COUNTER_VALUE_DO)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `RITENCLR`"]
pub enum RITENCLRW {
    #[doc = "The timer will be cleared to 0 whenever the counter value equals the masked compare value specified by the contents of RICOMPVAL and RIMASK registers. This will occur on the same clock that sets the interrupt flag."]
    THE_TIMER_WILL_BE_CL,
    #[doc = "The timer will not be cleared to 0."]
    THE_TIMER_WILL_NOT_B,
}
impl RITENCLRW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RITENCLRW::THE_TIMER_WILL_BE_CL => true,
            RITENCLRW::THE_TIMER_WILL_NOT_B => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RITENCLRW<'a> {
    w: &'a mut W,
}
impl<'a> _RITENCLRW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RITENCLRW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The timer will be cleared to 0 whenever the counter value equals the masked compare value specified by the contents of RICOMPVAL and RIMASK registers. This will occur on the same clock that sets the interrupt flag."]
    #[inline]
    pub fn the_timer_will_be_cl(self) -> &'a mut W {
        self.variant(RITENCLRW::THE_TIMER_WILL_BE_CL)
    }
    #[doc = "The timer will not be cleared to 0."]
    #[inline]
    pub fn the_timer_will_not_b(self) -> &'a mut W {
        self.variant(RITENCLRW::THE_TIMER_WILL_NOT_B)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 1;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `RITENBR`"]
pub enum RITENBRW {
    #[doc = "The timer is halted when the processor is halted for debugging."]
    THE_TIMER_IS_HALTED_,
    #[doc = "Debug has no effect on the timer operation."]
    DEBUG_HAS_NO_EFFECT_,
}
impl RITENBRW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RITENBRW::THE_TIMER_IS_HALTED_ => true,
            RITENBRW::DEBUG_HAS_NO_EFFECT_ => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RITENBRW<'a> {
    w: &'a mut W,
}
impl<'a> _RITENBRW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RITENBRW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The timer is halted when the processor is halted for debugging."]
    #[inline]
    pub fn the_timer_is_halted_(self) -> &'a mut W {
        self.variant(RITENBRW::THE_TIMER_IS_HALTED_)
    }
    #[doc = "Debug has no effect on the timer operation."]
    #[inline]
    pub fn debug_has_no_effect_(self) -> &'a mut W {
        self.variant(RITENBRW::DEBUG_HAS_NO_EFFECT_)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 2;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `RITEN`"]
pub enum RITENW {
    #[doc = "Timer enabled. This can be overruled by a debug halt if enabled in bit 2."]
    TIMER_ENABLED_THIS_,
    #[doc = "Timer disabled."]
    TIMER_DISABLED_,
}
impl RITENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RITENW::TIMER_ENABLED_THIS_ => true,
            RITENW::TIMER_DISABLED_ => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RITENW<'a> {
    w: &'a mut W,
}
impl<'a> _RITENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RITENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Timer enabled. This can be overruled by a debug halt if enabled in bit 2."]
    #[inline]
    pub fn timer_enabled_this_(self) -> &'a mut W {
        self.variant(RITENW::TIMER_ENABLED_THIS_)
    }
    #[doc = "Timer disabled."]
    #[inline]
    pub fn timer_disabled_(self) -> &'a mut W {
        self.variant(RITENW::TIMER_DISABLED_)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 3;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Interrupt flag"]
    #[inline]
    pub fn ritint(&self) -> RITINTR {
        RITINTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Timer enable clear"]
    #[inline]
    pub fn ritenclr(&self) -> RITENCLRR {
        RITENCLRR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - Timer enable for debug"]
    #[inline]
    pub fn ritenbr(&self) -> RITENBRR {
        RITENBRR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - Timer enable."]
    #[inline]
    pub fn riten(&self) -> RITENR {
        RITENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 12 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - Interrupt flag"]
    #[inline]
    pub fn ritint(&mut self) -> _RITINTW {
        _RITINTW { w: self }
    }
    #[doc = "Bit 1 - Timer enable clear"]
    #[inline]
    pub fn ritenclr(&mut self) -> _RITENCLRW {
        _RITENCLRW { w: self }
    }
    #[doc = "Bit 2 - Timer enable for debug"]
    #[inline]
    pub fn ritenbr(&mut self) -> _RITENBRW {
        _RITENBRW { w: self }
    }
    #[doc = "Bit 3 - Timer enable."]
    #[inline]
    pub fn riten(&mut self) -> _RITENW {
        _RITENW { w: self }
    }
}
