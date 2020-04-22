#[doc = "Reader of register PINMODE7"]
pub type R = crate::R<u32, super::PINMODE7>;
#[doc = "Writer for register PINMODE7"]
pub type W = crate::W<u32, super::PINMODE7>;
#[doc = "Register PINMODE7 `reset()`'s with value 0"]
impl crate::ResetValue for super::PINMODE7 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Port 3 pin 25 control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum P3_25MODE_A {
    #[doc = "0: Pull-up. P3.25 pin has a pull-up resistor enabled."]
    PULL_UP = 0,
    #[doc = "1: Repeater. P3.25 pin has repeater mode enabled."]
    REPEATER = 1,
    #[doc = "2: Disabled. P3.25 pin has neither pull-up nor pull-down."]
    DISABLED = 2,
    #[doc = "3: Pull-down. P3.25 has a pull-down resistor enabled."]
    PULL_DOWN = 3,
}
impl From<P3_25MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: P3_25MODE_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `P3_25MODE`"]
pub type P3_25MODE_R = crate::R<u8, P3_25MODE_A>;
impl P3_25MODE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> P3_25MODE_A {
        match self.bits {
            0 => P3_25MODE_A::PULL_UP,
            1 => P3_25MODE_A::REPEATER,
            2 => P3_25MODE_A::DISABLED,
            3 => P3_25MODE_A::PULL_DOWN,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PULL_UP`"]
    #[inline(always)]
    pub fn is_pull_up(&self) -> bool {
        *self == P3_25MODE_A::PULL_UP
    }
    #[doc = "Checks if the value of the field is `REPEATER`"]
    #[inline(always)]
    pub fn is_repeater(&self) -> bool {
        *self == P3_25MODE_A::REPEATER
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == P3_25MODE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `PULL_DOWN`"]
    #[inline(always)]
    pub fn is_pull_down(&self) -> bool {
        *self == P3_25MODE_A::PULL_DOWN
    }
}
#[doc = "Write proxy for field `P3_25MODE`"]
pub struct P3_25MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> P3_25MODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: P3_25MODE_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Pull-up. P3.25 pin has a pull-up resistor enabled."]
    #[inline(always)]
    pub fn pull_up(self) -> &'a mut W {
        self.variant(P3_25MODE_A::PULL_UP)
    }
    #[doc = "Repeater. P3.25 pin has repeater mode enabled."]
    #[inline(always)]
    pub fn repeater(self) -> &'a mut W {
        self.variant(P3_25MODE_A::REPEATER)
    }
    #[doc = "Disabled. P3.25 pin has neither pull-up nor pull-down."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(P3_25MODE_A::DISABLED)
    }
    #[doc = "Pull-down. P3.25 has a pull-down resistor enabled."]
    #[inline(always)]
    pub fn pull_down(self) -> &'a mut W {
        self.variant(P3_25MODE_A::PULL_DOWN)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 18)) | (((value as u32) & 0x03) << 18);
        self.w
    }
}
#[doc = "Port 3 pin 26 control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum P3_26MODE_A {
    #[doc = "0: Pull-up. P3.26 pin has a pull-up resistor enabled."]
    PULL_UP = 0,
    #[doc = "1: Repeater. P3.26 pin has repeater mode enabled."]
    REPEATER = 1,
    #[doc = "2: Disabled. P3.26 pin has neither pull-up nor pull-down."]
    DISABLED = 2,
    #[doc = "3: Pull-down. P3.26 has a pull-down resistor enabled."]
    PULL_DOWN = 3,
}
impl From<P3_26MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: P3_26MODE_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `P3_26MODE`"]
pub type P3_26MODE_R = crate::R<u8, P3_26MODE_A>;
impl P3_26MODE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> P3_26MODE_A {
        match self.bits {
            0 => P3_26MODE_A::PULL_UP,
            1 => P3_26MODE_A::REPEATER,
            2 => P3_26MODE_A::DISABLED,
            3 => P3_26MODE_A::PULL_DOWN,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PULL_UP`"]
    #[inline(always)]
    pub fn is_pull_up(&self) -> bool {
        *self == P3_26MODE_A::PULL_UP
    }
    #[doc = "Checks if the value of the field is `REPEATER`"]
    #[inline(always)]
    pub fn is_repeater(&self) -> bool {
        *self == P3_26MODE_A::REPEATER
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == P3_26MODE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `PULL_DOWN`"]
    #[inline(always)]
    pub fn is_pull_down(&self) -> bool {
        *self == P3_26MODE_A::PULL_DOWN
    }
}
#[doc = "Write proxy for field `P3_26MODE`"]
pub struct P3_26MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> P3_26MODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: P3_26MODE_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Pull-up. P3.26 pin has a pull-up resistor enabled."]
    #[inline(always)]
    pub fn pull_up(self) -> &'a mut W {
        self.variant(P3_26MODE_A::PULL_UP)
    }
    #[doc = "Repeater. P3.26 pin has repeater mode enabled."]
    #[inline(always)]
    pub fn repeater(self) -> &'a mut W {
        self.variant(P3_26MODE_A::REPEATER)
    }
    #[doc = "Disabled. P3.26 pin has neither pull-up nor pull-down."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(P3_26MODE_A::DISABLED)
    }
    #[doc = "Pull-down. P3.26 has a pull-down resistor enabled."]
    #[inline(always)]
    pub fn pull_down(self) -> &'a mut W {
        self.variant(P3_26MODE_A::PULL_DOWN)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 20)) | (((value as u32) & 0x03) << 20);
        self.w
    }
}
impl R {
    #[doc = "Bits 18:19 - Port 3 pin 25 control."]
    #[inline(always)]
    pub fn p3_25mode(&self) -> P3_25MODE_R {
        P3_25MODE_R::new(((self.bits >> 18) & 0x03) as u8)
    }
    #[doc = "Bits 20:21 - Port 3 pin 26 control."]
    #[inline(always)]
    pub fn p3_26mode(&self) -> P3_26MODE_R {
        P3_26MODE_R::new(((self.bits >> 20) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 18:19 - Port 3 pin 25 control."]
    #[inline(always)]
    pub fn p3_25mode(&mut self) -> P3_25MODE_W {
        P3_25MODE_W { w: self }
    }
    #[doc = "Bits 20:21 - Port 3 pin 26 control."]
    #[inline(always)]
    pub fn p3_26mode(&mut self) -> P3_26MODE_W {
        P3_26MODE_W { w: self }
    }
}
