#[doc = "Reader of register PINMODE9"]
pub type R = crate::R<u32, super::PINMODE9>;
#[doc = "Writer for register PINMODE9"]
pub type W = crate::W<u32, super::PINMODE9>;
#[doc = "Register PINMODE9 `reset()`'s with value 0"]
impl crate::ResetValue for super::PINMODE9 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Port 4 pin 28 control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum P4_28MODE_A {
    #[doc = "0: Pull-up. P4.28 pin has a pull-up resistor enabled."]
    PULL_UP = 0,
    #[doc = "1: Repeater. P4.28 pin has repeater mode enabled."]
    REPEATER = 1,
    #[doc = "2: Disabled. P4.28 pin has neither pull-up nor pull-down."]
    DISABLED = 2,
    #[doc = "3: Pull-down. P4.28 has a pull-down resistor enabled."]
    PULL_DOWN = 3,
}
impl From<P4_28MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: P4_28MODE_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `P4_28MODE`"]
pub type P4_28MODE_R = crate::R<u8, P4_28MODE_A>;
impl P4_28MODE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> P4_28MODE_A {
        match self.bits {
            0 => P4_28MODE_A::PULL_UP,
            1 => P4_28MODE_A::REPEATER,
            2 => P4_28MODE_A::DISABLED,
            3 => P4_28MODE_A::PULL_DOWN,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PULL_UP`"]
    #[inline(always)]
    pub fn is_pull_up(&self) -> bool {
        *self == P4_28MODE_A::PULL_UP
    }
    #[doc = "Checks if the value of the field is `REPEATER`"]
    #[inline(always)]
    pub fn is_repeater(&self) -> bool {
        *self == P4_28MODE_A::REPEATER
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == P4_28MODE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `PULL_DOWN`"]
    #[inline(always)]
    pub fn is_pull_down(&self) -> bool {
        *self == P4_28MODE_A::PULL_DOWN
    }
}
#[doc = "Write proxy for field `P4_28MODE`"]
pub struct P4_28MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> P4_28MODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: P4_28MODE_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Pull-up. P4.28 pin has a pull-up resistor enabled."]
    #[inline(always)]
    pub fn pull_up(self) -> &'a mut W {
        self.variant(P4_28MODE_A::PULL_UP)
    }
    #[doc = "Repeater. P4.28 pin has repeater mode enabled."]
    #[inline(always)]
    pub fn repeater(self) -> &'a mut W {
        self.variant(P4_28MODE_A::REPEATER)
    }
    #[doc = "Disabled. P4.28 pin has neither pull-up nor pull-down."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(P4_28MODE_A::DISABLED)
    }
    #[doc = "Pull-down. P4.28 has a pull-down resistor enabled."]
    #[inline(always)]
    pub fn pull_down(self) -> &'a mut W {
        self.variant(P4_28MODE_A::PULL_DOWN)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 24)) | (((value as u32) & 0x03) << 24);
        self.w
    }
}
#[doc = "Port 4 pin 29 control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum P4_29MODE_A {
    #[doc = "0: Pull-up. P4.29 pin has a pull-up resistor enabled."]
    PULL_UP = 0,
    #[doc = "1: Repeater. P4.29 pin has repeater mode enabled."]
    REPEATER = 1,
    #[doc = "2: Disabled. P4.29 pin has neither pull-up nor pull-down."]
    DISABLED = 2,
    #[doc = "3: Pull-down. P4.29 has a pull-down resistor enabled."]
    PULL_DOWN = 3,
}
impl From<P4_29MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: P4_29MODE_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `P4_29MODE`"]
pub type P4_29MODE_R = crate::R<u8, P4_29MODE_A>;
impl P4_29MODE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> P4_29MODE_A {
        match self.bits {
            0 => P4_29MODE_A::PULL_UP,
            1 => P4_29MODE_A::REPEATER,
            2 => P4_29MODE_A::DISABLED,
            3 => P4_29MODE_A::PULL_DOWN,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PULL_UP`"]
    #[inline(always)]
    pub fn is_pull_up(&self) -> bool {
        *self == P4_29MODE_A::PULL_UP
    }
    #[doc = "Checks if the value of the field is `REPEATER`"]
    #[inline(always)]
    pub fn is_repeater(&self) -> bool {
        *self == P4_29MODE_A::REPEATER
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == P4_29MODE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `PULL_DOWN`"]
    #[inline(always)]
    pub fn is_pull_down(&self) -> bool {
        *self == P4_29MODE_A::PULL_DOWN
    }
}
#[doc = "Write proxy for field `P4_29MODE`"]
pub struct P4_29MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> P4_29MODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: P4_29MODE_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Pull-up. P4.29 pin has a pull-up resistor enabled."]
    #[inline(always)]
    pub fn pull_up(self) -> &'a mut W {
        self.variant(P4_29MODE_A::PULL_UP)
    }
    #[doc = "Repeater. P4.29 pin has repeater mode enabled."]
    #[inline(always)]
    pub fn repeater(self) -> &'a mut W {
        self.variant(P4_29MODE_A::REPEATER)
    }
    #[doc = "Disabled. P4.29 pin has neither pull-up nor pull-down."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(P4_29MODE_A::DISABLED)
    }
    #[doc = "Pull-down. P4.29 has a pull-down resistor enabled."]
    #[inline(always)]
    pub fn pull_down(self) -> &'a mut W {
        self.variant(P4_29MODE_A::PULL_DOWN)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 26)) | (((value as u32) & 0x03) << 26);
        self.w
    }
}
impl R {
    #[doc = "Bits 24:25 - Port 4 pin 28 control."]
    #[inline(always)]
    pub fn p4_28mode(&self) -> P4_28MODE_R {
        P4_28MODE_R::new(((self.bits >> 24) & 0x03) as u8)
    }
    #[doc = "Bits 26:27 - Port 4 pin 29 control."]
    #[inline(always)]
    pub fn p4_29mode(&self) -> P4_29MODE_R {
        P4_29MODE_R::new(((self.bits >> 26) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 24:25 - Port 4 pin 28 control."]
    #[inline(always)]
    pub fn p4_28mode(&mut self) -> P4_28MODE_W {
        P4_28MODE_W { w: self }
    }
    #[doc = "Bits 26:27 - Port 4 pin 29 control."]
    #[inline(always)]
    pub fn p4_29mode(&mut self) -> P4_29MODE_W {
        P4_29MODE_W { w: self }
    }
}
