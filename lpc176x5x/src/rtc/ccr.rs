#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CCR {
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
#[doc = "Possible values of the field `CLKEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CLKENR {
    #[doc = "The time counters are enabled."]
    THE_TIME_COUNTERS_AR,
    #[doc = "The time counters are disabled so that they may be initialized."]
    THE_TIME_COUNTERS_AR,
}
impl CLKENR {
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
            CLKENR::THE_TIME_COUNTERS_AR => true,
            CLKENR::THE_TIME_COUNTERS_AR => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CLKENR {
        match value {
            true => CLKENR::THE_TIME_COUNTERS_AR,
            false => CLKENR::THE_TIME_COUNTERS_AR,
        }
    }
    #[doc = "Checks if the value of the field is `THE_TIME_COUNTERS_AR`"]
    #[inline]
    pub fn is_the_time_counters_ar(&self) -> bool {
        *self == CLKENR::THE_TIME_COUNTERS_AR
    }
    #[doc = "Checks if the value of the field is `THE_TIME_COUNTERS_AR`"]
    #[inline]
    pub fn is_the_time_counters_ar(&self) -> bool {
        *self == CLKENR::THE_TIME_COUNTERS_AR
    }
}
#[doc = "Possible values of the field `CTCRST`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CTCRSTR {
    #[doc = "When one, the elements in the internal oscillator divider are reset, and remain reset until CCR\\[1\\] is changed to zero. This is the divider that generates the 1 Hz clock from the 32.768 kHz crystal. The state of the divider is not visible to software."]
    RESET,
    #[doc = "No effect."]
    NO_EFFECT_,
}
impl CTCRSTR {
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
            CTCRSTR::RESET => true,
            CTCRSTR::NO_EFFECT_ => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CTCRSTR {
        match value {
            true => CTCRSTR::RESET,
            false => CTCRSTR::NO_EFFECT_,
        }
    }
    #[doc = "Checks if the value of the field is `RESET`"]
    #[inline]
    pub fn is_reset(&self) -> bool {
        *self == CTCRSTR::RESET
    }
    #[doc = "Checks if the value of the field is `NO_EFFECT_`"]
    #[inline]
    pub fn is_no_effect_(&self) -> bool {
        *self == CTCRSTR::NO_EFFECT_
    }
}
#[doc = "Possible values of the field `CCALEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CCALENR {
    #[doc = "The calibration counter is disabled and reset to zero."]
    THE_CALIBRATION_COUN,
    #[doc = "The calibration counter is enabled and counting, using the 1 Hz clock. When the calibration counter is equal to the value of the CALIBRATION register, the counter resets and repeats counting up to the value of the CALIBRATION register. See Section 30.6.4.2 and  Section 30.6.5."]
    THE_CALIBRATION_COUN,
}
impl CCALENR {
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
            CCALENR::THE_CALIBRATION_COUN => true,
            CCALENR::THE_CALIBRATION_COUN => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CCALENR {
        match value {
            true => CCALENR::THE_CALIBRATION_COUN,
            false => CCALENR::THE_CALIBRATION_COUN,
        }
    }
    #[doc = "Checks if the value of the field is `THE_CALIBRATION_COUN`"]
    #[inline]
    pub fn is_the_calibration_coun(&self) -> bool {
        *self == CCALENR::THE_CALIBRATION_COUN
    }
    #[doc = "Checks if the value of the field is `THE_CALIBRATION_COUN`"]
    #[inline]
    pub fn is_the_calibration_coun(&self) -> bool {
        *self == CCALENR::THE_CALIBRATION_COUN
    }
}
#[doc = "Values that can be written to the field `CLKEN`"]
pub enum CLKENW {
    #[doc = "The time counters are enabled."]
    THE_TIME_COUNTERS_AR,
    #[doc = "The time counters are disabled so that they may be initialized."]
    THE_TIME_COUNTERS_AR,
}
impl CLKENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CLKENW::THE_TIME_COUNTERS_AR => true,
            CLKENW::THE_TIME_COUNTERS_AR => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CLKENW<'a> {
    w: &'a mut W,
}
impl<'a> _CLKENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CLKENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The time counters are enabled."]
    #[inline]
    pub fn the_time_counters_ar(self) -> &'a mut W {
        self.variant(CLKENW::THE_TIME_COUNTERS_AR)
    }
    #[doc = "The time counters are disabled so that they may be initialized."]
    #[inline]
    pub fn the_time_counters_ar(self) -> &'a mut W {
        self.variant(CLKENW::THE_TIME_COUNTERS_AR)
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
#[doc = "Values that can be written to the field `CTCRST`"]
pub enum CTCRSTW {
    #[doc = "When one, the elements in the internal oscillator divider are reset, and remain reset until CCR\\[1\\] is changed to zero. This is the divider that generates the 1 Hz clock from the 32.768 kHz crystal. The state of the divider is not visible to software."]
    RESET,
    #[doc = "No effect."]
    NO_EFFECT_,
}
impl CTCRSTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CTCRSTW::RESET => true,
            CTCRSTW::NO_EFFECT_ => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CTCRSTW<'a> {
    w: &'a mut W,
}
impl<'a> _CTCRSTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CTCRSTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "When one, the elements in the internal oscillator divider are reset, and remain reset until CCR\\[1\\] is changed to zero. This is the divider that generates the 1 Hz clock from the 32.768 kHz crystal. The state of the divider is not visible to software."]
    #[inline]
    pub fn reset(self) -> &'a mut W {
        self.variant(CTCRSTW::RESET)
    }
    #[doc = "No effect."]
    #[inline]
    pub fn no_effect_(self) -> &'a mut W {
        self.variant(CTCRSTW::NO_EFFECT_)
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
#[doc = "Values that can be written to the field `CCALEN`"]
pub enum CCALENW {
    #[doc = "The calibration counter is disabled and reset to zero."]
    THE_CALIBRATION_COUN,
    #[doc = "The calibration counter is enabled and counting, using the 1 Hz clock. When the calibration counter is equal to the value of the CALIBRATION register, the counter resets and repeats counting up to the value of the CALIBRATION register. See Section 30.6.4.2 and  Section 30.6.5."]
    THE_CALIBRATION_COUN,
}
impl CCALENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CCALENW::THE_CALIBRATION_COUN => true,
            CCALENW::THE_CALIBRATION_COUN => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CCALENW<'a> {
    w: &'a mut W,
}
impl<'a> _CCALENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CCALENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The calibration counter is disabled and reset to zero."]
    #[inline]
    pub fn the_calibration_coun(self) -> &'a mut W {
        self.variant(CCALENW::THE_CALIBRATION_COUN)
    }
    #[doc = "The calibration counter is enabled and counting, using the 1 Hz clock. When the calibration counter is equal to the value of the CALIBRATION register, the counter resets and repeats counting up to the value of the CALIBRATION register. See Section 30.6.4.2 and Section 30.6.5."]
    #[inline]
    pub fn the_calibration_coun(self) -> &'a mut W {
        self.variant(CCALENW::THE_CALIBRATION_COUN)
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
        const OFFSET: u8 = 4;
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
    #[doc = "Bit 0 - Clock Enable."]
    #[inline]
    pub fn clken(&self) -> CLKENR {
        CLKENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - CTC Reset."]
    #[inline]
    pub fn ctcrst(&self) -> CTCRSTR {
        CTCRSTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - Calibration counter enable."]
    #[inline]
    pub fn ccalen(&self) -> CCALENR {
        CCALENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
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
    #[doc = "Bit 0 - Clock Enable."]
    #[inline]
    pub fn clken(&mut self) -> _CLKENW {
        _CLKENW { w: self }
    }
    #[doc = "Bit 1 - CTC Reset."]
    #[inline]
    pub fn ctcrst(&mut self) -> _CTCRSTW {
        _CTCRSTW { w: self }
    }
    #[doc = "Bit 4 - Calibration counter enable."]
    #[inline]
    pub fn ccalen(&mut self) -> _CCALENW {
        _CCALENW { w: self }
    }
}
