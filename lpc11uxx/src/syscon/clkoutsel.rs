#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CLKOUTSEL {
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
#[doc = "Possible values of the field `SEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SELR {
    #[doc = "IRC oscillator"]
    IRC_OSCILLATOR,
    #[doc = "Crystal oscillator (SYSOSC)"]
    CRYSTAL_OSCILLATOR,
    #[doc = "LF oscillator (watchdog oscillator)"]
    LF_OSCILLATOR_WATCH,
    #[doc = "Main clock"]
    MAIN_CLOCK,
}
impl SELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            SELR::IRC_OSCILLATOR => 0,
            SELR::CRYSTAL_OSCILLATOR => 1,
            SELR::LF_OSCILLATOR_WATCH => 2,
            SELR::MAIN_CLOCK => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> SELR {
        match value {
            0 => SELR::IRC_OSCILLATOR,
            1 => SELR::CRYSTAL_OSCILLATOR,
            2 => SELR::LF_OSCILLATOR_WATCH,
            3 => SELR::MAIN_CLOCK,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `IRC_OSCILLATOR`"]
    #[inline]
    pub fn is_irc_oscillator(&self) -> bool {
        *self == SELR::IRC_OSCILLATOR
    }
    #[doc = "Checks if the value of the field is `CRYSTAL_OSCILLATOR`"]
    #[inline]
    pub fn is_crystal_oscillator(&self) -> bool {
        *self == SELR::CRYSTAL_OSCILLATOR
    }
    #[doc = "Checks if the value of the field is `LF_OSCILLATOR_WATCH`"]
    #[inline]
    pub fn is_lf_oscillator_watch(&self) -> bool {
        *self == SELR::LF_OSCILLATOR_WATCH
    }
    #[doc = "Checks if the value of the field is `MAIN_CLOCK`"]
    #[inline]
    pub fn is_main_clock(&self) -> bool {
        *self == SELR::MAIN_CLOCK
    }
}
#[doc = "Values that can be written to the field `SEL`"]
pub enum SELW {
    #[doc = "IRC oscillator"]
    IRC_OSCILLATOR,
    #[doc = "Crystal oscillator (SYSOSC)"]
    CRYSTAL_OSCILLATOR,
    #[doc = "LF oscillator (watchdog oscillator)"]
    LF_OSCILLATOR_WATCH,
    #[doc = "Main clock"]
    MAIN_CLOCK,
}
impl SELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            SELW::IRC_OSCILLATOR => 0,
            SELW::CRYSTAL_OSCILLATOR => 1,
            SELW::LF_OSCILLATOR_WATCH => 2,
            SELW::MAIN_CLOCK => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SELW<'a> {
    w: &'a mut W,
}
impl<'a> _SELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SELW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "IRC oscillator"]
    #[inline]
    pub fn irc_oscillator(self) -> &'a mut W {
        self.variant(SELW::IRC_OSCILLATOR)
    }
    #[doc = "Crystal oscillator (SYSOSC)"]
    #[inline]
    pub fn crystal_oscillator(self) -> &'a mut W {
        self.variant(SELW::CRYSTAL_OSCILLATOR)
    }
    #[doc = "LF oscillator (watchdog oscillator)"]
    #[inline]
    pub fn lf_oscillator_watch(self) -> &'a mut W {
        self.variant(SELW::LF_OSCILLATOR_WATCH)
    }
    #[doc = "Main clock"]
    #[inline]
    pub fn main_clock(self) -> &'a mut W {
        self.variant(SELW::MAIN_CLOCK)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
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
    #[doc = "Bits 0:1 - CLKOUT clock source"]
    #[inline]
    pub fn sel(&self) -> SELR {
        SELR::_from({
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
    #[doc = "Bits 0:1 - CLKOUT clock source"]
    #[inline]
    pub fn sel(&mut self) -> _SELW {
        _SELW { w: self }
    }
}
