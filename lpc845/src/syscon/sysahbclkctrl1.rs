#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::SYSAHBCLKCTRL1 {
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
#[doc = "Possible values of the field `CAPT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CAPTR {
    #[doc = "disable"]
    DISABLE,
    #[doc = "enable"]
    ENABLE,
}
impl CAPTR {
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
            CAPTR::DISABLE => false,
            CAPTR::ENABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CAPTR {
        match value {
            false => CAPTR::DISABLE,
            true => CAPTR::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == CAPTR::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == CAPTR::ENABLE
    }
}
#[doc = "Possible values of the field `DAC1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DAC1R {
    #[doc = "disable"]
    DISABLE,
    #[doc = "enable"]
    ENABLE,
}
impl DAC1R {
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
            DAC1R::DISABLE => false,
            DAC1R::ENABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DAC1R {
        match value {
            false => DAC1R::DISABLE,
            true => DAC1R::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == DAC1R::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == DAC1R::ENABLE
    }
}
#[doc = "Values that can be written to the field `CAPT`"]
pub enum CAPTW {
    #[doc = "disable"]
    DISABLE,
    #[doc = "enable"]
    ENABLE,
}
impl CAPTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CAPTW::DISABLE => false,
            CAPTW::ENABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CAPTW<'a> {
    w: &'a mut W,
}
impl<'a> _CAPTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CAPTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "disable"]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(CAPTW::DISABLE)
    }
    #[doc = "enable"]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(CAPTW::ENABLE)
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
#[doc = "Values that can be written to the field `DAC1`"]
pub enum DAC1W {
    #[doc = "disable"]
    DISABLE,
    #[doc = "enable"]
    ENABLE,
}
impl DAC1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DAC1W::DISABLE => false,
            DAC1W::ENABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DAC1W<'a> {
    w: &'a mut W,
}
impl<'a> _DAC1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DAC1W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "disable"]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(DAC1W::DISABLE)
    }
    #[doc = "enable"]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(DAC1W::ENABLE)
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
    #[doc = "Bit 0 - Enables clock for CAPT."]
    #[inline]
    pub fn capt(&self) -> CAPTR {
        CAPTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Enables clock for DAC1."]
    #[inline]
    pub fn dac1(&self) -> DAC1R {
        DAC1R::_from({
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
    #[doc = "Bit 0 - Enables clock for CAPT."]
    #[inline]
    pub fn capt(&mut self) -> _CAPTW {
        _CAPTW { w: self }
    }
    #[doc = "Bit 1 - Enables clock for DAC1."]
    #[inline]
    pub fn dac1(&mut self) -> _DAC1W {
        _DAC1W { w: self }
    }
}
