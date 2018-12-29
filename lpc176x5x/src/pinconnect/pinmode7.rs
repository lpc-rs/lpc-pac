#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::PINMODE7 {
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
#[doc = "Possible values of the field `P3_25MODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P3_25MODER {
    #[doc = "Pull-up. P3.25 pin has a pull-up resistor enabled."]
    PULL_UP,
    #[doc = "Repeater. P3.25 pin has repeater mode enabled."]
    REPEATER,
    #[doc = "Disabled. P3.25 pin has neither pull-up nor pull-down."]
    DISABLED,
    #[doc = "Pull-down. P3.25 has a pull-down resistor enabled."]
    PULL_DOWN,
}
impl P3_25MODER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            P3_25MODER::PULL_UP => 0,
            P3_25MODER::REPEATER => 1,
            P3_25MODER::DISABLED => 2,
            P3_25MODER::PULL_DOWN => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> P3_25MODER {
        match value {
            0 => P3_25MODER::PULL_UP,
            1 => P3_25MODER::REPEATER,
            2 => P3_25MODER::DISABLED,
            3 => P3_25MODER::PULL_DOWN,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PULL_UP`"]
    #[inline]
    pub fn is_pull_up(&self) -> bool {
        *self == P3_25MODER::PULL_UP
    }
    #[doc = "Checks if the value of the field is `REPEATER`"]
    #[inline]
    pub fn is_repeater(&self) -> bool {
        *self == P3_25MODER::REPEATER
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == P3_25MODER::DISABLED
    }
    #[doc = "Checks if the value of the field is `PULL_DOWN`"]
    #[inline]
    pub fn is_pull_down(&self) -> bool {
        *self == P3_25MODER::PULL_DOWN
    }
}
#[doc = "Possible values of the field `P3_26MODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P3_26MODER {
    #[doc = "Pull-up. P3.26 pin has a pull-up resistor enabled."]
    PULL_UP,
    #[doc = "Repeater. P3.26 pin has repeater mode enabled."]
    REPEATER,
    #[doc = "Disabled. P3.26 pin has neither pull-up nor pull-down."]
    DISABLED,
    #[doc = "Pull-down. P3.26 has a pull-down resistor enabled."]
    PULL_DOWN,
}
impl P3_26MODER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            P3_26MODER::PULL_UP => 0,
            P3_26MODER::REPEATER => 1,
            P3_26MODER::DISABLED => 2,
            P3_26MODER::PULL_DOWN => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> P3_26MODER {
        match value {
            0 => P3_26MODER::PULL_UP,
            1 => P3_26MODER::REPEATER,
            2 => P3_26MODER::DISABLED,
            3 => P3_26MODER::PULL_DOWN,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PULL_UP`"]
    #[inline]
    pub fn is_pull_up(&self) -> bool {
        *self == P3_26MODER::PULL_UP
    }
    #[doc = "Checks if the value of the field is `REPEATER`"]
    #[inline]
    pub fn is_repeater(&self) -> bool {
        *self == P3_26MODER::REPEATER
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == P3_26MODER::DISABLED
    }
    #[doc = "Checks if the value of the field is `PULL_DOWN`"]
    #[inline]
    pub fn is_pull_down(&self) -> bool {
        *self == P3_26MODER::PULL_DOWN
    }
}
#[doc = "Values that can be written to the field `P3_25MODE`"]
pub enum P3_25MODEW {
    #[doc = "Pull-up. P3.25 pin has a pull-up resistor enabled."]
    PULL_UP,
    #[doc = "Repeater. P3.25 pin has repeater mode enabled."]
    REPEATER,
    #[doc = "Disabled. P3.25 pin has neither pull-up nor pull-down."]
    DISABLED,
    #[doc = "Pull-down. P3.25 has a pull-down resistor enabled."]
    PULL_DOWN,
}
impl P3_25MODEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            P3_25MODEW::PULL_UP => 0,
            P3_25MODEW::REPEATER => 1,
            P3_25MODEW::DISABLED => 2,
            P3_25MODEW::PULL_DOWN => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _P3_25MODEW<'a> {
    w: &'a mut W,
}
impl<'a> _P3_25MODEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: P3_25MODEW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Pull-up. P3.25 pin has a pull-up resistor enabled."]
    #[inline]
    pub fn pull_up(self) -> &'a mut W {
        self.variant(P3_25MODEW::PULL_UP)
    }
    #[doc = "Repeater. P3.25 pin has repeater mode enabled."]
    #[inline]
    pub fn repeater(self) -> &'a mut W {
        self.variant(P3_25MODEW::REPEATER)
    }
    #[doc = "Disabled. P3.25 pin has neither pull-up nor pull-down."]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(P3_25MODEW::DISABLED)
    }
    #[doc = "Pull-down. P3.25 has a pull-down resistor enabled."]
    #[inline]
    pub fn pull_down(self) -> &'a mut W {
        self.variant(P3_25MODEW::PULL_DOWN)
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
#[doc = "Values that can be written to the field `P3_26MODE`"]
pub enum P3_26MODEW {
    #[doc = "Pull-up. P3.26 pin has a pull-up resistor enabled."]
    PULL_UP,
    #[doc = "Repeater. P3.26 pin has repeater mode enabled."]
    REPEATER,
    #[doc = "Disabled. P3.26 pin has neither pull-up nor pull-down."]
    DISABLED,
    #[doc = "Pull-down. P3.26 has a pull-down resistor enabled."]
    PULL_DOWN,
}
impl P3_26MODEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            P3_26MODEW::PULL_UP => 0,
            P3_26MODEW::REPEATER => 1,
            P3_26MODEW::DISABLED => 2,
            P3_26MODEW::PULL_DOWN => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _P3_26MODEW<'a> {
    w: &'a mut W,
}
impl<'a> _P3_26MODEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: P3_26MODEW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Pull-up. P3.26 pin has a pull-up resistor enabled."]
    #[inline]
    pub fn pull_up(self) -> &'a mut W {
        self.variant(P3_26MODEW::PULL_UP)
    }
    #[doc = "Repeater. P3.26 pin has repeater mode enabled."]
    #[inline]
    pub fn repeater(self) -> &'a mut W {
        self.variant(P3_26MODEW::REPEATER)
    }
    #[doc = "Disabled. P3.26 pin has neither pull-up nor pull-down."]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(P3_26MODEW::DISABLED)
    }
    #[doc = "Pull-down. P3.26 has a pull-down resistor enabled."]
    #[inline]
    pub fn pull_down(self) -> &'a mut W {
        self.variant(P3_26MODEW::PULL_DOWN)
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
    #[doc = "Bits 18:19 - Port 3 pin 25 control."]
    #[inline]
    pub fn p3_25mode(&self) -> P3_25MODER {
        P3_25MODER::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 20:21 - Port 3 pin 26 control."]
    #[inline]
    pub fn p3_26mode(&self) -> P3_26MODER {
        P3_26MODER::_from({
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
    #[doc = "Bits 18:19 - Port 3 pin 25 control."]
    #[inline]
    pub fn p3_25mode(&mut self) -> _P3_25MODEW {
        _P3_25MODEW { w: self }
    }
    #[doc = "Bits 20:21 - Port 3 pin 26 control."]
    #[inline]
    pub fn p3_26mode(&mut self) -> _P3_26MODEW {
        _P3_26MODEW { w: self }
    }
}
