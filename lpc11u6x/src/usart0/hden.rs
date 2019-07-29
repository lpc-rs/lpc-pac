#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::HDEN {
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
#[doc = "Possible values of the field `HDEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HDENR {
    #[doc = "Disable half-duplex mode."]
    DISABLE_HALF_DUPLEX_,
    #[doc = "Enable half-duplex mode."]
    ENABLE_HALF_DUPLEX_M,
}
impl HDENR {
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
            HDENR::DISABLE_HALF_DUPLEX_ => false,
            HDENR::ENABLE_HALF_DUPLEX_M => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> HDENR {
        match value {
            false => HDENR::DISABLE_HALF_DUPLEX_,
            true => HDENR::ENABLE_HALF_DUPLEX_M,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE_HALF_DUPLEX_`"]
    #[inline]
    pub fn is_disable_half_duplex_(&self) -> bool {
        *self == HDENR::DISABLE_HALF_DUPLEX_
    }
    #[doc = "Checks if the value of the field is `ENABLE_HALF_DUPLEX_M`"]
    #[inline]
    pub fn is_enable_half_duplex_m(&self) -> bool {
        *self == HDENR::ENABLE_HALF_DUPLEX_M
    }
}
#[doc = "Values that can be written to the field `HDEN`"]
pub enum HDENW {
    #[doc = "Disable half-duplex mode."]
    DISABLE_HALF_DUPLEX_,
    #[doc = "Enable half-duplex mode."]
    ENABLE_HALF_DUPLEX_M,
}
impl HDENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            HDENW::DISABLE_HALF_DUPLEX_ => false,
            HDENW::ENABLE_HALF_DUPLEX_M => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _HDENW<'a> {
    w: &'a mut W,
}
impl<'a> _HDENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: HDENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable half-duplex mode."]
    #[inline]
    pub fn disable_half_duplex_(self) -> &'a mut W {
        self.variant(HDENW::DISABLE_HALF_DUPLEX_)
    }
    #[doc = "Enable half-duplex mode."]
    #[inline]
    pub fn enable_half_duplex_m(self) -> &'a mut W {
        self.variant(HDENW::ENABLE_HALF_DUPLEX_M)
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Half-duplex mode enable"]
    #[inline]
    pub fn hden(&self) -> HDENR {
        HDENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
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
    #[doc = "Bit 0 - Half-duplex mode enable"]
    #[inline]
    pub fn hden(&mut self) -> _HDENW {
        _HDENW { w: self }
    }
}
