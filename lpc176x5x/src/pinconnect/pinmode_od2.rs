#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::PINMODE_OD2 {
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
#[doc = "Possible values of the field `P2_00OD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P2_00ODR {
    #[doc = "Normal. P2.0 pin is in the normal (not open drain) mode."]
    NORMAL,
    #[doc = "Open-drain. P2.0 pin is in the open drain mode."]
    OPEN_DRAIN,
}
impl P2_00ODR {
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
            P2_00ODR::NORMAL => false,
            P2_00ODR::OPEN_DRAIN => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> P2_00ODR {
        match value {
            false => P2_00ODR::NORMAL,
            true => P2_00ODR::OPEN_DRAIN,
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline]
    pub fn is_normal(&self) -> bool {
        *self == P2_00ODR::NORMAL
    }
    #[doc = "Checks if the value of the field is `OPEN_DRAIN`"]
    #[inline]
    pub fn is_open_drain(&self) -> bool {
        *self == P2_00ODR::OPEN_DRAIN
    }
}
#[doc = "Possible values of the field `P2_01OD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P2_01ODR {
    #[doc = "Normal. P2.1 pin is in the normal (not open drain) mode."]
    NORMAL,
    #[doc = "Open-drain. P2.1p in is in the open drain mode."]
    OPEN_DRAIN,
}
impl P2_01ODR {
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
            P2_01ODR::NORMAL => false,
            P2_01ODR::OPEN_DRAIN => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> P2_01ODR {
        match value {
            false => P2_01ODR::NORMAL,
            true => P2_01ODR::OPEN_DRAIN,
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline]
    pub fn is_normal(&self) -> bool {
        *self == P2_01ODR::NORMAL
    }
    #[doc = "Checks if the value of the field is `OPEN_DRAIN`"]
    #[inline]
    pub fn is_open_drain(&self) -> bool {
        *self == P2_01ODR::OPEN_DRAIN
    }
}
#[doc = "Possible values of the field `P2_02OD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P2_02ODR {
    #[doc = "Normal. P2.2 pin is in the normal (not open drain) mode."]
    NORMAL,
    #[doc = "Open-drain. P2.2 pin is in the open drain mode."]
    OPEN_DRAIN,
}
impl P2_02ODR {
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
            P2_02ODR::NORMAL => false,
            P2_02ODR::OPEN_DRAIN => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> P2_02ODR {
        match value {
            false => P2_02ODR::NORMAL,
            true => P2_02ODR::OPEN_DRAIN,
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline]
    pub fn is_normal(&self) -> bool {
        *self == P2_02ODR::NORMAL
    }
    #[doc = "Checks if the value of the field is `OPEN_DRAIN`"]
    #[inline]
    pub fn is_open_drain(&self) -> bool {
        *self == P2_02ODR::OPEN_DRAIN
    }
}
#[doc = "Possible values of the field `P2_03OD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P2_03ODR {
    #[doc = "Normal. P2.3 pin is in the normal (not open drain) mode."]
    NORMAL,
    #[doc = "Open-drain. P2.3 pin is in the open drain mode."]
    OPEN_DRAIN,
}
impl P2_03ODR {
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
            P2_03ODR::NORMAL => false,
            P2_03ODR::OPEN_DRAIN => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> P2_03ODR {
        match value {
            false => P2_03ODR::NORMAL,
            true => P2_03ODR::OPEN_DRAIN,
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline]
    pub fn is_normal(&self) -> bool {
        *self == P2_03ODR::NORMAL
    }
    #[doc = "Checks if the value of the field is `OPEN_DRAIN`"]
    #[inline]
    pub fn is_open_drain(&self) -> bool {
        *self == P2_03ODR::OPEN_DRAIN
    }
}
#[doc = "Possible values of the field `P2_04OD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P2_04ODR {
    #[doc = "Normal. P2.4 pin is in the normal (not open drain) mode."]
    NORMAL,
    #[doc = "Open-drain. P2.4 pin is in the open drain mode."]
    OPEN_DRAIN,
}
impl P2_04ODR {
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
            P2_04ODR::NORMAL => false,
            P2_04ODR::OPEN_DRAIN => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> P2_04ODR {
        match value {
            false => P2_04ODR::NORMAL,
            true => P2_04ODR::OPEN_DRAIN,
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline]
    pub fn is_normal(&self) -> bool {
        *self == P2_04ODR::NORMAL
    }
    #[doc = "Checks if the value of the field is `OPEN_DRAIN`"]
    #[inline]
    pub fn is_open_drain(&self) -> bool {
        *self == P2_04ODR::OPEN_DRAIN
    }
}
#[doc = "Possible values of the field `P2_05OD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P2_05ODR {
    #[doc = "Normal. P2.5 pin is in the normal (not open drain) mode."]
    NORMAL,
    #[doc = "Open-drain. P2.5 pin is in the open drain mode."]
    OPEN_DRAIN,
}
impl P2_05ODR {
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
            P2_05ODR::NORMAL => false,
            P2_05ODR::OPEN_DRAIN => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> P2_05ODR {
        match value {
            false => P2_05ODR::NORMAL,
            true => P2_05ODR::OPEN_DRAIN,
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline]
    pub fn is_normal(&self) -> bool {
        *self == P2_05ODR::NORMAL
    }
    #[doc = "Checks if the value of the field is `OPEN_DRAIN`"]
    #[inline]
    pub fn is_open_drain(&self) -> bool {
        *self == P2_05ODR::OPEN_DRAIN
    }
}
#[doc = "Possible values of the field `P2_06OD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P2_06ODR {
    #[doc = "Normal. P2.6 pin is in the normal (not open drain) mode."]
    NORMAL,
    #[doc = "Open-drain. P2.6 pin is in the open drain mode."]
    OPEN_DRAIN,
}
impl P2_06ODR {
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
            P2_06ODR::NORMAL => false,
            P2_06ODR::OPEN_DRAIN => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> P2_06ODR {
        match value {
            false => P2_06ODR::NORMAL,
            true => P2_06ODR::OPEN_DRAIN,
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline]
    pub fn is_normal(&self) -> bool {
        *self == P2_06ODR::NORMAL
    }
    #[doc = "Checks if the value of the field is `OPEN_DRAIN`"]
    #[inline]
    pub fn is_open_drain(&self) -> bool {
        *self == P2_06ODR::OPEN_DRAIN
    }
}
#[doc = "Possible values of the field `P2_07OD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P2_07ODR {
    #[doc = "Normal. P2.7 pin is in the normal (not open drain) mode."]
    NORMAL,
    #[doc = "Open-drain. P2.7 pin is in the open drain mode."]
    OPEN_DRAIN,
}
impl P2_07ODR {
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
            P2_07ODR::NORMAL => false,
            P2_07ODR::OPEN_DRAIN => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> P2_07ODR {
        match value {
            false => P2_07ODR::NORMAL,
            true => P2_07ODR::OPEN_DRAIN,
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline]
    pub fn is_normal(&self) -> bool {
        *self == P2_07ODR::NORMAL
    }
    #[doc = "Checks if the value of the field is `OPEN_DRAIN`"]
    #[inline]
    pub fn is_open_drain(&self) -> bool {
        *self == P2_07ODR::OPEN_DRAIN
    }
}
#[doc = "Possible values of the field `P2_08OD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P2_08ODR {
    #[doc = "Normal. P2.8 pin is in the normal (not open drain) mode."]
    NORMAL,
    #[doc = "Open-drain. P2.8 pin is in the open drain mode."]
    OPEN_DRAIN,
}
impl P2_08ODR {
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
            P2_08ODR::NORMAL => false,
            P2_08ODR::OPEN_DRAIN => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> P2_08ODR {
        match value {
            false => P2_08ODR::NORMAL,
            true => P2_08ODR::OPEN_DRAIN,
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline]
    pub fn is_normal(&self) -> bool {
        *self == P2_08ODR::NORMAL
    }
    #[doc = "Checks if the value of the field is `OPEN_DRAIN`"]
    #[inline]
    pub fn is_open_drain(&self) -> bool {
        *self == P2_08ODR::OPEN_DRAIN
    }
}
#[doc = "Possible values of the field `P2_09OD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P2_09ODR {
    #[doc = "Normal. P2.9 pin is in the normal (not open drain) mode."]
    NORMAL,
    #[doc = "Open-drain. P2.9 pin is in the open drain mode."]
    OPEN_DRAIN,
}
impl P2_09ODR {
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
            P2_09ODR::NORMAL => false,
            P2_09ODR::OPEN_DRAIN => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> P2_09ODR {
        match value {
            false => P2_09ODR::NORMAL,
            true => P2_09ODR::OPEN_DRAIN,
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline]
    pub fn is_normal(&self) -> bool {
        *self == P2_09ODR::NORMAL
    }
    #[doc = "Checks if the value of the field is `OPEN_DRAIN`"]
    #[inline]
    pub fn is_open_drain(&self) -> bool {
        *self == P2_09ODR::OPEN_DRAIN
    }
}
#[doc = "Possible values of the field `P2_10OD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P2_10ODR {
    #[doc = "Normal. P2.10 pin is in the normal (not open drain) mode."]
    NORMAL,
    #[doc = "Open-drain. P2.10 pin is in the open drain mode."]
    OPEN_DRAIN,
}
impl P2_10ODR {
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
            P2_10ODR::NORMAL => false,
            P2_10ODR::OPEN_DRAIN => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> P2_10ODR {
        match value {
            false => P2_10ODR::NORMAL,
            true => P2_10ODR::OPEN_DRAIN,
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline]
    pub fn is_normal(&self) -> bool {
        *self == P2_10ODR::NORMAL
    }
    #[doc = "Checks if the value of the field is `OPEN_DRAIN`"]
    #[inline]
    pub fn is_open_drain(&self) -> bool {
        *self == P2_10ODR::OPEN_DRAIN
    }
}
#[doc = "Possible values of the field `P2_11OD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P2_11ODR {
    #[doc = "Normal. P2.11 pin is in the normal (not open drain) mode."]
    NORMAL,
    #[doc = "Open-drain. P2.11 pin is in the open drain mode."]
    OPEN_DRAIN,
}
impl P2_11ODR {
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
            P2_11ODR::NORMAL => false,
            P2_11ODR::OPEN_DRAIN => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> P2_11ODR {
        match value {
            false => P2_11ODR::NORMAL,
            true => P2_11ODR::OPEN_DRAIN,
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline]
    pub fn is_normal(&self) -> bool {
        *self == P2_11ODR::NORMAL
    }
    #[doc = "Checks if the value of the field is `OPEN_DRAIN`"]
    #[inline]
    pub fn is_open_drain(&self) -> bool {
        *self == P2_11ODR::OPEN_DRAIN
    }
}
#[doc = "Possible values of the field `P2_12OD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P2_12ODR {
    #[doc = "Normal. P2.12 pin is in the normal (not open drain) mode."]
    NORMAL,
    #[doc = "Open-drain. P2.12 pin is in the open drain mode."]
    OPEN_DRAIN,
}
impl P2_12ODR {
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
            P2_12ODR::NORMAL => false,
            P2_12ODR::OPEN_DRAIN => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> P2_12ODR {
        match value {
            false => P2_12ODR::NORMAL,
            true => P2_12ODR::OPEN_DRAIN,
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline]
    pub fn is_normal(&self) -> bool {
        *self == P2_12ODR::NORMAL
    }
    #[doc = "Checks if the value of the field is `OPEN_DRAIN`"]
    #[inline]
    pub fn is_open_drain(&self) -> bool {
        *self == P2_12ODR::OPEN_DRAIN
    }
}
#[doc = "Possible values of the field `P2_13OD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P2_13ODR {
    #[doc = "Normal. P2.13 pin is in the normal (not open drain) mode."]
    NORMAL,
    #[doc = "Open-drain. P2.13 pin is in the open drain mode."]
    OPEN_DRAIN,
}
impl P2_13ODR {
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
            P2_13ODR::NORMAL => false,
            P2_13ODR::OPEN_DRAIN => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> P2_13ODR {
        match value {
            false => P2_13ODR::NORMAL,
            true => P2_13ODR::OPEN_DRAIN,
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline]
    pub fn is_normal(&self) -> bool {
        *self == P2_13ODR::NORMAL
    }
    #[doc = "Checks if the value of the field is `OPEN_DRAIN`"]
    #[inline]
    pub fn is_open_drain(&self) -> bool {
        *self == P2_13ODR::OPEN_DRAIN
    }
}
#[doc = "Values that can be written to the field `P2_00OD`"]
pub enum P2_00ODW {
    #[doc = "Normal. P2.0 pin is in the normal (not open drain) mode."]
    NORMAL,
    #[doc = "Open-drain. P2.0 pin is in the open drain mode."]
    OPEN_DRAIN,
}
impl P2_00ODW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            P2_00ODW::NORMAL => false,
            P2_00ODW::OPEN_DRAIN => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _P2_00ODW<'a> {
    w: &'a mut W,
}
impl<'a> _P2_00ODW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: P2_00ODW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Normal. P2.0 pin is in the normal (not open drain) mode."]
    #[inline]
    pub fn normal(self) -> &'a mut W {
        self.variant(P2_00ODW::NORMAL)
    }
    #[doc = "Open-drain. P2.0 pin is in the open drain mode."]
    #[inline]
    pub fn open_drain(self) -> &'a mut W {
        self.variant(P2_00ODW::OPEN_DRAIN)
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
#[doc = "Values that can be written to the field `P2_01OD`"]
pub enum P2_01ODW {
    #[doc = "Normal. P2.1 pin is in the normal (not open drain) mode."]
    NORMAL,
    #[doc = "Open-drain. P2.1p in is in the open drain mode."]
    OPEN_DRAIN,
}
impl P2_01ODW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            P2_01ODW::NORMAL => false,
            P2_01ODW::OPEN_DRAIN => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _P2_01ODW<'a> {
    w: &'a mut W,
}
impl<'a> _P2_01ODW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: P2_01ODW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Normal. P2.1 pin is in the normal (not open drain) mode."]
    #[inline]
    pub fn normal(self) -> &'a mut W {
        self.variant(P2_01ODW::NORMAL)
    }
    #[doc = "Open-drain. P2.1p in is in the open drain mode."]
    #[inline]
    pub fn open_drain(self) -> &'a mut W {
        self.variant(P2_01ODW::OPEN_DRAIN)
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
#[doc = "Values that can be written to the field `P2_02OD`"]
pub enum P2_02ODW {
    #[doc = "Normal. P2.2 pin is in the normal (not open drain) mode."]
    NORMAL,
    #[doc = "Open-drain. P2.2 pin is in the open drain mode."]
    OPEN_DRAIN,
}
impl P2_02ODW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            P2_02ODW::NORMAL => false,
            P2_02ODW::OPEN_DRAIN => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _P2_02ODW<'a> {
    w: &'a mut W,
}
impl<'a> _P2_02ODW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: P2_02ODW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Normal. P2.2 pin is in the normal (not open drain) mode."]
    #[inline]
    pub fn normal(self) -> &'a mut W {
        self.variant(P2_02ODW::NORMAL)
    }
    #[doc = "Open-drain. P2.2 pin is in the open drain mode."]
    #[inline]
    pub fn open_drain(self) -> &'a mut W {
        self.variant(P2_02ODW::OPEN_DRAIN)
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
        const OFFSET: u8 = 2;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `P2_03OD`"]
