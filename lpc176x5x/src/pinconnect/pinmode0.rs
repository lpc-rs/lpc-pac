#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::PINMODE0 {
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
#[doc = "Possible values of the field `P0_00MODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P0_00MODER {
    #[doc = "Pull-up. P0.0 pin has a pull-up resistor enabled."]
    PULL_UP,
    #[doc = "Repeater. P0.0 pin has repeater mode enabled."]
    REPEATER,
    #[doc = "Disabled. P0.0 pin has neither pull-up nor pull-down."]
    DISABLED,
    #[doc = "Pull-down. P0.0 has a pull-down resistor enabled."]
    PULL_DOWN,
}
impl P0_00MODER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            P0_00MODER::PULL_UP => 0,
            P0_00MODER::REPEATER => 1,
            P0_00MODER::DISABLED => 2,
            P0_00MODER::PULL_DOWN => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> P0_00MODER {
        match value {
            0 => P0_00MODER::PULL_UP,
            1 => P0_00MODER::REPEATER,
            2 => P0_00MODER::DISABLED,
            3 => P0_00MODER::PULL_DOWN,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PULL_UP`"]
    #[inline]
    pub fn is_pull_up(&self) -> bool {
        *self == P0_00MODER::PULL_UP
    }
    #[doc = "Checks if the value of the field is `REPEATER`"]
    #[inline]
    pub fn is_repeater(&self) -> bool {
        *self == P0_00MODER::REPEATER
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == P0_00MODER::DISABLED
    }
    #[doc = "Checks if the value of the field is `PULL_DOWN`"]
    #[inline]
    pub fn is_pull_down(&self) -> bool {
        *self == P0_00MODER::PULL_DOWN
    }
}
#[doc = "Possible values of the field `P0_01MODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P0_01MODER {
    #[doc = "Pull-up. P0.1 pin has a pull-up resistor enabled."]
    PULL_UP,
    #[doc = "Repeater. P0.1 pin has repeater mode enabled."]
    REPEATER,
    #[doc = "Disabled. P0.1 pin has neither pull-up nor pull-down."]
    DISABLED,
    #[doc = "Pull-down. P0.1 has a pull-down resistor enabled."]
    PULL_DOWN,
}
impl P0_01MODER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            P0_01MODER::PULL_UP => 0,
            P0_01MODER::REPEATER => 1,
            P0_01MODER::DISABLED => 2,
            P0_01MODER::PULL_DOWN => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> P0_01MODER {
        match value {
            0 => P0_01MODER::PULL_UP,
            1 => P0_01MODER::REPEATER,
            2 => P0_01MODER::DISABLED,
            3 => P0_01MODER::PULL_DOWN,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PULL_UP`"]
    #[inline]
    pub fn is_pull_up(&self) -> bool {
        *self == P0_01MODER::PULL_UP
    }
    #[doc = "Checks if the value of the field is `REPEATER`"]
    #[inline]
    pub fn is_repeater(&self) -> bool {
        *self == P0_01MODER::REPEATER
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == P0_01MODER::DISABLED
    }
    #[doc = "Checks if the value of the field is `PULL_DOWN`"]
    #[inline]
    pub fn is_pull_down(&self) -> bool {
        *self == P0_01MODER::PULL_DOWN
    }
}
#[doc = "Possible values of the field `P0_02MODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P0_02MODER {
    #[doc = "Pull-up. P0.2 pin has a pull-up resistor enabled."]
    PULL_UP,
    #[doc = "Repeater. P0.2 pin has repeater mode enabled."]
    REPEATER,
    #[doc = "Disabled. P0.2 pin has neither pull-up nor pull-down."]
    DISABLED,
    #[doc = "Pull-down. P0.2 has a pull-down resistor enabled."]
    PULL_DOWN,
}
impl P0_02MODER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            P0_02MODER::PULL_UP => 0,
            P0_02MODER::REPEATER => 1,
            P0_02MODER::DISABLED => 2,
            P0_02MODER::PULL_DOWN => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> P0_02MODER {
        match value {
            0 => P0_02MODER::PULL_UP,
            1 => P0_02MODER::REPEATER,
            2 => P0_02MODER::DISABLED,
            3 => P0_02MODER::PULL_DOWN,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PULL_UP`"]
    #[inline]
    pub fn is_pull_up(&self) -> bool {
        *self == P0_02MODER::PULL_UP
    }
    #[doc = "Checks if the value of the field is `REPEATER`"]
    #[inline]
    pub fn is_repeater(&self) -> bool {
        *self == P0_02MODER::REPEATER
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == P0_02MODER::DISABLED
    }
    #[doc = "Checks if the value of the field is `PULL_DOWN`"]
    #[inline]
    pub fn is_pull_down(&self) -> bool {
        *self == P0_02MODER::PULL_DOWN
    }
}
#[doc = "Possible values of the field `P0_03MODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P0_03MODER {
    #[doc = "Pull-up. P0.3 pin has a pull-up resistor enabled."]
    PULL_UP,
    #[doc = "Repeater. P0.3 pin has repeater mode enabled."]
    REPEATER,
    #[doc = "Disabled. P0.3 pin has neither pull-up nor pull-down."]
    DISABLED,
    #[doc = "Pull-down. P0.3 has a pull-down resistor enabled."]
    PULL_DOWN,
}
impl P0_03MODER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            P0_03MODER::PULL_UP => 0,
            P0_03MODER::REPEATER => 1,
            P0_03MODER::DISABLED => 2,
            P0_03MODER::PULL_DOWN => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> P0_03MODER {
        match value {
            0 => P0_03MODER::PULL_UP,
            1 => P0_03MODER::REPEATER,
            2 => P0_03MODER::DISABLED,
            3 => P0_03MODER::PULL_DOWN,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PULL_UP`"]
    #[inline]
    pub fn is_pull_up(&self) -> bool {
        *self == P0_03MODER::PULL_UP
    }
    #[doc = "Checks if the value of the field is `REPEATER`"]
    #[inline]
    pub fn is_repeater(&self) -> bool {
        *self == P0_03MODER::REPEATER
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == P0_03MODER::DISABLED
    }
    #[doc = "Checks if the value of the field is `PULL_DOWN`"]
    #[inline]
    pub fn is_pull_down(&self) -> bool {
        *self == P0_03MODER::PULL_DOWN
    }
}
#[doc = "Possible values of the field `P0_04MODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P0_04MODER {
    #[doc = "Pull-up. P0.4 pin has a pull-up resistor enabled."]
    PULL_UP,
    #[doc = "Repeater. P0.4 pin has repeater mode enabled."]
    REPEATER,
    #[doc = "Disabled. P0.4 pin has neither pull-up nor pull-down."]
    DISABLED,
    #[doc = "Pull-down. P0.4 has a pull-down resistor enabled."]
    PULL_DOWN,
}
impl P0_04MODER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            P0_04MODER::PULL_UP => 0,
            P0_04MODER::REPEATER => 1,
            P0_04MODER::DISABLED => 2,
            P0_04MODER::PULL_DOWN => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> P0_04MODER {
        match value {
            0 => P0_04MODER::PULL_UP,
            1 => P0_04MODER::REPEATER,
            2 => P0_04MODER::DISABLED,
            3 => P0_04MODER::PULL_DOWN,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PULL_UP`"]
    #[inline]
    pub fn is_pull_up(&self) -> bool {
        *self == P0_04MODER::PULL_UP
    }
    #[doc = "Checks if the value of the field is `REPEATER`"]
    #[inline]
    pub fn is_repeater(&self) -> bool {
        *self == P0_04MODER::REPEATER
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == P0_04MODER::DISABLED
    }
    #[doc = "Checks if the value of the field is `PULL_DOWN`"]
    #[inline]
    pub fn is_pull_down(&self) -> bool {
        *self == P0_04MODER::PULL_DOWN
    }
}
#[doc = "Possible values of the field `P0_05MODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P0_05MODER {
    #[doc = "Pull-up. P0.5 pin has a pull-up resistor enabled."]
    PULL_UP,
    #[doc = "Repeater. P0.5 pin has repeater mode enabled."]
    REPEATER,
    #[doc = "Disabled. P0.5 pin has neither pull-up nor pull-down."]
    DISABLED,
    #[doc = "Pull-down. P0.5 has a pull-down resistor enabled."]
    PULL_DOWN,
}
impl P0_05MODER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            P0_05MODER::PULL_UP => 0,
            P0_05MODER::REPEATER => 1,
            P0_05MODER::DISABLED => 2,
            P0_05MODER::PULL_DOWN => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> P0_05MODER {
        match value {
            0 => P0_05MODER::PULL_UP,
            1 => P0_05MODER::REPEATER,
            2 => P0_05MODER::DISABLED,
            3 => P0_05MODER::PULL_DOWN,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PULL_UP`"]
    #[inline]
    pub fn is_pull_up(&self) -> bool {
        *self == P0_05MODER::PULL_UP
    }
    #[doc = "Checks if the value of the field is `REPEATER`"]
    #[inline]
    pub fn is_repeater(&self) -> bool {
        *self == P0_05MODER::REPEATER
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == P0_05MODER::DISABLED
    }
    #[doc = "Checks if the value of the field is `PULL_DOWN`"]
    #[inline]
    pub fn is_pull_down(&self) -> bool {
        *self == P0_05MODER::PULL_DOWN
    }
}
#[doc = "Possible values of the field `P0_06MODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P0_06MODER {
    #[doc = "Pull-up. P0.6 pin has a pull-up resistor enabled."]
    PULL_UP,
    #[doc = "Disabled. Repeater. P0.6 pin has repeater mode enabled."]
    REPEATER,
    #[doc = "Disabled. P0.6 pin has neither pull-up nor pull-down."]
    DISABLED,
    #[doc = "Pull-down. P0.6 has a pull-down resistor enabled."]
    PULL_DOWN,
}
impl P0_06MODER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            P0_06MODER::PULL_UP => 0,
            P0_06MODER::REPEATER => 1,
            P0_06MODER::DISABLED => 2,
            P0_06MODER::PULL_DOWN => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> P0_06MODER {
        match value {
            0 => P0_06MODER::PULL_UP,
            1 => P0_06MODER::REPEATER,
            2 => P0_06MODER::DISABLED,
            3 => P0_06MODER::PULL_DOWN,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PULL_UP`"]
    #[inline]
    pub fn is_pull_up(&self) -> bool {
        *self == P0_06MODER::PULL_UP
    }
    #[doc = "Checks if the value of the field is `REPEATER`"]
    #[inline]
    pub fn is_repeater(&self) -> bool {
        *self == P0_06MODER::REPEATER
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == P0_06MODER::DISABLED
    }
    #[doc = "Checks if the value of the field is `PULL_DOWN`"]
    #[inline]
    pub fn is_pull_down(&self) -> bool {
        *self == P0_06MODER::PULL_DOWN
    }
}
#[doc = "Possible values of the field `P0_07MODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P0_07MODER {
    #[doc = "Pull-up. P0.7 pin has a pull-up resistor enabled."]
    PULL_UP,
    #[doc = "Repeater. P0.7 pin has repeater mode enabled."]
    REPEATER,
    #[doc = "Disabled. P0.7 pin has neither pull-up nor pull-down."]
    DISABLED,
    #[doc = "Pull-down. P0.7 has a pull-down resistor enabled."]
    PULL_DOWN,
}
impl P0_07MODER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            P0_07MODER::PULL_UP => 0,
            P0_07MODER::REPEATER => 1,
            P0_07MODER::DISABLED => 2,
            P0_07MODER::PULL_DOWN => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> P0_07MODER {
        match value {
            0 => P0_07MODER::PULL_UP,
            1 => P0_07MODER::REPEATER,
            2 => P0_07MODER::DISABLED,
            3 => P0_07MODER::PULL_DOWN,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PULL_UP`"]
    #[inline]
    pub fn is_pull_up(&self) -> bool {
        *self == P0_07MODER::PULL_UP
    }
    #[doc = "Checks if the value of the field is `REPEATER`"]
    #[inline]
    pub fn is_repeater(&self) -> bool {
        *self == P0_07MODER::REPEATER
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == P0_07MODER::DISABLED
    }
    #[doc = "Checks if the value of the field is `PULL_DOWN`"]
    #[inline]
    pub fn is_pull_down(&self) -> bool {
        *self == P0_07MODER::PULL_DOWN
    }
}
#[doc = "Possible values of the field `P0_08MODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P0_08MODER {
    #[doc = "Pull-up. P0.8 pin has a pull-up resistor enabled."]
    PULL_UP,
    #[doc = "Repeater. P0.8 pin has repeater mode enabled."]
    REPEATER,
    #[doc = "Disabled. P0.8 pin has neither pull-up nor pull-down."]
    DISABLED,
    #[doc = "Pull-down. P0.8 has a pull-down resistor enabled."]
    PULL_DOWN,
}
impl P0_08MODER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            P0_08MODER::PULL_UP => 0,
            P0_08MODER::REPEATER => 1,
            P0_08MODER::DISABLED => 2,
            P0_08MODER::PULL_DOWN => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> P0_08MODER {
        match value {
            0 => P0_08MODER::PULL_UP,
            1 => P0_08MODER::REPEATER,
            2 => P0_08MODER::DISABLED,
            3 => P0_08MODER::PULL_DOWN,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PULL_UP`"]
    #[inline]
    pub fn is_pull_up(&self) -> bool {
        *self == P0_08MODER::PULL_UP
    }
    #[doc = "Checks if the value of the field is `REPEATER`"]
    #[inline]
    pub fn is_repeater(&self) -> bool {
        *self == P0_08MODER::REPEATER
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == P0_08MODER::DISABLED
    }
    #[doc = "Checks if the value of the field is `PULL_DOWN`"]
    #[inline]
    pub fn is_pull_down(&self) -> bool {
        *self == P0_08MODER::PULL_DOWN
    }
}
#[doc = "Possible values of the field `P0_09MODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P0_09MODER {
    #[doc = "Pull-up. P0.9 pin has a pull-up resistor enabled."]
    PULL_UP,
    #[doc = "Repeater. P0.9 pin has repeater mode enabled."]
    REPEATER,
    #[doc = "Disabled. P0.9 pin has neither pull-up nor pull-down."]
    DISABLED,
    #[doc = "Pull-down. P0.9 has a pull-down resistor enabled."]
    PULL_DOWN,
}
impl P0_09MODER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            P0_09MODER::PULL_UP => 0,
            P0_09MODER::REPEATER => 1,
            P0_09MODER::DISABLED => 2,
            P0_09MODER::PULL_DOWN => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> P0_09MODER {
        match value {
            0 => P0_09MODER::PULL_UP,
            1 => P0_09MODER::REPEATER,
            2 => P0_09MODER::DISABLED,
            3 => P0_09MODER::PULL_DOWN,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PULL_UP`"]
    #[inline]
    pub fn is_pull_up(&self) -> bool {
        *self == P0_09MODER::PULL_UP
    }
    #[doc = "Checks if the value of the field is `REPEATER`"]
    #[inline]
    pub fn is_repeater(&self) -> bool {
        *self == P0_09MODER::REPEATER
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == P0_09MODER::DISABLED
    }
    #[doc = "Checks if the value of the field is `PULL_DOWN`"]
    #[inline]
    pub fn is_pull_down(&self) -> bool {
        *self == P0_09MODER::PULL_DOWN
    }
}
#[doc = "Possible values of the field `P0_10MODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P0_10MODER {
    #[doc = "Pull-up. P0.10 pin has a pull-up resistor enabled."]
    PULL_UP,
    #[doc = "Repeater. P0.10 pin has repeater mode enabled."]
    REPEATER,
    #[doc = "Disabled. P0.10 pin has neither pull-up nor pull-down."]
    DISABLED,
    #[doc = "Pull-down. P0.10 has a pull-down resistor enabled."]
    PULL_DOWN,
}
impl P0_10MODER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            P0_10MODER::PULL_UP => 0,
            P0_10MODER::REPEATER => 1,
            P0_10MODER::DISABLED => 2,
            P0_10MODER::PULL_DOWN => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> P0_10MODER {
        match value {
            0 => P0_10MODER::PULL_UP,
            1 => P0_10MODER::REPEATER,
            2 => P0_10MODER::DISABLED,
            3 => P0_10MODER::PULL_DOWN,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PULL_UP`"]
    #[inline]
    pub fn is_pull_up(&self) -> bool {
        *self == P0_10MODER::PULL_UP
    }
    #[doc = "Checks if the value of the field is `REPEATER`"]
    #[inline]
    pub fn is_repeater(&self) -> bool {
        *self == P0_10MODER::REPEATER
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == P0_10MODER::DISABLED
    }
    #[doc = "Checks if the value of the field is `PULL_DOWN`"]
    #[inline]
    pub fn is_pull_down(&self) -> bool {
        *self == P0_10MODER::PULL_DOWN
    }
}
#[doc = "Possible values of the field `P0_11MODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P0_11MODER {
    #[doc = "Pull-up. P0.11 pin has a pull-up resistor enabled."]
    PULL_UP,
    #[doc = "Repeater. P0.11 pin has repeater mode enabled."]
    REPEATER,
    #[doc = "Disabled. P0.11 pin has neither pull-up nor pull-down."]
    DISABLED,
    #[doc = "Pull-down. P0.11 has a pull-down resistor enabled."]
    PULL_DOWN,
}
impl P0_11MODER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            P0_11MODER::PULL_UP => 0,
            P0_11MODER::REPEATER => 1,
            P0_11MODER::DISABLED => 2,
            P0_11MODER::PULL_DOWN => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> P0_11MODER {
        match value {
            0 => P0_11MODER::PULL_UP,
            1 => P0_11MODER::REPEATER,
            2 => P0_11MODER::DISABLED,
            3 => P0_11MODER::PULL_DOWN,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PULL_UP`"]
    #[inline]
    pub fn is_pull_up(&self) -> bool {
        *self == P0_11MODER::PULL_UP
    }
    #[doc = "Checks if the value of the field is `REPEATER`"]
    #[inline]
    pub fn is_repeater(&self) -> bool {
        *self == P0_11MODER::REPEATER
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == P0_11MODER::DISABLED
    }
    #[doc = "Checks if the value of the field is `PULL_DOWN`"]
    #[inline]
    pub fn is_pull_down(&self) -> bool {
        *self == P0_11MODER::PULL_DOWN
    }
}
#[doc = "Possible values of the field `P0_15MODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P0_15MODER {
    #[doc = "Pull-up. P0.15 pin has a pull-up resistor enabled."]
    PULL_UP,
    #[doc = "Repeater. P0.15 pin has repeater mode enabled."]
    REPEATER,
    #[doc = "Disabled. P0.15 pin has neither pull-up nor pull-down."]
    DISABLED,
    #[doc = "Pull-down. P0.15 has a pull-down resistor enabled."]
    PULL_DOWN,
}
impl P0_15MODER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            P0_15MODER::PULL_UP => 0,
            P0_15MODER::REPEATER => 1,
            P0_15MODER::DISABLED => 2,
            P0_15MODER::PULL_DOWN => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> P0_15MODER {
        match value {
            0 => P0_15MODER::PULL_UP,
            1 => P0_15MODER::REPEATER,
            2 => P0_15MODER::DISABLED,
            3 => P0_15MODER::PULL_DOWN,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PULL_UP`"]
    #[inline]
    pub fn is_pull_up(&self) -> bool {
        *self == P0_15MODER::PULL_UP
    }
    #[doc = "Checks if the value of the field is `REPEATER`"]
    #[inline]
    pub fn is_repeater(&self) -> bool {
        *self == P0_15MODER::REPEATER
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == P0_15MODER::DISABLED
    }
    #[doc = "Checks if the value of the field is `PULL_DOWN`"]
    #[inline]
    pub fn is_pull_down(&self) -> bool {
        *self == P0_15MODER::PULL_DOWN
    }
}
#[doc = "Values that can be written to the field `P0_00MODE`"]
pub enum P0_00MODEW {
    #[doc = "Pull-up. P0.0 pin has a pull-up resistor enabled."]
    PULL_UP,
    #[doc = "Repeater. P0.0 pin has repeater mode enabled."]
    REPEATER,
    #[doc = "Disabled. P0.0 pin has neither pull-up nor pull-down."]
    DISABLED,
    #[doc = "Pull-down. P0.0 has a pull-down resistor enabled."]
    PULL_DOWN,
}
impl P0_00MODEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            P0_00MODEW::PULL_UP => 0,
            P0_00MODEW::REPEATER => 1,
            P0_00MODEW::DISABLED => 2,
            P0_00MODEW::PULL_DOWN => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _P0_00MODEW<'a> {
    w: &'a mut W,
}
impl<'a> _P0_00MODEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: P0_00MODEW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Pull-up. P0.0 pin has a pull-up resistor enabled."]
    #[inline]
    pub fn pull_up(self) -> &'a mut W {
        self.variant(P0_00MODEW::PULL_UP)
    }
    #[doc = "Repeater. P0.0 pin has repeater mode enabled."]
    #[inline]
    pub fn repeater(self) -> &'a mut W {
        self.variant(P0_00MODEW::REPEATER)
    }
    #[doc = "Disabled. P0.0 pin has neither pull-up nor pull-down."]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(P0_00MODEW::DISABLED)
    }
    #[doc = "Pull-down. P0.0 has a pull-down resistor enabled."]
    #[inline]
    pub fn pull_down(self) -> &'a mut W {
        self.variant(P0_00MODEW::PULL_DOWN)
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
#[doc = "Values that can be written to the field `P0_01MODE`"]
pub enum P0_01MODEW {
    #[doc = "Pull-up. P0.1 pin has a pull-up resistor enabled."]
    PULL_UP,
    #[doc = "Repeater. P0.1 pin has repeater mode enabled."]
    REPEATER,
    #[doc = "Disabled. P0.1 pin has neither pull-up nor pull-down."]
    DISABLED,
    #[doc = "Pull-down. P0.1 has a pull-down resistor enabled."]
    PULL_DOWN,
}
impl P0_01MODEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            P0_01MODEW::PULL_UP => 0,
            P0_01MODEW::REPEATER => 1,
            P0_01MODEW::DISABLED => 2,
            P0_01MODEW::PULL_DOWN => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _P0_01MODEW<'a> {
    w: &'a mut W,
}
impl<'a> _P0_01MODEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: P0_01MODEW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Pull-up. P0.1 pin has a pull-up resistor enabled."]
    #[inline]
    pub fn pull_up(self) -> &'a mut W {
        self.variant(P0_01MODEW::PULL_UP)
    }
    #[doc = "Repeater. P0.1 pin has repeater mode enabled."]
    #[inline]
    pub fn repeater(self) -> &'a mut W {
        self.variant(P0_01MODEW::REPEATER)
    }
    #[doc = "Disabled. P0.1 pin has neither pull-up nor pull-down."]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(P0_01MODEW::DISABLED)
    }
    #[doc = "Pull-down. P0.1 has a pull-down resistor enabled."]
    #[inline]
    pub fn pull_down(self) -> &'a mut W {
        self.variant(P0_01MODEW::PULL_DOWN)
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
#[doc = "Values that can be written to the field `P0_02MODE`"]
pub enum P0_02MODEW {
    #[doc = "Pull-up. P0.2 pin has a pull-up resistor enabled."]
    PULL_UP,
    #[doc = "Repeater. P0.2 pin has repeater mode enabled."]
    REPEATER,
    #[doc = "Disabled. P0.2 pin has neither pull-up nor pull-down."]
    DISABLED,
    #[doc = "Pull-down. P0.2 has a pull-down resistor enabled."]
    PULL_DOWN,
}
impl P0_02MODEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            P0_02MODEW::PULL_UP => 0,
            P0_02MODEW::REPEATER => 1,
            P0_02MODEW::DISABLED => 2,
            P0_02MODEW::PULL_DOWN => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _P0_02MODEW<'a> {
    w: &'a mut W,
}
impl<'a> _P0_02MODEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: P0_02MODEW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Pull-up. P0.2 pin has a pull-up resistor enabled."]
    #[inline]
    pub fn pull_up(self) -> &'a mut W {
        self.variant(P0_02MODEW::PULL_UP)
    }
    #[doc = "Repeater. P0.2 pin has repeater mode enabled."]
    #[inline]
    pub fn repeater(self) -> &'a mut W {
        self.variant(P0_02MODEW::REPEATER)
    }
    #[doc = "Disabled. P0.2 pin has neither pull-up nor pull-down."]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(P0_02MODEW::DISABLED)
    }
    #[doc = "Pull-down. P0.2 has a pull-down resistor enabled."]
    #[inline]
    pub fn pull_down(self) -> &'a mut W {
        self.variant(P0_02MODEW::PULL_DOWN)
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
#[doc = "Values that can be written to the field `P0_03MODE`"]
pub enum P0_03MODEW {
    #[doc = "Pull-up. P0.3 pin has a pull-up resistor enabled."]
    PULL_UP,
    #[doc = "Repeater. P0.3 pin has repeater mode enabled."]
    REPEATER,
    #[doc = "Disabled. P0.3 pin has neither pull-up nor pull-down."]
    DISABLED,
    #[doc = "Pull-down. P0.3 has a pull-down resistor enabled."]
    PULL_DOWN,
}
impl P0_03MODEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            P0_03MODEW::PULL_UP => 0,
            P0_03MODEW::REPEATER => 1,
            P0_03MODEW::DISABLED => 2,
            P0_03MODEW::PULL_DOWN => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _P0_03MODEW<'a> {
    w: &'a mut W,
}
impl<'a> _P0_03MODEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: P0_03MODEW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Pull-up. P0.3 pin has a pull-up resistor enabled."]
    #[inline]
    pub fn pull_up(self) -> &'a mut W {
        self.variant(P0_03MODEW::PULL_UP)
    }
    #[doc = "Repeater. P0.3 pin has repeater mode enabled."]
    #[inline]
    pub fn repeater(self) -> &'a mut W {
        self.variant(P0_03MODEW::REPEATER)
    }
    #[doc = "Disabled. P0.3 pin has neither pull-up nor pull-down."]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(P0_03MODEW::DISABLED)
    }
    #[doc = "Pull-down. P0.3 has a pull-down resistor enabled."]
    #[inline]
    pub fn pull_down(self) -> &'a mut W {
        self.variant(P0_03MODEW::PULL_DOWN)
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
#[doc = "Values that can be written to the field `P0_04MODE`"]
pub enum P0_04MODEW {
    #[doc = "Pull-up. P0.4 pin has a pull-up resistor enabled."]
    PULL_UP,
    #[doc = "Repeater. P0.4 pin has repeater mode enabled."]
    REPEATER,
    #[doc = "Disabled. P0.4 pin has neither pull-up nor pull-down."]
    DISABLED,
    #[doc = "Pull-down. P0.4 has a pull-down resistor enabled."]
    PULL_DOWN,
}
impl P0_04MODEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            P0_04MODEW::PULL_UP => 0,
            P0_04MODEW::REPEATER => 1,
            P0_04MODEW::DISABLED => 2,
            P0_04MODEW::PULL_DOWN => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _P0_04MODEW<'a> {
    w: &'a mut W,
}
impl<'a> _P0_04MODEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: P0_04MODEW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Pull-up. P0.4 pin has a pull-up resistor enabled."]
    #[inline]
    pub fn pull_up(self) -> &'a mut W {
        self.variant(P0_04MODEW::PULL_UP)
    }
    #[doc = "Repeater. P0.4 pin has repeater mode enabled."]
    #[inline]
    pub fn repeater(self) -> &'a mut W {
        self.variant(P0_04MODEW::REPEATER)
    }
    #[doc = "Disabled. P0.4 pin has neither pull-up nor pull-down."]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(P0_04MODEW::DISABLED)
    }
    #[doc = "Pull-down. P0.4 has a pull-down resistor enabled."]
    #[inline]
    pub fn pull_down(self) -> &'a mut W {
        self.variant(P0_04MODEW::PULL_DOWN)
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
#[doc = "Values that can be written to the field `P0_05MODE`"]
pub enum P0_05MODEW {
    #[doc = "Pull-up. P0.5 pin has a pull-up resistor enabled."]
    PULL_UP,
    #[doc = "Repeater. P0.5 pin has repeater mode enabled."]
    REPEATER,
    #[doc = "Disabled. P0.5 pin has neither pull-up nor pull-down."]
    DISABLED,
    #[doc = "Pull-down. P0.5 has a pull-down resistor enabled."]
    PULL_DOWN,
}
impl P0_05MODEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            P0_05MODEW::PULL_UP => 0,
            P0_05MODEW::REPEATER => 1,
            P0_05MODEW::DISABLED => 2,
            P0_05MODEW::PULL_DOWN => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _P0_05MODEW<'a> {
    w: &'a mut W,
}
impl<'a> _P0_05MODEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: P0_05MODEW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Pull-up. P0.5 pin has a pull-up resistor enabled."]
    #[inline]
    pub fn pull_up(self) -> &'a mut W {
        self.variant(P0_05MODEW::PULL_UP)
    }
    #[doc = "Repeater. P0.5 pin has repeater mode enabled."]
    #[inline]
    pub fn repeater(self) -> &'a mut W {
        self.variant(P0_05MODEW::REPEATER)
    }
    #[doc = "Disabled. P0.5 pin has neither pull-up nor pull-down."]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(P0_05MODEW::DISABLED)
    }
    #[doc = "Pull-down. P0.5 has a pull-down resistor enabled."]
    #[inline]
    pub fn pull_down(self) -> &'a mut W {
        self.variant(P0_05MODEW::PULL_DOWN)
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
#[doc = "Values that can be written to the field `P0_06MODE`"]
pub enum P0_06MODEW {
    #[doc = "Pull-up. P0.6 pin has a pull-up resistor enabled."]
    PULL_UP,
    #[doc = "Disabled. Repeater. P0.6 pin has repeater mode enabled."]
    REPEATER,
    #[doc = "Disabled. P0.6 pin has neither pull-up nor pull-down."]
    DISABLED,
    #[doc = "Pull-down. P0.6 has a pull-down resistor enabled."]
    PULL_DOWN,
}
impl P0_06MODEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            P0_06MODEW::PULL_UP => 0,
            P0_06MODEW::REPEATER => 1,
            P0_06MODEW::DISABLED => 2,
            P0_06MODEW::PULL_DOWN => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _P0_06MODEW<'a> {
    w: &'a mut W,
}
impl<'a> _P0_06MODEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: P0_06MODEW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Pull-up. P0.6 pin has a pull-up resistor enabled."]
    #[inline]
    pub fn pull_up(self) -> &'a mut W {
        self.variant(P0_06MODEW::PULL_UP)
    }
    #[doc = "Disabled. Repeater. P0.6 pin has repeater mode enabled."]
    #[inline]
    pub fn repeater(self) -> &'a mut W {
        self.variant(P0_06MODEW::REPEATER)
    }
    #[doc = "Disabled. P0.6 pin has neither pull-up nor pull-down."]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(P0_06MODEW::DISABLED)
    }
    #[doc = "Pull-down. P0.6 has a pull-down resistor enabled."]
    #[inline]
    pub fn pull_down(self) -> &'a mut W {
        self.variant(P0_06MODEW::PULL_DOWN)
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
#[doc = "Values that can be written to the field `P0_07MODE`"]
pub enum P0_07MODEW {
    #[doc = "Pull-up. P0.7 pin has a pull-up resistor enabled."]
    PULL_UP,
    #[doc = "Repeater. P0.7 pin has repeater mode enabled."]
    REPEATER,
    #[doc = "Disabled. P0.7 pin has neither pull-up nor pull-down."]
    DISABLED,
    #[doc = "Pull-down. P0.7 has a pull-down resistor enabled."]
    PULL_DOWN,
}
impl P0_07MODEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            P0_07MODEW::PULL_UP => 0,
            P0_07MODEW::REPEATER => 1,
            P0_07MODEW::DISABLED => 2,
            P0_07MODEW::PULL_DOWN => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _P0_07MODEW<'a> {
    w: &'a mut W,
}
impl<'a> _P0_07MODEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: P0_07MODEW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Pull-up. P0.7 pin has a pull-up resistor enabled."]
    #[inline]
    pub fn pull_up(self) -> &'a mut W {
        self.variant(P0_07MODEW::PULL_UP)
    }
    #[doc = "Repeater. P0.7 pin has repeater mode enabled."]
    #[inline]
    pub fn repeater(self) -> &'a mut W {
        self.variant(P0_07MODEW::REPEATER)
    }
    #[doc = "Disabled. P0.7 pin has neither pull-up nor pull-down."]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(P0_07MODEW::DISABLED)
    }
    #[doc = "Pull-down. P0.7 has a pull-down resistor enabled."]
    #[inline]
    pub fn pull_down(self) -> &'a mut W {
        self.variant(P0_07MODEW::PULL_DOWN)
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
#[doc = "Values that can be written to the field `P0_08MODE`"]
pub enum P0_08MODEW {
    #[doc = "Pull-up. P0.8 pin has a pull-up resistor enabled."]
    PULL_UP,
    #[doc = "Repeater. P0.8 pin has repeater mode enabled."]
    REPEATER,
    #[doc = "Disabled. P0.8 pin has neither pull-up nor pull-down."]
    DISABLED,
    #[doc = "Pull-down. P0.8 has a pull-down resistor enabled."]
    PULL_DOWN,
}
impl P0_08MODEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            P0_08MODEW::PULL_UP => 0,
            P0_08MODEW::REPEATER => 1,
            P0_08MODEW::DISABLED => 2,
            P0_08MODEW::PULL_DOWN => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _P0_08MODEW<'a> {
    w: &'a mut W,
}
impl<'a> _P0_08MODEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: P0_08MODEW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Pull-up. P0.8 pin has a pull-up resistor enabled."]
    #[inline]
    pub fn pull_up(self) -> &'a mut W {
        self.variant(P0_08MODEW::PULL_UP)
    }
    #[doc = "Repeater. P0.8 pin has repeater mode enabled."]
    #[inline]
    pub fn repeater(self) -> &'a mut W {
        self.variant(P0_08MODEW::REPEATER)
    }
    #[doc = "Disabled. P0.8 pin has neither pull-up nor pull-down."]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(P0_08MODEW::DISABLED)
    }
    #[doc = "Pull-down. P0.8 has a pull-down resistor enabled."]
    #[inline]
    pub fn pull_down(self) -> &'a mut W {
        self.variant(P0_08MODEW::PULL_DOWN)
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
#[doc = "Values that can be written to the field `P0_09MODE`"]
pub enum P0_09MODEW {
    #[doc = "Pull-up. P0.9 pin has a pull-up resistor enabled."]
    PULL_UP,
    #[doc = "Repeater. P0.9 pin has repeater mode enabled."]
    REPEATER,
    #[doc = "Disabled. P0.9 pin has neither pull-up nor pull-down."]
    DISABLED,
    #[doc = "Pull-down. P0.9 has a pull-down resistor enabled."]
    PULL_DOWN,
}
impl P0_09MODEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            P0_09MODEW::PULL_UP => 0,
            P0_09MODEW::REPEATER => 1,
            P0_09MODEW::DISABLED => 2,
            P0_09MODEW::PULL_DOWN => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _P0_09MODEW<'a> {
    w: &'a mut W,
}
impl<'a> _P0_09MODEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: P0_09MODEW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Pull-up. P0.9 pin has a pull-up resistor enabled."]
    #[inline]
    pub fn pull_up(self) -> &'a mut W {
        self.variant(P0_09MODEW::PULL_UP)
    }
    #[doc = "Repeater. P0.9 pin has repeater mode enabled."]
    #[inline]
    pub fn repeater(self) -> &'a mut W {
        self.variant(P0_09MODEW::REPEATER)
    }
    #[doc = "Disabled. P0.9 pin has neither pull-up nor pull-down."]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(P0_09MODEW::DISABLED)
    }
    #[doc = "Pull-down. P0.9 has a pull-down resistor enabled."]
    #[inline]
    pub fn pull_down(self) -> &'a mut W {
        self.variant(P0_09MODEW::PULL_DOWN)
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
#[doc = "Values that can be written to the field `P0_10MODE`"]
pub enum P0_10MODEW {
    #[doc = "Pull-up. P0.10 pin has a pull-up resistor enabled."]
    PULL_UP,
    #[doc = "Repeater. P0.10 pin has repeater mode enabled."]
    REPEATER,
    #[doc = "Disabled. P0.10 pin has neither pull-up nor pull-down."]
    DISABLED,
    #[doc = "Pull-down. P0.10 has a pull-down resistor enabled."]
    PULL_DOWN,
}
impl P0_10MODEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            P0_10MODEW::PULL_UP => 0,
            P0_10MODEW::REPEATER => 1,
            P0_10MODEW::DISABLED => 2,
            P0_10MODEW::PULL_DOWN => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _P0_10MODEW<'a> {
    w: &'a mut W,
}
impl<'a> _P0_10MODEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: P0_10MODEW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Pull-up. P0.10 pin has a pull-up resistor enabled."]
    #[inline]
    pub fn pull_up(self) -> &'a mut W {
        self.variant(P0_10MODEW::PULL_UP)
    }
    #[doc = "Repeater. P0.10 pin has repeater mode enabled."]
    #[inline]
    pub fn repeater(self) -> &'a mut W {
        self.variant(P0_10MODEW::REPEATER)
    }
    #[doc = "Disabled. P0.10 pin has neither pull-up nor pull-down."]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(P0_10MODEW::DISABLED)
    }
    #[doc = "Pull-down. P0.10 has a pull-down resistor enabled."]
    #[inline]
    pub fn pull_down(self) -> &'a mut W {
        self.variant(P0_10MODEW::PULL_DOWN)
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
#[doc = "Values that can be written to the field `P0_11MODE`"]
pub enum P0_11MODEW {
    #[doc = "Pull-up. P0.11 pin has a pull-up resistor enabled."]
    PULL_UP,
    #[doc = "Repeater. P0.11 pin has repeater mode enabled."]
    REPEATER,
    #[doc = "Disabled. P0.11 pin has neither pull-up nor pull-down."]
    DISABLED,
    #[doc = "Pull-down. P0.11 has a pull-down resistor enabled."]
    PULL_DOWN,
}
impl P0_11MODEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            P0_11MODEW::PULL_UP => 0,
            P0_11MODEW::REPEATER => 1,
            P0_11MODEW::DISABLED => 2,
            P0_11MODEW::PULL_DOWN => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _P0_11MODEW<'a> {
    w: &'a mut W,
}
impl<'a> _P0_11MODEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: P0_11MODEW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Pull-up. P0.11 pin has a pull-up resistor enabled."]
    #[inline]
    pub fn pull_up(self) -> &'a mut W {
        self.variant(P0_11MODEW::PULL_UP)
    }
    #[doc = "Repeater. P0.11 pin has repeater mode enabled."]
    #[inline]
    pub fn repeater(self) -> &'a mut W {
        self.variant(P0_11MODEW::REPEATER)
    }
    #[doc = "Disabled. P0.11 pin has neither pull-up nor pull-down."]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(P0_11MODEW::DISABLED)
    }
    #[doc = "Pull-down. P0.11 has a pull-down resistor enabled."]
    #[inline]
    pub fn pull_down(self) -> &'a mut W {
        self.variant(P0_11MODEW::PULL_DOWN)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 22;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `P0_15MODE`"]
pub enum P0_15MODEW {
    #[doc = "Pull-up. P0.15 pin has a pull-up resistor enabled."]
    PULL_UP,
    #[doc = "Repeater. P0.15 pin has repeater mode enabled."]
    REPEATER,
    #[doc = "Disabled. P0.15 pin has neither pull-up nor pull-down."]
    DISABLED,
    #[doc = "Pull-down. P0.15 has a pull-down resistor enabled."]
    PULL_DOWN,
}
impl P0_15MODEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            P0_15MODEW::PULL_UP => 0,
            P0_15MODEW::REPEATER => 1,
            P0_15MODEW::DISABLED => 2,
            P0_15MODEW::PULL_DOWN => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _P0_15MODEW<'a> {
    w: &'a mut W,
}
impl<'a> _P0_15MODEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: P0_15MODEW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Pull-up. P0.15 pin has a pull-up resistor enabled."]
    #[inline]
    pub fn pull_up(self) -> &'a mut W {
        self.variant(P0_15MODEW::PULL_UP)
    }
    #[doc = "Repeater. P0.15 pin has repeater mode enabled."]
    #[inline]
    pub fn repeater(self) -> &'a mut W {
        self.variant(P0_15MODEW::REPEATER)
    }
    #[doc = "Disabled. P0.15 pin has neither pull-up nor pull-down."]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(P0_15MODEW::DISABLED)
    }
    #[doc = "Pull-down. P0.15 has a pull-down resistor enabled."]
    #[inline]
    pub fn pull_down(self) -> &'a mut W {
        self.variant(P0_15MODEW::PULL_DOWN)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 30;
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
    #[doc = "Bits 0:1 - Port 0 pin 0 on-chip pull-up/down resistor control."]
    #[inline]
    pub fn p0_00mode(&self) -> P0_00MODER {
        P0_00MODER::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 2:3 - Port 0 pin 1 control."]
    #[inline]
    pub fn p0_01mode(&self) -> P0_01MODER {
        P0_01MODER::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 4:5 - Port 0 pin 2 control."]
    #[inline]
    pub fn p0_02mode(&self) -> P0_02MODER {
        P0_02MODER::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 6:7 - Port 0 pin 3 control."]
    #[inline]
    pub fn p0_03mode(&self) -> P0_03MODER {
        P0_03MODER::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 8:9 - Port 0 pin 4 control."]
    #[inline]
    pub fn p0_04mode(&self) -> P0_04MODER {
        P0_04MODER::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 10:11 - Port 0 pin 5 control."]
    #[inline]
    pub fn p0_05mode(&self) -> P0_05MODER {
        P0_05MODER::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 12:13 - Port 0 pin 6 control."]
    #[inline]
    pub fn p0_06mode(&self) -> P0_06MODER {
        P0_06MODER::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 14:15 - Port 0 pin 7 control."]
    #[inline]
    pub fn p0_07mode(&self) -> P0_07MODER {
        P0_07MODER::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 16:17 - Port 0 pin 8 control."]
    #[inline]
    pub fn p0_08mode(&self) -> P0_08MODER {
        P0_08MODER::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 18:19 - Port 0 pin 9 control."]
    #[inline]
    pub fn p0_09mode(&self) -> P0_09MODER {
        P0_09MODER::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 20:21 - Port 0 pin 10 control."]
    #[inline]
    pub fn p0_10mode(&self) -> P0_10MODER {
        P0_10MODER::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 22:23 - Port 0 pin 11 control."]
    #[inline]
    pub fn p0_11mode(&self) -> P0_11MODER {
        P0_11MODER::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 22;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 30:31 - Port 0 pin 15 control."]
    #[inline]
    pub fn p0_15mode(&self) -> P0_15MODER {
        P0_15MODER::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 30;
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
    #[doc = "Bits 0:1 - Port 0 pin 0 on-chip pull-up/down resistor control."]
    #[inline]
    pub fn p0_00mode(&mut self) -> _P0_00MODEW {
        _P0_00MODEW { w: self }
    }
    #[doc = "Bits 2:3 - Port 0 pin 1 control."]
    #[inline]
    pub fn p0_01mode(&mut self) -> _P0_01MODEW {
        _P0_01MODEW { w: self }
    }
    #[doc = "Bits 4:5 - Port 0 pin 2 control."]
    #[inline]
    pub fn p0_02mode(&mut self) -> _P0_02MODEW {
        _P0_02MODEW { w: self }
    }
    #[doc = "Bits 6:7 - Port 0 pin 3 control."]
    #[inline]
    pub fn p0_03mode(&mut self) -> _P0_03MODEW {
        _P0_03MODEW { w: self }
    }
    #[doc = "Bits 8:9 - Port 0 pin 4 control."]
    #[inline]
    pub fn p0_04mode(&mut self) -> _P0_04MODEW {
        _P0_04MODEW { w: self }
    }
    #[doc = "Bits 10:11 - Port 0 pin 5 control."]
    #[inline]
    pub fn p0_05mode(&mut self) -> _P0_05MODEW {
        _P0_05MODEW { w: self }
    }
    #[doc = "Bits 12:13 - Port 0 pin 6 control."]
    #[inline]
    pub fn p0_06mode(&mut self) -> _P0_06MODEW {
        _P0_06MODEW { w: self }
    }
    #[doc = "Bits 14:15 - Port 0 pin 7 control."]
    #[inline]
    pub fn p0_07mode(&mut self) -> _P0_07MODEW {
        _P0_07MODEW { w: self }
    }
    #[doc = "Bits 16:17 - Port 0 pin 8 control."]
    #[inline]
    pub fn p0_08mode(&mut self) -> _P0_08MODEW {
        _P0_08MODEW { w: self }
    }
    #[doc = "Bits 18:19 - Port 0 pin 9 control."]
    #[inline]
    pub fn p0_09mode(&mut self) -> _P0_09MODEW {
        _P0_09MODEW { w: self }
    }
    #[doc = "Bits 20:21 - Port 0 pin 10 control."]
    #[inline]
    pub fn p0_10mode(&mut self) -> _P0_10MODEW {
        _P0_10MODEW { w: self }
    }
    #[doc = "Bits 22:23 - Port 0 pin 11 control."]
    #[inline]
    pub fn p0_11mode(&mut self) -> _P0_11MODEW {
        _P0_11MODEW { w: self }
    }
    #[doc = "Bits 30:31 - Port 0 pin 15 control."]
    #[inline]
    pub fn p0_15mode(&mut self) -> _P0_15MODEW {
        _P0_15MODEW { w: self }
    }
}
