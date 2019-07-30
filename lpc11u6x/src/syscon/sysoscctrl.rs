#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::SYSOSCCTRL {
    #[doc = r"Modifies the contents of the register"]
    #[inline(always)]
    pub fn modify<F>(&self, f: F)
    where
        for<'w> F: FnOnce(&R, &'w mut W) -> &'w mut W,
    {
        let bits = self.register.get();
        self.register.set(f(&R { bits }, &mut W { bits }).bits);
    }
    #[doc = r"Reads the contents of the register"]
    #[inline(always)]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
    #[doc = r"Writes to the register"]
    #[inline(always)]
    pub fn write<F>(&self, f: F)
    where
        F: FnOnce(&mut W) -> &mut W,
    {
        self.register.set(
            f(&mut W {
                bits: Self::reset_value(),
            })
            .bits,
        );
    }
    #[doc = r"Reset value of the register"]
    #[inline(always)]
    pub const fn reset_value() -> u32 {
        0
    }
    #[doc = r"Writes the reset value to the register"]
    #[inline(always)]
    pub fn reset(&self) {
        self.register.set(Self::reset_value())
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
impl crate::ToBits<bool> for BYPASSR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            BYPASSR::OSCILLATOR_IS_NOT_BY => false,
            BYPASSR::BYPASS_ENABLED => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type BYPASS_R = crate::FR<bool, BYPASSR>;
impl BYPASS_R {
    #[doc = "Checks if the value of the field is `OSCILLATOR_IS_NOT_BY`"]
    #[inline(always)]
    pub fn is_oscillator_is_not_by(&self) -> bool {
        *self == BYPASSR::OSCILLATOR_IS_NOT_BY
    }
    #[doc = "Checks if the value of the field is `BYPASS_ENABLED`"]
    #[inline(always)]
    pub fn is_bypass_enabled(&self) -> bool {
        *self == BYPASSR::BYPASS_ENABLED
    }
}
#[doc = "Values that can be written to the field `BYPASS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BYPASSW {
    #[doc = "Oscillator is not bypassed."]
    OSCILLATOR_IS_NOT_BY,
    #[doc = "Bypass enabled. PLL input (sys_osc_clk) is fed directly from the XTALIN pin bypassing the oscillator. Use this mode when using an external clock source instead of the crystal oscillator."]
    BYPASS_ENABLED,
}
impl BYPASSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            BYPASSW::OSCILLATOR_IS_NOT_BY => false,
            BYPASSW::BYPASS_ENABLED => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _BYPASSW<'a> {
    w: &'a mut W,
}
impl<'a> _BYPASSW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BYPASSW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Oscillator is not bypassed."]
    #[inline(always)]
    pub fn oscillator_is_not_by(self) -> &'a mut W {
        self.variant(BYPASSW::OSCILLATOR_IS_NOT_BY)
    }
    #[doc = "Bypass enabled. PLL input (sys_osc_clk) is fed directly from the XTALIN pin bypassing the oscillator. Use this mode when using an external clock source instead of the crystal oscillator."]
    #[inline(always)]
    pub fn bypass_enabled(self) -> &'a mut W {
        self.variant(BYPASSW::BYPASS_ENABLED)
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
#[doc = "Possible values of the field `FREQRANGE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FREQRANGER {
    #[doc = "Low. 1 - 20 MHz frequency range."]
    LOW,
    #[doc = "High. 15 - 25 MHz frequency range."]
    HIGH,
}
impl crate::ToBits<bool> for FREQRANGER {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            FREQRANGER::LOW => false,
            FREQRANGER::HIGH => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type FREQRANGE_R = crate::FR<bool, FREQRANGER>;
impl FREQRANGE_R {
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == FREQRANGER::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == FREQRANGER::HIGH
    }
}
#[doc = "Values that can be written to the field `FREQRANGE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FREQRANGEW {
    #[doc = "Low. 1 - 20 MHz frequency range."]
    LOW,
    #[doc = "High. 15 - 25 MHz frequency range."]
    HIGH,
}
impl FREQRANGEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            FREQRANGEW::LOW => false,
            FREQRANGEW::HIGH => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _FREQRANGEW<'a> {
    w: &'a mut W,
}
impl<'a> _FREQRANGEW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FREQRANGEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Low. 1 - 20 MHz frequency range."]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(FREQRANGEW::LOW)
    }
    #[doc = "High. 15 - 25 MHz frequency range."]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(FREQRANGEW::HIGH)
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
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Bypass system oscillator"]
    #[inline(always)]
    pub fn bypass(&self) -> BYPASS_R {
        BYPASS_R::new((self.bits() & 0x01) != 0)
    }
    #[doc = "Bit 1 - Determines frequency range for Low-power oscillator."]
    #[inline(always)]
    pub fn freqrange(&self) -> FREQRANGE_R {
        FREQRANGE_R::new(((self.bits() >> 1) & 0x01) != 0)
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - Bypass system oscillator"]
    #[inline(always)]
    pub fn bypass(&mut self) -> _BYPASSW {
        _BYPASSW { w: self }
    }
    #[doc = "Bit 1 - Determines frequency range for Low-power oscillator."]
    #[inline(always)]
    pub fn freqrange(&mut self) -> _FREQRANGEW {
        _FREQRANGEW { w: self }
    }
}
