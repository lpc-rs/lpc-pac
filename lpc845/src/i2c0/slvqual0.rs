#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::SLVQUAL0 {
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
#[doc = "Possible values of the field `QUALMODE0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum QUALMODE0R {
    #[doc = "Mask. The SLVQUAL0 field is used as a logical mask for matching address 0."]
    MASK,
    #[doc = "Extend. The SLVQUAL0 field is used to extend address 0 matching in a range of addresses."]
    EXTEND,
}
impl QUALMODE0R {
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
            QUALMODE0R::MASK => false,
            QUALMODE0R::EXTEND => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> QUALMODE0R {
        match value {
            false => QUALMODE0R::MASK,
            true => QUALMODE0R::EXTEND,
        }
    }
    #[doc = "Checks if the value of the field is `MASK`"]
    #[inline]
    pub fn is_mask(&self) -> bool {
        *self == QUALMODE0R::MASK
    }
    #[doc = "Checks if the value of the field is `EXTEND`"]
    #[inline]
    pub fn is_extend(&self) -> bool {
        *self == QUALMODE0R::EXTEND
    }
}
#[doc = r" Value of the field"]
pub struct SLVQUAL0R {
    bits: u8,
}
impl SLVQUAL0R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Values that can be written to the field `QUALMODE0`"]
pub enum QUALMODE0W {
    #[doc = "Mask. The SLVQUAL0 field is used as a logical mask for matching address 0."]
    MASK,
    #[doc = "Extend. The SLVQUAL0 field is used to extend address 0 matching in a range of addresses."]
    EXTEND,
}
impl QUALMODE0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            QUALMODE0W::MASK => false,
            QUALMODE0W::EXTEND => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _QUALMODE0W<'a> {
    w: &'a mut W,
}
impl<'a> _QUALMODE0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: QUALMODE0W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Mask. The SLVQUAL0 field is used as a logical mask for matching address 0."]
    #[inline]
    pub fn mask(self) -> &'a mut W {
        self.variant(QUALMODE0W::MASK)
    }
    #[doc = "Extend. The SLVQUAL0 field is used to extend address 0 matching in a range of addresses."]
    #[inline]
    pub fn extend(self) -> &'a mut W {
        self.variant(QUALMODE0W::EXTEND)
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
#[doc = r" Proxy"]
pub struct _SLVQUAL0W<'a> {
    w: &'a mut W,
}
impl<'a> _SLVQUAL0W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 127;
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
    #[doc = "Bit 0 - Qualify mode for slave address 0."]
    #[inline]
    pub fn qualmode0(&self) -> QUALMODE0R {
        QUALMODE0R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 1:7 - Slave address Qualifier for address 0. A value of 0 causes the address in SLVADR0 to be used as-is, assuming that it is enabled. If QUALMODE0 = 0, any bit in this field which is set to 1 will cause an automatic match of the corresponding bit of the received address when it is compared to the SLVADR0 register. If QUALMODE0 = 1, an address range is matched for address 0. This range extends from the value defined by SLVADR0 to the address defined by SLVQUAL0 (address matches when SLVADR0\\[7:1\\] <= received address <= SLVQUAL0\\[7:1\\])."]
    #[inline]
    pub fn slvqual0(&self) -> SLVQUAL0R {
        let bits = {
            const MASK: u8 = 127;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        SLVQUAL0R { bits }
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
    #[doc = "Bit 0 - Qualify mode for slave address 0."]
    #[inline]
    pub fn qualmode0(&mut self) -> _QUALMODE0W {
        _QUALMODE0W { w: self }
    }
    #[doc = "Bits 1:7 - Slave address Qualifier for address 0. A value of 0 causes the address in SLVADR0 to be used as-is, assuming that it is enabled. If QUALMODE0 = 0, any bit in this field which is set to 1 will cause an automatic match of the corresponding bit of the received address when it is compared to the SLVADR0 register. If QUALMODE0 = 1, an address range is matched for address 0. This range extends from the value defined by SLVADR0 to the address defined by SLVQUAL0 (address matches when SLVADR0\\[7:1\\] <= received address <= SLVQUAL0\\[7:1\\])."]
    #[inline]
    pub fn slvqual0(&mut self) -> _SLVQUAL0W {
        _SLVQUAL0W { w: self }
    }
}
