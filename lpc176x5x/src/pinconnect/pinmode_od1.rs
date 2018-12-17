#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::PINMODE_OD1 {
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
#[doc = "Possible values of the field `P1_00OD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P1_00ODR {
    #[doc = "Normal. P1.0 pin is in the normal (not open drain) mode."]
    NORMAL,
    #[doc = "Open-drain. P1.0 pin is in the open drain mode."]
    OPEN_DRAIN,
}
impl P1_00ODR {
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
            P1_00ODR::NORMAL => false,
            P1_00ODR::OPEN_DRAIN => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> P1_00ODR {
        match value {
            false => P1_00ODR::NORMAL,
            true => P1_00ODR::OPEN_DRAIN,
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline]
    pub fn is_normal(&self) -> bool {
        *self == P1_00ODR::NORMAL
    }
    #[doc = "Checks if the value of the field is `OPEN_DRAIN`"]
    #[inline]
    pub fn is_open_drain(&self) -> bool {
        *self == P1_00ODR::OPEN_DRAIN
    }
}
#[doc = "Possible values of the field `P1_01OD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P1_01ODR {
    #[doc = "Normal. P1.1 pin is in the normal (not open drain) mode."]
    NORMAL,
    #[doc = "Open-drain. P1.1 pin is in the open drain mode."]
    OPEN_DRAIN,
}
impl P1_01ODR {
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
            P1_01ODR::NORMAL => false,
            P1_01ODR::OPEN_DRAIN => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> P1_01ODR {
        match value {
            false => P1_01ODR::NORMAL,
            true => P1_01ODR::OPEN_DRAIN,
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline]
    pub fn is_normal(&self) -> bool {
        *self == P1_01ODR::NORMAL
    }
    #[doc = "Checks if the value of the field is `OPEN_DRAIN`"]
    #[inline]
    pub fn is_open_drain(&self) -> bool {
        *self == P1_01ODR::OPEN_DRAIN
    }
}
#[doc = "Possible values of the field `P1_04OD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P1_04ODR {
    #[doc = "Normal. P1.4 pin is in the normal (not open drain) mode."]
    NORMAL,
    #[doc = "Open-drain. P1.4 pin is in the open drain mode."]
    OPEN_DRAIN,
}
impl P1_04ODR {
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
            P1_04ODR::NORMAL => false,
            P1_04ODR::OPEN_DRAIN => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> P1_04ODR {
        match value {
            false => P1_04ODR::NORMAL,
            true => P1_04ODR::OPEN_DRAIN,
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline]
    pub fn is_normal(&self) -> bool {
        *self == P1_04ODR::NORMAL
    }
    #[doc = "Checks if the value of the field is `OPEN_DRAIN`"]
    #[inline]
    pub fn is_open_drain(&self) -> bool {
        *self == P1_04ODR::OPEN_DRAIN
    }
}
#[doc = "Possible values of the field `P1_08OD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P1_08ODR {
    #[doc = "Normal. P1.8 pin is in the normal (not open drain) mode."]
    NORMAL,
    #[doc = "Open-drain. P1.8 pin is in the open drain mode."]
    OPEN_DRAIN,
}
impl P1_08ODR {
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
            P1_08ODR::NORMAL => false,
            P1_08ODR::OPEN_DRAIN => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> P1_08ODR {
        match value {
            false => P1_08ODR::NORMAL,
            true => P1_08ODR::OPEN_DRAIN,
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline]
    pub fn is_normal(&self) -> bool {
        *self == P1_08ODR::NORMAL
    }
    #[doc = "Checks if the value of the field is `OPEN_DRAIN`"]
    #[inline]
    pub fn is_open_drain(&self) -> bool {
        *self == P1_08ODR::OPEN_DRAIN
    }
}
#[doc = "Possible values of the field `P1_09OD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P1_09ODR {
    #[doc = "Normal. P1.9 pin is in the normal (not open drain) mode."]
    NORMAL,
    #[doc = "Open-drain. P1.9 pin is in the open drain mode."]
    OPEN_DRAIN,
}
impl P1_09ODR {
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
            P1_09ODR::NORMAL => false,
            P1_09ODR::OPEN_DRAIN => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> P1_09ODR {
        match value {
            false => P1_09ODR::NORMAL,
            true => P1_09ODR::OPEN_DRAIN,
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline]
    pub fn is_normal(&self) -> bool {
        *self == P1_09ODR::NORMAL
    }
    #[doc = "Checks if the value of the field is `OPEN_DRAIN`"]
    #[inline]
    pub fn is_open_drain(&self) -> bool {
        *self == P1_09ODR::OPEN_DRAIN
    }
}
#[doc = "Possible values of the field `P1_10OD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P1_10ODR {
    #[doc = "Normal. P1.10 pin is in the normal (not open drain) mode."]
    NORMAL,
    #[doc = "Open-drain. P1.10 pin is in the open drain mode."]
    OPEN_DRAIN,
}
impl P1_10ODR {
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
            P1_10ODR::NORMAL => false,
            P1_10ODR::OPEN_DRAIN => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> P1_10ODR {
        match value {
            false => P1_10ODR::NORMAL,
            true => P1_10ODR::OPEN_DRAIN,
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline]
    pub fn is_normal(&self) -> bool {
        *self == P1_10ODR::NORMAL
    }
    #[doc = "Checks if the value of the field is `OPEN_DRAIN`"]
    #[inline]
    pub fn is_open_drain(&self) -> bool {
        *self == P1_10ODR::OPEN_DRAIN
    }
}
#[doc = "Possible values of the field `P1_14OD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P1_14ODR {
    #[doc = "Normal. P1.14 pin is in the normal (not open drain) mode."]
    NORMAL,
    #[doc = "Open-drain. P1.14 pin is in the open drain mode."]
    OPEN_DRAIN,
}
impl P1_14ODR {
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
            P1_14ODR::NORMAL => false,
            P1_14ODR::OPEN_DRAIN => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> P1_14ODR {
        match value {
            false => P1_14ODR::NORMAL,
            true => P1_14ODR::OPEN_DRAIN,
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline]
    pub fn is_normal(&self) -> bool {
        *self == P1_14ODR::NORMAL
    }
    #[doc = "Checks if the value of the field is `OPEN_DRAIN`"]
    #[inline]
    pub fn is_open_drain(&self) -> bool {
        *self == P1_14ODR::OPEN_DRAIN
    }
}
#[doc = "Possible values of the field `P1_15OD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P1_15ODR {
    #[doc = "Normal. P1.15 pin is in the normal (not open drain) mode."]
    NORMAL,
    #[doc = "Open-drain. P1.15 pin is in the open drain mode."]
    OPEN_DRAIN,
}
impl P1_15ODR {
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
            P1_15ODR::NORMAL => false,
            P1_15ODR::OPEN_DRAIN => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> P1_15ODR {
        match value {
            false => P1_15ODR::NORMAL,
            true => P1_15ODR::OPEN_DRAIN,
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline]
    pub fn is_normal(&self) -> bool {
        *self == P1_15ODR::NORMAL
    }
    #[doc = "Checks if the value of the field is `OPEN_DRAIN`"]
    #[inline]
    pub fn is_open_drain(&self) -> bool {
        *self == P1_15ODR::OPEN_DRAIN
    }
}
#[doc = "Possible values of the field `P1_16OD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P1_16ODR {
    #[doc = "Normal. P1.16 pin is in the normal (not open drain) mode."]
    NORMAL,
    #[doc = "Open-drain. P1.16 pin is in the open drain mode."]
    OPEN_DRAIN,
}
impl P1_16ODR {
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
            P1_16ODR::NORMAL => false,
            P1_16ODR::OPEN_DRAIN => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> P1_16ODR {
        match value {
            false => P1_16ODR::NORMAL,
            true => P1_16ODR::OPEN_DRAIN,
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline]
    pub fn is_normal(&self) -> bool {
        *self == P1_16ODR::NORMAL
    }
    #[doc = "Checks if the value of the field is `OPEN_DRAIN`"]
    #[inline]
    pub fn is_open_drain(&self) -> bool {
        *self == P1_16ODR::OPEN_DRAIN
    }
}
#[doc = "Possible values of the field `P1_17OD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P1_17ODR {
    #[doc = "Normal. P1.17 pin is in the normal (not open drain) mode."]
    NORMAL,
    #[doc = "Open-drain. P1.17 pin is in the open drain mode."]
    OPEN_DRAIN,
}
impl P1_17ODR {
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
            P1_17ODR::NORMAL => false,
            P1_17ODR::OPEN_DRAIN => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> P1_17ODR {
        match value {
            false => P1_17ODR::NORMAL,
            true => P1_17ODR::OPEN_DRAIN,
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline]
    pub fn is_normal(&self) -> bool {
        *self == P1_17ODR::NORMAL
    }
    #[doc = "Checks if the value of the field is `OPEN_DRAIN`"]
    #[inline]
    pub fn is_open_drain(&self) -> bool {
        *self == P1_17ODR::OPEN_DRAIN
    }
}
#[doc = "Possible values of the field `P1_18OD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P1_18ODR {
    #[doc = "Normal. P1.18 pin is in the normal (not open drain) mode."]
    NORMAL,
    #[doc = "Open-drain. P1.18 pin is in the open drain mode."]
    OPEN_DRAIN,
}
impl P1_18ODR {
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
            P1_18ODR::NORMAL => false,
            P1_18ODR::OPEN_DRAIN => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> P1_18ODR {
        match value {
            false => P1_18ODR::NORMAL,
            true => P1_18ODR::OPEN_DRAIN,
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline]
    pub fn is_normal(&self) -> bool {
        *self == P1_18ODR::NORMAL
    }
    #[doc = "Checks if the value of the field is `OPEN_DRAIN`"]
    #[inline]
    pub fn is_open_drain(&self) -> bool {
        *self == P1_18ODR::OPEN_DRAIN
    }
}
#[doc = "Possible values of the field `P1_19OD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P1_19ODR {
    #[doc = "Normal. P1.19 pin is in the normal (not open drain) mode."]
    NORMAL,
    #[doc = "Open-drain. P1.19 pin is in the open drain mode."]
    OPEN_DRAIN,
}
impl P1_19ODR {
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
            P1_19ODR::NORMAL => false,
            P1_19ODR::OPEN_DRAIN => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> P1_19ODR {
        match value {
            false => P1_19ODR::NORMAL,
            true => P1_19ODR::OPEN_DRAIN,
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline]
    pub fn is_normal(&self) -> bool {
        *self == P1_19ODR::NORMAL
    }
    #[doc = "Checks if the value of the field is `OPEN_DRAIN`"]
    #[inline]
    pub fn is_open_drain(&self) -> bool {
        *self == P1_19ODR::OPEN_DRAIN
    }
}
#[doc = "Possible values of the field `P1_20OD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P1_20ODR {
    #[doc = "Normal. P1.20 pin is in the normal (not open drain) mode."]
    NORMAL,
    #[doc = "Open-drain. P1.20 pin is in the open drain mode."]
    OPEN_DRAIN,
}
impl P1_20ODR {
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
            P1_20ODR::NORMAL => false,
            P1_20ODR::OPEN_DRAIN => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> P1_20ODR {
        match value {
            false => P1_20ODR::NORMAL,
            true => P1_20ODR::OPEN_DRAIN,
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline]
    pub fn is_normal(&self) -> bool {
        *self == P1_20ODR::NORMAL
    }
    #[doc = "Checks if the value of the field is `OPEN_DRAIN`"]
    #[inline]
    pub fn is_open_drain(&self) -> bool {
        *self == P1_20ODR::OPEN_DRAIN
    }
}
#[doc = "Possible values of the field `P1_21OD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P1_21ODR {
    #[doc = "Normal. P1.21 pin is in the normal (not open drain) mode."]
    NORMAL,
    #[doc = "Open-drain. P1.21 pin is in the open drain mode."]
    OPEN_DRAIN,
}
impl P1_21ODR {
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
            P1_21ODR::NORMAL => false,
            P1_21ODR::OPEN_DRAIN => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> P1_21ODR {
        match value {
            false => P1_21ODR::NORMAL,
            true => P1_21ODR::OPEN_DRAIN,
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline]
    pub fn is_normal(&self) -> bool {
        *self == P1_21ODR::NORMAL
    }
    #[doc = "Checks if the value of the field is `OPEN_DRAIN`"]
    #[inline]
    pub fn is_open_drain(&self) -> bool {
        *self == P1_21ODR::OPEN_DRAIN
    }
}
#[doc = "Possible values of the field `P1_22OD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P1_22ODR {
    #[doc = "Normal. P1.22 pin is in the normal (not open drain) mode."]
    NORMAL,
    #[doc = "Open-drain. P1.22 pin is in the open drain mode."]
    OPEN_DRAIN,
}
impl P1_22ODR {
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
            P1_22ODR::NORMAL => false,
            P1_22ODR::OPEN_DRAIN => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> P1_22ODR {
        match value {
            false => P1_22ODR::NORMAL,
            true => P1_22ODR::OPEN_DRAIN,
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline]
    pub fn is_normal(&self) -> bool {
        *self == P1_22ODR::NORMAL
    }
    #[doc = "Checks if the value of the field is `OPEN_DRAIN`"]
    #[inline]
    pub fn is_open_drain(&self) -> bool {
        *self == P1_22ODR::OPEN_DRAIN
    }
}
#[doc = "Possible values of the field `P1_23OD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P1_23ODR {
    #[doc = "Normal. P1.23 pin is in the normal (not open drain) mode."]
    NORMAL,
    #[doc = "Open-drain. P1.23 pin is in the open drain mode."]
    OPEN_DRAIN,
}
impl P1_23ODR {
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
            P1_23ODR::NORMAL => false,
            P1_23ODR::OPEN_DRAIN => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> P1_23ODR {
        match value {
            false => P1_23ODR::NORMAL,
            true => P1_23ODR::OPEN_DRAIN,
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline]
    pub fn is_normal(&self) -> bool {
        *self == P1_23ODR::NORMAL
    }
    #[doc = "Checks if the value of the field is `OPEN_DRAIN`"]
    #[inline]
    pub fn is_open_drain(&self) -> bool {
        *self == P1_23ODR::OPEN_DRAIN
    }
}
#[doc = "Possible values of the field `P1_24OD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P1_24ODR {
    #[doc = "Normal. P1.24 pin is in the normal (not open drain) mode."]
    NORMAL,
    #[doc = "Open-drain. P1.24 pin is in the open drain mode."]
    OPEN_DRAIN,
}
impl P1_24ODR {
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
            P1_24ODR::NORMAL => false,
            P1_24ODR::OPEN_DRAIN => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> P1_24ODR {
        match value {
            false => P1_24ODR::NORMAL,
            true => P1_24ODR::OPEN_DRAIN,
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline]
    pub fn is_normal(&self) -> bool {
        *self == P1_24ODR::NORMAL
    }
    #[doc = "Checks if the value of the field is `OPEN_DRAIN`"]
    #[inline]
    pub fn is_open_drain(&self) -> bool {
        *self == P1_24ODR::OPEN_DRAIN
    }
}
#[doc = "Possible values of the field `P1_25OD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P1_25ODR {
    #[doc = "Normal. P1.25 pin is in the normal (not open drain) mode."]
    NORMAL,
    #[doc = "Open-drain. P1.25 pin is in the open drain mode."]
    OPEN_DRAIN,
}
impl P1_25ODR {
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
            P1_25ODR::NORMAL => false,
            P1_25ODR::OPEN_DRAIN => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> P1_25ODR {
        match value {
            false => P1_25ODR::NORMAL,
            true => P1_25ODR::OPEN_DRAIN,
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline]
    pub fn is_normal(&self) -> bool {
        *self == P1_25ODR::NORMAL
    }
    #[doc = "Checks if the value of the field is `OPEN_DRAIN`"]
    #[inline]
    pub fn is_open_drain(&self) -> bool {
        *self == P1_25ODR::OPEN_DRAIN
    }
}
#[doc = "Possible values of the field `P1_26OD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P1_26ODR {
    #[doc = "Normal. P1.26 pin is in the normal (not open drain) mode."]
    NORMAL,
    #[doc = "Open-drain. P1.26 pin is in the open drain mode."]
    OPEN_DRAIN,
}
impl P1_26ODR {
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
            P1_26ODR::NORMAL => false,
            P1_26ODR::OPEN_DRAIN => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> P1_26ODR {
        match value {
            false => P1_26ODR::NORMAL,
            true => P1_26ODR::OPEN_DRAIN,
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline]
    pub fn is_normal(&self) -> bool {
        *self == P1_26ODR::NORMAL
    }
    #[doc = "Checks if the value of the field is `OPEN_DRAIN`"]
    #[inline]
    pub fn is_open_drain(&self) -> bool {
        *self == P1_26ODR::OPEN_DRAIN
    }
}
#[doc = "Possible values of the field `P1_27OD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P1_27ODR {
    #[doc = "Normal. P1.27 pin is in the normal (not open drain) mode."]
    NORMAL,
    #[doc = "Open-drain. P1.27 pin is in the open drain mode."]
    OPEN_DRAIN,
}
impl P1_27ODR {
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
            P1_27ODR::NORMAL => false,
            P1_27ODR::OPEN_DRAIN => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> P1_27ODR {
        match value {
            false => P1_27ODR::NORMAL,
            true => P1_27ODR::OPEN_DRAIN,
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline]
    pub fn is_normal(&self) -> bool {
        *self == P1_27ODR::NORMAL
    }
    #[doc = "Checks if the value of the field is `OPEN_DRAIN`"]
    #[inline]
    pub fn is_open_drain(&self) -> bool {
        *self == P1_27ODR::OPEN_DRAIN
    }
}
#[doc = "Possible values of the field `P1_28OD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P1_28ODR {
    #[doc = "Normal. P1.28 pin is in the normal (not open drain) mode."]
    NORMAL,
    #[doc = "Open-drain. P1.28 pin is in the open drain mode."]
    OPEN_DRAIN,
}
impl P1_28ODR {
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
            P1_28ODR::NORMAL => false,
            P1_28ODR::OPEN_DRAIN => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> P1_28ODR {
        match value {
            false => P1_28ODR::NORMAL,
            true => P1_28ODR::OPEN_DRAIN,
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline]
    pub fn is_normal(&self) -> bool {
        *self == P1_28ODR::NORMAL
    }
    #[doc = "Checks if the value of the field is `OPEN_DRAIN`"]
    #[inline]
    pub fn is_open_drain(&self) -> bool {
        *self == P1_28ODR::OPEN_DRAIN
    }
}
#[doc = "Possible values of the field `P1_29OD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P1_29ODR {
    #[doc = "Normal. P1.29 pin is in the normal (not open drain) mode."]
    NORMAL,
    #[doc = "Open-drain. P1.29 pin is in the open drain mode."]
    OPEN_DRAIN,
}
impl P1_29ODR {
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
            P1_29ODR::NORMAL => false,
            P1_29ODR::OPEN_DRAIN => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> P1_29ODR {
        match value {
            false => P1_29ODR::NORMAL,
            true => P1_29ODR::OPEN_DRAIN,
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline]
    pub fn is_normal(&self) -> bool {
        *self == P1_29ODR::NORMAL
    }
    #[doc = "Checks if the value of the field is `OPEN_DRAIN`"]
    #[inline]
    pub fn is_open_drain(&self) -> bool {
        *self == P1_29ODR::OPEN_DRAIN
    }
}
#[doc = "Possible values of the field `P1_30OD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P1_30ODR {
    #[doc = "Normal. P1.30 pin is in the normal (not open drain) mode."]
    NORMAL,
    #[doc = "Open-drain. P1.30 pin is in the open drain mode."]
    OPEN_DRAIN,
}
impl P1_30ODR {
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
            P1_30ODR::NORMAL => false,
            P1_30ODR::OPEN_DRAIN => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> P1_30ODR {
        match value {
            false => P1_30ODR::NORMAL,
            true => P1_30ODR::OPEN_DRAIN,
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline]
    pub fn is_normal(&self) -> bool {
        *self == P1_30ODR::NORMAL
    }
    #[doc = "Checks if the value of the field is `OPEN_DRAIN`"]
    #[inline]
    pub fn is_open_drain(&self) -> bool {
        *self == P1_30ODR::OPEN_DRAIN
    }
}
#[doc = "Possible values of the field `P1_31OD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P1_31ODR {
    #[doc = "Normal. P1.31 pin is in the normal (not open drain) mode."]
    NORMAL,
    #[doc = "Open-drain. P1.31 pin is in the open drain mode."]
    OPEN_DRAIN,
}
impl P1_31ODR {
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
            P1_31ODR::NORMAL => false,
            P1_31ODR::OPEN_DRAIN => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> P1_31ODR {
        match value {
            false => P1_31ODR::NORMAL,
            true => P1_31ODR::OPEN_DRAIN,
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline]
    pub fn is_normal(&self) -> bool {
        *self == P1_31ODR::NORMAL
    }
    #[doc = "Checks if the value of the field is `OPEN_DRAIN`"]
    #[inline]
    pub fn is_open_drain(&self) -> bool {
        *self == P1_31ODR::OPEN_DRAIN
    }
}
#[doc = "Values that can be written to the field `P1_00OD`"]
pub enum P1_00ODW {
    #[doc = "Normal. P1.0 pin is in the normal (not open drain) mode."]
    NORMAL,
    #[doc = "Open-drain. P1.0 pin is in the open drain mode."]
    OPEN_DRAIN,
}
impl P1_00ODW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            P1_00ODW::NORMAL => false,
            P1_00ODW::OPEN_DRAIN => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _P1_00ODW<'a> {
    w: &'a mut W,
}
impl<'a> _P1_00ODW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: P1_00ODW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Normal. P1.0 pin is in the normal (not open drain) mode."]
    #[inline]
    pub fn normal(self) -> &'a mut W {
        self.variant(P1_00ODW::NORMAL)
    }
    #[doc = "Open-drain. P1.0 pin is in the open drain mode."]
    #[inline]
    pub fn open_drain(self) -> &'a mut W {
        self.variant(P1_00ODW::OPEN_DRAIN)
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
#[doc = "Values that can be written to the field `P1_01OD`"]
pub enum P1_01ODW {
    #[doc = "Normal. P1.1 pin is in the normal (not open drain) mode."]
    NORMAL,
    #[doc = "Open-drain. P1.1 pin is in the open drain mode."]
    OPEN_DRAIN,
}
impl P1_01ODW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            P1_01ODW::NORMAL => false,
            P1_01ODW::OPEN_DRAIN => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _P1_01ODW<'a> {
    w: &'a mut W,
}
impl<'a> _P1_01ODW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: P1_01ODW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Normal. P1.1 pin is in the normal (not open drain) mode."]
    #[inline]
    pub fn normal(self) -> &'a mut W {
        self.variant(P1_01ODW::NORMAL)
    }
    #[doc = "Open-drain. P1.1 pin is in the open drain mode."]
    #[inline]
    pub fn open_drain(self) -> &'a mut W {
        self.variant(P1_01ODW::OPEN_DRAIN)
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
#[doc = "Values that can be written to the field `P1_04OD`"]
pub enum P1_04ODW {
    #[doc = "Normal. P1.4 pin is in the normal (not open drain) mode."]
    NORMAL,
    #[doc = "Open-drain. P1.4 pin is in the open drain mode."]
    OPEN_DRAIN,
}
impl P1_04ODW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            P1_04ODW::NORMAL => false,
            P1_04ODW::OPEN_DRAIN => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _P1_04ODW<'a> {
    w: &'a mut W,
}
impl<'a> _P1_04ODW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: P1_04ODW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Normal. P1.4 pin is in the normal (not open drain) mode."]
    #[inline]
    pub fn normal(self) -> &'a mut W {
        self.variant(P1_04ODW::NORMAL)
    }
    #[doc = "Open-drain. P1.4 pin is in the open drain mode."]
    #[inline]
    pub fn open_drain(self) -> &'a mut W {
        self.variant(P1_04ODW::OPEN_DRAIN)
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
#[doc = "Values that can be written to the field `P1_08OD`"]
pub enum P1_08ODW {
    #[doc = "Normal. P1.8 pin is in the normal (not open drain) mode."]
    NORMAL,
    #[doc = "Open-drain. P1.8 pin is in the open drain mode."]
    OPEN_DRAIN,
}
impl P1_08ODW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            P1_08ODW::NORMAL => false,
            P1_08ODW::OPEN_DRAIN => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _P1_08ODW<'a> {
    w: &'a mut W,
}
impl<'a> _P1_08ODW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: P1_08ODW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Normal. P1.8 pin is in the normal (not open drain) mode."]
    #[inline]
    pub fn normal(self) -> &'a mut W {
        self.variant(P1_08ODW::NORMAL)
    }
    #[doc = "Open-drain. P1.8 pin is in the open drain mode."]
    #[inline]
    pub fn open_drain(self) -> &'a mut W {
        self.variant(P1_08ODW::OPEN_DRAIN)
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
#[doc = "Values that can be written to the field `P1_09OD`"]
pub enum P1_09ODW {
    #[doc = "Normal. P1.9 pin is in the normal (not open drain) mode."]
    NORMAL,
    #[doc = "Open-drain. P1.9 pin is in the open drain mode."]
    OPEN_DRAIN,
}
impl P1_09ODW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            P1_09ODW::NORMAL => false,
            P1_09ODW::OPEN_DRAIN => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _P1_09ODW<'a> {
    w: &'a mut W,
}
impl<'a> _P1_09ODW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: P1_09ODW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Normal. P1.9 pin is in the normal (not open drain) mode."]
    #[inline]
    pub fn normal(self) -> &'a mut W {
        self.variant(P1_09ODW::NORMAL)
    }
    #[doc = "Open-drain. P1.9 pin is in the open drain mode."]
    #[inline]
    pub fn open_drain(self) -> &'a mut W {
        self.variant(P1_09ODW::OPEN_DRAIN)
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
#[doc = "Values that can be written to the field `P1_10OD`"]
pub enum P1_10ODW {
    #[doc = "Normal. P1.10 pin is in the normal (not open drain) mode."]
    NORMAL,
    #[doc = "Open-drain. P1.10 pin is in the open drain mode."]
    OPEN_DRAIN,
}
impl P1_10ODW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            P1_10ODW::NORMAL => false,
            P1_10ODW::OPEN_DRAIN => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _P1_10ODW<'a> {
    w: &'a mut W,
}
impl<'a> _P1_10ODW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: P1_10ODW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Normal. P1.10 pin is in the normal (not open drain) mode."]
    #[inline]
    pub fn normal(self) -> &'a mut W {
        self.variant(P1_10ODW::NORMAL)
    }
    #[doc = "Open-drain. P1.10 pin is in the open drain mode."]
    #[inline]
    pub fn open_drain(self) -> &'a mut W {
        self.variant(P1_10ODW::OPEN_DRAIN)
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
#[doc = "Values that can be written to the field `P1_14OD`"]
pub enum P1_14ODW {
    #[doc = "Normal. P1.14 pin is in the normal (not open drain) mode."]
    NORMAL,
    #[doc = "Open-drain. P1.14 pin is in the open drain mode."]
    OPEN_DRAIN,
}
impl P1_14ODW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            P1_14ODW::NORMAL => false,
            P1_14ODW::OPEN_DRAIN => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _P1_14ODW<'a> {
    w: &'a mut W,
}
impl<'a> _P1_14ODW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: P1_14ODW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Normal. P1.14 pin is in the normal (not open drain) mode."]
    #[inline]
    pub fn normal(self) -> &'a mut W {
        self.variant(P1_14ODW::NORMAL)
    }
    #[doc = "Open-drain. P1.14 pin is in the open drain mode."]
    #[inline]
    pub fn open_drain(self) -> &'a mut W {
        self.variant(P1_14ODW::OPEN_DRAIN)
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
        const OFFSET: u8 = 14;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `P1_15OD`"]
