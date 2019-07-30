#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::SYSPLLCLKSEL {
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
#[doc = "Possible values of the field `SEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SELR {
    #[doc = "IRC"]
    IRC,
    #[doc = "System oscillator. Crystal Oscillator (SYSOSC)"]
    SYSTEM_OSCILLATOR,
    #[doc = "32 kHz clock.Select this option when the 32 kHz clock is the clock source for the main clock and select the pll input in the MAINCLKSEL register. Do not use the 32 kHz clock with the PLL."]
    _32_KHZ_CLOCK,
}
impl crate::ToBits<u8> for SELR {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            SELR::IRC => 0,
            SELR::SYSTEM_OSCILLATOR => 1,
            SELR::_32_KHZ_CLOCK => 3,
        }
    }
}
#[doc = r"Reader of the field"]
pub type SEL_R = crate::FR<u8, SELR>;
impl SEL_R {
    #[doc = "Checks if the value of the field is `IRC`"]
    #[inline(always)]
    pub fn is_irc(&self) -> bool {
        *self == SELR::IRC
    }
    #[doc = "Checks if the value of the field is `SYSTEM_OSCILLATOR`"]
    #[inline(always)]
    pub fn is_system_oscillator(&self) -> bool {
        *self == SELR::SYSTEM_OSCILLATOR
    }
    #[doc = "Checks if the value of the field is `_32_KHZ_CLOCK`"]
    #[inline(always)]
    pub fn is_32_khz_clock(&self) -> bool {
        *self == SELR::_32_KHZ_CLOCK
    }
}
#[doc = "Values that can be written to the field `SEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SELW {
    #[doc = "IRC"]
    IRC,
    #[doc = "System oscillator. Crystal Oscillator (SYSOSC)"]
    SYSTEM_OSCILLATOR,
    #[doc = "32 kHz clock.Select this option when the 32 kHz clock is the clock source for the main clock and select the pll input in the MAINCLKSEL register. Do not use the 32 kHz clock with the PLL."]
    _32_KHZ_CLOCK,
}
impl SELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            SELW::IRC => 0,
            SELW::SYSTEM_OSCILLATOR => 1,
            SELW::_32_KHZ_CLOCK => 3,
        }
    }
}
#[doc = r"Proxy"]
pub struct _SELW<'a> {
    w: &'a mut W,
}
impl<'a> _SELW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SELW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "IRC"]
    #[inline(always)]
    pub fn irc(self) -> &'a mut W {
        self.variant(SELW::IRC)
    }
    #[doc = "System oscillator. Crystal Oscillator (SYSOSC)"]
    #[inline(always)]
    pub fn system_oscillator(self) -> &'a mut W {
        self.variant(SELW::SYSTEM_OSCILLATOR)
    }
    #[doc = "32 kHz clock.Select this option when the 32 kHz clock is the clock source for the main clock and select the pll input in the MAINCLKSEL register. Do not use the 32 kHz clock with the PLL."]
    #[inline(always)]
    pub fn _32_khz_clock(self) -> &'a mut W {
        self.variant(SELW::_32_KHZ_CLOCK)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:1 - System PLL clock source"]
    #[inline(always)]
    pub fn sel(&self) -> SEL_R {
        SEL_R::new((self.bits() & 0x03) as u8)
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:1 - System PLL clock source"]
    #[inline(always)]
    pub fn sel(&mut self) -> _SELW {
        _SELW { w: self }
    }
}
