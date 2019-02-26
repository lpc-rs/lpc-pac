#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CHAN_THRSEL {
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
#[doc = "Possible values of the field `CH0_THRSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH0_THRSELR {
    #[doc = "Threshold 0. Channel 0 results will be compared against the threshold levels indicated in the THR0_LOW and THR0_HIGH registers"]
    THRESHOLD_0,
    #[doc = "Threshold 1. Channel 0 results will be compared against the threshold levels indicated in the THR1_LOW and THR1_HIGH registers"]
    THRESHOLD_1,
}
impl CH0_THRSELR {
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
            CH0_THRSELR::THRESHOLD_0 => false,
            CH0_THRSELR::THRESHOLD_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CH0_THRSELR {
        match value {
            false => CH0_THRSELR::THRESHOLD_0,
            true => CH0_THRSELR::THRESHOLD_1,
        }
    }
    #[doc = "Checks if the value of the field is `THRESHOLD_0`"]
    #[inline]
    pub fn is_threshold_0(&self) -> bool {
        *self == CH0_THRSELR::THRESHOLD_0
    }
    #[doc = "Checks if the value of the field is `THRESHOLD_1`"]
    #[inline]
    pub fn is_threshold_1(&self) -> bool {
        *self == CH0_THRSELR::THRESHOLD_1
    }
}
#[doc = "Possible values of the field `CH1_THRSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH1_THRSELR {
    #[doc = "Threshold 0. Channel 1 results will be compared against the threshold levels indicated in the THR0_LOW and THR0_HIGH registers"]
    THRESHOLD_0,
    #[doc = "Threshold 1. Channel 1 results will be compared against the threshold levels indicated in the THR1_LOW and THR1_HIGH registers"]
    THRESHOLD_1,
}
impl CH1_THRSELR {
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
            CH1_THRSELR::THRESHOLD_0 => false,
            CH1_THRSELR::THRESHOLD_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CH1_THRSELR {
        match value {
            false => CH1_THRSELR::THRESHOLD_0,
            true => CH1_THRSELR::THRESHOLD_1,
        }
    }
    #[doc = "Checks if the value of the field is `THRESHOLD_0`"]
    #[inline]
    pub fn is_threshold_0(&self) -> bool {
        *self == CH1_THRSELR::THRESHOLD_0
    }
    #[doc = "Checks if the value of the field is `THRESHOLD_1`"]
    #[inline]
    pub fn is_threshold_1(&self) -> bool {
        *self == CH1_THRSELR::THRESHOLD_1
    }
}
#[doc = "Possible values of the field `CH2_THRSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH2_THRSELR {
    #[doc = "Threshold 0. Channel 2 results will be compared against the threshold levels indicated in the THR0_LOW and THR0_HIGH registers"]
    THRESHOLD_0,
    #[doc = "Threshold 1. Channel 2 results will be compared against the threshold levels indicated in the THR1_LOW and THR1_HIGH registers"]
    THRESHOLD_1,
}
impl CH2_THRSELR {
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
            CH2_THRSELR::THRESHOLD_0 => false,
            CH2_THRSELR::THRESHOLD_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CH2_THRSELR {
        match value {
            false => CH2_THRSELR::THRESHOLD_0,
            true => CH2_THRSELR::THRESHOLD_1,
        }
    }
    #[doc = "Checks if the value of the field is `THRESHOLD_0`"]
    #[inline]
    pub fn is_threshold_0(&self) -> bool {
        *self == CH2_THRSELR::THRESHOLD_0
    }
    #[doc = "Checks if the value of the field is `THRESHOLD_1`"]
    #[inline]
    pub fn is_threshold_1(&self) -> bool {
        *self == CH2_THRSELR::THRESHOLD_1
    }
}
#[doc = "Possible values of the field `CH3_THRSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH3_THRSELR {
    #[doc = "Threshold 0. Channel 3 results will be compared against the threshold levels indicated in the THR0_LOW and THR0_HIGH registers"]
    THRESHOLD_0,
    #[doc = "Threshold 1. Channel 3 results will be compared against the threshold levels indicated in the THR1_LOW and THR1_HIGH registers"]
    THRESHOLD_1,
}
impl CH3_THRSELR {
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
            CH3_THRSELR::THRESHOLD_0 => false,
            CH3_THRSELR::THRESHOLD_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CH3_THRSELR {
        match value {
            false => CH3_THRSELR::THRESHOLD_0,
            true => CH3_THRSELR::THRESHOLD_1,
        }
    }
    #[doc = "Checks if the value of the field is `THRESHOLD_0`"]
    #[inline]
    pub fn is_threshold_0(&self) -> bool {
        *self == CH3_THRSELR::THRESHOLD_0
    }
    #[doc = "Checks if the value of the field is `THRESHOLD_1`"]
    #[inline]
    pub fn is_threshold_1(&self) -> bool {
        *self == CH3_THRSELR::THRESHOLD_1
    }
}
#[doc = "Possible values of the field `CH4_THRSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH4_THRSELR {
    #[doc = "Threshold 0. Channel 4 results will be compared against the threshold levels indicated in the THR0_LOW and THR0_HIGH registers"]
    THRESHOLD_0,
    #[doc = "Threshold 1. Channel 4 results will be compared against the threshold levels indicated in the THR1_LOW and THR1_HIGH registers"]
    THRESHOLD_1,
}
impl CH4_THRSELR {
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
            CH4_THRSELR::THRESHOLD_0 => false,
            CH4_THRSELR::THRESHOLD_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CH4_THRSELR {
        match value {
            false => CH4_THRSELR::THRESHOLD_0,
            true => CH4_THRSELR::THRESHOLD_1,
        }
    }
    #[doc = "Checks if the value of the field is `THRESHOLD_0`"]
    #[inline]
    pub fn is_threshold_0(&self) -> bool {
        *self == CH4_THRSELR::THRESHOLD_0
    }
    #[doc = "Checks if the value of the field is `THRESHOLD_1`"]
    #[inline]
    pub fn is_threshold_1(&self) -> bool {
        *self == CH4_THRSELR::THRESHOLD_1
    }
}
#[doc = "Possible values of the field `CH5_THRSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH5_THRSELR {
    #[doc = "Threshold 0. Channel 5 results will be compared against the threshold levels indicated in the THR0_LOW and THR0_HIGH registers"]
    THRESHOLD_0,
    #[doc = "Threshold 1. Channel 5 results will be compared against the threshold levels indicated in the THR1_LOW and THR1_HIGH registers"]
    THRESHOLD_1,
}
impl CH5_THRSELR {
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
            CH5_THRSELR::THRESHOLD_0 => false,
            CH5_THRSELR::THRESHOLD_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CH5_THRSELR {
        match value {
            false => CH5_THRSELR::THRESHOLD_0,
            true => CH5_THRSELR::THRESHOLD_1,
        }
    }
    #[doc = "Checks if the value of the field is `THRESHOLD_0`"]
    #[inline]
    pub fn is_threshold_0(&self) -> bool {
        *self == CH5_THRSELR::THRESHOLD_0
    }
    #[doc = "Checks if the value of the field is `THRESHOLD_1`"]
    #[inline]
    pub fn is_threshold_1(&self) -> bool {
        *self == CH5_THRSELR::THRESHOLD_1
    }
}
#[doc = "Possible values of the field `CH6_THRSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH6_THRSELR {
    #[doc = "Threshold 0. Channel 6 results will be compared against the threshold levels indicated in the THR0_LOW and THR0_HIGH registers"]
    THRESHOLD_0,
    #[doc = "Threshold 1. Channel 6 results will be compared against the threshold levels indicated in the THR1_LOW and THR1_HIGH registers"]
    THRESHOLD_1,
}
impl CH6_THRSELR {
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
            CH6_THRSELR::THRESHOLD_0 => false,
            CH6_THRSELR::THRESHOLD_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CH6_THRSELR {
        match value {
            false => CH6_THRSELR::THRESHOLD_0,
            true => CH6_THRSELR::THRESHOLD_1,
        }
    }
    #[doc = "Checks if the value of the field is `THRESHOLD_0`"]
    #[inline]
    pub fn is_threshold_0(&self) -> bool {
        *self == CH6_THRSELR::THRESHOLD_0
    }
    #[doc = "Checks if the value of the field is `THRESHOLD_1`"]
    #[inline]
    pub fn is_threshold_1(&self) -> bool {
        *self == CH6_THRSELR::THRESHOLD_1
    }
}
#[doc = "Possible values of the field `CH7_THRSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH7_THRSELR {
    #[doc = "Threshold 0. Channel 7 results will be compared against the threshold levels indicated in the THR0_LOW and THR0_HIGH registers"]
    THRESHOLD_0,
    #[doc = "Threshold 1. Channel 7 results will be compared against the threshold levels indicated in the THR1_LOW and THR1_HIGH registers"]
    THRESHOLD_1,
}
impl CH7_THRSELR {
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
            CH7_THRSELR::THRESHOLD_0 => false,
            CH7_THRSELR::THRESHOLD_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CH7_THRSELR {
        match value {
            false => CH7_THRSELR::THRESHOLD_0,
            true => CH7_THRSELR::THRESHOLD_1,
        }
    }
    #[doc = "Checks if the value of the field is `THRESHOLD_0`"]
    #[inline]
    pub fn is_threshold_0(&self) -> bool {
        *self == CH7_THRSELR::THRESHOLD_0
    }
    #[doc = "Checks if the value of the field is `THRESHOLD_1`"]
    #[inline]
    pub fn is_threshold_1(&self) -> bool {
        *self == CH7_THRSELR::THRESHOLD_1
    }
}
#[doc = "Possible values of the field `CH8_THRSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH8_THRSELR {
    #[doc = "Threshold 0. Channel 8 results will be compared against the threshold levels indicated in the THR0_LOW and THR0_HIGH registers"]
    THRESHOLD_0,
    #[doc = "Threshold 1. Channel 8 results will be compared against the threshold levels indicated in the THR1_LOW and THR1_HIGH registers"]
    THRESHOLD_1,
}
impl CH8_THRSELR {
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
            CH8_THRSELR::THRESHOLD_0 => false,
            CH8_THRSELR::THRESHOLD_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CH8_THRSELR {
        match value {
            false => CH8_THRSELR::THRESHOLD_0,
            true => CH8_THRSELR::THRESHOLD_1,
        }
    }
    #[doc = "Checks if the value of the field is `THRESHOLD_0`"]
    #[inline]
    pub fn is_threshold_0(&self) -> bool {
        *self == CH8_THRSELR::THRESHOLD_0
    }
    #[doc = "Checks if the value of the field is `THRESHOLD_1`"]
    #[inline]
    pub fn is_threshold_1(&self) -> bool {
        *self == CH8_THRSELR::THRESHOLD_1
    }
}
#[doc = "Possible values of the field `CH9_THRSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH9_THRSELR {
    #[doc = "Threshold 0. Channel 9 results will be compared against the threshold levels indicated in the THR0_LOW and THR0_HIGH registers"]
    THRESHOLD_0,
    #[doc = "Threshold 1. Channel 9 results will be compared against the threshold levels indicated in the THR1_LOW and THR1_HIGH registers"]
    THRESHOLD_1,
}
impl CH9_THRSELR {
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
            CH9_THRSELR::THRESHOLD_0 => false,
            CH9_THRSELR::THRESHOLD_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CH9_THRSELR {
        match value {
            false => CH9_THRSELR::THRESHOLD_0,
            true => CH9_THRSELR::THRESHOLD_1,
        }
    }
    #[doc = "Checks if the value of the field is `THRESHOLD_0`"]
    #[inline]
    pub fn is_threshold_0(&self) -> bool {
        *self == CH9_THRSELR::THRESHOLD_0
    }
    #[doc = "Checks if the value of the field is `THRESHOLD_1`"]
    #[inline]
    pub fn is_threshold_1(&self) -> bool {
        *self == CH9_THRSELR::THRESHOLD_1
    }
}
#[doc = "Possible values of the field `CH10_THRSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH10_THRSELR {
    #[doc = "Threshold 0. Channel 10 results will be compared against the threshold levels indicated in the THR0_LOW and THR0_HIGH registers"]
    THRESHOLD_0,
    #[doc = "Threshold 1. Channel 10 results will be compared against the threshold levels indicated in the THR1_LOW and THR1_HIGH registers"]
    THRESHOLD_1,
}
impl CH10_THRSELR {
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
            CH10_THRSELR::THRESHOLD_0 => false,
            CH10_THRSELR::THRESHOLD_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CH10_THRSELR {
        match value {
            false => CH10_THRSELR::THRESHOLD_0,
            true => CH10_THRSELR::THRESHOLD_1,
        }
    }
    #[doc = "Checks if the value of the field is `THRESHOLD_0`"]
    #[inline]
    pub fn is_threshold_0(&self) -> bool {
        *self == CH10_THRSELR::THRESHOLD_0
    }
    #[doc = "Checks if the value of the field is `THRESHOLD_1`"]
    #[inline]
    pub fn is_threshold_1(&self) -> bool {
        *self == CH10_THRSELR::THRESHOLD_1
    }
}
#[doc = "Possible values of the field `CH11_THRSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH11_THRSELR {
    #[doc = "Threshold 0. Channel 11 results will be compared against the threshold levels indicated in the THR0_LOW and THR0_HIGH registers"]
    THRESHOLD_0,
    #[doc = "Threshold 1. Channel 11 results will be compared against the threshold levels indicated in the THR1_LOW and THR1_HIGH registers"]
    THRESHOLD_1,
}
impl CH11_THRSELR {
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
            CH11_THRSELR::THRESHOLD_0 => false,
            CH11_THRSELR::THRESHOLD_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CH11_THRSELR {
        match value {
            false => CH11_THRSELR::THRESHOLD_0,
            true => CH11_THRSELR::THRESHOLD_1,
        }
    }
    #[doc = "Checks if the value of the field is `THRESHOLD_0`"]
    #[inline]
    pub fn is_threshold_0(&self) -> bool {
        *self == CH11_THRSELR::THRESHOLD_0
    }
    #[doc = "Checks if the value of the field is `THRESHOLD_1`"]
    #[inline]
    pub fn is_threshold_1(&self) -> bool {
        *self == CH11_THRSELR::THRESHOLD_1
    }
}
#[doc = "Values that can be written to the field `CH0_THRSEL`"]
pub enum CH0_THRSELW {
    #[doc = "Threshold 0. Channel 0 results will be compared against the threshold levels indicated in the THR0_LOW and THR0_HIGH registers"]
    THRESHOLD_0,
    #[doc = "Threshold 1. Channel 0 results will be compared against the threshold levels indicated in the THR1_LOW and THR1_HIGH registers"]
    THRESHOLD_1,
}
impl CH0_THRSELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CH0_THRSELW::THRESHOLD_0 => false,
            CH0_THRSELW::THRESHOLD_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CH0_THRSELW<'a> {
    w: &'a mut W,
}
impl<'a> _CH0_THRSELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CH0_THRSELW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Threshold 0. Channel 0 results will be compared against the threshold levels indicated in the THR0_LOW and THR0_HIGH registers"]
    #[inline]
    pub fn threshold_0(self) -> &'a mut W {
        self.variant(CH0_THRSELW::THRESHOLD_0)
    }
    #[doc = "Threshold 1. Channel 0 results will be compared against the threshold levels indicated in the THR1_LOW and THR1_HIGH registers"]
    #[inline]
    pub fn threshold_1(self) -> &'a mut W {
        self.variant(CH0_THRSELW::THRESHOLD_1)
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
#[doc = "Values that can be written to the field `CH1_THRSEL`"]
pub enum CH1_THRSELW {
    #[doc = "Threshold 0. Channel 1 results will be compared against the threshold levels indicated in the THR0_LOW and THR0_HIGH registers"]
    THRESHOLD_0,
    #[doc = "Threshold 1. Channel 1 results will be compared against the threshold levels indicated in the THR1_LOW and THR1_HIGH registers"]
    THRESHOLD_1,
}
impl CH1_THRSELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CH1_THRSELW::THRESHOLD_0 => false,
            CH1_THRSELW::THRESHOLD_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CH1_THRSELW<'a> {
    w: &'a mut W,
}
impl<'a> _CH1_THRSELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CH1_THRSELW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Threshold 0. Channel 1 results will be compared against the threshold levels indicated in the THR0_LOW and THR0_HIGH registers"]
    #[inline]
    pub fn threshold_0(self) -> &'a mut W {
        self.variant(CH1_THRSELW::THRESHOLD_0)
    }
    #[doc = "Threshold 1. Channel 1 results will be compared against the threshold levels indicated in the THR1_LOW and THR1_HIGH registers"]
    #[inline]
    pub fn threshold_1(self) -> &'a mut W {
        self.variant(CH1_THRSELW::THRESHOLD_1)
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
#[doc = "Values that can be written to the field `CH2_THRSEL`"]
pub enum CH2_THRSELW {
    #[doc = "Threshold 0. Channel 2 results will be compared against the threshold levels indicated in the THR0_LOW and THR0_HIGH registers"]
    THRESHOLD_0,
    #[doc = "Threshold 1. Channel 2 results will be compared against the threshold levels indicated in the THR1_LOW and THR1_HIGH registers"]
    THRESHOLD_1,
}
impl CH2_THRSELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CH2_THRSELW::THRESHOLD_0 => false,
            CH2_THRSELW::THRESHOLD_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CH2_THRSELW<'a> {
    w: &'a mut W,
}
impl<'a> _CH2_THRSELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CH2_THRSELW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Threshold 0. Channel 2 results will be compared against the threshold levels indicated in the THR0_LOW and THR0_HIGH registers"]
    #[inline]
    pub fn threshold_0(self) -> &'a mut W {
        self.variant(CH2_THRSELW::THRESHOLD_0)
    }
    #[doc = "Threshold 1. Channel 2 results will be compared against the threshold levels indicated in the THR1_LOW and THR1_HIGH registers"]
    #[inline]
    pub fn threshold_1(self) -> &'a mut W {
        self.variant(CH2_THRSELW::THRESHOLD_1)
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
#[doc = "Values that can be written to the field `CH3_THRSEL`"]
pub enum CH3_THRSELW {
    #[doc = "Threshold 0. Channel 3 results will be compared against the threshold levels indicated in the THR0_LOW and THR0_HIGH registers"]
    THRESHOLD_0,
    #[doc = "Threshold 1. Channel 3 results will be compared against the threshold levels indicated in the THR1_LOW and THR1_HIGH registers"]
    THRESHOLD_1,
}
impl CH3_THRSELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CH3_THRSELW::THRESHOLD_0 => false,
            CH3_THRSELW::THRESHOLD_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CH3_THRSELW<'a> {
    w: &'a mut W,
}
impl<'a> _CH3_THRSELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CH3_THRSELW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Threshold 0. Channel 3 results will be compared against the threshold levels indicated in the THR0_LOW and THR0_HIGH registers"]
    #[inline]
    pub fn threshold_0(self) -> &'a mut W {
        self.variant(CH3_THRSELW::THRESHOLD_0)
    }
    #[doc = "Threshold 1. Channel 3 results will be compared against the threshold levels indicated in the THR1_LOW and THR1_HIGH registers"]
    #[inline]
    pub fn threshold_1(self) -> &'a mut W {
        self.variant(CH3_THRSELW::THRESHOLD_1)
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
#[doc = "Values that can be written to the field `CH4_THRSEL`"]
pub enum CH4_THRSELW {
    #[doc = "Threshold 0. Channel 4 results will be compared against the threshold levels indicated in the THR0_LOW and THR0_HIGH registers"]
    THRESHOLD_0,
    #[doc = "Threshold 1. Channel 4 results will be compared against the threshold levels indicated in the THR1_LOW and THR1_HIGH registers"]
    THRESHOLD_1,
}
impl CH4_THRSELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CH4_THRSELW::THRESHOLD_0 => false,
            CH4_THRSELW::THRESHOLD_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CH4_THRSELW<'a> {
    w: &'a mut W,
}
impl<'a> _CH4_THRSELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CH4_THRSELW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Threshold 0. Channel 4 results will be compared against the threshold levels indicated in the THR0_LOW and THR0_HIGH registers"]
    #[inline]
    pub fn threshold_0(self) -> &'a mut W {
        self.variant(CH4_THRSELW::THRESHOLD_0)
    }
    #[doc = "Threshold 1. Channel 4 results will be compared against the threshold levels indicated in the THR1_LOW and THR1_HIGH registers"]
    #[inline]
    pub fn threshold_1(self) -> &'a mut W {
        self.variant(CH4_THRSELW::THRESHOLD_1)
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
#[doc = "Values that can be written to the field `CH5_THRSEL`"]
pub enum CH5_THRSELW {
    #[doc = "Threshold 0. Channel 5 results will be compared against the threshold levels indicated in the THR0_LOW and THR0_HIGH registers"]
    THRESHOLD_0,
    #[doc = "Threshold 1. Channel 5 results will be compared against the threshold levels indicated in the THR1_LOW and THR1_HIGH registers"]
    THRESHOLD_1,
}
impl CH5_THRSELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CH5_THRSELW::THRESHOLD_0 => false,
            CH5_THRSELW::THRESHOLD_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CH5_THRSELW<'a> {
    w: &'a mut W,
}
impl<'a> _CH5_THRSELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CH5_THRSELW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Threshold 0. Channel 5 results will be compared against the threshold levels indicated in the THR0_LOW and THR0_HIGH registers"]
    #[inline]
    pub fn threshold_0(self) -> &'a mut W {
        self.variant(CH5_THRSELW::THRESHOLD_0)
    }
    #[doc = "Threshold 1. Channel 5 results will be compared against the threshold levels indicated in the THR1_LOW and THR1_HIGH registers"]
    #[inline]
    pub fn threshold_1(self) -> &'a mut W {
        self.variant(CH5_THRSELW::THRESHOLD_1)
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
#[doc = "Values that can be written to the field `CH6_THRSEL`"]
pub enum CH6_THRSELW {
    #[doc = "Threshold 0. Channel 6 results will be compared against the threshold levels indicated in the THR0_LOW and THR0_HIGH registers"]
    THRESHOLD_0,
    #[doc = "Threshold 1. Channel 6 results will be compared against the threshold levels indicated in the THR1_LOW and THR1_HIGH registers"]
    THRESHOLD_1,
}
impl CH6_THRSELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CH6_THRSELW::THRESHOLD_0 => false,
            CH6_THRSELW::THRESHOLD_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CH6_THRSELW<'a> {
    w: &'a mut W,
}
impl<'a> _CH6_THRSELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CH6_THRSELW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Threshold 0. Channel 6 results will be compared against the threshold levels indicated in the THR0_LOW and THR0_HIGH registers"]
    #[inline]
    pub fn threshold_0(self) -> &'a mut W {
        self.variant(CH6_THRSELW::THRESHOLD_0)
    }
    #[doc = "Threshold 1. Channel 6 results will be compared against the threshold levels indicated in the THR1_LOW and THR1_HIGH registers"]
    #[inline]
    pub fn threshold_1(self) -> &'a mut W {
        self.variant(CH6_THRSELW::THRESHOLD_1)
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
#[doc = "Values that can be written to the field `CH7_THRSEL`"]
pub enum CH7_THRSELW {
    #[doc = "Threshold 0. Channel 7 results will be compared against the threshold levels indicated in the THR0_LOW and THR0_HIGH registers"]
    THRESHOLD_0,
    #[doc = "Threshold 1. Channel 7 results will be compared against the threshold levels indicated in the THR1_LOW and THR1_HIGH registers"]
    THRESHOLD_1,
}
impl CH7_THRSELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CH7_THRSELW::THRESHOLD_0 => false,
            CH7_THRSELW::THRESHOLD_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CH7_THRSELW<'a> {
    w: &'a mut W,
}
impl<'a> _CH7_THRSELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CH7_THRSELW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Threshold 0. Channel 7 results will be compared against the threshold levels indicated in the THR0_LOW and THR0_HIGH registers"]
    #[inline]
    pub fn threshold_0(self) -> &'a mut W {
        self.variant(CH7_THRSELW::THRESHOLD_0)
    }
    #[doc = "Threshold 1. Channel 7 results will be compared against the threshold levels indicated in the THR1_LOW and THR1_HIGH registers"]
    #[inline]
    pub fn threshold_1(self) -> &'a mut W {
        self.variant(CH7_THRSELW::THRESHOLD_1)
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
#[doc = "Values that can be written to the field `CH8_THRSEL`"]
pub enum CH8_THRSELW {
    #[doc = "Threshold 0. Channel 8 results will be compared against the threshold levels indicated in the THR0_LOW and THR0_HIGH registers"]
    THRESHOLD_0,
    #[doc = "Threshold 1. Channel 8 results will be compared against the threshold levels indicated in the THR1_LOW and THR1_HIGH registers"]
    THRESHOLD_1,
}
impl CH8_THRSELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CH8_THRSELW::THRESHOLD_0 => false,
            CH8_THRSELW::THRESHOLD_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CH8_THRSELW<'a> {
    w: &'a mut W,
}
impl<'a> _CH8_THRSELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CH8_THRSELW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Threshold 0. Channel 8 results will be compared against the threshold levels indicated in the THR0_LOW and THR0_HIGH registers"]
    #[inline]
    pub fn threshold_0(self) -> &'a mut W {
        self.variant(CH8_THRSELW::THRESHOLD_0)
    }
    #[doc = "Threshold 1. Channel 8 results will be compared against the threshold levels indicated in the THR1_LOW and THR1_HIGH registers"]
    #[inline]
    pub fn threshold_1(self) -> &'a mut W {
        self.variant(CH8_THRSELW::THRESHOLD_1)
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
#[doc = "Values that can be written to the field `CH9_THRSEL`"]
pub enum CH9_THRSELW {
    #[doc = "Threshold 0. Channel 9 results will be compared against the threshold levels indicated in the THR0_LOW and THR0_HIGH registers"]
    THRESHOLD_0,
    #[doc = "Threshold 1. Channel 9 results will be compared against the threshold levels indicated in the THR1_LOW and THR1_HIGH registers"]
    THRESHOLD_1,
}
impl CH9_THRSELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CH9_THRSELW::THRESHOLD_0 => false,
            CH9_THRSELW::THRESHOLD_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CH9_THRSELW<'a> {
    w: &'a mut W,
}
impl<'a> _CH9_THRSELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CH9_THRSELW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Threshold 0. Channel 9 results will be compared against the threshold levels indicated in the THR0_LOW and THR0_HIGH registers"]
    #[inline]
    pub fn threshold_0(self) -> &'a mut W {
        self.variant(CH9_THRSELW::THRESHOLD_0)
    }
    #[doc = "Threshold 1. Channel 9 results will be compared against the threshold levels indicated in the THR1_LOW and THR1_HIGH registers"]
    #[inline]
    pub fn threshold_1(self) -> &'a mut W {
        self.variant(CH9_THRSELW::THRESHOLD_1)
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
#[doc = "Values that can be written to the field `CH10_THRSEL`"]
pub enum CH10_THRSELW {
    #[doc = "Threshold 0. Channel 10 results will be compared against the threshold levels indicated in the THR0_LOW and THR0_HIGH registers"]
    THRESHOLD_0,
    #[doc = "Threshold 1. Channel 10 results will be compared against the threshold levels indicated in the THR1_LOW and THR1_HIGH registers"]
    THRESHOLD_1,
}
impl CH10_THRSELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CH10_THRSELW::THRESHOLD_0 => false,
            CH10_THRSELW::THRESHOLD_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CH10_THRSELW<'a> {
    w: &'a mut W,
}
impl<'a> _CH10_THRSELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CH10_THRSELW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Threshold 0. Channel 10 results will be compared against the threshold levels indicated in the THR0_LOW and THR0_HIGH registers"]
    #[inline]
    pub fn threshold_0(self) -> &'a mut W {
        self.variant(CH10_THRSELW::THRESHOLD_0)
    }
    #[doc = "Threshold 1. Channel 10 results will be compared against the threshold levels indicated in the THR1_LOW and THR1_HIGH registers"]
    #[inline]
    pub fn threshold_1(self) -> &'a mut W {
        self.variant(CH10_THRSELW::THRESHOLD_1)
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
#[doc = "Values that can be written to the field `CH11_THRSEL`"]
pub enum CH11_THRSELW {
    #[doc = "Threshold 0. Channel 11 results will be compared against the threshold levels indicated in the THR0_LOW and THR0_HIGH registers"]
    THRESHOLD_0,
    #[doc = "Threshold 1. Channel 11 results will be compared against the threshold levels indicated in the THR1_LOW and THR1_HIGH registers"]
    THRESHOLD_1,
}
impl CH11_THRSELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CH11_THRSELW::THRESHOLD_0 => false,
            CH11_THRSELW::THRESHOLD_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CH11_THRSELW<'a> {
    w: &'a mut W,
}
impl<'a> _CH11_THRSELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CH11_THRSELW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Threshold 0. Channel 11 results will be compared against the threshold levels indicated in the THR0_LOW and THR0_HIGH registers"]
    #[inline]
    pub fn threshold_0(self) -> &'a mut W {
        self.variant(CH11_THRSELW::THRESHOLD_0)
    }
    #[doc = "Threshold 1. Channel 11 results will be compared against the threshold levels indicated in the THR1_LOW and THR1_HIGH registers"]
    #[inline]
    pub fn threshold_1(self) -> &'a mut W {
        self.variant(CH11_THRSELW::THRESHOLD_1)
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Threshold select by channel."]
    #[inline]
    pub fn ch0_thrsel(&self) -> CH0_THRSELR {
        CH0_THRSELR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Threshold select by channel."]
    #[inline]
    pub fn ch1_thrsel(&self) -> CH1_THRSELR {
        CH1_THRSELR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - Threshold select by channel."]
    #[inline]
    pub fn ch2_thrsel(&self) -> CH2_THRSELR {
        CH2_THRSELR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - Threshold select by channel."]
    #[inline]
    pub fn ch3_thrsel(&self) -> CH3_THRSELR {
        CH3_THRSELR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - Threshold select by channel."]
    #[inline]
    pub fn ch4_thrsel(&self) -> CH4_THRSELR {
        CH4_THRSELR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - Threshold select by channel."]
    #[inline]
    pub fn ch5_thrsel(&self) -> CH5_THRSELR {
        CH5_THRSELR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 6 - Threshold select by channel."]
    #[inline]
    pub fn ch6_thrsel(&self) -> CH6_THRSELR {
        CH6_THRSELR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 7 - Threshold select by channel."]
    #[inline]
    pub fn ch7_thrsel(&self) -> CH7_THRSELR {
        CH7_THRSELR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 8 - Threshold select by channel."]
    #[inline]
    pub fn ch8_thrsel(&self) -> CH8_THRSELR {
        CH8_THRSELR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 9 - Threshold select by channel."]
    #[inline]
    pub fn ch9_thrsel(&self) -> CH9_THRSELR {
        CH9_THRSELR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 10 - Threshold select by channel."]
    #[inline]
    pub fn ch10_thrsel(&self) -> CH10_THRSELR {
        CH10_THRSELR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 11 - Threshold select by channel."]
    #[inline]
    pub fn ch11_thrsel(&self) -> CH11_THRSELR {
        CH11_THRSELR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 11;
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
    #[doc = "Bit 0 - Threshold select by channel."]
    #[inline]
    pub fn ch0_thrsel(&mut self) -> _CH0_THRSELW {
        _CH0_THRSELW { w: self }
    }
    #[doc = "Bit 1 - Threshold select by channel."]
    #[inline]
    pub fn ch1_thrsel(&mut self) -> _CH1_THRSELW {
        _CH1_THRSELW { w: self }
    }
    #[doc = "Bit 2 - Threshold select by channel."]
    #[inline]
    pub fn ch2_thrsel(&mut self) -> _CH2_THRSELW {
        _CH2_THRSELW { w: self }
    }
    #[doc = "Bit 3 - Threshold select by channel."]
    #[inline]
    pub fn ch3_thrsel(&mut self) -> _CH3_THRSELW {
        _CH3_THRSELW { w: self }
    }
    #[doc = "Bit 4 - Threshold select by channel."]
    #[inline]
    pub fn ch4_thrsel(&mut self) -> _CH4_THRSELW {
        _CH4_THRSELW { w: self }
    }
    #[doc = "Bit 5 - Threshold select by channel."]
    #[inline]
    pub fn ch5_thrsel(&mut self) -> _CH5_THRSELW {
        _CH5_THRSELW { w: self }
    }
    #[doc = "Bit 6 - Threshold select by channel."]
    #[inline]
    pub fn ch6_thrsel(&mut self) -> _CH6_THRSELW {
        _CH6_THRSELW { w: self }
    }
    #[doc = "Bit 7 - Threshold select by channel."]
    #[inline]
    pub fn ch7_thrsel(&mut self) -> _CH7_THRSELW {
        _CH7_THRSELW { w: self }
    }
    #[doc = "Bit 8 - Threshold select by channel."]
    #[inline]
    pub fn ch8_thrsel(&mut self) -> _CH8_THRSELW {
        _CH8_THRSELW { w: self }
    }
    #[doc = "Bit 9 - Threshold select by channel."]
    #[inline]
    pub fn ch9_thrsel(&mut self) -> _CH9_THRSELW {
        _CH9_THRSELW { w: self }
    }
    #[doc = "Bit 10 - Threshold select by channel."]
    #[inline]
    pub fn ch10_thrsel(&mut self) -> _CH10_THRSELW {
        _CH10_THRSELW { w: self }
    }
    #[doc = "Bit 11 - Threshold select by channel."]
    #[inline]
    pub fn ch11_thrsel(&mut self) -> _CH11_THRSELW {
        _CH11_THRSELW { w: self }
    }
}