pub enum P1_15ODW {
    #[doc = "Normal. P1.15 pin is in the normal (not open drain) mode."]
    NORMAL,
    #[doc = "Open-drain. P1.15 pin is in the open drain mode."]
    OPEN_DRAIN,
}
impl P1_15ODW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            P1_15ODW::NORMAL => false,
            P1_15ODW::OPEN_DRAIN => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _P1_15ODW<'a> {
    w: &'a mut W,
}
impl<'a> _P1_15ODW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: P1_15ODW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Normal. P1.15 pin is in the normal (not open drain) mode."]
    #[inline]
    pub fn normal(self) -> &'a mut W {
        self.variant(P1_15ODW::NORMAL)
    }
    #[doc = "Open-drain. P1.15 pin is in the open drain mode."]
    #[inline]
    pub fn open_drain(self) -> &'a mut W {
        self.variant(P1_15ODW::OPEN_DRAIN)
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
        const OFFSET: u8 = 15;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `P1_16OD`"]
pub enum P1_16ODW {
    #[doc = "Normal. P1.16 pin is in the normal (not open drain) mode."]
    NORMAL,
    #[doc = "Open-drain. P1.16 pin is in the open drain mode."]
    OPEN_DRAIN,
}
impl P1_16ODW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            P1_16ODW::NORMAL => false,
            P1_16ODW::OPEN_DRAIN => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _P1_16ODW<'a> {
    w: &'a mut W,
}
impl<'a> _P1_16ODW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: P1_16ODW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Normal. P1.16 pin is in the normal (not open drain) mode."]
    #[inline]
    pub fn normal(self) -> &'a mut W {
        self.variant(P1_16ODW::NORMAL)
    }
    #[doc = "Open-drain. P1.16 pin is in the open drain mode."]
    #[inline]
    pub fn open_drain(self) -> &'a mut W {
        self.variant(P1_16ODW::OPEN_DRAIN)
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
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `P1_17OD`"]
pub enum P1_17ODW {
    #[doc = "Normal. P1.17 pin is in the normal (not open drain) mode."]
    NORMAL,
    #[doc = "Open-drain. P1.17 pin is in the open drain mode."]
    OPEN_DRAIN,
}
impl P1_17ODW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            P1_17ODW::NORMAL => false,
            P1_17ODW::OPEN_DRAIN => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _P1_17ODW<'a> {
    w: &'a mut W,
}
impl<'a> _P1_17ODW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: P1_17ODW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Normal. P1.17 pin is in the normal (not open drain) mode."]
    #[inline]
    pub fn normal(self) -> &'a mut W {
        self.variant(P1_17ODW::NORMAL)
    }
    #[doc = "Open-drain. P1.17 pin is in the open drain mode."]
    #[inline]
    pub fn open_drain(self) -> &'a mut W {
        self.variant(P1_17ODW::OPEN_DRAIN)
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
#[doc = "Values that can be written to the field `P1_18OD`"]
pub enum P1_18ODW {
    #[doc = "Normal. P1.18 pin is in the normal (not open drain) mode."]
    NORMAL,
    #[doc = "Open-drain. P1.18 pin is in the open drain mode."]
    OPEN_DRAIN,
}
impl P1_18ODW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            P1_18ODW::NORMAL => false,
            P1_18ODW::OPEN_DRAIN => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _P1_18ODW<'a> {
    w: &'a mut W,
}
impl<'a> _P1_18ODW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: P1_18ODW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Normal. P1.18 pin is in the normal (not open drain) mode."]
    #[inline]
    pub fn normal(self) -> &'a mut W {
        self.variant(P1_18ODW::NORMAL)
    }
    #[doc = "Open-drain. P1.18 pin is in the open drain mode."]
    #[inline]
    pub fn open_drain(self) -> &'a mut W {
        self.variant(P1_18ODW::OPEN_DRAIN)
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
        const OFFSET: u8 = 18;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `P1_19OD`"]