pub enum P2_03ODW {
    #[doc = "Normal. P2.3 pin is in the normal (not open drain) mode."]
    NORMAL,
    #[doc = "Open-drain. P2.3 pin is in the open drain mode."]
    OPEN_DRAIN,
}
impl P2_03ODW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            P2_03ODW::NORMAL => false,
            P2_03ODW::OPEN_DRAIN => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _P2_03ODW<'a> {
    w: &'a mut W,
}
impl<'a> _P2_03ODW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: P2_03ODW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Normal. P2.3 pin is in the normal (not open drain) mode."]
    #[inline]
    pub fn normal(self) -> &'a mut W {
        self.variant(P2_03ODW::NORMAL)
    }
    #[doc = "Open-drain. P2.3 pin is in the open drain mode."]
    #[inline]
    pub fn open_drain(self) -> &'a mut W {
        self.variant(P2_03ODW::OPEN_DRAIN)
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
        const OFFSET: u8 = 3;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `P2_04OD`"]
pub enum P2_04ODW {
    #[doc = "Normal. P2.4 pin is in the normal (not open drain) mode."]
    NORMAL,
    #[doc = "Open-drain. P2.4 pin is in the open drain mode."]
    OPEN_DRAIN,
}
impl P2_04ODW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            P2_04ODW::NORMAL => false,
            P2_04ODW::OPEN_DRAIN => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _P2_04ODW<'a> {
    w: &'a mut W,
}
impl<'a> _P2_04ODW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: P2_04ODW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Normal. P2.4 pin is in the normal (not open drain) mode."]
    #[inline]
    pub fn normal(self) -> &'a mut W {
        self.variant(P2_04ODW::NORMAL)
    }
    #[doc = "Open-drain. P2.4 pin is in the open drain mode."]
    #[inline]
    pub fn open_drain(self) -> &'a mut W {
        self.variant(P2_04ODW::OPEN_DRAIN)
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
#[doc = "Values that can be written to the field `P2_05OD`"]
pub enum P2_05ODW {
    #[doc = "Normal. P2.5 pin is in the normal (not open drain) mode."]
    NORMAL,
    #[doc = "Open-drain. P2.5 pin is in the open drain mode."]
    OPEN_DRAIN,
}
impl P2_05ODW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            P2_05ODW::NORMAL => false,
            P2_05ODW::OPEN_DRAIN => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _P2_05ODW<'a> {
    w: &'a mut W,
}
impl<'a> _P2_05ODW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: P2_05ODW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Normal. P2.5 pin is in the normal (not open drain) mode."]
    #[inline]
    pub fn normal(self) -> &'a mut W {
        self.variant(P2_05ODW::NORMAL)
    }
    #[doc = "Open-drain. P2.5 pin is in the open drain mode."]
    #[inline]
    pub fn open_drain(self) -> &'a mut W {
        self.variant(P2_05ODW::OPEN_DRAIN)
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
#[doc = "Values that can be written to the field `P2_06OD`"]
pub enum P2_06ODW {
    #[doc = "Normal. P2.6 pin is in the normal (not open drain) mode."]
    NORMAL,
    #[doc = "Open-drain. P2.6 pin is in the open drain mode."]
    OPEN_DRAIN,
}
impl P2_06ODW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            P2_06ODW::NORMAL => false,
            P2_06ODW::OPEN_DRAIN => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _P2_06ODW<'a> {
    w: &'a mut W,
}
impl<'a> _P2_06ODW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: P2_06ODW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Normal. P2.6 pin is in the normal (not open drain) mode."]
    #[inline]
    pub fn normal(self) -> &'a mut W {
        self.variant(P2_06ODW::NORMAL)
    }
    #[doc = "Open-drain. P2.6 pin is in the open drain mode."]
    #[inline]
    pub fn open_drain(self) -> &'a mut W {
        self.variant(P2_06ODW::OPEN_DRAIN)
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
#[doc = "Values that can be written to the field `P2_07OD`"]
pub enum P2_07ODW {
    #[doc = "Normal. P2.7 pin is in the normal (not open drain) mode."]
    NORMAL,
    #[doc = "Open-drain. P2.7 pin is in the open drain mode."]
    OPEN_DRAIN,
}
impl P2_07ODW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            P2_07ODW::NORMAL => false,
            P2_07ODW::OPEN_DRAIN => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _P2_07ODW<'a> {
    w: &'a mut W,
}
impl<'a> _P2_07ODW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: P2_07ODW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Normal. P2.7 pin is in the normal (not open drain) mode."]
    #[inline]
    pub fn normal(self) -> &'a mut W {
        self.variant(P2_07ODW::NORMAL)
    }
    #[doc = "Open-drain. P2.7 pin is in the open drain mode."]
    #[inline]
    pub fn open_drain(self) -> &'a mut W {
        self.variant(P2_07ODW::OPEN_DRAIN)
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
        const OFFSET: u8 = 7;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `P2_08OD`"]
