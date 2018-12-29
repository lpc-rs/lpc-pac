#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::PINMODE9 {
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
#[doc = "Possible values of the field `P4_28MODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P4_28MODER {
    #[doc = "Pull-up. P4.28 pin has a pull-up resistor enabled."]
    PULL_UP,
    #[doc = "Repeater. P4.28 pin has repeater mode enabled."]
    REPEATER,
    #[doc = "Disabled. P4.28 pin has neither pull-up nor pull-down."]
    DISABLED,
    #[doc = "Pull-down. P4.28 has a pull-down resistor enabled."]
    PULL_DOWN,
}
impl P4_28MODER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            P4_28MODER::PULL_UP => 0,
            P4_28MODER::REPEATER => 1,
            P4_28MODER::DISABLED => 2,
            P4_28MODER::PULL_DOWN => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> P4_28MODER {
        match value {
            0 => P4_28MODER::PULL_UP,
            1 => P4_28MODER::REPEATER,
            2 => P4_28MODER::DISABLED,
            3 => P4_28MODER::PULL_DOWN,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PULL_UP`"]
    #[inline]
    pub fn is_pull_up(&self) -> bool {
        *self == P4_28MODER::PULL_UP
    }
    #[doc = "Checks if the value of the field is `REPEATER`"]
    #[inline]
    pub fn is_repeater(&self) -> bool {
        *self == P4_28MODER::REPEATER
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == P4_28MODER::DISABLED
    }
    #[doc = "Checks if the value of the field is `PULL_DOWN`"]
    #[inline]
    pub fn is_pull_down(&self) -> bool {
        *self == P4_28MODER::PULL_DOWN
    }
}
#[doc = "Possible values of the field `P4_29MODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P4_29MODER {
    #[doc = "Pull-up. P4.29 pin has a pull-up resistor enabled."]
    PULL_UP,
    #[doc = "Repeater. P4.29 pin has repeater mode enabled."]
    REPEATER,
    #[doc = "Disabled. P4.29 pin has neither pull-up nor pull-down."]
    DISABLED,
    #[doc = "Pull-down. P4.29 has a pull-down resistor enabled."]
    PULL_DOWN,
}
impl P4_29MODER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            P4_29MODER::PULL_UP => 0,
            P4_29MODER::REPEATER => 1,
            P4_29MODER::DISABLED => 2,
            P4_29MODER::PULL_DOWN => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> P4_29MODER {
        match value {
            0 => P4_29MODER::PULL_UP,
            1 => P4_29MODER::REPEATER,
            2 => P4_29MODER::DISABLED,
            3 => P4_29MODER::PULL_DOWN,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PULL_UP`"]
    #[inline]
    pub fn is_pull_up(&self) -> bool {
        *self == P4_29MODER::PULL_UP
    }
    #[doc = "Checks if the value of the field is `REPEATER`"]
    #[inline]
    pub fn is_repeater(&self) -> bool {
        *self == P4_29MODER::REPEATER
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == P4_29MODER::DISABLED
    }
    #[doc = "Checks if the value of the field is `PULL_DOWN`"]
    #[inline]
    pub fn is_pull_down(&self) -> bool {
        *self == P4_29MODER::PULL_DOWN
    }
}
#[doc = "Values that can be written to the field `P4_28MODE`"]
pub enum P4_28MODEW {
    #[doc = "Pull-up. P4.28 pin has a pull-up resistor enabled."]
    PULL_UP,
    #[doc = "Repeater. P4.28 pin has repeater mode enabled."]
    REPEATER,
    #[doc = "Disabled. P4.28 pin has neither pull-up nor pull-down."]
    DISABLED,
    #[doc = "Pull-down. P4.28 has a pull-down resistor enabled."]
    PULL_DOWN,
}
impl P4_28MODEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            P4_28MODEW::PULL_UP => 0,
            P4_28MODEW::REPEATER => 1,
            P4_28MODEW::DISABLED => 2,
            P4_28MODEW::PULL_DOWN => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _P4_28MODEW<'a> {
    w: &'a mut W,
}
impl<'a> _P4_28MODEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: P4_28MODEW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Pull-up. P4.28 pin has a pull-up resistor enabled."]
    #[inline]
    pub fn pull_up(self) -> &'a mut W {
        self.variant(P4_28MODEW::PULL_UP)
    }
    #[doc = "Repeater. P4.28 pin has repeater mode enabled."]
    #[inline]
    pub fn repeater(self) -> &'a mut W {
        self.variant(P4_28MODEW::REPEATER)
    }
    #[doc = "Disabled. P4.28 pin has neither pull-up nor pull-down."]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(P4_28MODEW::DISABLED)
    }
    #[doc = "Pull-down. P4.28 has a pull-down resistor enabled."]
    #[inline]
    pub fn pull_down(self) -> &'a mut W {
        self.variant(P4_28MODEW::PULL_DOWN)
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
#[doc = "Values that can be written to the field `P4_29MODE`"]
pub enum P4_29MODEW {
    #[doc = "Pull-up. P4.29 pin has a pull-up resistor enabled."]
    PULL_UP,
    #[doc = "Repeater. P4.29 pin has repeater mode enabled."]
    REPEATER,
    #[doc = "Disabled. P4.29 pin has neither pull-up nor pull-down."]
    DISABLED,
    #[doc = "Pull-down. P4.29 has a pull-down resistor enabled."]
    PULL_DOWN,
}
impl P4_29MODEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            P4_29MODEW::PULL_UP => 0,
            P4_29MODEW::REPEATER => 1,
            P4_29MODEW::DISABLED => 2,
            P4_29MODEW::PULL_DOWN => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _P4_29MODEW<'a> {
    w: &'a mut W,
}
impl<'a> _P4_29MODEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: P4_29MODEW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Pull-up. P4.29 pin has a pull-up resistor enabled."]
    #[inline]
    pub fn pull_up(self) -> &'a mut W {
        self.variant(P4_29MODEW::PULL_UP)
    }
    #[doc = "Repeater. P4.29 pin has repeater mode enabled."]
    #[inline]
    pub fn repeater(self) -> &'a mut W {
        self.variant(P4_29MODEW::REPEATER)
    }
    #[doc = "Disabled. P4.29 pin has neither pull-up nor pull-down."]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(P4_29MODEW::DISABLED)
    }
    #[doc = "Pull-down. P4.29 has a pull-down resistor enabled."]
    #[inline]
    pub fn pull_down(self) -> &'a mut W {
        self.variant(P4_29MODEW::PULL_DOWN)
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
    #[doc = "Bits 24:25 - Port 4 pin 28 control."]
    #[inline]
    pub fn p4_28mode(&self) -> P4_28MODER {
        P4_28MODER::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 26:27 - Port 4 pin 29 control."]
    #[inline]
    pub fn p4_29mode(&self) -> P4_29MODER {
        P4_29MODER::_from({
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
    #[doc = "Bits 24:25 - Port 4 pin 28 control."]
    #[inline]
    pub fn p4_28mode(&mut self) -> _P4_28MODEW {
        _P4_28MODEW { w: self }
    }
    #[doc = "Bits 26:27 - Port 4 pin 29 control."]
    #[inline]
    pub fn p4_29mode(&mut self) -> _P4_29MODEW {
        _P4_29MODEW { w: self }
    }
}
