#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::SYSOSCCTRL {
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
#[doc = "Possible values of the field `BYPASS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BYPASSR {
    #[doc = "Oscillator is not bypassed."]
    OSCILLATOR_IS_NOT_BY,
    #[doc = "Bypass enabled. PLL input (sys_osc_clk) is fed directly from the XTALIN pin bypassing the oscillator. Use this mode when using an external clock source instead of the crystal oscillator."]
    BYPASS_ENABLED,
}
impl BYPASSR {
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
            BYPASSR::OSCILLATOR_IS_NOT_BY => false,
            BYPASSR::BYPASS_ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> BYPASSR {
        match value {
            false => BYPASSR::OSCILLATOR_IS_NOT_BY,
            true => BYPASSR::BYPASS_ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `OSCILLATOR_IS_NOT_BY`"]
    #[inline]
    pub fn is_oscillator_is_not_by(&self) -> bool {
        *self == BYPASSR::OSCILLATOR_IS_NOT_BY
    }
    #[doc = "Checks if the value of the field is `BYPASS_ENABLED`"]
    #[inline]
    pub fn is_bypass_enabled(&self) -> bool {
        *self == BYPASSR::BYPASS_ENABLED
    }
}
#[doc = "Possible values of the field `FREQRANGE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FREQRANGER {
    #[doc = "Low. 1 - 20 MHz frequency range."]
    LOW,
    #[doc = "High. 15 - 25 MHz frequency range."]
    HIGH,
}
impl FREQRANGER {
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
            FREQRANGER::LOW => false,
            FREQRANGER::HIGH => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FREQRANGER {
        match value {
            false => FREQRANGER::LOW,
            true => FREQRANGER::HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline]
    pub fn is_low(&self) -> bool {
        *self == FREQRANGER::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline]
    pub fn is_high(&self) -> bool {
        *self == FREQRANGER::HIGH
    }
}
#[doc = "Values that can be written to the field `BYPASS`"]
pub enum BYPASSW {
    #[doc = "Oscillator is not bypassed."]
    OSCILLATOR_IS_NOT_BY,
    #[doc = "Bypass enabled. PLL input (sys_osc_clk) is fed directly from the XTALIN pin bypassing the oscillator. Use this mode when using an external clock source instead of the crystal oscillator."]
    BYPASS_ENABLED,
}
impl BYPASSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            BYPASSW::OSCILLATOR_IS_NOT_BY => false,
            BYPASSW::BYPASS_ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _BYPASSW<'a> {
    w: &'a mut W,
}
impl<'a> _BYPASSW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BYPASSW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Oscillator is not bypassed."]
    #[inline]
    pub fn oscillator_is_not_by(self) -> &'a mut W {
        self.variant(BYPASSW::OSCILLATOR_IS_NOT_BY)
    }
    #[doc = "Bypass enabled. PLL input (sys_osc_clk) is fed directly from the XTALIN pin bypassing the oscillator. Use this mode when using an external clock source instead of the crystal oscillator."]
    #[inline]
    pub fn bypass_enabled(self) -> &'a mut W {
        self.variant(BYPASSW::BYPASS_ENABLED)
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
#[doc = "Values that can be written to the field `FREQRANGE`"]
pub enum FREQRANGEW {
    #[doc = "Low. 1 - 20 MHz frequency range."]
    LOW,
    #[doc = "High. 15 - 25 MHz frequency range."]
    HIGH,
}
impl FREQRANGEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            FREQRANGEW::LOW => false,
            FREQRANGEW::HIGH => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FREQRANGEW<'a> {
    w: &'a mut W,
}
impl<'a> _FREQRANGEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FREQRANGEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Low. 1 - 20 MHz frequency range."]
    #[inline]
    pub fn low(self) -> &'a mut W {
        self.variant(FREQRANGEW::LOW)
    }
    #[doc = "High. 15 - 25 MHz frequency range."]
    #[inline]
    pub fn high(self) -> &'a mut W {
        self.variant(FREQRANGEW::HIGH)
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
    #[doc = "Bit 0 - Bypass system oscillator"]
    #[inline]
    pub fn bypass(&self) -> BYPASSR {
        BYPASSR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Determines frequency range for Low-power oscillator."]
    #[inline]
    pub fn freqrange(&self) -> FREQRANGER {
        FREQRANGER::_from({
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
    #[doc = "Bit 0 - Bypass system oscillator"]
    #[inline]
    pub fn bypass(&mut self) -> _BYPASSW {
        _BYPASSW { w: self }
    }
    #[doc = "Bit 1 - Determines frequency range for Low-power oscillator."]
    #[inline]
    pub fn freqrange(&mut self) -> _FREQRANGEW {
        _FREQRANGEW { w: self }
    }
}
