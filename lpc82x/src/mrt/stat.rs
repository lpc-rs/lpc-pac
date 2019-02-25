#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::STAT {
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
#[doc = "Possible values of the field `INTFLAG`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INTFLAGR {
    #[doc = "No pending interrupt. Writing a zero is equivalent to no operation."]
    NO_PENDING_INTERRUPT,
    #[doc = "Pending interrupt. The interrupt is pending because TIMERn has reached the end of the time interval. If the INTEN bit in the CONTROLn is also set to 1, the interrupt for timer channel n and the global interrupt are raised.  Writing a 1 to this bit clears the interrupt request."]
    PENDING_INTERRUPT_T,
}
impl INTFLAGR {
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
            INTFLAGR::NO_PENDING_INTERRUPT => false,
            INTFLAGR::PENDING_INTERRUPT_T => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> INTFLAGR {
        match value {
            false => INTFLAGR::NO_PENDING_INTERRUPT,
            true => INTFLAGR::PENDING_INTERRUPT_T,
        }
    }
    #[doc = "Checks if the value of the field is `NO_PENDING_INTERRUPT`"]
    #[inline]
    pub fn is_no_pending_interrupt(&self) -> bool {
        *self == INTFLAGR::NO_PENDING_INTERRUPT
    }
    #[doc = "Checks if the value of the field is `PENDING_INTERRUPT_T`"]
    #[inline]
    pub fn is_pending_interrupt_t(&self) -> bool {
        *self == INTFLAGR::PENDING_INTERRUPT_T
    }
}
#[doc = "Possible values of the field `RUN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RUNR {
    #[doc = "Idle state. TIMERn is stopped."]
    IDLE_STATE_TIMERN_I,
    #[doc = "Running. TIMERn is running."]
    RUNNING_TIMERN_IS_R,
}
impl RUNR {
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
            RUNR::IDLE_STATE_TIMERN_I => false,
            RUNR::RUNNING_TIMERN_IS_R => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RUNR {
        match value {
            false => RUNR::IDLE_STATE_TIMERN_I,
            true => RUNR::RUNNING_TIMERN_IS_R,
        }
    }
    #[doc = "Checks if the value of the field is `IDLE_STATE_TIMERN_I`"]
    #[inline]
    pub fn is_idle_state_timern_i(&self) -> bool {
        *self == RUNR::IDLE_STATE_TIMERN_I
    }
    #[doc = "Checks if the value of the field is `RUNNING_TIMERN_IS_R`"]
    #[inline]
    pub fn is_running_timern_is_r(&self) -> bool {
        *self == RUNR::RUNNING_TIMERN_IS_R
    }
}
#[doc = "Values that can be written to the field `INTFLAG`"]
pub enum INTFLAGW {
    #[doc = "No pending interrupt. Writing a zero is equivalent to no operation."]
    NO_PENDING_INTERRUPT,
    #[doc = "Pending interrupt. The interrupt is pending because TIMERn has reached the end of the time interval. If the INTEN bit in the CONTROLn is also set to 1, the interrupt for timer channel n and the global interrupt are raised.  Writing a 1 to this bit clears the interrupt request."]
    PENDING_INTERRUPT_T,
}
impl INTFLAGW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            INTFLAGW::NO_PENDING_INTERRUPT => false,
            INTFLAGW::PENDING_INTERRUPT_T => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _INTFLAGW<'a> {
    w: &'a mut W,
}
impl<'a> _INTFLAGW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: INTFLAGW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No pending interrupt. Writing a zero is equivalent to no operation."]
    #[inline]
    pub fn no_pending_interrupt(self) -> &'a mut W {
        self.variant(INTFLAGW::NO_PENDING_INTERRUPT)
    }
    #[doc = "Pending interrupt. The interrupt is pending because TIMERn has reached the end of the time interval. If the INTEN bit in the CONTROLn is also set to 1, the interrupt for timer channel n and the global interrupt are raised. Writing a 1 to this bit clears the interrupt request."]
    #[inline]
    pub fn pending_interrupt_t(self) -> &'a mut W {
        self.variant(INTFLAGW::PENDING_INTERRUPT_T)
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
#[doc = "Values that can be written to the field `RUN`"]
pub enum RUNW {
    #[doc = "Idle state. TIMERn is stopped."]
    IDLE_STATE_TIMERN_I,
    #[doc = "Running. TIMERn is running."]
    RUNNING_TIMERN_IS_R,
}
impl RUNW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RUNW::IDLE_STATE_TIMERN_I => false,
            RUNW::RUNNING_TIMERN_IS_R => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RUNW<'a> {
    w: &'a mut W,
}
impl<'a> _RUNW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RUNW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Idle state. TIMERn is stopped."]
    #[inline]
    pub fn idle_state_timern_i(self) -> &'a mut W {
        self.variant(RUNW::IDLE_STATE_TIMERN_I)
    }
    #[doc = "Running. TIMERn is running."]
    #[inline]
    pub fn running_timern_is_r(self) -> &'a mut W {
        self.variant(RUNW::RUNNING_TIMERN_IS_R)
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
    #[doc = "Bit 0 - Monitors the interrupt flag."]
    #[inline]
    pub fn intflag(&self) -> INTFLAGR {
        INTFLAGR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Indicates the state of TIMERn. This bit is read-only."]
    #[inline]
    pub fn run(&self) -> RUNR {
        RUNR::_from({
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
    #[doc = "Bit 0 - Monitors the interrupt flag."]
    #[inline]
    pub fn intflag(&mut self) -> _INTFLAGW {
        _INTFLAGW { w: self }
    }
    #[doc = "Bit 1 - Indicates the state of TIMERn. This bit is read-only."]
    #[inline]
    pub fn run(&mut self) -> _RUNW {
        _RUNW { w: self }
    }
}
