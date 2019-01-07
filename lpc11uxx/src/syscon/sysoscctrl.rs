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
    DISABLED,
    #[doc = "Bypass enabled. PLL input (sys_osc_clk) is fed directly from the XTALIN pin bypassing the oscillator. Use this mode when using an external clock source instead of the crystal oscillator."]
    ENABLED,
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
            BYPASSR::DISABLED => false,
            BYPASSR::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> BYPASSR {
        match value {
            false => BYPASSR::DISABLED,
            true => BYPASSR::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == BYPASSR::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == BYPASSR::ENABLED
    }
}
#[doc = "Possible values of the field `FREQRANGE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FREQRANGER {
    #[doc = "1 - 20 MHz frequency range."]
    _1_20_MHZ_FREQUENCY,
    #[doc = "15 - 25 MHz frequency range"]
    _15_25_MHZ_FREQUENC,
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
            FREQRANGER::_1_20_MHZ_FREQUENCY => false,
            FREQRANGER::_15_25_MHZ_FREQUENC => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FREQRANGER {
        match value {
            false => FREQRANGER::_1_20_MHZ_FREQUENCY,
            true => FREQRANGER::_15_25_MHZ_FREQUENC,
        }
    }
    #[doc = "Checks if the value of the field is `_1_20_MHZ_FREQUENCY`"]
    #[inline]
    pub fn is_1_20_mhz_frequency(&self) -> bool {
        *self == FREQRANGER::_1_20_MHZ_FREQUENCY
    }
    #[doc = "Checks if the value of the field is `_15_25_MHZ_FREQUENC`"]
    #[inline]
    pub fn is_15_25_mhz_frequenc(&self) -> bool {
        *self == FREQRANGER::_15_25_MHZ_FREQUENC
    }
}
#[doc = "Values that can be written to the field `BYPASS`"]
pub enum BYPASSW {
    #[doc = "Oscillator is not bypassed."]
    DISABLED,
    #[doc = "Bypass enabled. PLL input (sys_osc_clk) is fed directly from the XTALIN pin bypassing the oscillator. Use this mode when using an external clock source instead of the crystal oscillator."]
    ENABLED,
}
impl BYPASSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            BYPASSW::DISABLED => false,
            BYPASSW::ENABLED => true,
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
    pub fn disabled(self) -> &'a mut W {
        self.variant(BYPASSW::DISABLED)
    }
    #[doc = "Bypass enabled. PLL input (sys_osc_clk) is fed directly from the XTALIN pin bypassing the oscillator. Use this mode when using an external clock source instead of the crystal oscillator."]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(BYPASSW::ENABLED)
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
    #[doc = "1 - 20 MHz frequency range."]
    _1_20_MHZ_FREQUENCY,
    #[doc = "15 - 25 MHz frequency range"]
    _15_25_MHZ_FREQUENC,
}
impl FREQRANGEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            FREQRANGEW::_1_20_MHZ_FREQUENCY => false,
            FREQRANGEW::_15_25_MHZ_FREQUENC => true,
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
    #[doc = "1 - 20 MHz frequency range."]
    #[inline]
    pub fn _1_20_mhz_frequency(self) -> &'a mut W {
        self.variant(FREQRANGEW::_1_20_MHZ_FREQUENCY)
    }
    #[doc = "15 - 25 MHz frequency range"]
    #[inline]
    pub fn _15_25_mhz_frequenc(self) -> &'a mut W {
        self.variant(FREQRANGEW::_15_25_MHZ_FREQUENC)
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
