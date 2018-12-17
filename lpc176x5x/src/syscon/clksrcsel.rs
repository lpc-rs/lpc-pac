#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CLKSRCSEL {
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
#[doc = "Possible values of the field `CLKSRC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CLKSRCR {
    #[doc = "Selects the Internal RC oscillator as the PLL0 clock source (default)."]
    SELECTS_THE_INTERNAL,
    #[doc = "Selects the main oscillator as the PLL0 clock source.  Select the main oscillator as PLL0 clock source if the PLL0 clock output is used for USB or for CAN with baudrates > 100 kBit/s."]
    SELECTS_THE_MAIN_OSC,
    #[doc = "Selects the RTC oscillator as the PLL0 clock source."]
    SELECTS_THE_RTC_OSCI,
}
impl CLKSRCR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            CLKSRCR::SELECTS_THE_INTERNAL => 0,
            CLKSRCR::SELECTS_THE_MAIN_OSC => 1,
            CLKSRCR::SELECTS_THE_RTC_OSCI => 2,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> CLKSRCR {
        match value {
            0 => CLKSRCR::SELECTS_THE_INTERNAL,
            1 => CLKSRCR::SELECTS_THE_MAIN_OSC,
            2 => CLKSRCR::SELECTS_THE_RTC_OSCI,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `SELECTS_THE_INTERNAL`"]
    #[inline]
    pub fn is_selects_the_internal(&self) -> bool {
        *self == CLKSRCR::SELECTS_THE_INTERNAL
    }
    #[doc = "Checks if the value of the field is `SELECTS_THE_MAIN_OSC`"]
    #[inline]
    pub fn is_selects_the_main_osc(&self) -> bool {
        *self == CLKSRCR::SELECTS_THE_MAIN_OSC
    }
    #[doc = "Checks if the value of the field is `SELECTS_THE_RTC_OSCI`"]
    #[inline]
    pub fn is_selects_the_rtc_osci(&self) -> bool {
        *self == CLKSRCR::SELECTS_THE_RTC_OSCI
    }
}
#[doc = "Values that can be written to the field `CLKSRC`"]
pub enum CLKSRCW {
    #[doc = "Selects the Internal RC oscillator as the PLL0 clock source (default)."]
    SELECTS_THE_INTERNAL,
    #[doc = "Selects the main oscillator as the PLL0 clock source.  Select the main oscillator as PLL0 clock source if the PLL0 clock output is used for USB or for CAN with baudrates > 100 kBit/s."]
    SELECTS_THE_MAIN_OSC,
    #[doc = "Selects the RTC oscillator as the PLL0 clock source."]
    SELECTS_THE_RTC_OSCI,
}
impl CLKSRCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            CLKSRCW::SELECTS_THE_INTERNAL => 0,
            CLKSRCW::SELECTS_THE_MAIN_OSC => 1,
            CLKSRCW::SELECTS_THE_RTC_OSCI => 2,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CLKSRCW<'a> {
    w: &'a mut W,
}
impl<'a> _CLKSRCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CLKSRCW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Selects the Internal RC oscillator as the PLL0 clock source (default)."]
    #[inline]
    pub fn selects_the_internal(self) -> &'a mut W {
        self.variant(CLKSRCW::SELECTS_THE_INTERNAL)
    }
    #[doc = "Selects the main oscillator as the PLL0 clock source. Select the main oscillator as PLL0 clock source if the PLL0 clock output is used for USB or for CAN with baudrates > 100 kBit/s."]
    #[inline]
    pub fn selects_the_main_osc(self) -> &'a mut W {
        self.variant(CLKSRCW::SELECTS_THE_MAIN_OSC)
    }
    #[doc = "Selects the RTC oscillator as the PLL0 clock source."]
    #[inline]
    pub fn selects_the_rtc_osci(self) -> &'a mut W {
        self.variant(CLKSRCW::SELECTS_THE_RTC_OSCI)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 0;
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
    #[doc = "Bits 0:1 - Selects the clock source for PLL0 as follows. Warning: Improper setting of this value, or an incorrect sequence of changing this value may result in incorrect operation of the device."]
    #[inline]
    pub fn clksrc(&self) -> CLKSRCR {
        CLKSRCR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
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
    #[doc = "Bits 0:1 - Selects the clock source for PLL0 as follows. Warning: Improper setting of this value, or an incorrect sequence of changing this value may result in incorrect operation of the device."]
    #[inline]
    pub fn clksrc(&mut self) -> _CLKSRCW {
        _CLKSRCW { w: self }
    }
}