pub enum P1_19ODW {
    #[doc = "Normal. P1.19 pin is in the normal (not open drain) mode."]
    NORMAL,
    #[doc = "Open-drain. P1.19 pin is in the open drain mode."]
    OPEN_DRAIN,
}
impl P1_19ODW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            P1_19ODW::NORMAL => false,
            P1_19ODW::OPEN_DRAIN => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _P1_19ODW<'a> {
    w: &'a mut W,
}
impl<'a> _P1_19ODW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: P1_19ODW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Normal. P1.19 pin is in the normal (not open drain) mode."]
    #[inline]
    pub fn normal(self) -> &'a mut W {
        self.variant(P1_19ODW::NORMAL)
    }
    #[doc = "Open-drain. P1.19 pin is in the open drain mode."]
    #[inline]
    pub fn open_drain(self) -> &'a mut W {
        self.variant(P1_19ODW::OPEN_DRAIN)
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
        const OFFSET: u8 = 19;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `P1_20OD`"]
pub enum P1_20ODW {
    #[doc = "Normal. P1.20 pin is in the normal (not open drain) mode."]
    NORMAL,
    #[doc = "Open-drain. P1.20 pin is in the open drain mode."]
    OPEN_DRAIN,
}
impl P1_20ODW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            P1_20ODW::NORMAL => false,
            P1_20ODW::OPEN_DRAIN => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _P1_20ODW<'a> {
    w: &'a mut W,
}
impl<'a> _P1_20ODW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: P1_20ODW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Normal. P1.20 pin is in the normal (not open drain) mode."]
    #[inline]
    pub fn normal(self) -> &'a mut W {
        self.variant(P1_20ODW::NORMAL)
    }
    #[doc = "Open-drain. P1.20 pin is in the open drain mode."]
    #[inline]
    pub fn open_drain(self) -> &'a mut W {
        self.variant(P1_20ODW::OPEN_DRAIN)
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
        const OFFSET: u8 = 20;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `P1_21OD`"]
