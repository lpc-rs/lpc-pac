#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::MAINCLKUEN {
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
    #[doc = "no change"]
    NO_CHANGE,
    #[doc = "update clock source"]
    UPDATED,
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
            ENAR::UPDATED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ENAR {
        match value {
            false => ENAR::NO_CHANGE,
            true => ENAR::UPDATED,
        }
    }
    #[doc = "Checks if the value of the field is `NO_CHANGE`"]
    #[inline]
    pub fn is_no_change(&self) -> bool {
        *self == ENAR::NO_CHANGE
    }
    #[doc = "Checks if the value of the field is `UPDATED`"]
    #[inline]
    pub fn is_updated(&self) -> bool {
        *self == ENAR::UPDATED
    }
}
#[doc = "Values that can be written to the field `ENA`"]
pub enum ENAW {
    #[doc = "no change"]
    NO_CHANGE,
    #[doc = "update clock source"]
    UPDATED,
}
impl ENAW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ENAW::NO_CHANGE => false,
            ENAW::UPDATED => true,
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
    #[doc = "no change"]
    #[inline]
    pub fn no_change(self) -> &'a mut W {
        self.variant(ENAW::NO_CHANGE)
    }
    #[doc = "update clock source"]
    #[inline]
    pub fn updated(self) -> &'a mut W {
        self.variant(ENAW::UPDATED)
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
    #[doc = "Bit 0 - Enable main clock source update"]
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
    #[doc = "Bit 0 - Enable main clock source update"]
    #[inline]
    pub fn ena(&mut self) -> _ENAW {
        _ENAW { w: self }
    }
}
