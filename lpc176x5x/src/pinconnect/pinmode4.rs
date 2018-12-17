#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::PINMODE4 {
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
#[doc = "Possible values of the field `P2_00MODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P2_00MODER {
    #[doc = "Pull-up. P2.0 pin has a pull-up resistor enabled."]
    PULL_UP,
    #[doc = "Repeater. P2.0 pin has repeater mode enabled."]
    REPEATER,
    #[doc = "Disabled. P2.0 pin has neither pull-up nor pull-down."]
    DISABLED,
    #[doc = "Pull-down. P2.0 has a pull-down resistor enabled."]
    PULL_DOWN,
}
impl P2_00MODER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            P2_00MODER::PULL_UP => 0,
            P2_00MODER::REPEATER => 1,
            P2_00MODER::DISABLED => 2,
            P2_00MODER::PULL_DOWN => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> P2_00MODER {
        match value {
            0 => P2_00MODER::PULL_UP,
            1 => P2_00MODER::REPEATER,
            2 => P2_00MODER::DISABLED,
            3 => P2_00MODER::PULL_DOWN,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PULL_UP`"]
    #[inline]
    pub fn is_pull_up(&self) -> bool {
        *self == P2_00MODER::PULL_UP
    }
    #[doc = "Checks if the value of the field is `REPEATER`"]
    #[inline]
    pub fn is_repeater(&self) -> bool {
        *self == P2_00MODER::REPEATER
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == P2_00MODER::DISABLED
    }
    #[doc = "Checks if the value of the field is `PULL_DOWN`"]
    #[inline]
    pub fn is_pull_down(&self) -> bool {
        *self == P2_00MODER::PULL_DOWN
    }
}
#[doc = "Possible values of the field `P2_01MODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P2_01MODER {
    #[doc = "Pull-up. P2.1 pin has a pull-up resistor enabled."]
    PULL_UP,
    #[doc = "Repeater. P2.1 pin has repeater mode enabled."]
    REPEATER,
    #[doc = "Disabled. P2.1 pin has neither pull-up nor pull-down."]
    DISABLED,
    #[doc = "Pull-down. P2.1 has a pull-down resistor enabled."]
    PULL_DOWN,
}
impl P2_01MODER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            P2_01MODER::PULL_UP => 0,
            P2_01MODER::REPEATER => 1,
            P2_01MODER::DISABLED => 2,
            P2_01MODER::PULL_DOWN => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> P2_01MODER {
        match value {
            0 => P2_01MODER::PULL_UP,
            1 => P2_01MODER::REPEATER,
            2 => P2_01MODER::DISABLED,
            3 => P2_01MODER::PULL_DOWN,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PULL_UP`"]
    #[inline]
    pub fn is_pull_up(&self) -> bool {
        *self == P2_01MODER::PULL_UP
    }
    #[doc = "Checks if the value of the field is `REPEATER`"]
    #[inline]
    pub fn is_repeater(&self) -> bool {
        *self == P2_01MODER::REPEATER
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == P2_01MODER::DISABLED
    }
    #[doc = "Checks if the value of the field is `PULL_DOWN`"]
    #[inline]
    pub fn is_pull_down(&self) -> bool {
        *self == P2_01MODER::PULL_DOWN
    }
}
#[doc = "Possible values of the field `P2_02MODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P2_02MODER {
    #[doc = "Pull-up. P2.2 pin has a pull-up resistor enabled."]
    PULL_UP,
    #[doc = "Repeater. P2.2 pin has repeater mode enabled."]
    REPEATER,
    #[doc = "Disabled. P2.2 pin has neither pull-up nor pull-down."]
    DISABLED,
    #[doc = "Pull-down. P2.2 has a pull-down resistor enabled."]
    PULL_DOWN,
}
impl P2_02MODER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            P2_02MODER::PULL_UP => 0,
            P2_02MODER::REPEATER => 1,
            P2_02MODER::DISABLED => 2,
            P2_02MODER::PULL_DOWN => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> P2_02MODER {
        match value {
            0 => P2_02MODER::PULL_UP,
            1 => P2_02MODER::REPEATER,
            2 => P2_02MODER::DISABLED,
            3 => P2_02MODER::PULL_DOWN,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PULL_UP`"]
    #[inline]
    pub fn is_pull_up(&self) -> bool {
        *self == P2_02MODER::PULL_UP
    }
    #[doc = "Checks if the value of the field is `REPEATER`"]
    #[inline]
    pub fn is_repeater(&self) -> bool {
        *self == P2_02MODER::REPEATER
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == P2_02MODER::DISABLED
    }
    #[doc = "Checks if the value of the field is `PULL_DOWN`"]
    #[inline]
    pub fn is_pull_down(&self) -> bool {
        *self == P2_02MODER::PULL_DOWN
    }
}
#[doc = "Possible values of the field `P2_03MODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P2_03MODER {
    #[doc = "Pull-up. P2.3 pin has a pull-up resistor enabled."]
    PULL_UP,
    #[doc = "Repeater. P2.3 pin has repeater mode enabled."]
    REPEATER,
    #[doc = "Disabled. P2.3 pin has neither pull-up nor pull-down."]
    DISABLED,
    #[doc = "Pull-down. P2.3 has a pull-down resistor enabled."]
    PULL_DOWN,
}
impl P2_03MODER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            P2_03MODER::PULL_UP => 0,
            P2_03MODER::REPEATER => 1,
            P2_03MODER::DISABLED => 2,
            P2_03MODER::PULL_DOWN => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> P2_03MODER {
        match value {
            0 => P2_03MODER::PULL_UP,
            1 => P2_03MODER::REPEATER,
            2 => P2_03MODER::DISABLED,
            3 => P2_03MODER::PULL_DOWN,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PULL_UP`"]
    #[inline]
    pub fn is_pull_up(&self) -> bool {
        *self == P2_03MODER::PULL_UP
    }
    #[doc = "Checks if the value of the field is `REPEATER`"]
    #[inline]
    pub fn is_repeater(&self) -> bool {
        *self == P2_03MODER::REPEATER
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == P2_03MODER::DISABLED
    }
    #[doc = "Checks if the value of the field is `PULL_DOWN`"]
    #[inline]
    pub fn is_pull_down(&self) -> bool {
        *self == P2_03MODER::PULL_DOWN
    }
}
#[doc = "Possible values of the field `P2_04MODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P2_04MODER {
    #[doc = "Pull-up. P2.4 pin has a pull-up resistor enabled."]
    PULL_UP,
    #[doc = "Repeater. P2.4 pin has repeater mode enabled."]
    REPEATER,
    #[doc = "Disabled. P2.4 pin has neither pull-up nor pull-down."]
    DISABLED,
    #[doc = "Pull-down. P2.4 has a pull-down resistor enabled."]
    PULL_DOWN,
}
impl P2_04MODER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            P2_04MODER::PULL_UP => 0,
            P2_04MODER::REPEATER => 1,
            P2_04MODER::DISABLED => 2,
            P2_04MODER::PULL_DOWN => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> P2_04MODER {
        match value {
            0 => P2_04MODER::PULL_UP,
            1 => P2_04MODER::REPEATER,
            2 => P2_04MODER::DISABLED,
            3 => P2_04MODER::PULL_DOWN,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PULL_UP`"]
    #[inline]
    pub fn is_pull_up(&self) -> bool {
        *self == P2_04MODER::PULL_UP
    }
    #[doc = "Checks if the value of the field is `REPEATER`"]
    #[inline]
    pub fn is_repeater(&self) -> bool {
        *self == P2_04MODER::REPEATER
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == P2_04MODER::DISABLED
    }
    #[doc = "Checks if the value of the field is `PULL_DOWN`"]
    #[inline]
    pub fn is_pull_down(&self) -> bool {
        *self == P2_04MODER::PULL_DOWN
    }
}
#[doc = "Possible values of the field `P2_05MODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P2_05MODER {
    #[doc = "Pull-up. P2.5 pin has a pull-up resistor enabled."]
    PULL_UP,
    #[doc = "Repeater. P2.5 pin has repeater mode enabled."]
    REPEATER,
    #[doc = "Disabled. P2.5 pin has neither pull-up nor pull-down."]
    DISABLED,
    #[doc = "Pull-down. P2.5 has a pull-down resistor enabled."]
    PULL_DOWN,
}
impl P2_05MODER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            P2_05MODER::PULL_UP => 0,
            P2_05MODER::REPEATER => 1,
            P2_05MODER::DISABLED => 2,
            P2_05MODER::PULL_DOWN => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> P2_05MODER {
        match value {
            0 => P2_05MODER::PULL_UP,
            1 => P2_05MODER::REPEATER,
            2 => P2_05MODER::DISABLED,
            3 => P2_05MODER::PULL_DOWN,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PULL_UP`"]
    #[inline]
    pub fn is_pull_up(&self) -> bool {
        *self == P2_05MODER::PULL_UP
    }
    #[doc = "Checks if the value of the field is `REPEATER`"]
    #[inline]
    pub fn is_repeater(&self) -> bool {
        *self == P2_05MODER::REPEATER
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == P2_05MODER::DISABLED
    }
    #[doc = "Checks if the value of the field is `PULL_DOWN`"]
    #[inline]
    pub fn is_pull_down(&self) -> bool {
        *self == P2_05MODER::PULL_DOWN
    }
}
#[doc = "Possible values of the field `P2_06MODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P2_06MODER {
    #[doc = "Pull-up. P2.6 pin has a pull-up resistor enabled."]
    PULL_UP,
    #[doc = "Repeater. P2.6 pin has repeater mode enabled."]
    REPEATER,
    #[doc = "Disabled. P2.6 pin has neither pull-up nor pull-down."]
    DISABLED,
    #[doc = "Pull-down. P2.6 has a pull-down resistor enabled."]
    PULL_DOWN,
}
impl P2_06MODER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            P2_06MODER::PULL_UP => 0,
            P2_06MODER::REPEATER => 1,
            P2_06MODER::DISABLED => 2,
            P2_06MODER::PULL_DOWN => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> P2_06MODER {
        match value {
            0 => P2_06MODER::PULL_UP,
            1 => P2_06MODER::REPEATER,
            2 => P2_06MODER::DISABLED,
            3 => P2_06MODER::PULL_DOWN,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PULL_UP`"]
    #[inline]
    pub fn is_pull_up(&self) -> bool {
        *self == P2_06MODER::PULL_UP
    }
    #[doc = "Checks if the value of the field is `REPEATER`"]
    #[inline]
    pub fn is_repeater(&self) -> bool {
        *self == P2_06MODER::REPEATER
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == P2_06MODER::DISABLED
    }
    #[doc = "Checks if the value of the field is `PULL_DOWN`"]
    #[inline]
    pub fn is_pull_down(&self) -> bool {
        *self == P2_06MODER::PULL_DOWN
    }
}
#[doc = "Possible values of the field `P2_07MODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P2_07MODER {
    #[doc = "Pull-up. P2.7 pin has a pull-up resistor enabled."]
    PULL_UP,
    #[doc = "Repeater. P2.7 pin has repeater mode enabled."]
    REPEATER,
    #[doc = "Disabled. P2.7 pin has neither pull-up nor pull-down."]
    DISABLED,
    #[doc = "Pull-down. P2.7 has a pull-down resistor enabled."]
    PULL_DOWN,
}
impl P2_07MODER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            P2_07MODER::PULL_UP => 0,
            P2_07MODER::REPEATER => 1,
            P2_07MODER::DISABLED => 2,
            P2_07MODER::PULL_DOWN => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> P2_07MODER {
        match value {
            0 => P2_07MODER::PULL_UP,
            1 => P2_07MODER::REPEATER,
            2 => P2_07MODER::DISABLED,
            3 => P2_07MODER::PULL_DOWN,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PULL_UP`"]
    #[inline]
    pub fn is_pull_up(&self) -> bool {
        *self == P2_07MODER::PULL_UP
    }
    #[doc = "Checks if the value of the field is `REPEATER`"]
    #[inline]
    pub fn is_repeater(&self) -> bool {
        *self == P2_07MODER::REPEATER
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == P2_07MODER::DISABLED
    }
    #[doc = "Checks if the value of the field is `PULL_DOWN`"]
    #[inline]
    pub fn is_pull_down(&self) -> bool {
        *self == P2_07MODER::PULL_DOWN
    }
}
#[doc = "Possible values of the field `P2_08MODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P2_08MODER {
    #[doc = "Pull-up. P2.8 pin has a pull-up resistor enabled."]
    PULL_UP,
    #[doc = "Repeater. P2.8 pin has repeater mode enabled."]
    REPEATER,
    #[doc = "Disabled. P2.8 pin has neither pull-up nor pull-down."]
    DISABLED,
    #[doc = "Pull-down. P2.8 has a pull-down resistor enabled."]
    PULL_DOWN,
}
impl P2_08MODER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            P2_08MODER::PULL_UP => 0,
            P2_08MODER::REPEATER => 1,
            P2_08MODER::DISABLED => 2,
            P2_08MODER::PULL_DOWN => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> P2_08MODER {
        match value {
            0 => P2_08MODER::PULL_UP,
            1 => P2_08MODER::REPEATER,
            2 => P2_08MODER::DISABLED,
            3 => P2_08MODER::PULL_DOWN,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PULL_UP`"]
    #[inline]
    pub fn is_pull_up(&self) -> bool {
        *self == P2_08MODER::PULL_UP
    }
    #[doc = "Checks if the value of the field is `REPEATER`"]
    #[inline]
    pub fn is_repeater(&self) -> bool {
        *self == P2_08MODER::REPEATER
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == P2_08MODER::DISABLED
    }
    #[doc = "Checks if the value of the field is `PULL_DOWN`"]
    #[inline]
    pub fn is_pull_down(&self) -> bool {
        *self == P2_08MODER::PULL_DOWN
    }
}
#[doc = "Possible values of the field `P2_09MODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P2_09MODER {
    #[doc = "Pull-up. P2.9 pin has a pull-up resistor enabled."]
    PULL_UP,
    #[doc = "Repeater. P2.9 pin has repeater mode enabled."]
    REPEATER,
    #[doc = "Disabled. P2.9 pin has neither pull-up nor pull-down."]
    DISABLED,
    #[doc = "Pull-down. P2.9 has a pull-down resistor enabled."]
    PULL_DOWN,
}
impl P2_09MODER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            P2_09MODER::PULL_UP => 0,
            P2_09MODER::REPEATER => 1,
            P2_09MODER::DISABLED => 2,
            P2_09MODER::PULL_DOWN => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> P2_09MODER {
        match value {
            0 => P2_09MODER::PULL_UP,
            1 => P2_09MODER::REPEATER,
            2 => P2_09MODER::DISABLED,
            3 => P2_09MODER::PULL_DOWN,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PULL_UP`"]
    #[inline]
    pub fn is_pull_up(&self) -> bool {
        *self == P2_09MODER::PULL_UP
    }
    #[doc = "Checks if the value of the field is `REPEATER`"]
    #[inline]
    pub fn is_repeater(&self) -> bool {
        *self == P2_09MODER::REPEATER
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == P2_09MODER::DISABLED
    }
    #[doc = "Checks if the value of the field is `PULL_DOWN`"]
    #[inline]
    pub fn is_pull_down(&self) -> bool {
        *self == P2_09MODER::PULL_DOWN
    }
}
#[doc = "Possible values of the field `P2_10MODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P2_10MODER {
    #[doc = "Pull-up. P2.10 pin has a pull-up resistor enabled."]
    PULL_UP,
    #[doc = "Repeater. P2.10 pin has repeater mode enabled."]
    REPEATER,
    #[doc = "Disabled. P2.10 pin has neither pull-up nor pull-down."]
    DISABLED,
    #[doc = "Pull-down. P2.10 has a pull-down resistor enabled."]
    PULL_DOWN,
}
impl P2_10MODER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            P2_10MODER::PULL_UP => 0,
            P2_10MODER::REPEATER => 1,
            P2_10MODER::DISABLED => 2,
            P2_10MODER::PULL_DOWN => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> P2_10MODER {
        match value {
            0 => P2_10MODER::PULL_UP,
            1 => P2_10MODER::REPEATER,
            2 => P2_10MODER::DISABLED,
            3 => P2_10MODER::PULL_DOWN,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PULL_UP`"]
    #[inline]
    pub fn is_pull_up(&self) -> bool {
        *self == P2_10MODER::PULL_UP
    }
    #[doc = "Checks if the value of the field is `REPEATER`"]
    #[inline]
    pub fn is_repeater(&self) -> bool {
        *self == P2_10MODER::REPEATER
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == P2_10MODER::DISABLED
    }
    #[doc = "Checks if the value of the field is `PULL_DOWN`"]
    #[inline]
    pub fn is_pull_down(&self) -> bool {
        *self == P2_10MODER::PULL_DOWN
    }
}
#[doc = "Possible values of the field `P2_11MODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P2_11MODER {
    #[doc = "Pull-up. P2.11 pin has a pull-up resistor enabled."]
    PULL_UP,
    #[doc = "Repeater. P2.11 pin has repeater mode enabled."]
    REPEATER,
    #[doc = "Disabled. P2.11 pin has neither pull-up nor pull-down."]
    DISABLED,
    #[doc = "Pull-down. P2.11 has a pull-down resistor enabled."]
    PULL_DOWN,
}
impl P2_11MODER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            P2_11MODER::PULL_UP => 0,
            P2_11MODER::REPEATER => 1,
            P2_11MODER::DISABLED => 2,
            P2_11MODER::PULL_DOWN => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> P2_11MODER {
        match value {
            0 => P2_11MODER::PULL_UP,
            1 => P2_11MODER::REPEATER,
            2 => P2_11MODER::DISABLED,
            3 => P2_11MODER::PULL_DOWN,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PULL_UP`"]
    #[inline]
    pub fn is_pull_up(&self) -> bool {
        *self == P2_11MODER::PULL_UP
    }
    #[doc = "Checks if the value of the field is `REPEATER`"]
    #[inline]
    pub fn is_repeater(&self) -> bool {
        *self == P2_11MODER::REPEATER
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == P2_11MODER::DISABLED
    }
    #[doc = "Checks if the value of the field is `PULL_DOWN`"]
    #[inline]
    pub fn is_pull_down(&self) -> bool {
        *self == P2_11MODER::PULL_DOWN
    }
}
#[doc = "Possible values of the field `P2_12MODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P2_12MODER {
    #[doc = "Pull-up. P2.12 pin has a pull-up resistor enabled."]
    PULL_UP,
    #[doc = "Repeater. P2.12 pin has repeater mode enabled."]
    REPEATER,
    #[doc = "Disabled. P2.12 pin has neither pull-up nor pull-down."]
    DISABLED,
    #[doc = "Pull-down. P2.12 has a pull-down resistor enabled."]
    PULL_DOWN,
}
impl P2_12MODER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            P2_12MODER::PULL_UP => 0,
            P2_12MODER::REPEATER => 1,
            P2_12MODER::DISABLED => 2,
            P2_12MODER::PULL_DOWN => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> P2_12MODER {
        match value {
            0 => P2_12MODER::PULL_UP,
            1 => P2_12MODER::REPEATER,
            2 => P2_12MODER::DISABLED,
            3 => P2_12MODER::PULL_DOWN,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PULL_UP`"]
    #[inline]
    pub fn is_pull_up(&self) -> bool {
        *self == P2_12MODER::PULL_UP
    }
    #[doc = "Checks if the value of the field is `REPEATER`"]
    #[inline]
    pub fn is_repeater(&self) -> bool {
        *self == P2_12MODER::REPEATER
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == P2_12MODER::DISABLED
    }
    #[doc = "Checks if the value of the field is `PULL_DOWN`"]
    #[inline]
    pub fn is_pull_down(&self) -> bool {
        *self == P2_12MODER::PULL_DOWN
    }
}
#[doc = "Possible values of the field `P2_13MODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P2_13MODER {
    #[doc = "Pull-up. P2.13 pin has a pull-up resistor enabled."]
    PULL_UP,
    #[doc = "Repeater. P2.13 pin has repeater mode enabled."]
    REPEATER,
    #[doc = "Disabled. P2.13 pin has neither pull-up nor pull-down."]
    DISABLED,
    #[doc = "Pull-down. P2.13 has a pull-down resistor enabled."]
    PULL_DOWN,
}
impl P2_13MODER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            P2_13MODER::PULL_UP => 0,
            P2_13MODER::REPEATER => 1,
            P2_13MODER::DISABLED => 2,
            P2_13MODER::PULL_DOWN => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> P2_13MODER {
        match value {
            0 => P2_13MODER::PULL_UP,
            1 => P2_13MODER::REPEATER,
            2 => P2_13MODER::DISABLED,
            3 => P2_13MODER::PULL_DOWN,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PULL_UP`"]
    #[inline]
    pub fn is_pull_up(&self) -> bool {
        *self == P2_13MODER::PULL_UP
    }
    #[doc = "Checks if the value of the field is `REPEATER`"]
    #[inline]
    pub fn is_repeater(&self) -> bool {
        *self == P2_13MODER::REPEATER
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == P2_13MODER::DISABLED
    }
    #[doc = "Checks if the value of the field is `PULL_DOWN`"]
    #[inline]
    pub fn is_pull_down(&self) -> bool {
        *self == P2_13MODER::PULL_DOWN
    }
}
#[doc = "Values that can be written to the field `P2_00MODE`"]
pub enum P2_00MODEW {
    #[doc = "Pull-up. P2.0 pin has a pull-up resistor enabled."]
    PULL_UP,
    #[doc = "Repeater. P2.0 pin has repeater mode enabled."]
    REPEATER,
    #[doc = "Disabled. P2.0 pin has neither pull-up nor pull-down."]
    DISABLED,
    #[doc = "Pull-down. P2.0 has a pull-down resistor enabled."]
    PULL_DOWN,
}
impl P2_00MODEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            P2_00MODEW::PULL_UP => 0,
            P2_00MODEW::REPEATER => 1,
            P2_00MODEW::DISABLED => 2,
            P2_00MODEW::PULL_DOWN => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _P2_00MODEW<'a> {
    w: &'a mut W,
}
impl<'a> _P2_00MODEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: P2_00MODEW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Pull-up. P2.0 pin has a pull-up resistor enabled."]
    #[inline]
    pub fn pull_up(self) -> &'a mut W {
        self.variant(P2_00MODEW::PULL_UP)
    }
    #[doc = "Repeater. P2.0 pin has repeater mode enabled."]
    #[inline]
    pub fn repeater(self) -> &'a mut W {
        self.variant(P2_00MODEW::REPEATER)
    }
    #[doc = "Disabled. P2.0 pin has neither pull-up nor pull-down."]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(P2_00MODEW::DISABLED)
    }
    #[doc = "Pull-down. P2.0 has a pull-down resistor enabled."]
    #[inline]
    pub fn pull_down(self) -> &'a mut W {
        self.variant(P2_00MODEW::PULL_DOWN)
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
#[doc = "Values that can be written to the field `P2_01MODE`"]
pub enum P2_01MODEW {
    #[doc = "Pull-up. P2.1 pin has a pull-up resistor enabled."]
    PULL_UP,
    #[doc = "Repeater. P2.1 pin has repeater mode enabled."]
    REPEATER,
    #[doc = "Disabled. P2.1 pin has neither pull-up nor pull-down."]
    DISABLED,
    #[doc = "Pull-down. P2.1 has a pull-down resistor enabled."]
    PULL_DOWN,
}
impl P2_01MODEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            P2_01MODEW::PULL_UP => 0,
            P2_01MODEW::REPEATER => 1,
            P2_01MODEW::DISABLED => 2,
            P2_01MODEW::PULL_DOWN => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _P2_01MODEW<'a> {
    w: &'a mut W,
}
impl<'a> _P2_01MODEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: P2_01MODEW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Pull-up. P2.1 pin has a pull-up resistor enabled."]
    #[inline]
    pub fn pull_up(self) -> &'a mut W {
        self.variant(P2_01MODEW::PULL_UP)
    }
    #[doc = "Repeater. P2.1 pin has repeater mode enabled."]
    #[inline]
    pub fn repeater(self) -> &'a mut W {
        self.variant(P2_01MODEW::REPEATER)
    }
    #[doc = "Disabled. P2.1 pin has neither pull-up nor pull-down."]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(P2_01MODEW::DISABLED)
    }
    #[doc = "Pull-down. P2.1 has a pull-down resistor enabled."]
    #[inline]
    pub fn pull_down(self) -> &'a mut W {
        self.variant(P2_01MODEW::PULL_DOWN)
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
#[doc = "Values that can be written to the field `P2_02MODE`"]
pub enum P2_02MODEW {
    #[doc = "Pull-up. P2.2 pin has a pull-up resistor enabled."]
    PULL_UP,
    #[doc = "Repeater. P2.2 pin has repeater mode enabled."]
    REPEATER,
    #[doc = "Disabled. P2.2 pin has neither pull-up nor pull-down."]
    DISABLED,
    #[doc = "Pull-down. P2.2 has a pull-down resistor enabled."]
    PULL_DOWN,
}
impl P2_02MODEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            P2_02MODEW::PULL_UP => 0,
            P2_02MODEW::REPEATER => 1,
            P2_02MODEW::DISABLED => 2,
            P2_02MODEW::PULL_DOWN => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _P2_02MODEW<'a> {
    w: &'a mut W,
}
impl<'a> _P2_02MODEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: P2_02MODEW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Pull-up. P2.2 pin has a pull-up resistor enabled."]
    #[inline]
    pub fn pull_up(self) -> &'a mut W {
        self.variant(P2_02MODEW::PULL_UP)
    }
    #[doc = "Repeater. P2.2 pin has repeater mode enabled."]
    #[inline]
    pub fn repeater(self) -> &'a mut W {
        self.variant(P2_02MODEW::REPEATER)
    }
    #[doc = "Disabled. P2.2 pin has neither pull-up nor pull-down."]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(P2_02MODEW::DISABLED)
    }
    #[doc = "Pull-down. P2.2 has a pull-down resistor enabled."]
    #[inline]
    pub fn pull_down(self) -> &'a mut W {
        self.variant(P2_02MODEW::PULL_DOWN)
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
#[doc = "Values that can be written to the field `P2_03MODE`"]
pub enum P2_03MODEW {
    #[doc = "Pull-up. P2.3 pin has a pull-up resistor enabled."]
    PULL_UP,
    #[doc = "Repeater. P2.3 pin has repeater mode enabled."]
    REPEATER,
    #[doc = "Disabled. P2.3 pin has neither pull-up nor pull-down."]
    DISABLED,
    #[doc = "Pull-down. P2.3 has a pull-down resistor enabled."]
    PULL_DOWN,
}
impl P2_03MODEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            P2_03MODEW::PULL_UP => 0,
            P2_03MODEW::REPEATER => 1,
            P2_03MODEW::DISABLED => 2,
            P2_03MODEW::PULL_DOWN => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _P2_03MODEW<'a> {
    w: &'a mut W,
}
impl<'a> _P2_03MODEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: P2_03MODEW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Pull-up. P2.3 pin has a pull-up resistor enabled."]
    #[inline]
    pub fn pull_up(self) -> &'a mut W {
        self.variant(P2_03MODEW::PULL_UP)
    }
    #[doc = "Repeater. P2.3 pin has repeater mode enabled."]
    #[inline]
    pub fn repeater(self) -> &'a mut W {
        self.variant(P2_03MODEW::REPEATER)
    }
    #[doc = "Disabled. P2.3 pin has neither pull-up nor pull-down."]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(P2_03MODEW::DISABLED)
    }
    #[doc = "Pull-down. P2.3 has a pull-down resistor enabled."]
    #[inline]
    pub fn pull_down(self) -> &'a mut W {
        self.variant(P2_03MODEW::PULL_DOWN)
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
#[doc = "Values that can be written to the field `P2_04MODE`"]
pub enum P2_04MODEW {
    #[doc = "Pull-up. P2.4 pin has a pull-up resistor enabled."]
    PULL_UP,
    #[doc = "Repeater. P2.4 pin has repeater mode enabled."]
    REPEATER,
    #[doc = "Disabled. P2.4 pin has neither pull-up nor pull-down."]
    DISABLED,
    #[doc = "Pull-down. P2.4 has a pull-down resistor enabled."]
    PULL_DOWN,
}
impl P2_04MODEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            P2_04MODEW::PULL_UP => 0,
            P2_04MODEW::REPEATER => 1,
            P2_04MODEW::DISABLED => 2,
            P2_04MODEW::PULL_DOWN => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _P2_04MODEW<'a> {
    w: &'a mut W,
}
impl<'a> _P2_04MODEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: P2_04MODEW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Pull-up. P2.4 pin has a pull-up resistor enabled."]
    #[inline]
    pub fn pull_up(self) -> &'a mut W {
        self.variant(P2_04MODEW::PULL_UP)
    }
    #[doc = "Repeater. P2.4 pin has repeater mode enabled."]
    #[inline]
    pub fn repeater(self) -> &'a mut W {
        self.variant(P2_04MODEW::REPEATER)
    }
    #[doc = "Disabled. P2.4 pin has neither pull-up nor pull-down."]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(P2_04MODEW::DISABLED)
    }
    #[doc = "Pull-down. P2.4 has a pull-down resistor enabled."]
    #[inline]
    pub fn pull_down(self) -> &'a mut W {
        self.variant(P2_04MODEW::PULL_DOWN)
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
#[doc = "Values that can be written to the field `P2_05MODE`"]
pub enum P2_05MODEW {
    #[doc = "Pull-up. P2.5 pin has a pull-up resistor enabled."]
    PULL_UP,
    #[doc = "Repeater. P2.5 pin has repeater mode enabled."]
    REPEATER,
    #[doc = "Disabled. P2.5 pin has neither pull-up nor pull-down."]
    DISABLED,
    #[doc = "Pull-down. P2.5 has a pull-down resistor enabled."]
    PULL_DOWN,
}
impl P2_05MODEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            P2_05MODEW::PULL_UP => 0,
            P2_05MODEW::REPEATER => 1,
            P2_05MODEW::DISABLED => 2,
            P2_05MODEW::PULL_DOWN => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _P2_05MODEW<'a> {
    w: &'a mut W,
}
impl<'a> _P2_05MODEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: P2_05MODEW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Pull-up. P2.5 pin has a pull-up resistor enabled."]
    #[inline]
    pub fn pull_up(self) -> &'a mut W {
        self.variant(P2_05MODEW::PULL_UP)
    }
    #[doc = "Repeater. P2.5 pin has repeater mode enabled."]
    #[inline]
    pub fn repeater(self) -> &'a mut W {
        self.variant(P2_05MODEW::REPEATER)
    }
    #[doc = "Disabled. P2.5 pin has neither pull-up nor pull-down."]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(P2_05MODEW::DISABLED)
    }
    #[doc = "Pull-down. P2.5 has a pull-down resistor enabled."]
    #[inline]
    pub fn pull_down(self) -> &'a mut W {
        self.variant(P2_05MODEW::PULL_DOWN)
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
#[doc = "Values that can be written to the field `P2_06MODE`"]
pub enum P2_06MODEW {
    #[doc = "Pull-up. P2.6 pin has a pull-up resistor enabled."]
    PULL_UP,
    #[doc = "Repeater. P2.6 pin has repeater mode enabled."]
    REPEATER,
    #[doc = "Disabled. P2.6 pin has neither pull-up nor pull-down."]
    DISABLED,
    #[doc = "Pull-down. P2.6 has a pull-down resistor enabled."]
    PULL_DOWN,
}
impl P2_06MODEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            P2_06MODEW::PULL_UP => 0,
            P2_06MODEW::REPEATER => 1,
            P2_06MODEW::DISABLED => 2,
            P2_06MODEW::PULL_DOWN => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _P2_06MODEW<'a> {
    w: &'a mut W,
}
impl<'a> _P2_06MODEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: P2_06MODEW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Pull-up. P2.6 pin has a pull-up resistor enabled."]
    #[inline]
    pub fn pull_up(self) -> &'a mut W {
        self.variant(P2_06MODEW::PULL_UP)
    }
    #[doc = "Repeater. P2.6 pin has repeater mode enabled."]
    #[inline]
    pub fn repeater(self) -> &'a mut W {
        self.variant(P2_06MODEW::REPEATER)
    }
    #[doc = "Disabled. P2.6 pin has neither pull-up nor pull-down."]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(P2_06MODEW::DISABLED)
    }
    #[doc = "Pull-down. P2.6 has a pull-down resistor enabled."]
    #[inline]
    pub fn pull_down(self) -> &'a mut W {
        self.variant(P2_06MODEW::PULL_DOWN)
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
#[doc = "Values that can be written to the field `P2_07MODE`"]
pub enum P2_07MODEW {
    #[doc = "Pull-up. P2.7 pin has a pull-up resistor enabled."]
    PULL_UP,
    #[doc = "Repeater. P2.7 pin has repeater mode enabled."]
    REPEATER,
    #[doc = "Disabled. P2.7 pin has neither pull-up nor pull-down."]
    DISABLED,
    #[doc = "Pull-down. P2.7 has a pull-down resistor enabled."]
    PULL_DOWN,
}
impl P2_07MODEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            P2_07MODEW::PULL_UP => 0,
            P2_07MODEW::REPEATER => 1,
            P2_07MODEW::DISABLED => 2,
            P2_07MODEW::PULL_DOWN => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _P2_07MODEW<'a> {
    w: &'a mut W,
}
impl<'a> _P2_07MODEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: P2_07MODEW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Pull-up. P2.7 pin has a pull-up resistor enabled."]
    #[inline]
    pub fn pull_up(self) -> &'a mut W {
        self.variant(P2_07MODEW::PULL_UP)
    }
    #[doc = "Repeater. P2.7 pin has repeater mode enabled."]
    #[inline]
    pub fn repeater(self) -> &'a mut W {
        self.variant(P2_07MODEW::REPEATER)
    }
    #[doc = "Disabled. P2.7 pin has neither pull-up nor pull-down."]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(P2_07MODEW::DISABLED)
    }
    #[doc = "Pull-down. P2.7 has a pull-down resistor enabled."]
    #[inline]
    pub fn pull_down(self) -> &'a mut W {
        self.variant(P2_07MODEW::PULL_DOWN)
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
#[doc = "Values that can be written to the field `P2_08MODE`"]
pub enum P2_08MODEW {
    #[doc = "Pull-up. P2.8 pin has a pull-up resistor enabled."]
    PULL_UP,
    #[doc = "Repeater. P2.8 pin has repeater mode enabled."]
    REPEATER,
    #[doc = "Disabled. P2.8 pin has neither pull-up nor pull-down."]
    DISABLED,
    #[doc = "Pull-down. P2.8 has a pull-down resistor enabled."]
    PULL_DOWN,
}
impl P2_08MODEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            P2_08MODEW::PULL_UP => 0,
            P2_08MODEW::REPEATER => 1,
            P2_08MODEW::DISABLED => 2,
            P2_08MODEW::PULL_DOWN => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _P2_08MODEW<'a> {
    w: &'a mut W,
}
impl<'a> _P2_08MODEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: P2_08MODEW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Pull-up. P2.8 pin has a pull-up resistor enabled."]
    #[inline]
    pub fn pull_up(self) -> &'a mut W {
        self.variant(P2_08MODEW::PULL_UP)
    }
    #[doc = "Repeater. P2.8 pin has repeater mode enabled."]
    #[inline]
    pub fn repeater(self) -> &'a mut W {
        self.variant(P2_08MODEW::REPEATER)
    }
    #[doc = "Disabled. P2.8 pin has neither pull-up nor pull-down."]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(P2_08MODEW::DISABLED)
    }
    #[doc = "Pull-down. P2.8 has a pull-down resistor enabled."]
    #[inline]
    pub fn pull_down(self) -> &'a mut W {
        self.variant(P2_08MODEW::PULL_DOWN)
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
#[doc = "Values that can be written to the field `P2_09MODE`"]
pub enum P2_09MODEW {
    #[doc = "Pull-up. P2.9 pin has a pull-up resistor enabled."]
    PULL_UP,
    #[doc = "Repeater. P2.9 pin has repeater mode enabled."]
    REPEATER,
    #[doc = "Disabled. P2.9 pin has neither pull-up nor pull-down."]
    DISABLED,
    #[doc = "Pull-down. P2.9 has a pull-down resistor enabled."]
    PULL_DOWN,
}
impl P2_09MODEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            P2_09MODEW::PULL_UP => 0,
            P2_09MODEW::REPEATER => 1,
            P2_09MODEW::DISABLED => 2,
            P2_09MODEW::PULL_DOWN => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _P2_09MODEW<'a> {
    w: &'a mut W,
}
impl<'a> _P2_09MODEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: P2_09MODEW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Pull-up. P2.9 pin has a pull-up resistor enabled."]
    #[inline]
    pub fn pull_up(self) -> &'a mut W {
        self.variant(P2_09MODEW::PULL_UP)
    }
    #[doc = "Repeater. P2.9 pin has repeater mode enabled."]
    #[inline]
    pub fn repeater(self) -> &'a mut W {
        self.variant(P2_09MODEW::REPEATER)
    }
    #[doc = "Disabled. P2.9 pin has neither pull-up nor pull-down."]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(P2_09MODEW::DISABLED)
    }
    #[doc = "Pull-down. P2.9 has a pull-down resistor enabled."]
    #[inline]
    pub fn pull_down(self) -> &'a mut W {
        self.variant(P2_09MODEW::PULL_DOWN)
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
#[doc = "Values that can be written to the field `P2_10MODE`"]
pub enum P2_10MODEW {
    #[doc = "Pull-up. P2.10 pin has a pull-up resistor enabled."]
    PULL_UP,
    #[doc = "Repeater. P2.10 pin has repeater mode enabled."]
    REPEATER,
    #[doc = "Disabled. P2.10 pin has neither pull-up nor pull-down."]
    DISABLED,
    #[doc = "Pull-down. P2.10 has a pull-down resistor enabled."]
    PULL_DOWN,
}
impl P2_10MODEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            P2_10MODEW::PULL_UP => 0,
            P2_10MODEW::REPEATER => 1,
            P2_10MODEW::DISABLED => 2,
            P2_10MODEW::PULL_DOWN => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _P2_10MODEW<'a> {
    w: &'a mut W,
}
impl<'a> _P2_10MODEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: P2_10MODEW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Pull-up. P2.10 pin has a pull-up resistor enabled."]
    #[inline]
    pub fn pull_up(self) -> &'a mut W {
        self.variant(P2_10MODEW::PULL_UP)
    }
    #[doc = "Repeater. P2.10 pin has repeater mode enabled."]
    #[inline]
    pub fn repeater(self) -> &'a mut W {
        self.variant(P2_10MODEW::REPEATER)
    }
    #[doc = "Disabled. P2.10 pin has neither pull-up nor pull-down."]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(P2_10MODEW::DISABLED)
    }
    #[doc = "Pull-down. P2.10 has a pull-down resistor enabled."]
    #[inline]
    pub fn pull_down(self) -> &'a mut W {
        self.variant(P2_10MODEW::PULL_DOWN)
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
#[doc = "Values that can be written to the field `P2_11MODE`"]
pub enum P2_11MODEW {
    #[doc = "Pull-up. P2.11 pin has a pull-up resistor enabled."]
    PULL_UP,
    #[doc = "Repeater. P2.11 pin has repeater mode enabled."]
    REPEATER,
    #[doc = "Disabled. P2.11 pin has neither pull-up nor pull-down."]
    DISABLED,
    #[doc = "Pull-down. P2.11 has a pull-down resistor enabled."]
    PULL_DOWN,
}
impl P2_11MODEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            P2_11MODEW::PULL_UP => 0,
            P2_11MODEW::REPEATER => 1,
            P2_11MODEW::DISABLED => 2,
            P2_11MODEW::PULL_DOWN => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _P2_11MODEW<'a> {
    w: &'a mut W,
}
impl<'a> _P2_11MODEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: P2_11MODEW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Pull-up. P2.11 pin has a pull-up resistor enabled."]
    #[inline]
    pub fn pull_up(self) -> &'a mut W {
        self.variant(P2_11MODEW::PULL_UP)
    }
    #[doc = "Repeater. P2.11 pin has repeater mode enabled."]
    #[inline]
    pub fn repeater(self) -> &'a mut W {
        self.variant(P2_11MODEW::REPEATER)
    }
    #[doc = "Disabled. P2.11 pin has neither pull-up nor pull-down."]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(P2_11MODEW::DISABLED)
    }
    #[doc = "Pull-down. P2.11 has a pull-down resistor enabled."]
    #[inline]
    pub fn pull_down(self) -> &'a mut W {
        self.variant(P2_11MODEW::PULL_DOWN)
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
#[doc = "Values that can be written to the field `P2_12MODE`"]
pub enum P2_12MODEW {
    #[doc = "Pull-up. P2.12 pin has a pull-up resistor enabled."]
    PULL_UP,
    #[doc = "Repeater. P2.12 pin has repeater mode enabled."]
    REPEATER,
    #[doc = "Disabled. P2.12 pin has neither pull-up nor pull-down."]
    DISABLED,
    #[doc = "Pull-down. P2.12 has a pull-down resistor enabled."]
    PULL_DOWN,
}
impl P2_12MODEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            P2_12MODEW::PULL_UP => 0,
            P2_12MODEW::REPEATER => 1,
            P2_12MODEW::DISABLED => 2,
            P2_12MODEW::PULL_DOWN => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _P2_12MODEW<'a> {
    w: &'a mut W,
}
impl<'a> _P2_12MODEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: P2_12MODEW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Pull-up. P2.12 pin has a pull-up resistor enabled."]
    #[inline]
    pub fn pull_up(self) -> &'a mut W {
        self.variant(P2_12MODEW::PULL_UP)
    }
    #[doc = "Repeater. P2.12 pin has repeater mode enabled."]
    #[inline]
    pub fn repeater(self) -> &'a mut W {
        self.variant(P2_12MODEW::REPEATER)
    }
    #[doc = "Disabled. P2.12 pin has neither pull-up nor pull-down."]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(P2_12MODEW::DISABLED)
    }
    #[doc = "Pull-down. P2.12 has a pull-down resistor enabled."]
    #[inline]
    pub fn pull_down(self) -> &'a mut W {
        self.variant(P2_12MODEW::PULL_DOWN)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 24;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `P2_13MODE`"]
pub enum P2_13MODEW {
    #[doc = "Pull-up. P2.13 pin has a pull-up resistor enabled."]
    PULL_UP,
    #[doc = "Repeater. P2.13 pin has repeater mode enabled."]
    REPEATER,
    #[doc = "Disabled. P2.13 pin has neither pull-up nor pull-down."]
    DISABLED,
    #[doc = "Pull-down. P2.13 has a pull-down resistor enabled."]
    PULL_DOWN,
}
impl P2_13MODEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            P2_13MODEW::PULL_UP => 0,
            P2_13MODEW::REPEATER => 1,
            P2_13MODEW::DISABLED => 2,
            P2_13MODEW::PULL_DOWN => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _P2_13MODEW<'a> {
    w: &'a mut W,
}
impl<'a> _P2_13MODEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: P2_13MODEW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Pull-up. P2.13 pin has a pull-up resistor enabled."]
    #[inline]
    pub fn pull_up(self) -> &'a mut W {
        self.variant(P2_13MODEW::PULL_UP)
    }
    #[doc = "Repeater. P2.13 pin has repeater mode enabled."]
    #[inline]
    pub fn repeater(self) -> &'a mut W {
        self.variant(P2_13MODEW::REPEATER)
    }
    #[doc = "Disabled. P2.13 pin has neither pull-up nor pull-down."]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(P2_13MODEW::DISABLED)
    }
    #[doc = "Pull-down. P2.13 has a pull-down resistor enabled."]
    #[inline]
    pub fn pull_down(self) -> &'a mut W {
        self.variant(P2_13MODEW::PULL_DOWN)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
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
    #[doc = "Bits 0:1 - Port 2 pin 0 control."]
    #[inline]
    pub fn p2_00mode(&self) -> P2_00MODER {
        P2_00MODER::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 2:3 - Port 2 pin 1 control."]
    #[inline]
    pub fn p2_01mode(&self) -> P2_01MODER {
        P2_01MODER::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 4:5 - Port 2 pin 2 control."]
    #[inline]
    pub fn p2_02mode(&self) -> P2_02MODER {
        P2_02MODER::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 6:7 - Port 2 pin 3 control."]
    #[inline]
    pub fn p2_03mode(&self) -> P2_03MODER {
        P2_03MODER::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 8:9 - Port 2 pin 4 control."]
    #[inline]
    pub fn p2_04mode(&self) -> P2_04MODER {
        P2_04MODER::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 10:11 - Port 2 pin 5 control."]
    #[inline]
    pub fn p2_05mode(&self) -> P2_05MODER {
        P2_05MODER::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 12:13 - Port 2 pin 6 control."]
    #[inline]
    pub fn p2_06mode(&self) -> P2_06MODER {
        P2_06MODER::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 14:15 - Port 2 pin 7 control."]
    #[inline]
    pub fn p2_07mode(&self) -> P2_07MODER {
        P2_07MODER::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 16:17 - Port 2 pin 8 control."]
    #[inline]
    pub fn p2_08mode(&self) -> P2_08MODER {
        P2_08MODER::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 18:19 - Port 2 pin 9 control."]
    #[inline]
    pub fn p2_09mode(&self) -> P2_09MODER {
        P2_09MODER::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 20:21 - Port 2 pin 10 control."]
    #[inline]
    pub fn p2_10mode(&self) -> P2_10MODER {
        P2_10MODER::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 22:23 - Port 2 pin 11 control."]
    #[inline]
    pub fn p2_11mode(&self) -> P2_11MODER {
        P2_11MODER::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 22;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 24:25 - Port 2 pin 12 control."]
    #[inline]
    pub fn p2_12mode(&self) -> P2_12MODER {
        P2_12MODER::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 26:27 - Port 2 pin 13 control."]
    #[inline]
    pub fn p2_13mode(&self) -> P2_13MODER {
        P2_13MODER::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 26;
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
    #[doc = "Bits 0:1 - Port 2 pin 0 control."]
    #[inline]
    pub fn p2_00mode(&mut self) -> _P2_00MODEW {
        _P2_00MODEW { w: self }
    }
    #[doc = "Bits 2:3 - Port 2 pin 1 control."]
    #[inline]
    pub fn p2_01mode(&mut self) -> _P2_01MODEW {
        _P2_01MODEW { w: self }
    }
    #[doc = "Bits 4:5 - Port 2 pin 2 control."]
    #[inline]
    pub fn p2_02mode(&mut self) -> _P2_02MODEW {
        _P2_02MODEW { w: self }
    }
    #[doc = "Bits 6:7 - Port 2 pin 3 control."]
    #[inline]
    pub fn p2_03mode(&mut self) -> _P2_03MODEW {
        _P2_03MODEW { w: self }
    }
    #[doc = "Bits 8:9 - Port 2 pin 4 control."]
    #[inline]
    pub fn p2_04mode(&mut self) -> _P2_04MODEW {
        _P2_04MODEW { w: self }
    }
    #[doc = "Bits 10:11 - Port 2 pin 5 control."]
    #[inline]
    pub fn p2_05mode(&mut self) -> _P2_05MODEW {
        _P2_05MODEW { w: self }
    }
    #[doc = "Bits 12:13 - Port 2 pin 6 control."]
    #[inline]
    pub fn p2_06mode(&mut self) -> _P2_06MODEW {
        _P2_06MODEW { w: self }
    }
    #[doc = "Bits 14:15 - Port 2 pin 7 control."]
    #[inline]
    pub fn p2_07mode(&mut self) -> _P2_07MODEW {
        _P2_07MODEW { w: self }
    }
    #[doc = "Bits 16:17 - Port 2 pin 8 control."]
    #[inline]
    pub fn p2_08mode(&mut self) -> _P2_08MODEW {
        _P2_08MODEW { w: self }
    }
    #[doc = "Bits 18:19 - Port 2 pin 9 control."]
    #[inline]
    pub fn p2_09mode(&mut self) -> _P2_09MODEW {
        _P2_09MODEW { w: self }
    }
    #[doc = "Bits 20:21 - Port 2 pin 10 control."]
    #[inline]
    pub fn p2_10mode(&mut self) -> _P2_10MODEW {
        _P2_10MODEW { w: self }
    }
    #[doc = "Bits 22:23 - Port 2 pin 11 control."]
    #[inline]
    pub fn p2_11mode(&mut self) -> _P2_11MODEW {
        _P2_11MODEW { w: self }
    }
    #[doc = "Bits 24:25 - Port 2 pin 12 control."]
    #[inline]
    pub fn p2_12mode(&mut self) -> _P2_12MODEW {
        _P2_12MODEW { w: self }
    }
    #[doc = "Bits 26:27 - Port 2 pin 13 control."]
    #[inline]
    pub fn p2_13mode(&mut self) -> _P2_13MODEW {
        _P2_13MODEW { w: self }
    }
}
