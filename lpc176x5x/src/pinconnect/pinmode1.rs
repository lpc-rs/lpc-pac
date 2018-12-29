#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::PINMODE1 {
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
#[doc = "Possible values of the field `P0_16MODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P0_16MODER {
    #[doc = "Pull-up. P0.16 pin has a pull-up resistor enabled."]
    PULL_UP,
    #[doc = "Repeater. P0.16 pin has repeater mode enabled."]
    REPEATER,
    #[doc = "Disabled. P0.16 pin has neither pull-up nor pull-down."]
    DISABLED,
    #[doc = "Pull-down. P0.16 has a pull-down resistor enabled."]
    PULL_DOWN,
}
impl P0_16MODER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            P0_16MODER::PULL_UP => 0,
            P0_16MODER::REPEATER => 1,
            P0_16MODER::DISABLED => 2,
            P0_16MODER::PULL_DOWN => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> P0_16MODER {
        match value {
            0 => P0_16MODER::PULL_UP,
            1 => P0_16MODER::REPEATER,
            2 => P0_16MODER::DISABLED,
            3 => P0_16MODER::PULL_DOWN,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PULL_UP`"]
    #[inline]
    pub fn is_pull_up(&self) -> bool {
        *self == P0_16MODER::PULL_UP
    }
    #[doc = "Checks if the value of the field is `REPEATER`"]
    #[inline]
    pub fn is_repeater(&self) -> bool {
        *self == P0_16MODER::REPEATER
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == P0_16MODER::DISABLED
    }
    #[doc = "Checks if the value of the field is `PULL_DOWN`"]
    #[inline]
    pub fn is_pull_down(&self) -> bool {
        *self == P0_16MODER::PULL_DOWN
    }
}
#[doc = "Possible values of the field `P0_17MODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P0_17MODER {
    #[doc = "Pull-up. P0.17 pin has a pull-up resistor enabled."]
    PULL_UP,
    #[doc = "Repeater. P0.17 pin has repeater mode enabled."]
    REPEATER,
    #[doc = "Disabled. P0.17 pin has neither pull-up nor pull-down."]
    DISABLED,
    #[doc = "Pull-down. P0.17 has a pull-down resistor enabled."]
    PULL_DOWN,
}
impl P0_17MODER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            P0_17MODER::PULL_UP => 0,
            P0_17MODER::REPEATER => 1,
            P0_17MODER::DISABLED => 2,
            P0_17MODER::PULL_DOWN => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> P0_17MODER {
        match value {
            0 => P0_17MODER::PULL_UP,
            1 => P0_17MODER::REPEATER,
            2 => P0_17MODER::DISABLED,
            3 => P0_17MODER::PULL_DOWN,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PULL_UP`"]
    #[inline]
    pub fn is_pull_up(&self) -> bool {
        *self == P0_17MODER::PULL_UP
    }
    #[doc = "Checks if the value of the field is `REPEATER`"]
    #[inline]
    pub fn is_repeater(&self) -> bool {
        *self == P0_17MODER::REPEATER
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == P0_17MODER::DISABLED
    }
    #[doc = "Checks if the value of the field is `PULL_DOWN`"]
    #[inline]
    pub fn is_pull_down(&self) -> bool {
        *self == P0_17MODER::PULL_DOWN
    }
}
#[doc = "Possible values of the field `P0_18MODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P0_18MODER {
    #[doc = "Pull-up. P0.18 pin has a pull-up resistor enabled."]
    PULL_UP,
    #[doc = "Repeater. P0.18 pin has repeater mode enabled."]
    REPEATER,
    #[doc = "Disabled. P0.18 pin has neither pull-up nor pull-down."]
    DISABLED,
    #[doc = "Pull-down. P0.18 has a pull-down resistor enabled."]
    PULL_DOWN,
}
impl P0_18MODER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            P0_18MODER::PULL_UP => 0,
            P0_18MODER::REPEATER => 1,
            P0_18MODER::DISABLED => 2,
            P0_18MODER::PULL_DOWN => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> P0_18MODER {
        match value {
            0 => P0_18MODER::PULL_UP,
            1 => P0_18MODER::REPEATER,
            2 => P0_18MODER::DISABLED,
            3 => P0_18MODER::PULL_DOWN,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PULL_UP`"]
    #[inline]
    pub fn is_pull_up(&self) -> bool {
        *self == P0_18MODER::PULL_UP
    }
    #[doc = "Checks if the value of the field is `REPEATER`"]
    #[inline]
    pub fn is_repeater(&self) -> bool {
        *self == P0_18MODER::REPEATER
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == P0_18MODER::DISABLED
    }
    #[doc = "Checks if the value of the field is `PULL_DOWN`"]
    #[inline]
    pub fn is_pull_down(&self) -> bool {
        *self == P0_18MODER::PULL_DOWN
    }
}
#[doc = "Possible values of the field `P0_19MODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P0_19MODER {
    #[doc = "Pull-up. P0.19 pin has a pull-up resistor enabled."]
    PULL_UP,
    #[doc = "Repeater. P0.19 pin has repeater mode enabled."]
    REPEATER,
    #[doc = "Disabled. P0.19 pin has neither pull-up nor pull-down."]
    DISABLED,
    #[doc = "Pull-down. P0.19 has a pull-down resistor enabled."]
    PULL_DOWN,
}
impl P0_19MODER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            P0_19MODER::PULL_UP => 0,
            P0_19MODER::REPEATER => 1,
            P0_19MODER::DISABLED => 2,
            P0_19MODER::PULL_DOWN => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> P0_19MODER {
        match value {
            0 => P0_19MODER::PULL_UP,
            1 => P0_19MODER::REPEATER,
            2 => P0_19MODER::DISABLED,
            3 => P0_19MODER::PULL_DOWN,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PULL_UP`"]
    #[inline]
    pub fn is_pull_up(&self) -> bool {
        *self == P0_19MODER::PULL_UP
    }
    #[doc = "Checks if the value of the field is `REPEATER`"]
    #[inline]
    pub fn is_repeater(&self) -> bool {
        *self == P0_19MODER::REPEATER
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == P0_19MODER::DISABLED
    }
    #[doc = "Checks if the value of the field is `PULL_DOWN`"]
    #[inline]
    pub fn is_pull_down(&self) -> bool {
        *self == P0_19MODER::PULL_DOWN
    }
}
#[doc = "Possible values of the field `P0_20MODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P0_20MODER {
    #[doc = "Pull-up. P0.20 pin has a pull-up resistor enabled."]
    PULL_UP,
    #[doc = "Repeater. P0.20 pin has repeater mode enabled."]
    REPEATER,
    #[doc = "Disabled. P0.20 pin has neither pull-up nor pull-down."]
    DISABLED,
    #[doc = "Pull-down. P0.20 has a pull-down resistor enabled."]
    PULL_DOWN,
}
impl P0_20MODER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            P0_20MODER::PULL_UP => 0,
            P0_20MODER::REPEATER => 1,
            P0_20MODER::DISABLED => 2,
            P0_20MODER::PULL_DOWN => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> P0_20MODER {
        match value {
            0 => P0_20MODER::PULL_UP,
            1 => P0_20MODER::REPEATER,
            2 => P0_20MODER::DISABLED,
            3 => P0_20MODER::PULL_DOWN,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PULL_UP`"]
    #[inline]
    pub fn is_pull_up(&self) -> bool {
        *self == P0_20MODER::PULL_UP
    }
    #[doc = "Checks if the value of the field is `REPEATER`"]
    #[inline]
    pub fn is_repeater(&self) -> bool {
        *self == P0_20MODER::REPEATER
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == P0_20MODER::DISABLED
    }
    #[doc = "Checks if the value of the field is `PULL_DOWN`"]
    #[inline]
    pub fn is_pull_down(&self) -> bool {
        *self == P0_20MODER::PULL_DOWN
    }
}
#[doc = "Possible values of the field `P0_21MODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P0_21MODER {
    #[doc = "Pull-up. P0.21 pin has a pull-up resistor enabled."]
    PULL_UP,
    #[doc = "Repeater. P0.21 pin has repeater mode enabled."]
    REPEATER,
    #[doc = "Disabled. P0.21 pin has neither pull-up nor pull-down."]
    DISABLED,
    #[doc = "Pull-down. P0.21 has a pull-down resistor enabled."]
    PULL_DOWN,
}
impl P0_21MODER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            P0_21MODER::PULL_UP => 0,
            P0_21MODER::REPEATER => 1,
            P0_21MODER::DISABLED => 2,
            P0_21MODER::PULL_DOWN => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> P0_21MODER {
        match value {
            0 => P0_21MODER::PULL_UP,
            1 => P0_21MODER::REPEATER,
            2 => P0_21MODER::DISABLED,
            3 => P0_21MODER::PULL_DOWN,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PULL_UP`"]
    #[inline]
    pub fn is_pull_up(&self) -> bool {
        *self == P0_21MODER::PULL_UP
    }
    #[doc = "Checks if the value of the field is `REPEATER`"]
    #[inline]
    pub fn is_repeater(&self) -> bool {
        *self == P0_21MODER::REPEATER
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == P0_21MODER::DISABLED
    }
    #[doc = "Checks if the value of the field is `PULL_DOWN`"]
    #[inline]
    pub fn is_pull_down(&self) -> bool {
        *self == P0_21MODER::PULL_DOWN
    }
}
#[doc = "Possible values of the field `P0_22MODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P0_22MODER {
    #[doc = "Pull-up. P0.22 pin has a pull-up resistor enabled."]
    PULL_UP,
    #[doc = "Repeater. P0.22 pin has repeater mode enabled."]
    REPEATER,
    #[doc = "Disabled. P0.22 pin has neither pull-up nor pull-down."]
    DISABLED,
    #[doc = "Pull-down. P0.22 has a pull-down resistor enabled."]
    PULL_DOWN,
}
impl P0_22MODER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            P0_22MODER::PULL_UP => 0,
            P0_22MODER::REPEATER => 1,
            P0_22MODER::DISABLED => 2,
            P0_22MODER::PULL_DOWN => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> P0_22MODER {
        match value {
            0 => P0_22MODER::PULL_UP,
            1 => P0_22MODER::REPEATER,
            2 => P0_22MODER::DISABLED,
            3 => P0_22MODER::PULL_DOWN,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PULL_UP`"]
    #[inline]
    pub fn is_pull_up(&self) -> bool {
        *self == P0_22MODER::PULL_UP
    }
    #[doc = "Checks if the value of the field is `REPEATER`"]
    #[inline]
    pub fn is_repeater(&self) -> bool {
        *self == P0_22MODER::REPEATER
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == P0_22MODER::DISABLED
    }
    #[doc = "Checks if the value of the field is `PULL_DOWN`"]
    #[inline]
    pub fn is_pull_down(&self) -> bool {
        *self == P0_22MODER::PULL_DOWN
    }
}
#[doc = "Possible values of the field `P0_23MODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P0_23MODER {
    #[doc = "Pull-up. P0.23 pin has a pull-up resistor enabled."]
    PULL_UP,
    #[doc = "Repeater. P0.23 pin has repeater mode enabled."]
    REPEATER,
    #[doc = "Disabled. P0.23 pin has neither pull-up nor pull-down."]
    DISABLED,
    #[doc = "Pull-down. P0.23 has a pull-down resistor enabled."]
    PULL_DOWN,
}
impl P0_23MODER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            P0_23MODER::PULL_UP => 0,
            P0_23MODER::REPEATER => 1,
            P0_23MODER::DISABLED => 2,
            P0_23MODER::PULL_DOWN => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> P0_23MODER {
        match value {
            0 => P0_23MODER::PULL_UP,
            1 => P0_23MODER::REPEATER,
            2 => P0_23MODER::DISABLED,
            3 => P0_23MODER::PULL_DOWN,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PULL_UP`"]
    #[inline]
    pub fn is_pull_up(&self) -> bool {
        *self == P0_23MODER::PULL_UP
    }
    #[doc = "Checks if the value of the field is `REPEATER`"]
    #[inline]
    pub fn is_repeater(&self) -> bool {
        *self == P0_23MODER::REPEATER
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == P0_23MODER::DISABLED
    }
    #[doc = "Checks if the value of the field is `PULL_DOWN`"]
    #[inline]
    pub fn is_pull_down(&self) -> bool {
        *self == P0_23MODER::PULL_DOWN
    }
}
#[doc = "Possible values of the field `P0_24MODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P0_24MODER {
    #[doc = "Pull-up. P0.24 pin has a pull-up resistor enabled."]
    PULL_UP,
    #[doc = "Repeater. P0.24 pin has repeater mode enabled."]
    REPEATER,
    #[doc = "Disabled. P0.24 pin has neither pull-up nor pull-down."]
    DISABLED,
    #[doc = "Pull-down. P0.24 has a pull-down resistor enabled."]
    PULL_DOWN,
}
impl P0_24MODER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            P0_24MODER::PULL_UP => 0,
            P0_24MODER::REPEATER => 1,
            P0_24MODER::DISABLED => 2,
            P0_24MODER::PULL_DOWN => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> P0_24MODER {
        match value {
            0 => P0_24MODER::PULL_UP,
            1 => P0_24MODER::REPEATER,
            2 => P0_24MODER::DISABLED,
            3 => P0_24MODER::PULL_DOWN,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PULL_UP`"]
    #[inline]
    pub fn is_pull_up(&self) -> bool {
        *self == P0_24MODER::PULL_UP
    }
    #[doc = "Checks if the value of the field is `REPEATER`"]
    #[inline]
    pub fn is_repeater(&self) -> bool {
        *self == P0_24MODER::REPEATER
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == P0_24MODER::DISABLED
    }
    #[doc = "Checks if the value of the field is `PULL_DOWN`"]
    #[inline]
    pub fn is_pull_down(&self) -> bool {
        *self == P0_24MODER::PULL_DOWN
    }
}
#[doc = "Possible values of the field `P0_25MODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P0_25MODER {
    #[doc = "Pull-up. P0.25 pin has a pull-up resistor enabled."]
    PULL_UP,
    #[doc = "Repeater. P0.25 pin has repeater mode enabled."]
    REPEATER,
    #[doc = "Disabled. P0.25 pin has neither pull-up nor pull-down."]
    DISABLED,
    #[doc = "Pull-down. P0.25 has a pull-down resistor enabled."]
    PULL_DOWN,
}
impl P0_25MODER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            P0_25MODER::PULL_UP => 0,
            P0_25MODER::REPEATER => 1,
            P0_25MODER::DISABLED => 2,
            P0_25MODER::PULL_DOWN => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> P0_25MODER {
        match value {
            0 => P0_25MODER::PULL_UP,
            1 => P0_25MODER::REPEATER,
            2 => P0_25MODER::DISABLED,
            3 => P0_25MODER::PULL_DOWN,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PULL_UP`"]
    #[inline]
    pub fn is_pull_up(&self) -> bool {
        *self == P0_25MODER::PULL_UP
    }
    #[doc = "Checks if the value of the field is `REPEATER`"]
    #[inline]
    pub fn is_repeater(&self) -> bool {
        *self == P0_25MODER::REPEATER
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == P0_25MODER::DISABLED
    }
    #[doc = "Checks if the value of the field is `PULL_DOWN`"]
    #[inline]
    pub fn is_pull_down(&self) -> bool {
        *self == P0_25MODER::PULL_DOWN
    }
}
#[doc = "Possible values of the field `P0_26MODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P0_26MODER {
    #[doc = "Pull-up. P0.26 pin has a pull-up resistor enabled."]
    PULL_UP,
    #[doc = "Repeater. P0.26 pin has repeater mode enabled."]
    REPEATER,
    #[doc = "Disabled. P0.26 pin has neither pull-up nor pull-down."]
    DISABLED,
    #[doc = "Pull-down. P0.26 has a pull-down resistor enabled."]
    PULL_DOWN,
}
impl P0_26MODER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            P0_26MODER::PULL_UP => 0,
            P0_26MODER::REPEATER => 1,
            P0_26MODER::DISABLED => 2,
            P0_26MODER::PULL_DOWN => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> P0_26MODER {
        match value {
            0 => P0_26MODER::PULL_UP,
            1 => P0_26MODER::REPEATER,
            2 => P0_26MODER::DISABLED,
            3 => P0_26MODER::PULL_DOWN,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PULL_UP`"]
    #[inline]
    pub fn is_pull_up(&self) -> bool {
        *self == P0_26MODER::PULL_UP
    }
    #[doc = "Checks if the value of the field is `REPEATER`"]
    #[inline]
    pub fn is_repeater(&self) -> bool {
        *self == P0_26MODER::REPEATER
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == P0_26MODER::DISABLED
    }
    #[doc = "Checks if the value of the field is `PULL_DOWN`"]
    #[inline]
    pub fn is_pull_down(&self) -> bool {
        *self == P0_26MODER::PULL_DOWN
    }
}
#[doc = "Values that can be written to the field `P0_16MODE`"]
pub enum P0_16MODEW {
    #[doc = "Pull-up. P0.16 pin has a pull-up resistor enabled."]
    PULL_UP,
    #[doc = "Repeater. P0.16 pin has repeater mode enabled."]
    REPEATER,
    #[doc = "Disabled. P0.16 pin has neither pull-up nor pull-down."]
    DISABLED,
    #[doc = "Pull-down. P0.16 has a pull-down resistor enabled."]
    PULL_DOWN,
}
impl P0_16MODEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            P0_16MODEW::PULL_UP => 0,
            P0_16MODEW::REPEATER => 1,
            P0_16MODEW::DISABLED => 2,
            P0_16MODEW::PULL_DOWN => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _P0_16MODEW<'a> {
    w: &'a mut W,
}
impl<'a> _P0_16MODEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: P0_16MODEW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Pull-up. P0.16 pin has a pull-up resistor enabled."]
    #[inline]
    pub fn pull_up(self) -> &'a mut W {
        self.variant(P0_16MODEW::PULL_UP)
    }
    #[doc = "Repeater. P0.16 pin has repeater mode enabled."]
    #[inline]
    pub fn repeater(self) -> &'a mut W {
        self.variant(P0_16MODEW::REPEATER)
    }
    #[doc = "Disabled. P0.16 pin has neither pull-up nor pull-down."]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(P0_16MODEW::DISABLED)
    }
    #[doc = "Pull-down. P0.16 has a pull-down resistor enabled."]
    #[inline]
    pub fn pull_down(self) -> &'a mut W {
        self.variant(P0_16MODEW::PULL_DOWN)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `P0_17MODE`"]
pub enum P0_17MODEW {
    #[doc = "Pull-up. P0.17 pin has a pull-up resistor enabled."]
    PULL_UP,
    #[doc = "Repeater. P0.17 pin has repeater mode enabled."]
    REPEATER,
    #[doc = "Disabled. P0.17 pin has neither pull-up nor pull-down."]
    DISABLED,
    #[doc = "Pull-down. P0.17 has a pull-down resistor enabled."]
    PULL_DOWN,
}
impl P0_17MODEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            P0_17MODEW::PULL_UP => 0,
            P0_17MODEW::REPEATER => 1,
            P0_17MODEW::DISABLED => 2,
            P0_17MODEW::PULL_DOWN => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _P0_17MODEW<'a> {
    w: &'a mut W,
}
impl<'a> _P0_17MODEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: P0_17MODEW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Pull-up. P0.17 pin has a pull-up resistor enabled."]
    #[inline]
    pub fn pull_up(self) -> &'a mut W {
        self.variant(P0_17MODEW::PULL_UP)
    }
    #[doc = "Repeater. P0.17 pin has repeater mode enabled."]
    #[inline]
    pub fn repeater(self) -> &'a mut W {
        self.variant(P0_17MODEW::REPEATER)
    }
    #[doc = "Disabled. P0.17 pin has neither pull-up nor pull-down."]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(P0_17MODEW::DISABLED)
    }
    #[doc = "Pull-down. P0.17 has a pull-down resistor enabled."]
    #[inline]
    pub fn pull_down(self) -> &'a mut W {
        self.variant(P0_17MODEW::PULL_DOWN)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 2;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `P0_18MODE`"]
pub enum P0_18MODEW {
    #[doc = "Pull-up. P0.18 pin has a pull-up resistor enabled."]
    PULL_UP,
    #[doc = "Repeater. P0.18 pin has repeater mode enabled."]
    REPEATER,
    #[doc = "Disabled. P0.18 pin has neither pull-up nor pull-down."]
    DISABLED,
    #[doc = "Pull-down. P0.18 has a pull-down resistor enabled."]
    PULL_DOWN,
}
impl P0_18MODEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            P0_18MODEW::PULL_UP => 0,
            P0_18MODEW::REPEATER => 1,
            P0_18MODEW::DISABLED => 2,
            P0_18MODEW::PULL_DOWN => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _P0_18MODEW<'a> {
    w: &'a mut W,
}
impl<'a> _P0_18MODEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: P0_18MODEW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Pull-up. P0.18 pin has a pull-up resistor enabled."]
    #[inline]
    pub fn pull_up(self) -> &'a mut W {
        self.variant(P0_18MODEW::PULL_UP)
    }
    #[doc = "Repeater. P0.18 pin has repeater mode enabled."]
    #[inline]
    pub fn repeater(self) -> &'a mut W {
        self.variant(P0_18MODEW::REPEATER)
    }
    #[doc = "Disabled. P0.18 pin has neither pull-up nor pull-down."]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(P0_18MODEW::DISABLED)
    }
    #[doc = "Pull-down. P0.18 has a pull-down resistor enabled."]
    #[inline]
    pub fn pull_down(self) -> &'a mut W {
        self.variant(P0_18MODEW::PULL_DOWN)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `P0_19MODE`"]
pub enum P0_19MODEW {
    #[doc = "Pull-up. P0.19 pin has a pull-up resistor enabled."]
    PULL_UP,
    #[doc = "Repeater. P0.19 pin has repeater mode enabled."]
    REPEATER,
    #[doc = "Disabled. P0.19 pin has neither pull-up nor pull-down."]
    DISABLED,
    #[doc = "Pull-down. P0.19 has a pull-down resistor enabled."]
    PULL_DOWN,
}
impl P0_19MODEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            P0_19MODEW::PULL_UP => 0,
            P0_19MODEW::REPEATER => 1,
            P0_19MODEW::DISABLED => 2,
            P0_19MODEW::PULL_DOWN => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _P0_19MODEW<'a> {
    w: &'a mut W,
}
impl<'a> _P0_19MODEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: P0_19MODEW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Pull-up. P0.19 pin has a pull-up resistor enabled."]
    #[inline]
    pub fn pull_up(self) -> &'a mut W {
        self.variant(P0_19MODEW::PULL_UP)
    }
    #[doc = "Repeater. P0.19 pin has repeater mode enabled."]
    #[inline]
    pub fn repeater(self) -> &'a mut W {
        self.variant(P0_19MODEW::REPEATER)
    }
    #[doc = "Disabled. P0.19 pin has neither pull-up nor pull-down."]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(P0_19MODEW::DISABLED)
    }
    #[doc = "Pull-down. P0.19 has a pull-down resistor enabled."]
    #[inline]
    pub fn pull_down(self) -> &'a mut W {
        self.variant(P0_19MODEW::PULL_DOWN)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 6;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `P0_20MODE`"]
pub enum P0_20MODEW {
    #[doc = "Pull-up. P0.20 pin has a pull-up resistor enabled."]
    PULL_UP,
    #[doc = "Repeater. P0.20 pin has repeater mode enabled."]
    REPEATER,
    #[doc = "Disabled. P0.20 pin has neither pull-up nor pull-down."]
    DISABLED,
    #[doc = "Pull-down. P0.20 has a pull-down resistor enabled."]
    PULL_DOWN,
}
impl P0_20MODEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            P0_20MODEW::PULL_UP => 0,
            P0_20MODEW::REPEATER => 1,
            P0_20MODEW::DISABLED => 2,
            P0_20MODEW::PULL_DOWN => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _P0_20MODEW<'a> {
    w: &'a mut W,
}
impl<'a> _P0_20MODEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: P0_20MODEW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Pull-up. P0.20 pin has a pull-up resistor enabled."]
    #[inline]
    pub fn pull_up(self) -> &'a mut W {
        self.variant(P0_20MODEW::PULL_UP)
    }
    #[doc = "Repeater. P0.20 pin has repeater mode enabled."]
    #[inline]
    pub fn repeater(self) -> &'a mut W {
        self.variant(P0_20MODEW::REPEATER)
    }
    #[doc = "Disabled. P0.20 pin has neither pull-up nor pull-down."]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(P0_20MODEW::DISABLED)
    }
    #[doc = "Pull-down. P0.20 has a pull-down resistor enabled."]
    #[inline]
    pub fn pull_down(self) -> &'a mut W {
        self.variant(P0_20MODEW::PULL_DOWN)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `P0_21MODE`"]
pub enum P0_21MODEW {
    #[doc = "Pull-up. P0.21 pin has a pull-up resistor enabled."]
    PULL_UP,
    #[doc = "Repeater. P0.21 pin has repeater mode enabled."]
    REPEATER,
    #[doc = "Disabled. P0.21 pin has neither pull-up nor pull-down."]
    DISABLED,
    #[doc = "Pull-down. P0.21 has a pull-down resistor enabled."]
    PULL_DOWN,
}
impl P0_21MODEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            P0_21MODEW::PULL_UP => 0,
            P0_21MODEW::REPEATER => 1,
            P0_21MODEW::DISABLED => 2,
            P0_21MODEW::PULL_DOWN => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _P0_21MODEW<'a> {
    w: &'a mut W,
}
impl<'a> _P0_21MODEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: P0_21MODEW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Pull-up. P0.21 pin has a pull-up resistor enabled."]
    #[inline]
    pub fn pull_up(self) -> &'a mut W {
        self.variant(P0_21MODEW::PULL_UP)
    }
    #[doc = "Repeater. P0.21 pin has repeater mode enabled."]
    #[inline]
    pub fn repeater(self) -> &'a mut W {
        self.variant(P0_21MODEW::REPEATER)
    }
    #[doc = "Disabled. P0.21 pin has neither pull-up nor pull-down."]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(P0_21MODEW::DISABLED)
    }
    #[doc = "Pull-down. P0.21 has a pull-down resistor enabled."]
    #[inline]
    pub fn pull_down(self) -> &'a mut W {
        self.variant(P0_21MODEW::PULL_DOWN)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 10;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `P0_22MODE`"]
pub enum P0_22MODEW {
    #[doc = "Pull-up. P0.22 pin has a pull-up resistor enabled."]
    PULL_UP,
    #[doc = "Repeater. P0.22 pin has repeater mode enabled."]
    REPEATER,
    #[doc = "Disabled. P0.22 pin has neither pull-up nor pull-down."]
    DISABLED,
    #[doc = "Pull-down. P0.22 has a pull-down resistor enabled."]
    PULL_DOWN,
}
impl P0_22MODEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            P0_22MODEW::PULL_UP => 0,
            P0_22MODEW::REPEATER => 1,
            P0_22MODEW::DISABLED => 2,
            P0_22MODEW::PULL_DOWN => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _P0_22MODEW<'a> {
    w: &'a mut W,
}
impl<'a> _P0_22MODEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: P0_22MODEW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Pull-up. P0.22 pin has a pull-up resistor enabled."]
    #[inline]
    pub fn pull_up(self) -> &'a mut W {
        self.variant(P0_22MODEW::PULL_UP)
    }
    #[doc = "Repeater. P0.22 pin has repeater mode enabled."]
    #[inline]
    pub fn repeater(self) -> &'a mut W {
        self.variant(P0_22MODEW::REPEATER)
    }
    #[doc = "Disabled. P0.22 pin has neither pull-up nor pull-down."]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(P0_22MODEW::DISABLED)
    }
    #[doc = "Pull-down. P0.22 has a pull-down resistor enabled."]
    #[inline]
    pub fn pull_down(self) -> &'a mut W {
        self.variant(P0_22MODEW::PULL_DOWN)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 12;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `P0_23MODE`"]
pub enum P0_23MODEW {
    #[doc = "Pull-up. P0.23 pin has a pull-up resistor enabled."]
    PULL_UP,
    #[doc = "Repeater. P0.23 pin has repeater mode enabled."]
    REPEATER,
    #[doc = "Disabled. P0.23 pin has neither pull-up nor pull-down."]
    DISABLED,
    #[doc = "Pull-down. P0.23 has a pull-down resistor enabled."]
    PULL_DOWN,
}
impl P0_23MODEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            P0_23MODEW::PULL_UP => 0,
            P0_23MODEW::REPEATER => 1,
            P0_23MODEW::DISABLED => 2,
            P0_23MODEW::PULL_DOWN => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _P0_23MODEW<'a> {
    w: &'a mut W,
}
impl<'a> _P0_23MODEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: P0_23MODEW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Pull-up. P0.23 pin has a pull-up resistor enabled."]
    #[inline]
    pub fn pull_up(self) -> &'a mut W {
        self.variant(P0_23MODEW::PULL_UP)
    }
    #[doc = "Repeater. P0.23 pin has repeater mode enabled."]
    #[inline]
    pub fn repeater(self) -> &'a mut W {
        self.variant(P0_23MODEW::REPEATER)
    }
    #[doc = "Disabled. P0.23 pin has neither pull-up nor pull-down."]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(P0_23MODEW::DISABLED)
    }
    #[doc = "Pull-down. P0.23 has a pull-down resistor enabled."]
    #[inline]
    pub fn pull_down(self) -> &'a mut W {
        self.variant(P0_23MODEW::PULL_DOWN)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 14;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `P0_24MODE`"]
pub enum P0_24MODEW {
    #[doc = "Pull-up. P0.24 pin has a pull-up resistor enabled."]
    PULL_UP,
    #[doc = "Repeater. P0.24 pin has repeater mode enabled."]
    REPEATER,
    #[doc = "Disabled. P0.24 pin has neither pull-up nor pull-down."]
    DISABLED,
    #[doc = "Pull-down. P0.24 has a pull-down resistor enabled."]
    PULL_DOWN,
}
impl P0_24MODEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            P0_24MODEW::PULL_UP => 0,
            P0_24MODEW::REPEATER => 1,
            P0_24MODEW::DISABLED => 2,
            P0_24MODEW::PULL_DOWN => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _P0_24MODEW<'a> {
    w: &'a mut W,
}
impl<'a> _P0_24MODEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: P0_24MODEW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Pull-up. P0.24 pin has a pull-up resistor enabled."]
    #[inline]
    pub fn pull_up(self) -> &'a mut W {
        self.variant(P0_24MODEW::PULL_UP)
    }
    #[doc = "Repeater. P0.24 pin has repeater mode enabled."]
    #[inline]
    pub fn repeater(self) -> &'a mut W {
        self.variant(P0_24MODEW::REPEATER)
    }
    #[doc = "Disabled. P0.24 pin has neither pull-up nor pull-down."]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(P0_24MODEW::DISABLED)
    }
    #[doc = "Pull-down. P0.24 has a pull-down resistor enabled."]
    #[inline]
    pub fn pull_down(self) -> &'a mut W {
        self.variant(P0_24MODEW::PULL_DOWN)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `P0_25MODE`"]
pub enum P0_25MODEW {
    #[doc = "Pull-up. P0.25 pin has a pull-up resistor enabled."]
    PULL_UP,
    #[doc = "Repeater. P0.25 pin has repeater mode enabled."]
    REPEATER,
    #[doc = "Disabled. P0.25 pin has neither pull-up nor pull-down."]
    DISABLED,
    #[doc = "Pull-down. P0.25 has a pull-down resistor enabled."]
    PULL_DOWN,
}
impl P0_25MODEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            P0_25MODEW::PULL_UP => 0,
            P0_25MODEW::REPEATER => 1,
            P0_25MODEW::DISABLED => 2,
            P0_25MODEW::PULL_DOWN => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _P0_25MODEW<'a> {
    w: &'a mut W,
}
impl<'a> _P0_25MODEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: P0_25MODEW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Pull-up. P0.25 pin has a pull-up resistor enabled."]
    #[inline]
    pub fn pull_up(self) -> &'a mut W {
        self.variant(P0_25MODEW::PULL_UP)
    }
    #[doc = "Repeater. P0.25 pin has repeater mode enabled."]
    #[inline]
    pub fn repeater(self) -> &'a mut W {
        self.variant(P0_25MODEW::REPEATER)
    }
    #[doc = "Disabled. P0.25 pin has neither pull-up nor pull-down."]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(P0_25MODEW::DISABLED)
    }
    #[doc = "Pull-down. P0.25 has a pull-down resistor enabled."]
    #[inline]
    pub fn pull_down(self) -> &'a mut W {
        self.variant(P0_25MODEW::PULL_DOWN)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 18;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `P0_26MODE`"]
pub enum P0_26MODEW {
    #[doc = "Pull-up. P0.26 pin has a pull-up resistor enabled."]
    PULL_UP,
    #[doc = "Repeater. P0.26 pin has repeater mode enabled."]
    REPEATER,
    #[doc = "Disabled. P0.26 pin has neither pull-up nor pull-down."]
    DISABLED,
    #[doc = "Pull-down. P0.26 has a pull-down resistor enabled."]
    PULL_DOWN,
}
impl P0_26MODEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            P0_26MODEW::PULL_UP => 0,
            P0_26MODEW::REPEATER => 1,
            P0_26MODEW::DISABLED => 2,
            P0_26MODEW::PULL_DOWN => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _P0_26MODEW<'a> {
    w: &'a mut W,
}
impl<'a> _P0_26MODEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: P0_26MODEW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Pull-up. P0.26 pin has a pull-up resistor enabled."]
    #[inline]
    pub fn pull_up(self) -> &'a mut W {
        self.variant(P0_26MODEW::PULL_UP)
    }
    #[doc = "Repeater. P0.26 pin has repeater mode enabled."]
    #[inline]
    pub fn repeater(self) -> &'a mut W {
        self.variant(P0_26MODEW::REPEATER)
    }
    #[doc = "Disabled. P0.26 pin has neither pull-up nor pull-down."]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(P0_26MODEW::DISABLED)
    }
    #[doc = "Pull-down. P0.26 has a pull-down resistor enabled."]
    #[inline]
    pub fn pull_down(self) -> &'a mut W {
        self.variant(P0_26MODEW::PULL_DOWN)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 20;
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
    #[doc = "Bits 0:1 - Port 1 pin 16 control."]
    #[inline]
    pub fn p0_16mode(&self) -> P0_16MODER {
        P0_16MODER::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 2:3 - Port 1 pin 17 control."]
    #[inline]
    pub fn p0_17mode(&self) -> P0_17MODER {
        P0_17MODER::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 4:5 - Port 1 pin 18 control."]
    #[inline]
    pub fn p0_18mode(&self) -> P0_18MODER {
        P0_18MODER::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 6:7 - Port 1 pin 19 control."]
    #[inline]
    pub fn p0_19mode(&self) -> P0_19MODER {
        P0_19MODER::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 8:9 - Port 1 pin 20 control."]
    #[inline]
    pub fn p0_20mode(&self) -> P0_20MODER {
        P0_20MODER::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 10:11 - Port 1 pin 21 control."]
    #[inline]
    pub fn p0_21mode(&self) -> P0_21MODER {
        P0_21MODER::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 12:13 - Port 1 pin 22 control."]
    #[inline]
    pub fn p0_22mode(&self) -> P0_22MODER {
        P0_22MODER::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 14:15 - Port 1 pin 23 control."]
    #[inline]
    pub fn p0_23mode(&self) -> P0_23MODER {
        P0_23MODER::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 16:17 - Port 1 pin 24 control."]
    #[inline]
    pub fn p0_24mode(&self) -> P0_24MODER {
        P0_24MODER::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 18:19 - Port 1 pin 25 control."]
    #[inline]
    pub fn p0_25mode(&self) -> P0_25MODER {
        P0_25MODER::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 20:21 - Port 1 pin 26 control."]
    #[inline]
    pub fn p0_26mode(&self) -> P0_26MODER {
        P0_26MODER::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) as u8
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
    #[doc = "Bits 0:1 - Port 1 pin 16 control."]
    #[inline]
    pub fn p0_16mode(&mut self) -> _P0_16MODEW {
        _P0_16MODEW { w: self }
    }
    #[doc = "Bits 2:3 - Port 1 pin 17 control."]
    #[inline]
    pub fn p0_17mode(&mut self) -> _P0_17MODEW {
        _P0_17MODEW { w: self }
    }
    #[doc = "Bits 4:5 - Port 1 pin 18 control."]
    #[inline]
    pub fn p0_18mode(&mut self) -> _P0_18MODEW {
        _P0_18MODEW { w: self }
    }
    #[doc = "Bits 6:7 - Port 1 pin 19 control."]
    #[inline]
    pub fn p0_19mode(&mut self) -> _P0_19MODEW {
        _P0_19MODEW { w: self }
    }
    #[doc = "Bits 8:9 - Port 1 pin 20 control."]
    #[inline]
    pub fn p0_20mode(&mut self) -> _P0_20MODEW {
        _P0_20MODEW { w: self }
    }
    #[doc = "Bits 10:11 - Port 1 pin 21 control."]
    #[inline]
    pub fn p0_21mode(&mut self) -> _P0_21MODEW {
        _P0_21MODEW { w: self }
    }
    #[doc = "Bits 12:13 - Port 1 pin 22 control."]
    #[inline]
    pub fn p0_22mode(&mut self) -> _P0_22MODEW {
        _P0_22MODEW { w: self }
    }
    #[doc = "Bits 14:15 - Port 1 pin 23 control."]
    #[inline]
    pub fn p0_23mode(&mut self) -> _P0_23MODEW {
        _P0_23MODEW { w: self }
    }
    #[doc = "Bits 16:17 - Port 1 pin 24 control."]
    #[inline]
    pub fn p0_24mode(&mut self) -> _P0_24MODEW {
        _P0_24MODEW { w: self }
    }
    #[doc = "Bits 18:19 - Port 1 pin 25 control."]
    #[inline]
    pub fn p0_25mode(&mut self) -> _P0_25MODEW {
        _P0_25MODEW { w: self }
    }
    #[doc = "Bits 20:21 - Port 1 pin 26 control."]
    #[inline]
    pub fn p0_26mode(&mut self) -> _P0_26MODEW {
        _P0_26MODEW { w: self }
    }
}
