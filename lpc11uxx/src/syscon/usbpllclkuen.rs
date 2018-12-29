#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::USBPLLCLKUEN {
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
#[doc = "Possible values of the field `ENA`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENAR {
    #[doc = "No change"]
    NO_CHANGE,
    #[doc = "Update clock source"]
    UPDATE_CLOCK_SOURCE,
}
impl ENAR {
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
            ENAR::NO_CHANGE => false,
            ENAR::UPDATE_CLOCK_SOURCE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ENAR {
        match value {
            false => ENAR::NO_CHANGE,
            true => ENAR::UPDATE_CLOCK_SOURCE,
        }
    }
    #[doc = "Checks if the value of the field is `NO_CHANGE`"]
    #[inline]
    pub fn is_no_change(&self) -> bool {
        *self == ENAR::NO_CHANGE
    }
    #[doc = "Checks if the value of the field is `UPDATE_CLOCK_SOURCE`"]
    #[inline]
    pub fn is_update_clock_source(&self) -> bool {
        *self == ENAR::UPDATE_CLOCK_SOURCE
    }
}
#[doc = "Values that can be written to the field `ENA`"]
pub enum ENAW {
    #[doc = "No change"]
    NO_CHANGE,
    #[doc = "Update clock source"]
    UPDATE_CLOCK_SOURCE,
}
impl ENAW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ENAW::NO_CHANGE => false,
            ENAW::UPDATE_CLOCK_SOURCE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ENAW<'a> {
    w: &'a mut W,
}
impl<'a> _ENAW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ENAW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No change"]
    #[inline]
    pub fn no_change(self) -> &'a mut W {
        self.variant(ENAW::NO_CHANGE)
    }
    #[doc = "Update clock source"]
    #[inline]
    pub fn update_clock_source(self) -> &'a mut W {
        self.variant(ENAW::UPDATE_CLOCK_SOURCE)
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
    #[doc = "Bit 0 - Enable USB PLL clock source update"]
    #[inline]
    pub fn ena(&self) -> ENAR {
        ENAR::_from({
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
    #[doc = "Bit 0 - Enable USB PLL clock source update"]
    #[inline]
    pub fn ena(&mut self) -> _ENAW {
        _ENAW { w: self }
    }
}