pub enum P1_21ODW {
    #[doc = "Normal. P1.21 pin is in the normal (not open drain) mode."]
    NORMAL,
    #[doc = "Open-drain. P1.21 pin is in the open drain mode."]
    OPEN_DRAIN,
}
impl P1_21ODW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            P1_21ODW::NORMAL => false,
            P1_21ODW::OPEN_DRAIN => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _P1_21ODW<'a> {
    w: &'a mut W,
}
impl<'a> _P1_21ODW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: P1_21ODW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Normal. P1.21 pin is in the normal (not open drain) mode."]
    #[inline]
    pub fn normal(self) -> &'a mut W {
        self.variant(P1_21ODW::NORMAL)
    }
    #[doc = "Open-drain. P1.21 pin is in the open drain mode."]
    #[inline]
    pub fn open_drain(self) -> &'a mut W {
        self.variant(P1_21ODW::OPEN_DRAIN)
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
        const OFFSET: u8 = 21;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `P1_22OD`"]
pub enum P1_22ODW {
    #[doc = "Normal. P1.22 pin is in the normal (not open drain) mode."]
    NORMAL,
    #[doc = "Open-drain. P1.22 pin is in the open drain mode."]
    OPEN_DRAIN,
}
impl P1_22ODW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            P1_22ODW::NORMAL => false,
            P1_22ODW::OPEN_DRAIN => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _P1_22ODW<'a> {
    w: &'a mut W,
}
impl<'a> _P1_22ODW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: P1_22ODW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Normal. P1.22 pin is in the normal (not open drain) mode."]
    #[inline]
    pub fn normal(self) -> &'a mut W {
        self.variant(P1_22ODW::NORMAL)
    }
    #[doc = "Open-drain. P1.22 pin is in the open drain mode."]
    #[inline]
    pub fn open_drain(self) -> &'a mut W {
        self.variant(P1_22ODW::OPEN_DRAIN)
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
        const OFFSET: u8 = 22;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `P1_23OD`"]
