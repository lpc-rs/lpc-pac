#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::LAD {
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
#[doc = r" Value of the field"]
pub struct LADENR {
    bits: bool,
}
impl LADENR {
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
#[doc = r" Value of the field"]
pub struct LADSELR {
    bits: u8,
}
impl LADSELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `LADREF`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LADREFR {
    #[doc = "Supply pin VDD"]
    LADREF_0,
    #[doc = "VDDCMP pin"]
    LADREF_1,
}
impl LADREFR {
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
            LADREFR::LADREF_0 => false,
            LADREFR::LADREF_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> LADREFR {
        match value {
            false => LADREFR::LADREF_0,
            true => LADREFR::LADREF_1,
        }
    }
    #[doc = "Checks if the value of the field is `LADREF_0`"]
    #[inline]
    pub fn is_ladref_0(&self) -> bool {
        *self == LADREFR::LADREF_0
    }
    #[doc = "Checks if the value of the field is `LADREF_1`"]
    #[inline]
    pub fn is_ladref_1(&self) -> bool {
        *self == LADREFR::LADREF_1
    }
}
#[doc = r" Proxy"]
pub struct _LADENW<'a> {
    w: &'a mut W,
}
impl<'a> _LADENW<'a> {
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
pub struct _LADSELW<'a> {
    w: &'a mut W,
}
impl<'a> _LADSELW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 31;
        const OFFSET: u8 = 1;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `LADREF`"]
pub enum LADREFW {
    #[doc = "Supply pin VDD"]
    LADREF_0,
    #[doc = "VDDCMP pin"]
    LADREF_1,
}
impl LADREFW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            LADREFW::LADREF_0 => false,
            LADREFW::LADREF_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _LADREFW<'a> {
    w: &'a mut W,
}
impl<'a> _LADREFW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LADREFW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Supply pin VDD"]
    #[inline]
    pub fn ladref_0(self) -> &'a mut W {
        self.variant(LADREFW::LADREF_0)
    }
    #[doc = "VDDCMP pin"]
    #[inline]
    pub fn ladref_1(self) -> &'a mut W {
        self.variant(LADREFW::LADREF_1)
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
    #[doc = "Bit 0 - Voltage ladder enable"]
    #[inline]
    pub fn laden(&self) -> LADENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        LADENR { bits }
    }
    #[doc = "Bits 1:5 - Voltage ladder value. The reference voltage Vref depends on the LADREF bit below. 00000 = VSS 00001 = 1 x Vref/31 00010 = 2 x Vref/31 ... 11111 = Vref"]
    #[inline]
    pub fn ladsel(&self) -> LADSELR {
        let bits = {
            const MASK: u8 = 31;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        LADSELR { bits }
    }
    #[doc = "Bit 6 - Selects the reference voltage Vref for the voltage ladder."]
    #[inline]
    pub fn ladref(&self) -> LADREFR {
        LADREFR::_from({
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
    #[doc = "Bit 0 - Voltage ladder enable"]
    #[inline]
    pub fn laden(&mut self) -> _LADENW {
        _LADENW { w: self }
    }
    #[doc = "Bits 1:5 - Voltage ladder value. The reference voltage Vref depends on the LADREF bit below. 00000 = VSS 00001 = 1 x Vref/31 00010 = 2 x Vref/31 ... 11111 = Vref"]
    #[inline]
    pub fn ladsel(&mut self) -> _LADSELW {
        _LADSELW { w: self }
    }
    #[doc = "Bit 6 - Selects the reference voltage Vref for the voltage ladder."]
    #[inline]
    pub fn ladref(&mut self) -> _LADREFW {
        _LADREFW { w: self }
    }
}