pub enum P2_08ODW {
    #[doc = "Normal. P2.8 pin is in the normal (not open drain) mode."]
    NORMAL,
    #[doc = "Open-drain. P2.8 pin is in the open drain mode."]
    OPEN_DRAIN,
}
impl P2_08ODW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            P2_08ODW::NORMAL => false,
            P2_08ODW::OPEN_DRAIN => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _P2_08ODW<'a> {
    w: &'a mut W,
}
impl<'a> _P2_08ODW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: P2_08ODW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Normal. P2.8 pin is in the normal (not open drain) mode."]
    #[inline]
    pub fn normal(self) -> &'a mut W {
        self.variant(P2_08ODW::NORMAL)
    }
    #[doc = "Open-drain. P2.8 pin is in the open drain mode."]
    #[inline]
    pub fn open_drain(self) -> &'a mut W {
        self.variant(P2_08ODW::OPEN_DRAIN)
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
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `P2_09OD`"]
pub enum P2_09ODW {
    #[doc = "Normal. P2.9 pin is in the normal (not open drain) mode."]
    NORMAL,
    #[doc = "Open-drain. P2.9 pin is in the open drain mode."]
    OPEN_DRAIN,
}
impl P2_09ODW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            P2_09ODW::NORMAL => false,
            P2_09ODW::OPEN_DRAIN => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _P2_09ODW<'a> {
    w: &'a mut W,
}
impl<'a> _P2_09ODW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: P2_09ODW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Normal. P2.9 pin is in the normal (not open drain) mode."]
    #[inline]
    pub fn normal(self) -> &'a mut W {
        self.variant(P2_09ODW::NORMAL)
    }
    #[doc = "Open-drain. P2.9 pin is in the open drain mode."]
    #[inline]
    pub fn open_drain(self) -> &'a mut W {
        self.variant(P2_09ODW::OPEN_DRAIN)
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
        const OFFSET: u8 = 9;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `P2_10OD`"]
