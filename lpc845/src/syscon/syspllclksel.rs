#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::SYSPLLCLKSEL {
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
    #[doc = "FRO"]
    FRO,
    #[doc = "External clock"]
    EXT_CLK,
    #[doc = "Watchdog oscillator"]
    WDTOSC,
    #[doc = "FRO DIV"]
    FRODIV,
}
impl SELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            SELR::FRO => 0,
            SELR::EXT_CLK => 1,
            SELR::WDTOSC => 2,
            SELR::FRODIV => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> SELR {
        match value {
            0 => SELR::FRO,
            1 => SELR::EXT_CLK,
            2 => SELR::WDTOSC,
            3 => SELR::FRODIV,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `FRO`"]
    #[inline]
    pub fn is_fro(&self) -> bool {
        *self == SELR::FRO
    }
    #[doc = "Checks if the value of the field is `EXT_CLK`"]
    #[inline]
    pub fn is_ext_clk(&self) -> bool {
        *self == SELR::EXT_CLK
    }
    #[doc = "Checks if the value of the field is `WDTOSC`"]
    #[inline]
    pub fn is_wdtosc(&self) -> bool {
        *self == SELR::WDTOSC
    }
    #[doc = "Checks if the value of the field is `FRODIV`"]
    #[inline]
    pub fn is_frodiv(&self) -> bool {
        *self == SELR::FRODIV
    }
}
#[doc = "Values that can be written to the field `SEL`"]
pub enum SELW {
    #[doc = "FRO"]
    FRO,
    #[doc = "External clock"]
    EXT_CLK,
    #[doc = "Watchdog oscillator"]
    WDTOSC,
    #[doc = "FRO DIV"]
    FRODIV,
}
impl SELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            SELW::FRO => 0,
            SELW::EXT_CLK => 1,
            SELW::WDTOSC => 2,
            SELW::FRODIV => 3,
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
    #[doc = "FRO"]
    #[inline]
    pub fn fro(self) -> &'a mut W {
        self.variant(SELW::FRO)
    }
    #[doc = "External clock"]
    #[inline]
    pub fn ext_clk(self) -> &'a mut W {
        self.variant(SELW::EXT_CLK)
    }
    #[doc = "Watchdog oscillator"]
    #[inline]
    pub fn wdtosc(self) -> &'a mut W {
        self.variant(SELW::WDTOSC)
    }
    #[doc = "FRO DIV"]
    #[inline]
    pub fn frodiv(self) -> &'a mut W {
        self.variant(SELW::FRODIV)
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
    #[doc = "Bits 0:1 - System PLL clock source"]
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
    #[doc = "Bits 0:1 - System PLL clock source"]
    #[inline]
    pub fn sel(&mut self) -> _SELW {
        _SELW { w: self }
    }
}
