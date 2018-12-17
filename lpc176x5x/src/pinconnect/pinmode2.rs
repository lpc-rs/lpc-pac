#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::PINMODE2 {
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
#[doc = "Possible values of the field `P1_00MODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P1_00MODER {
    #[doc = "Pull-up. P1.0 pin has a pull-up resistor enabled."]
    PULL_UP,
    #[doc = "Repeater. P1.0 pin has repeater mode enabled."]
    REPEATER,
    #[doc = "Disabled. P1.0 pin has neither pull-up nor pull-down."]
    DISABLED,
    #[doc = "Pull-down. P1.0 has a pull-down resistor enabled."]
    PULL_DOWN,
}
impl P1_00MODER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            P1_00MODER::PULL_UP => 0,
            P1_00MODER::REPEATER => 1,
            P1_00MODER::DISABLED => 2,
            P1_00MODER::PULL_DOWN => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> P1_00MODER {
        match value {
            0 => P1_00MODER::PULL_UP,
            1 => P1_00MODER::REPEATER,
            2 => P1_00MODER::DISABLED,
            3 => P1_00MODER::PULL_DOWN,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PULL_UP`"]
    #[inline]
    pub fn is_pull_up(&self) -> bool {
        *self == P1_00MODER::PULL_UP
    }
    #[doc = "Checks if the value of the field is `REPEATER`"]
    #[inline]
    pub fn is_repeater(&self) -> bool {
        *self == P1_00MODER::REPEATER
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == P1_00MODER::DISABLED
    }
    #[doc = "Checks if the value of the field is `PULL_DOWN`"]
    #[inline]
    pub fn is_pull_down(&self) -> bool {
        *self == P1_00MODER::PULL_DOWN
    }
}
#[doc = "Possible values of the field `P1_01MODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P1_01MODER {
    #[doc = "Pull-up. P1.1 pin has a pull-up resistor enabled."]
    PULL_UP,
    #[doc = "Repeater. P1.1 pin has repeater mode enabled."]
    REPEATER,
    #[doc = "Disabled. P1.1 pin has neither pull-up nor pull-down."]
    DISABLED,
    #[doc = "Pull-down. P1.1 has a pull-down resistor enabled."]
    PULL_DOWN,
}
impl P1_01MODER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            P1_01MODER::PULL_UP => 0,
            P1_01MODER::REPEATER => 1,
            P1_01MODER::DISABLED => 2,
            P1_01MODER::PULL_DOWN => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> P1_01MODER {
        match value {
            0 => P1_01MODER::PULL_UP,
            1 => P1_01MODER::REPEATER,
            2 => P1_01MODER::DISABLED,
            3 => P1_01MODER::PULL_DOWN,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PULL_UP`"]
    #[inline]
    pub fn is_pull_up(&self) -> bool {
        *self == P1_01MODER::PULL_UP
    }
    #[doc = "Checks if the value of the field is `REPEATER`"]
    #[inline]
    pub fn is_repeater(&self) -> bool {
        *self == P1_01MODER::REPEATER
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == P1_01MODER::DISABLED
    }
    #[doc = "Checks if the value of the field is `PULL_DOWN`"]
    #[inline]
    pub fn is_pull_down(&self) -> bool {
        *self == P1_01MODER::PULL_DOWN
    }
}
#[doc = "Possible values of the field `P1_04MODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P1_04MODER {
    #[doc = "Pull-up. P1.4 pin has a pull-up resistor enabled."]
    PULL_UP,
    #[doc = "Repeater. P1.4 pin has repeater mode enabled."]
    REPEATER,
    #[doc = "Disabled. P1.4 pin has neither pull-up nor pull-down."]
    DISABLED,
    #[doc = "Pull-down. P1.4 has a pull-down resistor enabled."]
    PULL_DOWN,
}
impl P1_04MODER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            P1_04MODER::PULL_UP => 0,
            P1_04MODER::REPEATER => 1,
            P1_04MODER::DISABLED => 2,
            P1_04MODER::PULL_DOWN => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> P1_04MODER {
        match value {
            0 => P1_04MODER::PULL_UP,
            1 => P1_04MODER::REPEATER,
            2 => P1_04MODER::DISABLED,
            3 => P1_04MODER::PULL_DOWN,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PULL_UP`"]
    #[inline]
    pub fn is_pull_up(&self) -> bool {
        *self == P1_04MODER::PULL_UP
    }
    #[doc = "Checks if the value of the field is `REPEATER`"]
    #[inline]
    pub fn is_repeater(&self) -> bool {
        *self == P1_04MODER::REPEATER
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == P1_04MODER::DISABLED
    }
    #[doc = "Checks if the value of the field is `PULL_DOWN`"]
    #[inline]
    pub fn is_pull_down(&self) -> bool {
        *self == P1_04MODER::PULL_DOWN
    }
}
#[doc = "Possible values of the field `P1_08MODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P1_08MODER {
    #[doc = "Pull-up. P1.8 pin has a pull-up resistor enabled."]
    PULL_UP,
    #[doc = "Repeater. P1.8 pin has repeater mode enabled."]
    REPEATER,
    #[doc = "Disabled. P1.8 pin has neither pull-up nor pull-down."]
    DISABLED,
    #[doc = "Pull-down. P1.8 has a pull-down resistor enabled."]
    PULL_DOWN,
}
impl P1_08MODER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            P1_08MODER::PULL_UP => 0,
            P1_08MODER::REPEATER => 1,
            P1_08MODER::DISABLED => 2,
            P1_08MODER::PULL_DOWN => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> P1_08MODER {
        match value {
            0 => P1_08MODER::PULL_UP,
            1 => P1_08MODER::REPEATER,
            2 => P1_08MODER::DISABLED,
            3 => P1_08MODER::PULL_DOWN,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PULL_UP`"]
    #[inline]
    pub fn is_pull_up(&self) -> bool {
        *self == P1_08MODER::PULL_UP
    }
    #[doc = "Checks if the value of the field is `REPEATER`"]
    #[inline]
    pub fn is_repeater(&self) -> bool {
        *self == P1_08MODER::REPEATER
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == P1_08MODER::DISABLED
    }
    #[doc = "Checks if the value of the field is `PULL_DOWN`"]
    #[inline]
    pub fn is_pull_down(&self) -> bool {
        *self == P1_08MODER::PULL_DOWN
    }
}
#[doc = "Possible values of the field `P1_09MODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P1_09MODER {
    #[doc = "Pull-up. P1.9 pin has a pull-up resistor enabled."]
    PULL_UP,
    #[doc = "Repeater. P1.9 pin has repeater mode enabled."]
    REPEATER,
    #[doc = "Disabled. P1.9 pin has neither pull-up nor pull-down."]
    DISABLED,
    #[doc = "Pull-down. P1.9 has a pull-down resistor enabled."]
    PULL_DOWN,
}
impl P1_09MODER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            P1_09MODER::PULL_UP => 0,
            P1_09MODER::REPEATER => 1,
            P1_09MODER::DISABLED => 2,
            P1_09MODER::PULL_DOWN => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> P1_09MODER {
        match value {
            0 => P1_09MODER::PULL_UP,
            1 => P1_09MODER::REPEATER,
            2 => P1_09MODER::DISABLED,
            3 => P1_09MODER::PULL_DOWN,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PULL_UP`"]
    #[inline]
    pub fn is_pull_up(&self) -> bool {
        *self == P1_09MODER::PULL_UP
    }
    #[doc = "Checks if the value of the field is `REPEATER`"]
    #[inline]
    pub fn is_repeater(&self) -> bool {
        *self == P1_09MODER::REPEATER
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == P1_09MODER::DISABLED
    }
    #[doc = "Checks if the value of the field is `PULL_DOWN`"]
    #[inline]
    pub fn is_pull_down(&self) -> bool {
        *self == P1_09MODER::PULL_DOWN
    }
}
#[doc = "Possible values of the field `P1_10MODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P1_10MODER {
    #[doc = "Pull-up. P1.10 pin has a pull-up resistor enabled."]
    PULL_UP,
    #[doc = "Repeater. P1.10 pin has repeater mode enabled."]
    REPEATER,
    #[doc = "Disabled. P1.10 pin has neither pull-up nor pull-down."]
    DISABLED,
    #[doc = "Pull-down. P1.10 has a pull-down resistor enabled."]
    PULL_DOWN,
}
impl P1_10MODER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            P1_10MODER::PULL_UP => 0,
            P1_10MODER::REPEATER => 1,
            P1_10MODER::DISABLED => 2,
            P1_10MODER::PULL_DOWN => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> P1_10MODER {
        match value {
            0 => P1_10MODER::PULL_UP,
            1 => P1_10MODER::REPEATER,
            2 => P1_10MODER::DISABLED,
            3 => P1_10MODER::PULL_DOWN,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PULL_UP`"]
    #[inline]
    pub fn is_pull_up(&self) -> bool {
        *self == P1_10MODER::PULL_UP
    }
    #[doc = "Checks if the value of the field is `REPEATER`"]
    #[inline]
    pub fn is_repeater(&self) -> bool {
        *self == P1_10MODER::REPEATER
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == P1_10MODER::DISABLED
    }
    #[doc = "Checks if the value of the field is `PULL_DOWN`"]
    #[inline]
    pub fn is_pull_down(&self) -> bool {
        *self == P1_10MODER::PULL_DOWN
    }
}
#[doc = "Possible values of the field `P1_14MODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P1_14MODER {
    #[doc = "Pull-up. P1.14 pin has a pull-up resistor enabled."]
    PULL_UP,
    #[doc = "Repeater. P1.14 pin has repeater mode enabled."]
    REPEATER,
    #[doc = "Disabled. P1.14 pin has neither pull-up nor pull-down."]
    DISABLED,
    #[doc = "Pull-down. P1.14 has a pull-down resistor enabled."]
    PULL_DOWN,
}
impl P1_14MODER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            P1_14MODER::PULL_UP => 0,
            P1_14MODER::REPEATER => 1,
            P1_14MODER::DISABLED => 2,
            P1_14MODER::PULL_DOWN => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> P1_14MODER {
        match value {
            0 => P1_14MODER::PULL_UP,
            1 => P1_14MODER::REPEATER,
            2 => P1_14MODER::DISABLED,
            3 => P1_14MODER::PULL_DOWN,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PULL_UP`"]
    #[inline]
    pub fn is_pull_up(&self) -> bool {
        *self == P1_14MODER::PULL_UP
    }
    #[doc = "Checks if the value of the field is `REPEATER`"]
    #[inline]
    pub fn is_repeater(&self) -> bool {
        *self == P1_14MODER::REPEATER
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == P1_14MODER::DISABLED
    }
    #[doc = "Checks if the value of the field is `PULL_DOWN`"]
    #[inline]
    pub fn is_pull_down(&self) -> bool {
        *self == P1_14MODER::PULL_DOWN
    }
}
#[doc = "Possible values of the field `P1_15MODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P1_15MODER {
    #[doc = "Pull-up. P1.15 pin has a pull-up resistor enabled."]
    PULL_UP,
    #[doc = "Repeater. P1.15 pin has repeater mode enabled."]
    REPEATER,
    #[doc = "Disabled. P1.15 pin has neither pull-up nor pull-down."]
    DISABLED,
    #[doc = "Pull-down. P1.15 has a pull-down resistor enabled."]
    PULL_DOWN,
}
impl P1_15MODER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            P1_15MODER::PULL_UP => 0,
            P1_15MODER::REPEATER => 1,
            P1_15MODER::DISABLED => 2,
            P1_15MODER::PULL_DOWN => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> P1_15MODER {
        match value {
            0 => P1_15MODER::PULL_UP,
            1 => P1_15MODER::REPEATER,
            2 => P1_15MODER::DISABLED,
            3 => P1_15MODER::PULL_DOWN,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PULL_UP`"]
    #[inline]
    pub fn is_pull_up(&self) -> bool {
        *self == P1_15MODER::PULL_UP
    }
    #[doc = "Checks if the value of the field is `REPEATER`"]
    #[inline]
    pub fn is_repeater(&self) -> bool {
        *self == P1_15MODER::REPEATER
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == P1_15MODER::DISABLED
    }
    #[doc = "Checks if the value of the field is `PULL_DOWN`"]
    #[inline]
    pub fn is_pull_down(&self) -> bool {
        *self == P1_15MODER::PULL_DOWN
    }
}
#[doc = "Values that can be written to the field `P1_00MODE`"]
pub enum P1_00MODEW {
    #[doc = "Pull-up. P1.0 pin has a pull-up resistor enabled."]
    PULL_UP,
    #[doc = "Repeater. P1.0 pin has repeater mode enabled."]
    REPEATER,
    #[doc = "Disabled. P1.0 pin has neither pull-up nor pull-down."]
    DISABLED,
    #[doc = "Pull-down. P1.0 has a pull-down resistor enabled."]
    PULL_DOWN,
}
impl P1_00MODEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            P1_00MODEW::PULL_UP => 0,
            P1_00MODEW::REPEATER => 1,
            P1_00MODEW::DISABLED => 2,
            P1_00MODEW::PULL_DOWN => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _P1_00MODEW<'a> {
    w: &'a mut W,
}
impl<'a> _P1_00MODEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: P1_00MODEW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Pull-up. P1.0 pin has a pull-up resistor enabled."]
    #[inline]
    pub fn pull_up(self) -> &'a mut W {
        self.variant(P1_00MODEW::PULL_UP)
    }
    #[doc = "Repeater. P1.0 pin has repeater mode enabled."]
    #[inline]
    pub fn repeater(self) -> &'a mut W {
        self.variant(P1_00MODEW::REPEATER)
    }
    #[doc = "Disabled. P1.0 pin has neither pull-up nor pull-down."]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(P1_00MODEW::DISABLED)
    }
    #[doc = "Pull-down. P1.0 has a pull-down resistor enabled."]
    #[inline]
    pub fn pull_down(self) -> &'a mut W {
        self.variant(P1_00MODEW::PULL_DOWN)
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
#[doc = "Values that can be written to the field `P1_01MODE`"]
pub enum P1_01MODEW {
    #[doc = "Pull-up. P1.1 pin has a pull-up resistor enabled."]
    PULL_UP,
    #[doc = "Repeater. P1.1 pin has repeater mode enabled."]
    REPEATER,
    #[doc = "Disabled. P1.1 pin has neither pull-up nor pull-down."]
    DISABLED,
    #[doc = "Pull-down. P1.1 has a pull-down resistor enabled."]
    PULL_DOWN,
}
impl P1_01MODEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            P1_01MODEW::PULL_UP => 0,
            P1_01MODEW::REPEATER => 1,
            P1_01MODEW::DISABLED => 2,
            P1_01MODEW::PULL_DOWN => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _P1_01MODEW<'a> {
    w: &'a mut W,
}
impl<'a> _P1_01MODEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: P1_01MODEW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Pull-up. P1.1 pin has a pull-up resistor enabled."]
    #[inline]
    pub fn pull_up(self) -> &'a mut W {
        self.variant(P1_01MODEW::PULL_UP)
    }
    #[doc = "Repeater. P1.1 pin has repeater mode enabled."]
    #[inline]
    pub fn repeater(self) -> &'a mut W {
        self.variant(P1_01MODEW::REPEATER)
    }
    #[doc = "Disabled. P1.1 pin has neither pull-up nor pull-down."]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(P1_01MODEW::DISABLED)
    }
    #[doc = "Pull-down. P1.1 has a pull-down resistor enabled."]
    #[inline]
    pub fn pull_down(self) -> &'a mut W {
        self.variant(P1_01MODEW::PULL_DOWN)
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
#[doc = "Values that can be written to the field `P1_04MODE`"]
pub enum P1_04MODEW {
    #[doc = "Pull-up. P1.4 pin has a pull-up resistor enabled."]
    PULL_UP,
    #[doc = "Repeater. P1.4 pin has repeater mode enabled."]
    REPEATER,
    #[doc = "Disabled. P1.4 pin has neither pull-up nor pull-down."]
    DISABLED,
    #[doc = "Pull-down. P1.4 has a pull-down resistor enabled."]
    PULL_DOWN,
}
impl P1_04MODEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            P1_04MODEW::PULL_UP => 0,
            P1_04MODEW::REPEATER => 1,
            P1_04MODEW::DISABLED => 2,
            P1_04MODEW::PULL_DOWN => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _P1_04MODEW<'a> {
    w: &'a mut W,
}
impl<'a> _P1_04MODEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: P1_04MODEW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Pull-up. P1.4 pin has a pull-up resistor enabled."]
    #[inline]
    pub fn pull_up(self) -> &'a mut W {
        self.variant(P1_04MODEW::PULL_UP)
    }
    #[doc = "Repeater. P1.4 pin has repeater mode enabled."]
    #[inline]
    pub fn repeater(self) -> &'a mut W {
        self.variant(P1_04MODEW::REPEATER)
    }
    #[doc = "Disabled. P1.4 pin has neither pull-up nor pull-down."]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(P1_04MODEW::DISABLED)
    }
    #[doc = "Pull-down. P1.4 has a pull-down resistor enabled."]
    #[inline]
    pub fn pull_down(self) -> &'a mut W {
        self.variant(P1_04MODEW::PULL_DOWN)
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
#[doc = "Values that can be written to the field `P1_08MODE`"]
pub enum P1_08MODEW {
    #[doc = "Pull-up. P1.8 pin has a pull-up resistor enabled."]
    PULL_UP,
    #[doc = "Repeater. P1.8 pin has repeater mode enabled."]
    REPEATER,
    #[doc = "Disabled. P1.8 pin has neither pull-up nor pull-down."]
    DISABLED,
    #[doc = "Pull-down. P1.8 has a pull-down resistor enabled."]
    PULL_DOWN,
}
impl P1_08MODEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            P1_08MODEW::PULL_UP => 0,
            P1_08MODEW::REPEATER => 1,
            P1_08MODEW::DISABLED => 2,
            P1_08MODEW::PULL_DOWN => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _P1_08MODEW<'a> {
    w: &'a mut W,
}
impl<'a> _P1_08MODEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: P1_08MODEW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Pull-up. P1.8 pin has a pull-up resistor enabled."]
    #[inline]
    pub fn pull_up(self) -> &'a mut W {
        self.variant(P1_08MODEW::PULL_UP)
    }
    #[doc = "Repeater. P1.8 pin has repeater mode enabled."]
    #[inline]
    pub fn repeater(self) -> &'a mut W {
        self.variant(P1_08MODEW::REPEATER)
    }
    #[doc = "Disabled. P1.8 pin has neither pull-up nor pull-down."]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(P1_08MODEW::DISABLED)
    }
    #[doc = "Pull-down. P1.8 has a pull-down resistor enabled."]
    #[inline]
    pub fn pull_down(self) -> &'a mut W {
        self.variant(P1_08MODEW::PULL_DOWN)
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
#[doc = "Values that can be written to the field `P1_09MODE`"]
pub enum P1_09MODEW {
    #[doc = "Pull-up. P1.9 pin has a pull-up resistor enabled."]
    PULL_UP,
    #[doc = "Repeater. P1.9 pin has repeater mode enabled."]
    REPEATER,
    #[doc = "Disabled. P1.9 pin has neither pull-up nor pull-down."]
    DISABLED,
    #[doc = "Pull-down. P1.9 has a pull-down resistor enabled."]
    PULL_DOWN,
}
impl P1_09MODEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            P1_09MODEW::PULL_UP => 0,
            P1_09MODEW::REPEATER => 1,
            P1_09MODEW::DISABLED => 2,
            P1_09MODEW::PULL_DOWN => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _P1_09MODEW<'a> {
    w: &'a mut W,
}
impl<'a> _P1_09MODEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: P1_09MODEW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Pull-up. P1.9 pin has a pull-up resistor enabled."]
    #[inline]
    pub fn pull_up(self) -> &'a mut W {
        self.variant(P1_09MODEW::PULL_UP)
    }
    #[doc = "Repeater. P1.9 pin has repeater mode enabled."]
    #[inline]
    pub fn repeater(self) -> &'a mut W {
        self.variant(P1_09MODEW::REPEATER)
    }
    #[doc = "Disabled. P1.9 pin has neither pull-up nor pull-down."]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(P1_09MODEW::DISABLED)
    }
    #[doc = "Pull-down. P1.9 has a pull-down resistor enabled."]
    #[inline]
    pub fn pull_down(self) -> &'a mut W {
        self.variant(P1_09MODEW::PULL_DOWN)
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
#[doc = "Values that can be written to the field `P1_10MODE`"]
pub enum P1_10MODEW {
    #[doc = "Pull-up. P1.10 pin has a pull-up resistor enabled."]
    PULL_UP,
    #[doc = "Repeater. P1.10 pin has repeater mode enabled."]
    REPEATER,
    #[doc = "Disabled. P1.10 pin has neither pull-up nor pull-down."]
    DISABLED,
    #[doc = "Pull-down. P1.10 has a pull-down resistor enabled."]
    PULL_DOWN,
}
impl P1_10MODEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            P1_10MODEW::PULL_UP => 0,
            P1_10MODEW::REPEATER => 1,
            P1_10MODEW::DISABLED => 2,
            P1_10MODEW::PULL_DOWN => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _P1_10MODEW<'a> {
    w: &'a mut W,
}
impl<'a> _P1_10MODEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: P1_10MODEW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Pull-up. P1.10 pin has a pull-up resistor enabled."]
    #[inline]
    pub fn pull_up(self) -> &'a mut W {
        self.variant(P1_10MODEW::PULL_UP)
    }
    #[doc = "Repeater. P1.10 pin has repeater mode enabled."]
    #[inline]
    pub fn repeater(self) -> &'a mut W {
        self.variant(P1_10MODEW::REPEATER)
    }
    #[doc = "Disabled. P1.10 pin has neither pull-up nor pull-down."]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(P1_10MODEW::DISABLED)
    }
    #[doc = "Pull-down. P1.10 has a pull-down resistor enabled."]
    #[inline]
    pub fn pull_down(self) -> &'a mut W {
        self.variant(P1_10MODEW::PULL_DOWN)
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
#[doc = "Values that can be written to the field `P1_14MODE`"]
pub enum P1_14MODEW {
    #[doc = "Pull-up. P1.14 pin has a pull-up resistor enabled."]
    PULL_UP,
    #[doc = "Repeater. P1.14 pin has repeater mode enabled."]
    REPEATER,
    #[doc = "Disabled. P1.14 pin has neither pull-up nor pull-down."]
    DISABLED,
    #[doc = "Pull-down. P1.14 has a pull-down resistor enabled."]
    PULL_DOWN,
}
impl P1_14MODEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            P1_14MODEW::PULL_UP => 0,
            P1_14MODEW::REPEATER => 1,
            P1_14MODEW::DISABLED => 2,
            P1_14MODEW::PULL_DOWN => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _P1_14MODEW<'a> {
    w: &'a mut W,
}
impl<'a> _P1_14MODEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: P1_14MODEW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Pull-up. P1.14 pin has a pull-up resistor enabled."]
    #[inline]
    pub fn pull_up(self) -> &'a mut W {
        self.variant(P1_14MODEW::PULL_UP)
    }
    #[doc = "Repeater. P1.14 pin has repeater mode enabled."]
    #[inline]
    pub fn repeater(self) -> &'a mut W {
        self.variant(P1_14MODEW::REPEATER)
    }
    #[doc = "Disabled. P1.14 pin has neither pull-up nor pull-down."]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(P1_14MODEW::DISABLED)
    }
    #[doc = "Pull-down. P1.14 has a pull-down resistor enabled."]
    #[inline]
    pub fn pull_down(self) -> &'a mut W {
        self.variant(P1_14MODEW::PULL_DOWN)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 28;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `P1_15MODE`"]
pub enum P1_15MODEW {
    #[doc = "Pull-up. P1.15 pin has a pull-up resistor enabled."]
    PULL_UP,
    #[doc = "Repeater. P1.15 pin has repeater mode enabled."]
    REPEATER,
    #[doc = "Disabled. P1.15 pin has neither pull-up nor pull-down."]
    DISABLED,
    #[doc = "Pull-down. P1.15 has a pull-down resistor enabled."]
    PULL_DOWN,
}
impl P1_15MODEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            P1_15MODEW::PULL_UP => 0,
            P1_15MODEW::REPEATER => 1,
            P1_15MODEW::DISABLED => 2,
            P1_15MODEW::PULL_DOWN => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _P1_15MODEW<'a> {
    w: &'a mut W,
}
impl<'a> _P1_15MODEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: P1_15MODEW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Pull-up. P1.15 pin has a pull-up resistor enabled."]
    #[inline]
    pub fn pull_up(self) -> &'a mut W {
        self.variant(P1_15MODEW::PULL_UP)
    }
    #[doc = "Repeater. P1.15 pin has repeater mode enabled."]
    #[inline]
    pub fn repeater(self) -> &'a mut W {
        self.variant(P1_15MODEW::REPEATER)
    }
    #[doc = "Disabled. P1.15 pin has neither pull-up nor pull-down."]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(P1_15MODEW::DISABLED)
    }
    #[doc = "Pull-down. P1.15 has a pull-down resistor enabled."]
    #[inline]
    pub fn pull_down(self) -> &'a mut W {
        self.variant(P1_15MODEW::PULL_DOWN)
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
    #[doc = "Bits 0:1 - Port 1 pin 0 control."]
    #[inline]
    pub fn p1_00mode(&self) -> P1_00MODER {
        P1_00MODER::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 2:3 - Port 1 pin 1 control."]
    #[inline]
    pub fn p1_01mode(&self) -> P1_01MODER {
        P1_01MODER::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 8:9 - Port 1 pin 4 control."]
    #[inline]
    pub fn p1_04mode(&self) -> P1_04MODER {
        P1_04MODER::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 16:17 - Port 1 pin 8 control."]
    #[inline]
    pub fn p1_08mode(&self) -> P1_08MODER {
        P1_08MODER::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 18:19 - Port 1 pin 9 control."]
    #[inline]
    pub fn p1_09mode(&self) -> P1_09MODER {
        P1_09MODER::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 20:21 - Port 1 pin 10 control."]
    #[inline]
    pub fn p1_10mode(&self) -> P1_10MODER {
        P1_10MODER::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 28:29 - Port 1 pin 14 control."]
    #[inline]
    pub fn p1_14mode(&self) -> P1_14MODER {
        P1_14MODER::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 30:31 - Port 1 pin 15 control."]
    #[inline]
    pub fn p1_15mode(&self) -> P1_15MODER {
        P1_15MODER::_from({
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
    #[doc = "Bits 0:1 - Port 1 pin 0 control."]
    #[inline]
    pub fn p1_00mode(&mut self) -> _P1_00MODEW {
        _P1_00MODEW { w: self }
    }
    #[doc = "Bits 2:3 - Port 1 pin 1 control."]
    #[inline]
    pub fn p1_01mode(&mut self) -> _P1_01MODEW {
        _P1_01MODEW { w: self }
    }
    #[doc = "Bits 8:9 - Port 1 pin 4 control."]
    #[inline]
    pub fn p1_04mode(&mut self) -> _P1_04MODEW {
        _P1_04MODEW { w: self }
    }
    #[doc = "Bits 16:17 - Port 1 pin 8 control."]
    #[inline]
    pub fn p1_08mode(&mut self) -> _P1_08MODEW {
        _P1_08MODEW { w: self }
    }
    #[doc = "Bits 18:19 - Port 1 pin 9 control."]
    #[inline]
    pub fn p1_09mode(&mut self) -> _P1_09MODEW {
        _P1_09MODEW { w: self }
    }
    #[doc = "Bits 20:21 - Port 1 pin 10 control."]
    #[inline]
    pub fn p1_10mode(&mut self) -> _P1_10MODEW {
        _P1_10MODEW { w: self }
    }
    #[doc = "Bits 28:29 - Port 1 pin 14 control."]
    #[inline]
    pub fn p1_14mode(&mut self) -> _P1_14MODEW {
        _P1_14MODEW { w: self }
    }
    #[doc = "Bits 30:31 - Port 1 pin 15 control."]
    #[inline]
    pub fn p1_15mode(&mut self) -> _P1_15MODEW {
        _P1_15MODEW { w: self }
    }
}