pub enum P1_23ODW {
    #[doc = "Normal. P1.23 pin is in the normal (not open drain) mode."]
    NORMAL,
    #[doc = "Open-drain. P1.23 pin is in the open drain mode."]
    OPEN_DRAIN,
}
impl P1_23ODW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            P1_23ODW::NORMAL => false,
            P1_23ODW::OPEN_DRAIN => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _P1_23ODW<'a> {
    w: &'a mut W,
}
impl<'a> _P1_23ODW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: P1_23ODW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Normal. P1.23 pin is in the normal (not open drain) mode."]
    #[inline]
    pub fn normal(self) -> &'a mut W {
        self.variant(P1_23ODW::NORMAL)
    }
    #[doc = "Open-drain. P1.23 pin is in the open drain mode."]
    #[inline]
    pub fn open_drain(self) -> &'a mut W {
        self.variant(P1_23ODW::OPEN_DRAIN)
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
        const OFFSET: u8 = 23;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `P1_24OD`"]
pub enum P1_24ODW {
    #[doc = "Normal. P1.24 pin is in the normal (not open drain) mode."]
    NORMAL,
    #[doc = "Open-drain. P1.24 pin is in the open drain mode."]
    OPEN_DRAIN,
}
impl P1_24ODW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            P1_24ODW::NORMAL => false,
            P1_24ODW::OPEN_DRAIN => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _P1_24ODW<'a> {
    w: &'a mut W,
}
impl<'a> _P1_24ODW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: P1_24ODW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Normal. P1.24 pin is in the normal (not open drain) mode."]
    #[inline]
    pub fn normal(self) -> &'a mut W {
        self.variant(P1_24ODW::NORMAL)
    }
    #[doc = "Open-drain. P1.24 pin is in the open drain mode."]
    #[inline]
    pub fn open_drain(self) -> &'a mut W {
        self.variant(P1_24ODW::OPEN_DRAIN)
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
        const OFFSET: u8 = 24;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `P1_25OD`"]
