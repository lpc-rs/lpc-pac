#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::MAINCLKSEL {
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
    #[doc = "IRC Oscillator"]
    IRC_OSCILLATOR,
    #[doc = "PLL input"]
    PLL_INPUT,
    #[doc = "Watchdog oscillator"]
    WATCHDOG_OSCILLATOR,
    #[doc = "PLL output"]
    PLL_OUTPUT,
}
impl crate::ToBits<u8> for SELR {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            SELR::IRC_OSCILLATOR => 0,
            SELR::PLL_INPUT => 1,
            SELR::WATCHDOG_OSCILLATOR => 2,
            SELR::PLL_OUTPUT => 3,
        }
    }
}
#[doc = r"Reader of the field"]
pub type SEL_R = crate::FR<u8, SELR>;
impl SEL_R {
    #[doc = "Checks if the value of the field is `IRC_OSCILLATOR`"]
    #[inline(always)]
    pub fn is_irc_oscillator(&self) -> bool {
        *self == SELR::IRC_OSCILLATOR
    }
    #[doc = "Checks if the value of the field is `PLL_INPUT`"]
    #[inline(always)]
    pub fn is_pll_input(&self) -> bool {
        *self == SELR::PLL_INPUT
    }
    #[doc = "Checks if the value of the field is `WATCHDOG_OSCILLATOR`"]
    #[inline(always)]
    pub fn is_watchdog_oscillator(&self) -> bool {
        *self == SELR::WATCHDOG_OSCILLATOR
    }
    #[doc = "Checks if the value of the field is `PLL_OUTPUT`"]
    #[inline(always)]
    pub fn is_pll_output(&self) -> bool {
        *self == SELR::PLL_OUTPUT
    }
}
#[doc = "Values that can be written to the field `SEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SELW {
    #[doc = "IRC Oscillator"]
    IRC_OSCILLATOR,
    #[doc = "PLL input"]
    PLL_INPUT,
    #[doc = "Watchdog oscillator"]
    WATCHDOG_OSCILLATOR,
    #[doc = "PLL output"]
    PLL_OUTPUT,
}
impl SELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            SELW::IRC_OSCILLATOR => 0,
            SELW::PLL_INPUT => 1,
            SELW::WATCHDOG_OSCILLATOR => 2,
            SELW::PLL_OUTPUT => 3,
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
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "IRC Oscillator"]
    #[inline(always)]
    pub fn irc_oscillator(self) -> &'a mut W {
        self.variant(SELW::IRC_OSCILLATOR)
    }
    #[doc = "PLL input"]
    #[inline(always)]
    pub fn pll_input(self) -> &'a mut W {
        self.variant(SELW::PLL_INPUT)
    }
    #[doc = "Watchdog oscillator"]
    #[inline(always)]
    pub fn watchdog_oscillator(self) -> &'a mut W {
        self.variant(SELW::WATCHDOG_OSCILLATOR)
    }
    #[doc = "PLL output"]
    #[inline(always)]
    pub fn pll_output(self) -> &'a mut W {
        self.variant(SELW::PLL_OUTPUT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
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
    #[doc = "Bits 0:1 - Clock source for main clock"]
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
    #[doc = "Bits 0:1 - Clock source for main clock"]
    #[inline(always)]
    pub fn sel(&mut self) -> _SELW {
        _SELW { w: self }
    }
}
