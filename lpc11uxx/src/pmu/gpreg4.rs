#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::GPREG4 {
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
#[doc = "Possible values of the field `WAKEUPHYS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WAKEUPHYSR {
    #[doc = "Hysteresis for WAKEUP pin disabled."]
    DISABLED,
    #[doc = "Hysteresis for WAKEUP pin enabled."]
    ENABLED,
}
impl WAKEUPHYSR {
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
            WAKEUPHYSR::DISABLED => false,
            WAKEUPHYSR::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> WAKEUPHYSR {
        match value {
            false => WAKEUPHYSR::DISABLED,
            true => WAKEUPHYSR::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == WAKEUPHYSR::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == WAKEUPHYSR::ENABLED
    }
}
#[doc = r" Value of the field"]
pub struct GPDATAR {
    bits: u32,
}
impl GPDATAR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
}
#[doc = "Values that can be written to the field `WAKEUPHYS`"]
pub enum WAKEUPHYSW {
    #[doc = "Hysteresis for WAKEUP pin disabled."]
    DISABLED,
    #[doc = "Hysteresis for WAKEUP pin enabled."]
    ENABLED,
}
impl WAKEUPHYSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            WAKEUPHYSW::DISABLED => false,
            WAKEUPHYSW::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _WAKEUPHYSW<'a> {
    w: &'a mut W,
}
impl<'a> _WAKEUPHYSW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: WAKEUPHYSW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Hysteresis for WAKEUP pin disabled."]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(WAKEUPHYSW::DISABLED)
    }
    #[doc = "Hysteresis for WAKEUP pin enabled."]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(WAKEUPHYSW::ENABLED)
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
        const OFFSET: u8 = 10;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _GPDATAW<'a> {
    w: &'a mut W,
}
impl<'a> _GPDATAW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        const MASK: u32 = 2097151;
        const OFFSET: u8 = 11;
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
    #[doc = "Bit 10 - WAKEUP pin hysteresis enable"]
    #[inline]
    pub fn wakeuphys(&self) -> WAKEUPHYSR {
        WAKEUPHYSR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 11:31 - Data retained during Deep power-down mode."]
    #[inline]
    pub fn gpdata(&self) -> GPDATAR {
        let bits = {
            const MASK: u32 = 2097151;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) as u32
        };
        GPDATAR { bits }
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
    #[doc = "Bit 10 - WAKEUP pin hysteresis enable"]
    #[inline]
    pub fn wakeuphys(&mut self) -> _WAKEUPHYSW {
        _WAKEUPHYSW { w: self }
    }
    #[doc = "Bits 11:31 - Data retained during Deep power-down mode."]
    #[inline]
    pub fn gpdata(&mut self) -> _GPDATAW {
        _GPDATAW { w: self }
    }
}