pub enum P1_25ODW {
    #[doc = "Normal. P1.25 pin is in the normal (not open drain) mode."]
    NORMAL,
    #[doc = "Open-drain. P1.25 pin is in the open drain mode."]
    OPEN_DRAIN,
}
impl P1_25ODW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            P1_25ODW::NORMAL => false,
            P1_25ODW::OPEN_DRAIN => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _P1_25ODW<'a> {
    w: &'a mut W,
}
impl<'a> _P1_25ODW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: P1_25ODW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Normal. P1.25 pin is in the normal (not open drain) mode."]
    #[inline]
    pub fn normal(self) -> &'a mut W {
        self.variant(P1_25ODW::NORMAL)
    }
    #[doc = "Open-drain. P1.25 pin is in the open drain mode."]
    #[inline]
    pub fn open_drain(self) -> &'a mut W {
        self.variant(P1_25ODW::OPEN_DRAIN)
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
#[doc = "Values that can be written to the field `P1_26OD`"]
pub enum P1_26ODW {
    #[doc = "Normal. P1.26 pin is in the normal (not open drain) mode."]
    NORMAL,
    #[doc = "Open-drain. P1.26 pin is in the open drain mode."]
    OPEN_DRAIN,
}
impl P1_26ODW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            P1_26ODW::NORMAL => false,
            P1_26ODW::OPEN_DRAIN => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _P1_26ODW<'a> {
    w: &'a mut W,
}
impl<'a> _P1_26ODW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: P1_26ODW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Normal. P1.26 pin is in the normal (not open drain) mode."]
    #[inline]
    pub fn normal(self) -> &'a mut W {
        self.variant(P1_26ODW::NORMAL)
    }
    #[doc = "Open-drain. P1.26 pin is in the open drain mode."]
    #[inline]
    pub fn open_drain(self) -> &'a mut W {
        self.variant(P1_26ODW::OPEN_DRAIN)
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
        const OFFSET: u8 = 26;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `P1_27OD`"]
pub enum P1_27ODW {
    #[doc = "Normal. P1.27 pin is in the normal (not open drain) mode."]
    NORMAL,
    #[doc = "Open-drain. P1.27 pin is in the open drain mode."]
    OPEN_DRAIN,
}
impl P1_27ODW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            P1_27ODW::NORMAL => false,
            P1_27ODW::OPEN_DRAIN => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _P1_27ODW<'a> {
    w: &'a mut W,
}
impl<'a> _P1_27ODW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: P1_27ODW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Normal. P1.27 pin is in the normal (not open drain) mode."]
    #[inline]
    pub fn normal(self) -> &'a mut W {
        self.variant(P1_27ODW::NORMAL)
    }
    #[doc = "Open-drain. P1.27 pin is in the open drain mode."]
    #[inline]
    pub fn open_drain(self) -> &'a mut W {
        self.variant(P1_27ODW::OPEN_DRAIN)
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
        const OFFSET: u8 = 27;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `P1_28OD`"]
