#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::USBCLKCTRL {
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
#[doc = "Possible values of the field `AP_CLK`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AP_CLKR {
    #[doc = "Hardware. Under hardware control."]
    HARDWARE,
    #[doc = "Forced. Forced HIGH."]
    FORCED,
}
impl crate::ToBits<bool> for AP_CLKR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            AP_CLKR::HARDWARE => false,
            AP_CLKR::FORCED => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type AP_CLK_R = crate::FR<bool, AP_CLKR>;
impl AP_CLK_R {
    #[doc = "Checks if the value of the field is `HARDWARE`"]
    #[inline(always)]
    pub fn is_hardware(&self) -> bool {
        *self == AP_CLKR::HARDWARE
    }
    #[doc = "Checks if the value of the field is `FORCED`"]
    #[inline(always)]
    pub fn is_forced(&self) -> bool {
        *self == AP_CLKR::FORCED
    }
}
#[doc = "Values that can be written to the field `AP_CLK`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AP_CLKW {
    #[doc = "Hardware. Under hardware control."]
    HARDWARE,
    #[doc = "Forced. Forced HIGH."]
    FORCED,
}
impl AP_CLKW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            AP_CLKW::HARDWARE => false,
            AP_CLKW::FORCED => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _AP_CLKW<'a> {
    w: &'a mut W,
}
impl<'a> _AP_CLKW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AP_CLKW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Hardware. Under hardware control."]
    #[inline(always)]
    pub fn hardware(self) -> &'a mut W {
        self.variant(AP_CLKW::HARDWARE)
    }
    #[doc = "Forced. Forced HIGH."]
    #[inline(always)]
    pub fn forced(self) -> &'a mut W {
        self.variant(AP_CLKW::FORCED)
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
#[doc = "Possible values of the field `POL_CLK`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum POL_CLKR {
    #[doc = "Falling edge. Falling edge of the USB need_clock triggers the USB wake-up (default)."]
    FALLING_EDGE,
    #[doc = "Rising edge. Rising edge of the USB need_clock triggers the USB wake-up."]
    RISING_EDGE,
}
impl crate::ToBits<bool> for POL_CLKR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            POL_CLKR::FALLING_EDGE => false,
            POL_CLKR::RISING_EDGE => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type POL_CLK_R = crate::FR<bool, POL_CLKR>;
impl POL_CLK_R {
    #[doc = "Checks if the value of the field is `FALLING_EDGE`"]
    #[inline(always)]
    pub fn is_falling_edge(&self) -> bool {
        *self == POL_CLKR::FALLING_EDGE
    }
    #[doc = "Checks if the value of the field is `RISING_EDGE`"]
    #[inline(always)]
    pub fn is_rising_edge(&self) -> bool {
        *self == POL_CLKR::RISING_EDGE
    }
}
#[doc = "Values that can be written to the field `POL_CLK`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum POL_CLKW {
    #[doc = "Falling edge. Falling edge of the USB need_clock triggers the USB wake-up (default)."]
    FALLING_EDGE,
    #[doc = "Rising edge. Rising edge of the USB need_clock triggers the USB wake-up."]
    RISING_EDGE,
}
impl POL_CLKW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            POL_CLKW::FALLING_EDGE => false,
            POL_CLKW::RISING_EDGE => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _POL_CLKW<'a> {
    w: &'a mut W,
}
impl<'a> _POL_CLKW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: POL_CLKW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Falling edge. Falling edge of the USB need_clock triggers the USB wake-up (default)."]
    #[inline(always)]
    pub fn falling_edge(self) -> &'a mut W {
        self.variant(POL_CLKW::FALLING_EDGE)
    }
    #[doc = "Rising edge. Rising edge of the USB need_clock triggers the USB wake-up."]
    #[inline(always)]
    pub fn rising_edge(self) -> &'a mut W {
        self.variant(POL_CLKW::RISING_EDGE)
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
    #[doc = "Bit 0 - USB need_clock signal control"]
    #[inline(always)]
    pub fn ap_clk(&self) -> AP_CLK_R {
        AP_CLK_R::new((self.bits() & 0x01) != 0)
    }
    #[doc = "Bit 1 - USB need_clock polarity for triggering the USB wake-up interrupt"]
    #[inline(always)]
    pub fn pol_clk(&self) -> POL_CLK_R {
        POL_CLK_R::new(((self.bits() >> 1) & 0x01) != 0)
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - USB need_clock signal control"]
    #[inline(always)]
    pub fn ap_clk(&mut self) -> _AP_CLKW {
        _AP_CLKW { w: self }
    }
    #[doc = "Bit 1 - USB need_clock polarity for triggering the USB wake-up interrupt"]
    #[inline(always)]
    pub fn pol_clk(&mut self) -> _POL_CLKW {
        _POL_CLKW { w: self }
    }
}
