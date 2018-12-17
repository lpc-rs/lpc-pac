#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::TCR {
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
#[doc = "Possible values of the field `CEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CENR {
    #[doc = "The counters are disabled."]
    THE_COUNTERS_ARE_DIS,
    #[doc = "The Timer Counter and Prescale Counter are enabled for counting."]
    THE_TIMER_COUNTER_AN,
}
impl CENR {
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
            CENR::THE_COUNTERS_ARE_DIS => false,
            CENR::THE_TIMER_COUNTER_AN => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CENR {
        match value {
            false => CENR::THE_COUNTERS_ARE_DIS,
            true => CENR::THE_TIMER_COUNTER_AN,
        }
    }
    #[doc = "Checks if the value of the field is `THE_COUNTERS_ARE_DIS`"]
    #[inline]
    pub fn is_the_counters_are_dis(&self) -> bool {
        *self == CENR::THE_COUNTERS_ARE_DIS
    }
    #[doc = "Checks if the value of the field is `THE_TIMER_COUNTER_AN`"]
    #[inline]
    pub fn is_the_timer_counter_an(&self) -> bool {
        *self == CENR::THE_TIMER_COUNTER_AN
    }
}
#[doc = "Possible values of the field `CRST`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CRSTR {
    #[doc = "Do nothing."]
    DO_NOTHING_,
    #[doc = "The Timer Counter and the Prescale Counter are synchronously reset on the next positive edge of PCLK. The counters remain reset until TCR\\[1\\] is returned to zero."]
    THE_TIMER_COUNTER_AN,
}
impl CRSTR {
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
            CRSTR::DO_NOTHING_ => false,
            CRSTR::THE_TIMER_COUNTER_AN => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CRSTR {
        match value {
            false => CRSTR::DO_NOTHING_,
            true => CRSTR::THE_TIMER_COUNTER_AN,
        }
    }
    #[doc = "Checks if the value of the field is `DO_NOTHING_`"]
    #[inline]
    pub fn is_do_nothing_(&self) -> bool {
        *self == CRSTR::DO_NOTHING_
    }
    #[doc = "Checks if the value of the field is `THE_TIMER_COUNTER_AN`"]
    #[inline]
    pub fn is_the_timer_counter_an(&self) -> bool {
        *self == CRSTR::THE_TIMER_COUNTER_AN
    }
}
#[doc = "Values that can be written to the field `CEN`"]
pub enum CENW {
    #[doc = "The counters are disabled."]
    THE_COUNTERS_ARE_DIS,
    #[doc = "The Timer Counter and Prescale Counter are enabled for counting."]
    THE_TIMER_COUNTER_AN,
}
impl CENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CENW::THE_COUNTERS_ARE_DIS => false,
            CENW::THE_TIMER_COUNTER_AN => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CENW<'a> {
    w: &'a mut W,
}
impl<'a> _CENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The counters are disabled."]
    #[inline]
    pub fn the_counters_are_dis(self) -> &'a mut W {
        self.variant(CENW::THE_COUNTERS_ARE_DIS)
    }
    #[doc = "The Timer Counter and Prescale Counter are enabled for counting."]
    #[inline]
    pub fn the_timer_counter_an(self) -> &'a mut W {
        self.variant(CENW::THE_TIMER_COUNTER_AN)
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
#[doc = "Values that can be written to the field `CRST`"]
pub enum CRSTW {
    #[doc = "Do nothing."]
    DO_NOTHING_,
    #[doc = "The Timer Counter and the Prescale Counter are synchronously reset on the next positive edge of PCLK. The counters remain reset until TCR\\[1\\] is returned to zero."]
    THE_TIMER_COUNTER_AN,
}
impl CRSTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CRSTW::DO_NOTHING_ => false,
            CRSTW::THE_TIMER_COUNTER_AN => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CRSTW<'a> {
    w: &'a mut W,
}
impl<'a> _CRSTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CRSTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Do nothing."]
    #[inline]
    pub fn do_nothing_(self) -> &'a mut W {
        self.variant(CRSTW::DO_NOTHING_)
    }
    #[doc = "The Timer Counter and the Prescale Counter are synchronously reset on the next positive edge of PCLK. The counters remain reset until TCR\\[1\\] is returned to zero."]
    #[inline]
    pub fn the_timer_counter_an(self) -> &'a mut W {
        self.variant(CRSTW::THE_TIMER_COUNTER_AN)
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Counter enable."]
    #[inline]
    pub fn cen(&self) -> CENR {
        CENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Counter reset."]
    #[inline]
    pub fn crst(&self) -> CRSTR {
        CRSTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
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
    #[doc = "Bit 0 - Counter enable."]
    #[inline]
    pub fn cen(&mut self) -> _CENW {
        _CENW { w: self }
    }
    #[doc = "Bit 1 - Counter reset."]
    #[inline]
    pub fn crst(&mut self) -> _CRSTW {
        _CRSTW { w: self }
    }
}
