#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::FROOSCCTRL {
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
#[doc = "Possible values of the field `FRO_DIRECT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FRO_DIRECTR {
    #[doc = "fro clock is divider by 2 or 16,depend on FAIM slow boot value"]
    DISABLED,
    #[doc = "fro clock is direct from FRO oscillator"]
    ENABLED,
}
impl FRO_DIRECTR {
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
            FRO_DIRECTR::DISABLED => false,
            FRO_DIRECTR::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FRO_DIRECTR {
        match value {
            false => FRO_DIRECTR::DISABLED,
            true => FRO_DIRECTR::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == FRO_DIRECTR::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == FRO_DIRECTR::ENABLED
    }
}
#[doc = "Values that can be written to the field `FRO_DIRECT`"]
pub enum FRO_DIRECTW {
    #[doc = "fro clock is divider by 2 or 16,depend on FAIM slow boot value"]
    DISABLED,
    #[doc = "fro clock is direct from FRO oscillator"]
    ENABLED,
}
impl FRO_DIRECTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            FRO_DIRECTW::DISABLED => false,
            FRO_DIRECTW::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FRO_DIRECTW<'a> {
    w: &'a mut W,
}
impl<'a> _FRO_DIRECTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FRO_DIRECTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "fro clock is divider by 2 or 16,depend on FAIM slow boot value"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(FRO_DIRECTW::DISABLED)
    }
    #[doc = "fro clock is direct from FRO oscillator"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(FRO_DIRECTW::ENABLED)
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
        const OFFSET: u8 = 17;
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
    #[doc = "Bit 17 - fro direct clock select"]
    #[inline]
    pub fn fro_direct(&self) -> FRO_DIRECTR {
        FRO_DIRECTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 17;
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
    #[doc = "Bit 17 - fro direct clock select"]
    #[inline]
    pub fn fro_direct(&mut self) -> _FRO_DIRECTW {
        _FRO_DIRECTW { w: self }
    }
}