pub enum P2_10ODW {
    #[doc = "Normal. P2.10 pin is in the normal (not open drain) mode."]
    NORMAL,
    #[doc = "Open-drain. P2.10 pin is in the open drain mode."]
    OPEN_DRAIN,
}
impl P2_10ODW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            P2_10ODW::NORMAL => false,
            P2_10ODW::OPEN_DRAIN => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _P2_10ODW<'a> {
    w: &'a mut W,
}
impl<'a> _P2_10ODW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: P2_10ODW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Normal. P2.10 pin is in the normal (not open drain) mode."]
    #[inline]
    pub fn normal(self) -> &'a mut W {
        self.variant(P2_10ODW::NORMAL)
    }
    #[doc = "Open-drain. P2.10 pin is in the open drain mode."]
    #[inline]
    pub fn open_drain(self) -> &'a mut W {
        self.variant(P2_10ODW::OPEN_DRAIN)
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
#[doc = "Values that can be written to the field `P2_11OD`"]
pub enum P2_11ODW {
    #[doc = "Normal. P2.11 pin is in the normal (not open drain) mode."]
    NORMAL,
    #[doc = "Open-drain. P2.11 pin is in the open drain mode."]
    OPEN_DRAIN,
}
impl P2_11ODW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            P2_11ODW::NORMAL => false,
            P2_11ODW::OPEN_DRAIN => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _P2_11ODW<'a> {
    w: &'a mut W,
}
impl<'a> _P2_11ODW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: P2_11ODW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Normal. P2.11 pin is in the normal (not open drain) mode."]
    #[inline]
    pub fn normal(self) -> &'a mut W {
        self.variant(P2_11ODW::NORMAL)
    }
    #[doc = "Open-drain. P2.11 pin is in the open drain mode."]
    #[inline]
    pub fn open_drain(self) -> &'a mut W {
        self.variant(P2_11ODW::OPEN_DRAIN)
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
        const OFFSET: u8 = 11;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `P2_12OD`"]