pub enum P1_28ODW {
    #[doc = "Normal. P1.28 pin is in the normal (not open drain) mode."]
    NORMAL,
    #[doc = "Open-drain. P1.28 pin is in the open drain mode."]
    OPEN_DRAIN,
}
impl P1_28ODW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            P1_28ODW::NORMAL => false,
            P1_28ODW::OPEN_DRAIN => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _P1_28ODW<'a> {
    w: &'a mut W,
}
impl<'a> _P1_28ODW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: P1_28ODW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Normal. P1.28 pin is in the normal (not open drain) mode."]
    #[inline]
    pub fn normal(self) -> &'a mut W {
        self.variant(P1_28ODW::NORMAL)
    }
    #[doc = "Open-drain. P1.28 pin is in the open drain mode."]
    #[inline]
    pub fn open_drain(self) -> &'a mut W {
        self.variant(P1_28ODW::OPEN_DRAIN)
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
        const OFFSET: u8 = 28;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `P1_29OD`"]
pub enum P1_29ODW {
    #[doc = "Normal. P1.29 pin is in the normal (not open drain) mode."]
    NORMAL,
    #[doc = "Open-drain. P1.29 pin is in the open drain mode."]
    OPEN_DRAIN,
}
impl P1_29ODW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            P1_29ODW::NORMAL => false,
            P1_29ODW::OPEN_DRAIN => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _P1_29ODW<'a> {
    w: &'a mut W,
}
impl<'a> _P1_29ODW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: P1_29ODW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Normal. P1.29 pin is in the normal (not open drain) mode."]
    #[inline]
    pub fn normal(self) -> &'a mut W {
        self.variant(P1_29ODW::NORMAL)
    }
    #[doc = "Open-drain. P1.29 pin is in the open drain mode."]
    #[inline]
    pub fn open_drain(self) -> &'a mut W {
        self.variant(P1_29ODW::OPEN_DRAIN)
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
        const OFFSET: u8 = 29;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `P1_30OD`"]
pub enum P1_30ODW {
    #[doc = "Normal. P1.30 pin is in the normal (not open drain) mode."]
    NORMAL,
    #[doc = "Open-drain. P1.30 pin is in the open drain mode."]
    OPEN_DRAIN,
}
impl P1_30ODW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            P1_30ODW::NORMAL => false,
            P1_30ODW::OPEN_DRAIN => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _P1_30ODW<'a> {
    w: &'a mut W,
}
impl<'a> _P1_30ODW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: P1_30ODW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Normal. P1.30 pin is in the normal (not open drain) mode."]
    #[inline]
    pub fn normal(self) -> &'a mut W {
        self.variant(P1_30ODW::NORMAL)
    }
    #[doc = "Open-drain. P1.30 pin is in the open drain mode."]
    #[inline]
    pub fn open_drain(self) -> &'a mut W {
        self.variant(P1_30ODW::OPEN_DRAIN)
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
        const OFFSET: u8 = 30;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `P1_31OD`"]
