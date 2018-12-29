#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::SCS {
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
#[doc = "Possible values of the field `OSCRANGE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OSCRANGER {
    #[doc = "Low. The frequency range of the main oscillator is 1 MHz to 20 MHz."]
    LOW,
    #[doc = "High. The frequency range of the main oscillator is 15 MHz to 25 MHz."]
    HIGH,
}
impl OSCRANGER {
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
            OSCRANGER::LOW => false,
            OSCRANGER::HIGH => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> OSCRANGER {
        match value {
            false => OSCRANGER::LOW,
            true => OSCRANGER::HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline]
    pub fn is_low(&self) -> bool {
        *self == OSCRANGER::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline]
    pub fn is_high(&self) -> bool {
        *self == OSCRANGER::HIGH
    }
}
#[doc = "Possible values of the field `OSCEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OSCENR {
    #[doc = "Disabled. The main oscillator is disabled."]
    DISABLED,
    #[doc = "Enabled.The main oscillator is enabled, and will start up if the correct external circuitry is connected to the XTAL1 and XTAL2 pins."]
    ENABLED,
}
impl OSCENR {
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
            OSCENR::DISABLED => false,
            OSCENR::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> OSCENR {
        match value {
            false => OSCENR::DISABLED,
            true => OSCENR::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == OSCENR::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == OSCENR::ENABLED
    }
}
#[doc = "Possible values of the field `OSCSTAT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OSCSTATR {
    #[doc = "Not ready. The main oscillator is not ready to be used as a clock source."]
    NOT_READY,
    #[doc = "Ready. The main oscillator is ready to be used as a clock source. The main oscillator must be enabled via the OSCEN bit."]
    READY,
}
impl OSCSTATR {
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
            OSCSTATR::NOT_READY => false,
            OSCSTATR::READY => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> OSCSTATR {
        match value {
            false => OSCSTATR::NOT_READY,
            true => OSCSTATR::READY,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_READY`"]
    #[inline]
    pub fn is_not_ready(&self) -> bool {
        *self == OSCSTATR::NOT_READY
    }
    #[doc = "Checks if the value of the field is `READY`"]
    #[inline]
    pub fn is_ready(&self) -> bool {
        *self == OSCSTATR::READY
    }
}
#[doc = "Values that can be written to the field `OSCRANGE`"]
pub enum OSCRANGEW {
    #[doc = "Low. The frequency range of the main oscillator is 1 MHz to 20 MHz."]
    LOW,
    #[doc = "High. The frequency range of the main oscillator is 15 MHz to 25 MHz."]
    HIGH,
}
impl OSCRANGEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            OSCRANGEW::LOW => false,
            OSCRANGEW::HIGH => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _OSCRANGEW<'a> {
    w: &'a mut W,
}
impl<'a> _OSCRANGEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: OSCRANGEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Low. The frequency range of the main oscillator is 1 MHz to 20 MHz."]
    #[inline]
    pub fn low(self) -> &'a mut W {
        self.variant(OSCRANGEW::LOW)
    }
    #[doc = "High. The frequency range of the main oscillator is 15 MHz to 25 MHz."]
    #[inline]
    pub fn high(self) -> &'a mut W {
        self.variant(OSCRANGEW::HIGH)
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
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `OSCEN`"]
pub enum OSCENW {
    #[doc = "Disabled. The main oscillator is disabled."]
    DISABLED,
    #[doc = "Enabled.The main oscillator is enabled, and will start up if the correct external circuitry is connected to the XTAL1 and XTAL2 pins."]
    ENABLED,
}
impl OSCENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            OSCENW::DISABLED => false,
            OSCENW::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _OSCENW<'a> {
    w: &'a mut W,
}
impl<'a> _OSCENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: OSCENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disabled. The main oscillator is disabled."]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(OSCENW::DISABLED)
    }
    #[doc = "Enabled.The main oscillator is enabled, and will start up if the correct external circuitry is connected to the XTAL1 and XTAL2 pins."]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(OSCENW::ENABLED)
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
        const OFFSET: u8 = 5;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `OSCSTAT`"]
pub enum OSCSTATW {
    #[doc = "Not ready. The main oscillator is not ready to be used as a clock source."]
    NOT_READY,
    #[doc = "Ready. The main oscillator is ready to be used as a clock source. The main oscillator must be enabled via the OSCEN bit."]
    READY,
}
impl OSCSTATW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            OSCSTATW::NOT_READY => false,
            OSCSTATW::READY => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _OSCSTATW<'a> {
    w: &'a mut W,
}
impl<'a> _OSCSTATW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: OSCSTATW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Not ready. The main oscillator is not ready to be used as a clock source."]
    #[inline]
    pub fn not_ready(self) -> &'a mut W {
        self.variant(OSCSTATW::NOT_READY)
    }
    #[doc = "Ready. The main oscillator is ready to be used as a clock source. The main oscillator must be enabled via the OSCEN bit."]
    #[inline]
    pub fn ready(self) -> &'a mut W {
        self.variant(OSCSTATW::READY)
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
        const OFFSET: u8 = 6;
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
    #[doc = "Bit 4 - Main oscillator range select."]
    #[inline]
    pub fn oscrange(&self) -> OSCRANGER {
        OSCRANGER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - Main oscillator enable."]
    #[inline]
    pub fn oscen(&self) -> OSCENR {
        OSCENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 6 - Main oscillator status."]
    #[inline]
    pub fn oscstat(&self) -> OSCSTATR {
        OSCSTATR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
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
    #[doc = "Bit 4 - Main oscillator range select."]
    #[inline]
    pub fn oscrange(&mut self) -> _OSCRANGEW {
        _OSCRANGEW { w: self }
    }
    #[doc = "Bit 5 - Main oscillator enable."]
    #[inline]
    pub fn oscen(&mut self) -> _OSCENW {
        _OSCENW { w: self }
    }
    #[doc = "Bit 6 - Main oscillator status."]
    #[inline]
    pub fn oscstat(&mut self) -> _OSCSTATW {
        _OSCSTATW { w: self }
    }
}
