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
#[doc = "Possible values of the field `CLKSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CLKSELR {
    #[doc = "Divided IRC clock. This clock runs at 750 kHz and provides time-out periods of up to approximately 95 minutes in 1.33 us increments. This clock is not available in most low-power modes and must not be selected if the timer is to be used to wake up from one of these modes."]
    DIVIDED_IRC_CLOCK_T,
    #[doc = "Low power clock. This is the (nominally) 10 kHz clock and provides time-out periods of up to approximately 119 hours in 100 CTS can be from the input pin, or fs increments. The accuracy of this clock is limited to +/- 45 % over temperature and processing. This clock is available in all power modes. Prior to use, the low-power oscillator must be enabled. The oscillator must also be set to remain active in Deep power-down if needed."]
    LOW_POWER_CLOCK_THI,
}
impl CLKSELR {
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
            CLKSELR::DIVIDED_IRC_CLOCK_T => false,
            CLKSELR::LOW_POWER_CLOCK_THI => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CLKSELR {
        match value {
            false => CLKSELR::DIVIDED_IRC_CLOCK_T,
            true => CLKSELR::LOW_POWER_CLOCK_THI,
        }
    }
    #[doc = "Checks if the value of the field is `DIVIDED_IRC_CLOCK_T`"]
    #[inline]
    pub fn is_divided_irc_clock_t(&self) -> bool {
        *self == CLKSELR::DIVIDED_IRC_CLOCK_T
    }
    #[doc = "Checks if the value of the field is `LOW_POWER_CLOCK_THI`"]
    #[inline]
    pub fn is_low_power_clock_thi(&self) -> bool {
        *self == CLKSELR::LOW_POWER_CLOCK_THI
    }
}
#[doc = "Possible values of the field `ALARMFLAG`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ALARMFLAGR {
    #[doc = "No time-out. The self wake-up timer has not timed out. Writing a 0 to has no effect."]
    NO_TIME_OUT_THE_SEL,
    #[doc = "Time-out. The self wake-up timer has timed out. This flag generates an interrupt request which can wake up the part from any low power mode not deep power-down. Writing a 1 clears this status bit and the interrupt too?"]
    TIME_OUT_THE_SELF_W,
}
impl ALARMFLAGR {
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
            ALARMFLAGR::NO_TIME_OUT_THE_SEL => false,
            ALARMFLAGR::TIME_OUT_THE_SELF_W => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ALARMFLAGR {
        match value {
            false => ALARMFLAGR::NO_TIME_OUT_THE_SEL,
            true => ALARMFLAGR::TIME_OUT_THE_SELF_W,
        }
    }
    #[doc = "Checks if the value of the field is `NO_TIME_OUT_THE_SEL`"]
    #[inline]
    pub fn is_no_time_out_the_sel(&self) -> bool {
        *self == ALARMFLAGR::NO_TIME_OUT_THE_SEL
    }
    #[doc = "Checks if the value of the field is `TIME_OUT_THE_SELF_W`"]
    #[inline]
    pub fn is_time_out_the_self_w(&self) -> bool {
        *self == ALARMFLAGR::TIME_OUT_THE_SELF_W
    }
}
#[doc = "Possible values of the field `CLEARCTR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CLEARCTRR {
    #[doc = "No effect. Reading this bit always returns 0."]
    NO_EFFECT_READING_T,
    #[doc = "Clear the counter. Counting is halted until a new count value is loaded."]
    CLEAR_THE_COUNTER_C,
}
impl CLEARCTRR {
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
            CLEARCTRR::NO_EFFECT_READING_T => false,
            CLEARCTRR::CLEAR_THE_COUNTER_C => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CLEARCTRR {
        match value {
            false => CLEARCTRR::NO_EFFECT_READING_T,
            true => CLEARCTRR::CLEAR_THE_COUNTER_C,
        }
    }
    #[doc = "Checks if the value of the field is `NO_EFFECT_READING_T`"]
    #[inline]
    pub fn is_no_effect_reading_t(&self) -> bool {
        *self == CLEARCTRR::NO_EFFECT_READING_T
    }
    #[doc = "Checks if the value of the field is `CLEAR_THE_COUNTER_C`"]
    #[inline]
    pub fn is_clear_the_counter_c(&self) -> bool {
        *self == CLEARCTRR::CLEAR_THE_COUNTER_C
    }
}
#[doc = "Possible values of the field `SEL_EXTCLK`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SEL_EXTCLKR {
    #[doc = "Internal. The clock source is the internal clock selected by the CLKSEL bit."]
    INTERNAL,
    #[doc = "External. The self-wake-up timer uses the external WKTCLKIN pin."]
    EXTERNAL,
}
impl SEL_EXTCLKR {
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
            SEL_EXTCLKR::INTERNAL => false,
            SEL_EXTCLKR::EXTERNAL => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SEL_EXTCLKR {
        match value {
            false => SEL_EXTCLKR::INTERNAL,
            true => SEL_EXTCLKR::EXTERNAL,
        }
    }
    #[doc = "Checks if the value of the field is `INTERNAL`"]
    #[inline]
    pub fn is_internal(&self) -> bool {
        *self == SEL_EXTCLKR::INTERNAL
    }
    #[doc = "Checks if the value of the field is `EXTERNAL`"]
    #[inline]
    pub fn is_external(&self) -> bool {
        *self == SEL_EXTCLKR::EXTERNAL
    }
}
#[doc = "Values that can be written to the field `CLKSEL`"]
pub enum CLKSELW {
    #[doc = "Divided IRC clock. This clock runs at 750 kHz and provides time-out periods of up to approximately 95 minutes in 1.33 us increments. This clock is not available in most low-power modes and must not be selected if the timer is to be used to wake up from one of these modes."]
    DIVIDED_IRC_CLOCK_T,
    #[doc = "Low power clock. This is the (nominally) 10 kHz clock and provides time-out periods of up to approximately 119 hours in 100 CTS can be from the input pin, or fs increments. The accuracy of this clock is limited to +/- 45 % over temperature and processing. This clock is available in all power modes. Prior to use, the low-power oscillator must be enabled. The oscillator must also be set to remain active in Deep power-down if needed."]
    LOW_POWER_CLOCK_THI,
}
impl CLKSELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CLKSELW::DIVIDED_IRC_CLOCK_T => false,
            CLKSELW::LOW_POWER_CLOCK_THI => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CLKSELW<'a> {
    w: &'a mut W,
}
impl<'a> _CLKSELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CLKSELW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Divided IRC clock. This clock runs at 750 kHz and provides time-out periods of up to approximately 95 minutes in 1.33 us increments. This clock is not available in most low-power modes and must not be selected if the timer is to be used to wake up from one of these modes."]
    #[inline]
    pub fn divided_irc_clock_t(self) -> &'a mut W {
        self.variant(CLKSELW::DIVIDED_IRC_CLOCK_T)
    }
    #[doc = "Low power clock. This is the (nominally) 10 kHz clock and provides time-out periods of up to approximately 119 hours in 100 CTS can be from the input pin, or fs increments. The accuracy of this clock is limited to +/- 45 % over temperature and processing. This clock is available in all power modes. Prior to use, the low-power oscillator must be enabled. The oscillator must also be set to remain active in Deep power-down if needed."]
    #[inline]
    pub fn low_power_clock_thi(self) -> &'a mut W {
        self.variant(CLKSELW::LOW_POWER_CLOCK_THI)
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
#[doc = "Values that can be written to the field `ALARMFLAG`"]
pub enum ALARMFLAGW {
    #[doc = "No time-out. The self wake-up timer has not timed out. Writing a 0 to has no effect."]
    NO_TIME_OUT_THE_SEL,
    #[doc = "Time-out. The self wake-up timer has timed out. This flag generates an interrupt request which can wake up the part from any low power mode not deep power-down. Writing a 1 clears this status bit and the interrupt too?"]
    TIME_OUT_THE_SELF_W,
}
impl ALARMFLAGW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ALARMFLAGW::NO_TIME_OUT_THE_SEL => false,
            ALARMFLAGW::TIME_OUT_THE_SELF_W => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ALARMFLAGW<'a> {
    w: &'a mut W,
}
impl<'a> _ALARMFLAGW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ALARMFLAGW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No time-out. The self wake-up timer has not timed out. Writing a 0 to has no effect."]
    #[inline]
    pub fn no_time_out_the_sel(self) -> &'a mut W {
        self.variant(ALARMFLAGW::NO_TIME_OUT_THE_SEL)
    }
    #[doc = "Time-out. The self wake-up timer has timed out. This flag generates an interrupt request which can wake up the part from any low power mode not deep power-down. Writing a 1 clears this status bit and the interrupt too?"]
    #[inline]
    pub fn time_out_the_self_w(self) -> &'a mut W {
        self.variant(ALARMFLAGW::TIME_OUT_THE_SELF_W)
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
#[doc = "Values that can be written to the field `CLEARCTR`"]
pub enum CLEARCTRW {
    #[doc = "No effect. Reading this bit always returns 0."]
    NO_EFFECT_READING_T,
    #[doc = "Clear the counter. Counting is halted until a new count value is loaded."]
    CLEAR_THE_COUNTER_C,
}
impl CLEARCTRW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CLEARCTRW::NO_EFFECT_READING_T => false,
            CLEARCTRW::CLEAR_THE_COUNTER_C => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CLEARCTRW<'a> {
    w: &'a mut W,
}
impl<'a> _CLEARCTRW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CLEARCTRW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No effect. Reading this bit always returns 0."]
    #[inline]
    pub fn no_effect_reading_t(self) -> &'a mut W {
        self.variant(CLEARCTRW::NO_EFFECT_READING_T)
    }
    #[doc = "Clear the counter. Counting is halted until a new count value is loaded."]
    #[inline]
    pub fn clear_the_counter_c(self) -> &'a mut W {
        self.variant(CLEARCTRW::CLEAR_THE_COUNTER_C)
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
#[doc = "Values that can be written to the field `SEL_EXTCLK`"]
pub enum SEL_EXTCLKW {
    #[doc = "Internal. The clock source is the internal clock selected by the CLKSEL bit."]
    INTERNAL,
    #[doc = "External. The self-wake-up timer uses the external WKTCLKIN pin."]
    EXTERNAL,
}
impl SEL_EXTCLKW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SEL_EXTCLKW::INTERNAL => false,
            SEL_EXTCLKW::EXTERNAL => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SEL_EXTCLKW<'a> {
    w: &'a mut W,
}
impl<'a> _SEL_EXTCLKW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SEL_EXTCLKW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Internal. The clock source is the internal clock selected by the CLKSEL bit."]
    #[inline]
    pub fn internal(self) -> &'a mut W {
        self.variant(SEL_EXTCLKW::INTERNAL)
    }
    #[doc = "External. The self-wake-up timer uses the external WKTCLKIN pin."]
    #[inline]
    pub fn external(self) -> &'a mut W {
        self.variant(SEL_EXTCLKW::EXTERNAL)
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
    #[doc = "Bit 0 - Select the self wake-up timer clock source."]
    #[inline]
    pub fn clksel(&self) -> CLKSELR {
        CLKSELR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Wake-up or alarm timer flag."]
    #[inline]
    pub fn alarmflag(&self) -> ALARMFLAGR {
        ALARMFLAGR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - Clears the self wake-up timer."]
    #[inline]
    pub fn clearctr(&self) -> CLEARCTRR {
        CLEARCTRR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - Select external or internal clock source for the self-wake-up timer. The internal clock source is selected by the CLKSEL bit in this register if SET_EXTCLK is set to internal."]
    #[inline]
    pub fn sel_extclk(&self) -> SEL_EXTCLKR {
        SEL_EXTCLKR::_from({
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
        W { bits: 0 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - Select the self wake-up timer clock source."]
    #[inline]
    pub fn clksel(&mut self) -> _CLKSELW {
        _CLKSELW { w: self }
    }
    #[doc = "Bit 1 - Wake-up or alarm timer flag."]
    #[inline]
    pub fn alarmflag(&mut self) -> _ALARMFLAGW {
        _ALARMFLAGW { w: self }
    }
    #[doc = "Bit 2 - Clears the self wake-up timer."]
    #[inline]
    pub fn clearctr(&mut self) -> _CLEARCTRW {
        _CLEARCTRW { w: self }
    }
    #[doc = "Bit 3 - Select external or internal clock source for the self-wake-up timer. The internal clock source is selected by the CLKSEL bit in this register if SET_EXTCLK is set to internal."]
    #[inline]
    pub fn sel_extclk(&mut self) -> _SEL_EXTCLKW {
        _SEL_EXTCLKW { w: self }
    }
}