pub enum P1_31ODW {
    #[doc = "Normal. P1.31 pin is in the normal (not open drain) mode."]
    NORMAL,
    #[doc = "Open-drain. P1.31 pin is in the open drain mode."]
    OPEN_DRAIN,
}
impl P1_31ODW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            P1_31ODW::NORMAL => false,
            P1_31ODW::OPEN_DRAIN => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _P1_31ODW<'a> {
    w: &'a mut W,
}
impl<'a> _P1_31ODW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: P1_31ODW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Normal. P1.31 pin is in the normal (not open drain) mode."]
    #[inline]
    pub fn normal(self) -> &'a mut W {
        self.variant(P1_31ODW::NORMAL)
    }
    #[doc = "Open-drain. P1.31 pin is in the open drain mode."]
    #[inline]
    pub fn open_drain(self) -> &'a mut W {
        self.variant(P1_31ODW::OPEN_DRAIN)
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
        const OFFSET: u8 = 31;
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
    #[doc = "Bit 0 - Port 1 pin 0 open drain mode control."]
    #[inline]
    pub fn p1_00od(&self) -> P1_00ODR {
        P1_00ODR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Port 1 pin 1 open drain mode control, see P1.00OD"]
    #[inline]
    pub fn p1_01od(&self) -> P1_01ODR {
        P1_01ODR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - Port 1 pin 4 open drain mode control, see P1.00OD"]
    #[inline]
    pub fn p1_04od(&self) -> P1_04ODR {
        P1_04ODR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 8 - Port 1 pin 8 open drain mode control, see P1.00OD"]
    #[inline]
    pub fn p1_08od(&self) -> P1_08ODR {
        P1_08ODR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 9 - Port 1 pin 9 open drain mode control, see P1.00OD"]
    #[inline]
    pub fn p1_09od(&self) -> P1_09ODR {
        P1_09ODR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 10 - Port 1 pin 10 open drain mode control, see P1.00OD"]
    #[inline]
    pub fn p1_10od(&self) -> P1_10ODR {
        P1_10ODR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 14 - Port 1 pin 14 open drain mode control, see P1.00OD"]
    #[inline]
    pub fn p1_14od(&self) -> P1_14ODR {
        P1_14ODR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 15 - Port 1 pin 15 open drain mode control, see P1.00OD"]
    #[inline]
    pub fn p1_15od(&self) -> P1_15ODR {
        P1_15ODR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 16 - Port 1 pin 16 open drain mode control, see P1.00OD"]
    #[inline]
    pub fn p1_16od(&self) -> P1_16ODR {
        P1_16ODR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 17 - Port 1 pin 17 open drain mode control, see P1.00OD"]
    #[inline]
    pub fn p1_17od(&self) -> P1_17ODR {
        P1_17ODR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 18 - Port 1 pin 18 open drain mode control, see P1.00OD"]
    #[inline]
    pub fn p1_18od(&self) -> P1_18ODR {
        P1_18ODR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 19 - Port 1 pin 19 open drain mode control, see P1.00OD"]
    #[inline]
    pub fn p1_19od(&self) -> P1_19ODR {
        P1_19ODR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 19;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 20 - Port 1 pin 20open drain mode control, see P1.00OD"]
    #[inline]
    pub fn p1_20od(&self) -> P1_20ODR {
        P1_20ODR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 21 - Port 1 pin 21 open drain mode control, see P1.00OD"]
    #[inline]
    pub fn p1_21od(&self) -> P1_21ODR {
        P1_21ODR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 21;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 22 - Port 1 pin 22 open drain mode control, see P1.00OD"]
    #[inline]
    pub fn p1_22od(&self) -> P1_22ODR {
        P1_22ODR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 22;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 23 - Port 1 pin 23 open drain mode control, see P1.00OD"]
    #[inline]
    pub fn p1_23od(&self) -> P1_23ODR {
        P1_23ODR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 23;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 24 - Port 1 pin 24open drain mode control, see P1.00OD"]
    #[inline]
    pub fn p1_24od(&self) -> P1_24ODR {
        P1_24ODR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 25 - Port 1 pin 25 open drain mode control, see P1.00OD"]
    #[inline]
    pub fn p1_25od(&self) -> P1_25ODR {
        P1_25ODR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 25;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 26 - Port 1 pin 26 open drain mode control, see P1.00OD"]
    #[inline]
    pub fn p1_26od(&self) -> P1_26ODR {
        P1_26ODR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 26;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 27 - Port 1 pin 27 open drain mode control, see P1.00OD"]
    #[inline]
    pub fn p1_27od(&self) -> P1_27ODR {
        P1_27ODR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 27;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 28 - Port 1 pin 28 open drain mode control, see P1.00OD"]
    #[inline]
    pub fn p1_28od(&self) -> P1_28ODR {
        P1_28ODR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 29 - Port 1 pin 29 open drain mode control, see P1.00OD"]
    #[inline]
    pub fn p1_29od(&self) -> P1_29ODR {
        P1_29ODR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 29;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 30 - Port 1 pin 30 open drain mode control, see P1.00OD"]
    #[inline]
    pub fn p1_30od(&self) -> P1_30ODR {
        P1_30ODR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 30;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 31 - Port 1 pin 31 open drain mode control."]
    #[inline]
    pub fn p1_31od(&self) -> P1_31ODR {
        P1_31ODR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 31;
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
    #[doc = "Bit 0 - Port 1 pin 0 open drain mode control."]
    #[inline]
    pub fn p1_00od(&mut self) -> _P1_00ODW {
        _P1_00ODW { w: self }
    }
    #[doc = "Bit 1 - Port 1 pin 1 open drain mode control, see P1.00OD"]
    #[inline]
    pub fn p1_01od(&mut self) -> _P1_01ODW {
        _P1_01ODW { w: self }
    }
    #[doc = "Bit 4 - Port 1 pin 4 open drain mode control, see P1.00OD"]
    #[inline]
    pub fn p1_04od(&mut self) -> _P1_04ODW {
        _P1_04ODW { w: self }
    }
    #[doc = "Bit 8 - Port 1 pin 8 open drain mode control, see P1.00OD"]
    #[inline]
    pub fn p1_08od(&mut self) -> _P1_08ODW {
        _P1_08ODW { w: self }
    }
    #[doc = "Bit 9 - Port 1 pin 9 open drain mode control, see P1.00OD"]
    #[inline]
    pub fn p1_09od(&mut self) -> _P1_09ODW {
        _P1_09ODW { w: self }
    }
    #[doc = "Bit 10 - Port 1 pin 10 open drain mode control, see P1.00OD"]
    #[inline]
    pub fn p1_10od(&mut self) -> _P1_10ODW {
        _P1_10ODW { w: self }
    }
    #[doc = "Bit 14 - Port 1 pin 14 open drain mode control, see P1.00OD"]
    #[inline]
    pub fn p1_14od(&mut self) -> _P1_14ODW {
        _P1_14ODW { w: self }
    }
    #[doc = "Bit 15 - Port 1 pin 15 open drain mode control, see P1.00OD"]
    #[inline]
    pub fn p1_15od(&mut self) -> _P1_15ODW {
        _P1_15ODW { w: self }
    }
    #[doc = "Bit 16 - Port 1 pin 16 open drain mode control, see P1.00OD"]
    #[inline]
    pub fn p1_16od(&mut self) -> _P1_16ODW {
        _P1_16ODW { w: self }
    }
    #[doc = "Bit 17 - Port 1 pin 17 open drain mode control, see P1.00OD"]
    #[inline]
    pub fn p1_17od(&mut self) -> _P1_17ODW {
        _P1_17ODW { w: self }
    }
    #[doc = "Bit 18 - Port 1 pin 18 open drain mode control, see P1.00OD"]
    #[inline]
    pub fn p1_18od(&mut self) -> _P1_18ODW {
        _P1_18ODW { w: self }
    }
    #[doc = "Bit 19 - Port 1 pin 19 open drain mode control, see P1.00OD"]
    #[inline]
    pub fn p1_19od(&mut self) -> _P1_19ODW {
        _P1_19ODW { w: self }
    }
    #[doc = "Bit 20 - Port 1 pin 20open drain mode control, see P1.00OD"]
    #[inline]
    pub fn p1_20od(&mut self) -> _P1_20ODW {
        _P1_20ODW { w: self }
    }
    #[doc = "Bit 21 - Port 1 pin 21 open drain mode control, see P1.00OD"]
    #[inline]
    pub fn p1_21od(&mut self) -> _P1_21ODW {
        _P1_21ODW { w: self }
    }
    #[doc = "Bit 22 - Port 1 pin 22 open drain mode control, see P1.00OD"]
    #[inline]
    pub fn p1_22od(&mut self) -> _P1_22ODW {
        _P1_22ODW { w: self }
    }
    #[doc = "Bit 23 - Port 1 pin 23 open drain mode control, see P1.00OD"]
    #[inline]
    pub fn p1_23od(&mut self) -> _P1_23ODW {
        _P1_23ODW { w: self }
    }
    #[doc = "Bit 24 - Port 1 pin 24open drain mode control, see P1.00OD"]
    #[inline]
    pub fn p1_24od(&mut self) -> _P1_24ODW {
        _P1_24ODW { w: self }
    }
    #[doc = "Bit 25 - Port 1 pin 25 open drain mode control, see P1.00OD"]
    #[inline]
    pub fn p1_25od(&mut self) -> _P1_25ODW {
        _P1_25ODW { w: self }
    }
    #[doc = "Bit 26 - Port 1 pin 26 open drain mode control, see P1.00OD"]
    #[inline]
    pub fn p1_26od(&mut self) -> _P1_26ODW {
        _P1_26ODW { w: self }
    }
    #[doc = "Bit 27 - Port 1 pin 27 open drain mode control, see P1.00OD"]
    #[inline]
    pub fn p1_27od(&mut self) -> _P1_27ODW {
        _P1_27ODW { w: self }
    }
    #[doc = "Bit 28 - Port 1 pin 28 open drain mode control, see P1.00OD"]
    #[inline]
    pub fn p1_28od(&mut self) -> _P1_28ODW {
        _P1_28ODW { w: self }
    }
    #[doc = "Bit 29 - Port 1 pin 29 open drain mode control, see P1.00OD"]
    #[inline]
    pub fn p1_29od(&mut self) -> _P1_29ODW {
        _P1_29ODW { w: self }
    }
    #[doc = "Bit 30 - Port 1 pin 30 open drain mode control, see P1.00OD"]
    #[inline]
    pub fn p1_30od(&mut self) -> _P1_30ODW {
        _P1_30ODW { w: self }
    }
    #[doc = "Bit 31 - Port 1 pin 31 open drain mode control."]
    #[inline]
    pub fn p1_31od(&mut self) -> _P1_31ODW {
        _P1_31ODW { w: self }
    }
}
