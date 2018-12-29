#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::USBCLKSEL {
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
    #[doc = "USB PLL out"]
    USB_PLL_OUT,
    #[doc = "Main clock"]
    MAIN_CLOCK,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl SELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            SELR::USB_PLL_OUT => 0,
            SELR::MAIN_CLOCK => 1,
            SELR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> SELR {
        match value {
            0 => SELR::USB_PLL_OUT,
            1 => SELR::MAIN_CLOCK,
            i => SELR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `USB_PLL_OUT`"]
    #[inline]
    pub fn is_usb_pll_out(&self) -> bool {
        *self == SELR::USB_PLL_OUT
    }
    #[doc = "Checks if the value of the field is `MAIN_CLOCK`"]
    #[inline]
    pub fn is_main_clock(&self) -> bool {
        *self == SELR::MAIN_CLOCK
    }
}
#[doc = "Values that can be written to the field `SEL`"]
pub enum SELW {
    #[doc = "USB PLL out"]
    USB_PLL_OUT,
    #[doc = "Main clock"]
    MAIN_CLOCK,
}
impl SELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            SELW::USB_PLL_OUT => 0,
            SELW::MAIN_CLOCK => 1,
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
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "USB PLL out"]
    #[inline]
    pub fn usb_pll_out(self) -> &'a mut W {
        self.variant(SELW::USB_PLL_OUT)
    }
    #[doc = "Main clock"]
    #[inline]
    pub fn main_clock(self) -> &'a mut W {
        self.variant(SELW::MAIN_CLOCK)
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
    #[doc = "Bits 0:1 - USB clock source. Values 0x2 and 0x3 are reserved."]
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
    #[doc = "Bits 0:1 - USB clock source. Values 0x2 and 0x3 are reserved."]
    #[inline]
    pub fn sel(&mut self) -> _SELW {
        _SELW { w: self }
    }
}
