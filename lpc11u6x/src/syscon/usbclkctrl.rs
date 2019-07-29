#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::USBCLKCTRL {
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
#[doc = "Possible values of the field `AP_CLK`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AP_CLKR {
    #[doc = "Hardware. Under hardware control."]
    HARDWARE,
    #[doc = "Forced. Forced HIGH."]
    FORCED,
}
impl AP_CLKR {
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
            AP_CLKR::HARDWARE => false,
            AP_CLKR::FORCED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> AP_CLKR {
        match value {
            false => AP_CLKR::HARDWARE,
            true => AP_CLKR::FORCED,
        }
    }
    #[doc = "Checks if the value of the field is `HARDWARE`"]
    #[inline]
    pub fn is_hardware(&self) -> bool {
        *self == AP_CLKR::HARDWARE
    }
    #[doc = "Checks if the value of the field is `FORCED`"]
    #[inline]
    pub fn is_forced(&self) -> bool {
        *self == AP_CLKR::FORCED
    }
}
#[doc = "Possible values of the field `POL_CLK`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum POL_CLKR {
    #[doc = "Falling edge. Falling edge of the USB need_clock triggers the USB wake-up (default)."]
    FALLING_EDGE,
    #[doc = "Rising edge. Rising edge of the USB need_clock triggers the USB wake-up."]
    RISING_EDGE,
}
impl POL_CLKR {
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
            POL_CLKR::FALLING_EDGE => false,
            POL_CLKR::RISING_EDGE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> POL_CLKR {
        match value {
            false => POL_CLKR::FALLING_EDGE,
            true => POL_CLKR::RISING_EDGE,
        }
    }
    #[doc = "Checks if the value of the field is `FALLING_EDGE`"]
    #[inline]
    pub fn is_falling_edge(&self) -> bool {
        *self == POL_CLKR::FALLING_EDGE
    }
    #[doc = "Checks if the value of the field is `RISING_EDGE`"]
    #[inline]
    pub fn is_rising_edge(&self) -> bool {
        *self == POL_CLKR::RISING_EDGE
    }
}
#[doc = "Values that can be written to the field `AP_CLK`"]
pub enum AP_CLKW {
    #[doc = "Hardware. Under hardware control."]
    HARDWARE,
    #[doc = "Forced. Forced HIGH."]
    FORCED,
}
impl AP_CLKW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            AP_CLKW::HARDWARE => false,
            AP_CLKW::FORCED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _AP_CLKW<'a> {
    w: &'a mut W,
}
impl<'a> _AP_CLKW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: AP_CLKW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Hardware. Under hardware control."]
    #[inline]
    pub fn hardware(self) -> &'a mut W {
        self.variant(AP_CLKW::HARDWARE)
    }
    #[doc = "Forced. Forced HIGH."]
    #[inline]
    pub fn forced(self) -> &'a mut W {
        self.variant(AP_CLKW::FORCED)
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
#[doc = "Values that can be written to the field `POL_CLK`"]
pub enum POL_CLKW {
    #[doc = "Falling edge. Falling edge of the USB need_clock triggers the USB wake-up (default)."]
    FALLING_EDGE,
    #[doc = "Rising edge. Rising edge of the USB need_clock triggers the USB wake-up."]
    RISING_EDGE,
}
impl POL_CLKW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            POL_CLKW::FALLING_EDGE => false,
            POL_CLKW::RISING_EDGE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _POL_CLKW<'a> {
    w: &'a mut W,
}
impl<'a> _POL_CLKW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: POL_CLKW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Falling edge. Falling edge of the USB need_clock triggers the USB wake-up (default)."]
    #[inline]
    pub fn falling_edge(self) -> &'a mut W {
        self.variant(POL_CLKW::FALLING_EDGE)
    }
    #[doc = "Rising edge. Rising edge of the USB need_clock triggers the USB wake-up."]
    #[inline]
    pub fn rising_edge(self) -> &'a mut W {
        self.variant(POL_CLKW::RISING_EDGE)
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
    #[doc = "Bit 0 - USB need_clock signal control"]
    #[inline]
    pub fn ap_clk(&self) -> AP_CLKR {
        AP_CLKR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - USB need_clock polarity for triggering the USB wake-up interrupt"]
    #[inline]
    pub fn pol_clk(&self) -> POL_CLKR {
        POL_CLKR::_from({
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
    #[doc = "Bit 0 - USB need_clock signal control"]
    #[inline]
    pub fn ap_clk(&mut self) -> _AP_CLKW {
        _AP_CLKW { w: self }
    }
    #[doc = "Bit 1 - USB need_clock polarity for triggering the USB wake-up interrupt"]
    #[inline]
    pub fn pol_clk(&mut self) -> _POL_CLKW {
        _POL_CLKW { w: self }
    }
}
