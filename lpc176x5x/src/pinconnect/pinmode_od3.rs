#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::PINMODE_OD3 {
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
#[doc = "Possible values of the field `P3_25OD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P3_25ODR {
    #[doc = "Normal. P3.25 pin is in the normal (not open drain) mode."]
    NORMAL,
    #[doc = "Open-drain. P3.25 pin is in the open drain mode."]
    OPEN_DRAIN,
}
impl P3_25ODR {
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
            P3_25ODR::NORMAL => false,
            P3_25ODR::OPEN_DRAIN => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> P3_25ODR {
        match value {
            false => P3_25ODR::NORMAL,
            true => P3_25ODR::OPEN_DRAIN,
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline]
    pub fn is_normal(&self) -> bool {
        *self == P3_25ODR::NORMAL
    }
    #[doc = "Checks if the value of the field is `OPEN_DRAIN`"]
    #[inline]
    pub fn is_open_drain(&self) -> bool {
        *self == P3_25ODR::OPEN_DRAIN
    }
}
#[doc = r" Value of the field"]
pub struct P3_26ODR {
    bits: bool,
}
impl P3_26ODR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
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
}
#[doc = "Values that can be written to the field `P3_25OD`"]
pub enum P3_25ODW {
    #[doc = "Normal. P3.25 pin is in the normal (not open drain) mode."]
    NORMAL,
    #[doc = "Open-drain. P3.25 pin is in the open drain mode."]
    OPEN_DRAIN,
}
impl P3_25ODW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            P3_25ODW::NORMAL => false,
            P3_25ODW::OPEN_DRAIN => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _P3_25ODW<'a> {
    w: &'a mut W,
}
impl<'a> _P3_25ODW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: P3_25ODW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Normal. P3.25 pin is in the normal (not open drain) mode."]
    #[inline]
    pub fn normal(self) -> &'a mut W {
        self.variant(P3_25ODW::NORMAL)
    }
    #[doc = "Open-drain. P3.25 pin is in the open drain mode."]
    #[inline]
    pub fn open_drain(self) -> &'a mut W {
        self.variant(P3_25ODW::OPEN_DRAIN)
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
        const OFFSET: u8 = 25;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _P3_26ODW<'a> {
    w: &'a mut W,
}
impl<'a> _P3_26ODW<'a> {
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
        const OFFSET: u8 = 26;
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
    #[doc = "Bit 25 - Port 3 pin 25 open drain mode control."]
    #[inline]
    pub fn p3_25od(&self) -> P3_25ODR {
        P3_25ODR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 25;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 26 - Port 3 pin 26 open drain mode control, see P3.25OD"]
    #[inline]
    pub fn p3_26od(&self) -> P3_26ODR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 26;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        P3_26ODR { bits }
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
    #[doc = "Bit 25 - Port 3 pin 25 open drain mode control."]
    #[inline]
    pub fn p3_25od(&mut self) -> _P3_25ODW {
        _P3_25ODW { w: self }
    }
    #[doc = "Bit 26 - Port 3 pin 26 open drain mode control, see P3.25OD"]
    #[inline]
    pub fn p3_26od(&mut self) -> _P3_26ODW {
        _P3_26ODW { w: self }
    }
}