pub enum P2_12ODW {
    #[doc = "Normal. P2.12 pin is in the normal (not open drain) mode."]
    NORMAL,
    #[doc = "Open-drain. P2.12 pin is in the open drain mode."]
    OPEN_DRAIN,
}
impl P2_12ODW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            P2_12ODW::NORMAL => false,
            P2_12ODW::OPEN_DRAIN => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _P2_12ODW<'a> {
    w: &'a mut W,
}
impl<'a> _P2_12ODW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: P2_12ODW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Normal. P2.12 pin is in the normal (not open drain) mode."]
    #[inline]
    pub fn normal(self) -> &'a mut W {
        self.variant(P2_12ODW::NORMAL)
    }
    #[doc = "Open-drain. P2.12 pin is in the open drain mode."]
    #[inline]
    pub fn open_drain(self) -> &'a mut W {
        self.variant(P2_12ODW::OPEN_DRAIN)
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
        const OFFSET: u8 = 12;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `P2_13OD`"]
pub enum P2_13ODW {
    #[doc = "Normal. P2.13 pin is in the normal (not open drain) mode."]
    NORMAL,
    #[doc = "Open-drain. P2.13 pin is in the open drain mode."]
    OPEN_DRAIN,
}
impl P2_13ODW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            P2_13ODW::NORMAL => false,
            P2_13ODW::OPEN_DRAIN => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _P2_13ODW<'a> {
    w: &'a mut W,
}
impl<'a> _P2_13ODW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: P2_13ODW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Normal. P2.13 pin is in the normal (not open drain) mode."]
    #[inline]
    pub fn normal(self) -> &'a mut W {
        self.variant(P2_13ODW::NORMAL)
    }
    #[doc = "Open-drain. P2.13 pin is in the open drain mode."]
    #[inline]
    pub fn open_drain(self) -> &'a mut W {
        self.variant(P2_13ODW::OPEN_DRAIN)
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
        const OFFSET: u8 = 13;
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
    #[doc = "Bit 0 - Port 2 pin 0 open drain mode control."]
    #[inline]
    pub fn p2_00od(&self) -> P2_00ODR {
        P2_00ODR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Port 2 pin 1 open drain mode control, see P2.00OD"]
    #[inline]
    pub fn p2_01od(&self) -> P2_01ODR {
        P2_01ODR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - Port 2 pin 2 open drain mode control, see P2.00OD"]
    #[inline]
    pub fn p2_02od(&self) -> P2_02ODR {
        P2_02ODR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - Port 2 pin 3 open drain mode control, see P2.00OD"]
    #[inline]
    pub fn p2_03od(&self) -> P2_03ODR {
        P2_03ODR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - Port 2 pin 4 open drain mode control, see P2.00OD"]
    #[inline]
    pub fn p2_04od(&self) -> P2_04ODR {
        P2_04ODR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - Port 2 pin 5 open drain mode control, see P2.00OD"]
    #[inline]
    pub fn p2_05od(&self) -> P2_05ODR {
        P2_05ODR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 6 - Port 2 pin 6 open drain mode control, see P2.00OD"]
    #[inline]
    pub fn p2_06od(&self) -> P2_06ODR {
        P2_06ODR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 7 - Port 2 pin 7 open drain mode control, see P2.00OD"]
    #[inline]
    pub fn p2_07od(&self) -> P2_07ODR {
        P2_07ODR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 8 - Port 2 pin 8 open drain mode control, see P2.00OD"]
    #[inline]
    pub fn p2_08od(&self) -> P2_08ODR {
        P2_08ODR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 9 - Port 2 pin 9 open drain mode control, see P2.00OD"]
    #[inline]
    pub fn p2_09od(&self) -> P2_09ODR {
        P2_09ODR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 10 - Port 2 pin 10 open drain mode control, see P2.00OD"]
    #[inline]
    pub fn p2_10od(&self) -> P2_10ODR {
        P2_10ODR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 11 - Port 2 pin 11 open drain mode control, see P2.00OD"]
    #[inline]
    pub fn p2_11od(&self) -> P2_11ODR {
        P2_11ODR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 12 - Port 2 pin 12 open drain mode control, see P2.00OD"]
    #[inline]
    pub fn p2_12od(&self) -> P2_12ODR {
        P2_12ODR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 13 - Port 2 pin 13 open drain mode control, see P2.00OD"]
    #[inline]
    pub fn p2_13od(&self) -> P2_13ODR {
        P2_13ODR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 13;
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
    #[doc = "Bit 0 - Port 2 pin 0 open drain mode control."]
    #[inline]
    pub fn p2_00od(&mut self) -> _P2_00ODW {
        _P2_00ODW { w: self }
    }
    #[doc = "Bit 1 - Port 2 pin 1 open drain mode control, see P2.00OD"]
    #[inline]
    pub fn p2_01od(&mut self) -> _P2_01ODW {
        _P2_01ODW { w: self }
    }
    #[doc = "Bit 2 - Port 2 pin 2 open drain mode control, see P2.00OD"]
    #[inline]
    pub fn p2_02od(&mut self) -> _P2_02ODW {
        _P2_02ODW { w: self }
    }
    #[doc = "Bit 3 - Port 2 pin 3 open drain mode control, see P2.00OD"]
    #[inline]
    pub fn p2_03od(&mut self) -> _P2_03ODW {
        _P2_03ODW { w: self }
    }
    #[doc = "Bit 4 - Port 2 pin 4 open drain mode control, see P2.00OD"]
    #[inline]
    pub fn p2_04od(&mut self) -> _P2_04ODW {
        _P2_04ODW { w: self }
    }
    #[doc = "Bit 5 - Port 2 pin 5 open drain mode control, see P2.00OD"]
    #[inline]
    pub fn p2_05od(&mut self) -> _P2_05ODW {
        _P2_05ODW { w: self }
    }
    #[doc = "Bit 6 - Port 2 pin 6 open drain mode control, see P2.00OD"]
    #[inline]
    pub fn p2_06od(&mut self) -> _P2_06ODW {
        _P2_06ODW { w: self }
    }
    #[doc = "Bit 7 - Port 2 pin 7 open drain mode control, see P2.00OD"]
    #[inline]
    pub fn p2_07od(&mut self) -> _P2_07ODW {
        _P2_07ODW { w: self }
    }
    #[doc = "Bit 8 - Port 2 pin 8 open drain mode control, see P2.00OD"]
    #[inline]
    pub fn p2_08od(&mut self) -> _P2_08ODW {
        _P2_08ODW { w: self }
    }
    #[doc = "Bit 9 - Port 2 pin 9 open drain mode control, see P2.00OD"]
    #[inline]
    pub fn p2_09od(&mut self) -> _P2_09ODW {
        _P2_09ODW { w: self }
    }
    #[doc = "Bit 10 - Port 2 pin 10 open drain mode control, see P2.00OD"]
    #[inline]
    pub fn p2_10od(&mut self) -> _P2_10ODW {
        _P2_10ODW { w: self }
    }
    #[doc = "Bit 11 - Port 2 pin 11 open drain mode control, see P2.00OD"]
    #[inline]
    pub fn p2_11od(&mut self) -> _P2_11ODW {
        _P2_11ODW { w: self }
    }
    #[doc = "Bit 12 - Port 2 pin 12 open drain mode control, see P2.00OD"]
    #[inline]
    pub fn p2_12od(&mut self) -> _P2_12ODW {
        _P2_12ODW { w: self }
    }
    #[doc = "Bit 13 - Port 2 pin 13 open drain mode control, see P2.00OD"]
    #[inline]
    pub fn p2_13od(&mut self) -> _P2_13ODW {
        _P2_13ODW { w: self }
    }
}
