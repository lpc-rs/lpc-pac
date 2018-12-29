#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CLKOUTCFG {
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
#[doc = "Possible values of the field `CLKOUTSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CLKOUTSELR {
    #[doc = "Selects the CPU clock as the CLKOUT source."]
    SELECTS_THE_CPU_CLOC,
    #[doc = "Selects the main oscillator as the CLKOUT source."]
    SELECTS_THE_MAIN_OSC,
    #[doc = "Selects the Internal RC oscillator as the CLKOUT source."]
    SELECTS_THE_INTERNAL,
    #[doc = "Selects the USB clock as the CLKOUT source."]
    SELECTS_THE_USB_CLOC,
    #[doc = "Selects the RTC oscillator as the CLKOUT source."]
    SELECTS_THE_RTC_OSCI,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl CLKOUTSELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            CLKOUTSELR::SELECTS_THE_CPU_CLOC => 0,
            CLKOUTSELR::SELECTS_THE_MAIN_OSC => 1,
            CLKOUTSELR::SELECTS_THE_INTERNAL => 2,
            CLKOUTSELR::SELECTS_THE_USB_CLOC => 3,
            CLKOUTSELR::SELECTS_THE_RTC_OSCI => 4,
            CLKOUTSELR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> CLKOUTSELR {
        match value {
            0 => CLKOUTSELR::SELECTS_THE_CPU_CLOC,
            1 => CLKOUTSELR::SELECTS_THE_MAIN_OSC,
            2 => CLKOUTSELR::SELECTS_THE_INTERNAL,
            3 => CLKOUTSELR::SELECTS_THE_USB_CLOC,
            4 => CLKOUTSELR::SELECTS_THE_RTC_OSCI,
            i => CLKOUTSELR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `SELECTS_THE_CPU_CLOC`"]
    #[inline]
    pub fn is_selects_the_cpu_cloc(&self) -> bool {
        *self == CLKOUTSELR::SELECTS_THE_CPU_CLOC
    }
    #[doc = "Checks if the value of the field is `SELECTS_THE_MAIN_OSC`"]
    #[inline]
    pub fn is_selects_the_main_osc(&self) -> bool {
        *self == CLKOUTSELR::SELECTS_THE_MAIN_OSC
    }
    #[doc = "Checks if the value of the field is `SELECTS_THE_INTERNAL`"]
    #[inline]
    pub fn is_selects_the_internal(&self) -> bool {
        *self == CLKOUTSELR::SELECTS_THE_INTERNAL
    }
    #[doc = "Checks if the value of the field is `SELECTS_THE_USB_CLOC`"]
    #[inline]
    pub fn is_selects_the_usb_cloc(&self) -> bool {
        *self == CLKOUTSELR::SELECTS_THE_USB_CLOC
    }
    #[doc = "Checks if the value of the field is `SELECTS_THE_RTC_OSCI`"]
    #[inline]
    pub fn is_selects_the_rtc_osci(&self) -> bool {
        *self == CLKOUTSELR::SELECTS_THE_RTC_OSCI
    }
}
#[doc = r" Value of the field"]
pub struct CLKOUTDIVR {
    bits: u8,
}
impl CLKOUTDIVR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct CLKOUT_ENR {
    bits: bool,
}
impl CLKOUT_ENR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
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
}
#[doc = r" Value of the field"]
pub struct CLKOUT_ACTR {
    bits: bool,
}
impl CLKOUT_ACTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
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
}
#[doc = "Values that can be written to the field `CLKOUTSEL`"]
pub enum CLKOUTSELW {
    #[doc = "Selects the CPU clock as the CLKOUT source."]
    SELECTS_THE_CPU_CLOC,
    #[doc = "Selects the main oscillator as the CLKOUT source."]
    SELECTS_THE_MAIN_OSC,
    #[doc = "Selects the Internal RC oscillator as the CLKOUT source."]
    SELECTS_THE_INTERNAL,
    #[doc = "Selects the USB clock as the CLKOUT source."]
    SELECTS_THE_USB_CLOC,
    #[doc = "Selects the RTC oscillator as the CLKOUT source."]
    SELECTS_THE_RTC_OSCI,
}
impl CLKOUTSELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            CLKOUTSELW::SELECTS_THE_CPU_CLOC => 0,
            CLKOUTSELW::SELECTS_THE_MAIN_OSC => 1,
            CLKOUTSELW::SELECTS_THE_INTERNAL => 2,
            CLKOUTSELW::SELECTS_THE_USB_CLOC => 3,
            CLKOUTSELW::SELECTS_THE_RTC_OSCI => 4,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CLKOUTSELW<'a> {
    w: &'a mut W,
}
impl<'a> _CLKOUTSELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CLKOUTSELW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Selects the CPU clock as the CLKOUT source."]
    #[inline]
    pub fn selects_the_cpu_cloc(self) -> &'a mut W {
        self.variant(CLKOUTSELW::SELECTS_THE_CPU_CLOC)
    }
    #[doc = "Selects the main oscillator as the CLKOUT source."]
    #[inline]
    pub fn selects_the_main_osc(self) -> &'a mut W {
        self.variant(CLKOUTSELW::SELECTS_THE_MAIN_OSC)
    }
    #[doc = "Selects the Internal RC oscillator as the CLKOUT source."]
    #[inline]
    pub fn selects_the_internal(self) -> &'a mut W {
        self.variant(CLKOUTSELW::SELECTS_THE_INTERNAL)
    }
    #[doc = "Selects the USB clock as the CLKOUT source."]
    #[inline]
    pub fn selects_the_usb_cloc(self) -> &'a mut W {
        self.variant(CLKOUTSELW::SELECTS_THE_USB_CLOC)
    }
    #[doc = "Selects the RTC oscillator as the CLKOUT source."]
    #[inline]
    pub fn selects_the_rtc_osci(self) -> &'a mut W {
        self.variant(CLKOUTSELW::SELECTS_THE_RTC_OSCI)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _CLKOUTDIVW<'a> {
    w: &'a mut W,
}
impl<'a> _CLKOUTDIVW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _CLKOUT_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _CLKOUT_ENW<'a> {
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
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _CLKOUT_ACTW<'a> {
    w: &'a mut W,
}
impl<'a> _CLKOUT_ACTW<'a> {
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
        const OFFSET: u8 = 9;
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
    #[doc = "Bits 0:3 - Selects the clock source for the CLKOUT function. Other values are reserved. Do not use."]
    #[inline]
    pub fn clkoutsel(&self) -> CLKOUTSELR {
        CLKOUTSELR::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 4:7 - Integer value to divide the output clock by, minus one. 0 = Clock is divided by 1 1 = Clock is divided by 2. 2 = Clock is divided by 3. ... 15 = Clock is divided by 16."]
    #[inline]
    pub fn clkoutdiv(&self) -> CLKOUTDIVR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        CLKOUTDIVR { bits }
    }
    #[doc = "Bit 8 - CLKOUT enable control, allows switching the CLKOUT source without glitches. Clear to stop CLKOUT on the next falling edge. Set to enable CLKOUT."]
    #[inline]
    pub fn clkout_en(&self) -> CLKOUT_ENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CLKOUT_ENR { bits }
    }
    #[doc = "Bit 9 - CLKOUT activity indication. Reads as 1 when CLKOUT is enabled. Read as 0 when CLKOUT has been disabled via the CLKOUT_EN bit and the clock has completed being stopped."]
    #[inline]
    pub fn clkout_act(&self) -> CLKOUT_ACTR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CLKOUT_ACTR { bits }
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
    #[doc = "Bits 0:3 - Selects the clock source for the CLKOUT function. Other values are reserved. Do not use."]
    #[inline]
    pub fn clkoutsel(&mut self) -> _CLKOUTSELW {
        _CLKOUTSELW { w: self }
    }
    #[doc = "Bits 4:7 - Integer value to divide the output clock by, minus one. 0 = Clock is divided by 1 1 = Clock is divided by 2. 2 = Clock is divided by 3. ... 15 = Clock is divided by 16."]
    #[inline]
    pub fn clkoutdiv(&mut self) -> _CLKOUTDIVW {
        _CLKOUTDIVW { w: self }
    }
    #[doc = "Bit 8 - CLKOUT enable control, allows switching the CLKOUT source without glitches. Clear to stop CLKOUT on the next falling edge. Set to enable CLKOUT."]
    #[inline]
    pub fn clkout_en(&mut self) -> _CLKOUT_ENW {
        _CLKOUT_ENW { w: self }
    }
    #[doc = "Bit 9 - CLKOUT activity indication. Reads as 1 when CLKOUT is enabled. Read as 0 when CLKOUT has been disabled via the CLKOUT_EN bit and the clock has completed being stopped."]
    #[inline]
    pub fn clkout_act(&mut self) -> _CLKOUT_ACTW {
        _CLKOUT_ACTW { w: self }
    }
}
