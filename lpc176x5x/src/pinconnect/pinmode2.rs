#[doc = "Reader of register PINMODE2"]
pub type R = crate::R<u32, super::PINMODE2>;
#[doc = "Writer for register PINMODE2"]
pub type W = crate::W<u32, super::PINMODE2>;
#[doc = "Register PINMODE2 `reset()`'s with value 0"]
impl crate::ResetValue for super::PINMODE2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Port 1 pin 0 control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum P1_00MODE_A {
    #[doc = "0: Pull-up. P1.0 pin has a pull-up resistor enabled."]
    PULL_UP = 0,
    #[doc = "1: Repeater. P1.0 pin has repeater mode enabled."]
    REPEATER = 1,
    #[doc = "2: Disabled. P1.0 pin has neither pull-up nor pull-down."]
    DISABLED = 2,
    #[doc = "3: Pull-down. P1.0 has a pull-down resistor enabled."]
    PULL_DOWN = 3,
}
impl From<P1_00MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: P1_00MODE_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `P1_00MODE`"]
pub type P1_00MODE_R = crate::R<u8, P1_00MODE_A>;
impl P1_00MODE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> P1_00MODE_A {
        match self.bits {
            0 => P1_00MODE_A::PULL_UP,
            1 => P1_00MODE_A::REPEATER,
            2 => P1_00MODE_A::DISABLED,
            3 => P1_00MODE_A::PULL_DOWN,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PULL_UP`"]
    #[inline(always)]
    pub fn is_pull_up(&self) -> bool {
        *self == P1_00MODE_A::PULL_UP
    }
    #[doc = "Checks if the value of the field is `REPEATER`"]
    #[inline(always)]
    pub fn is_repeater(&self) -> bool {
        *self == P1_00MODE_A::REPEATER
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == P1_00MODE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `PULL_DOWN`"]
    #[inline(always)]
    pub fn is_pull_down(&self) -> bool {
        *self == P1_00MODE_A::PULL_DOWN
    }
}
#[doc = "Write proxy for field `P1_00MODE`"]
pub struct P1_00MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> P1_00MODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: P1_00MODE_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Pull-up. P1.0 pin has a pull-up resistor enabled."]
    #[inline(always)]
    pub fn pull_up(self) -> &'a mut W {
        self.variant(P1_00MODE_A::PULL_UP)
    }
    #[doc = "Repeater. P1.0 pin has repeater mode enabled."]
    #[inline(always)]
    pub fn repeater(self) -> &'a mut W {
        self.variant(P1_00MODE_A::REPEATER)
    }
    #[doc = "Disabled. P1.0 pin has neither pull-up nor pull-down."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(P1_00MODE_A::DISABLED)
    }
    #[doc = "Pull-down. P1.0 has a pull-down resistor enabled."]
    #[inline(always)]
    pub fn pull_down(self) -> &'a mut W {
        self.variant(P1_00MODE_A::PULL_DOWN)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
#[doc = "Port 1 pin 1 control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum P1_01MODE_A {
    #[doc = "0: Pull-up. P1.1 pin has a pull-up resistor enabled."]
    PULL_UP = 0,
    #[doc = "1: Repeater. P1.1 pin has repeater mode enabled."]
    REPEATER = 1,
    #[doc = "2: Disabled. P1.1 pin has neither pull-up nor pull-down."]
    DISABLED = 2,
    #[doc = "3: Pull-down. P1.1 has a pull-down resistor enabled."]
    PULL_DOWN = 3,
}
impl From<P1_01MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: P1_01MODE_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `P1_01MODE`"]
pub type P1_01MODE_R = crate::R<u8, P1_01MODE_A>;
impl P1_01MODE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> P1_01MODE_A {
        match self.bits {
            0 => P1_01MODE_A::PULL_UP,
            1 => P1_01MODE_A::REPEATER,
            2 => P1_01MODE_A::DISABLED,
            3 => P1_01MODE_A::PULL_DOWN,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PULL_UP`"]
    #[inline(always)]
    pub fn is_pull_up(&self) -> bool {
        *self == P1_01MODE_A::PULL_UP
    }
    #[doc = "Checks if the value of the field is `REPEATER`"]
    #[inline(always)]
    pub fn is_repeater(&self) -> bool {
        *self == P1_01MODE_A::REPEATER
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == P1_01MODE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `PULL_DOWN`"]
    #[inline(always)]
    pub fn is_pull_down(&self) -> bool {
        *self == P1_01MODE_A::PULL_DOWN
    }
}
#[doc = "Write proxy for field `P1_01MODE`"]
pub struct P1_01MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> P1_01MODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: P1_01MODE_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Pull-up. P1.1 pin has a pull-up resistor enabled."]
    #[inline(always)]
    pub fn pull_up(self) -> &'a mut W {
        self.variant(P1_01MODE_A::PULL_UP)
    }
    #[doc = "Repeater. P1.1 pin has repeater mode enabled."]
    #[inline(always)]
    pub fn repeater(self) -> &'a mut W {
        self.variant(P1_01MODE_A::REPEATER)
    }
    #[doc = "Disabled. P1.1 pin has neither pull-up nor pull-down."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(P1_01MODE_A::DISABLED)
    }
    #[doc = "Pull-down. P1.1 has a pull-down resistor enabled."]
    #[inline(always)]
    pub fn pull_down(self) -> &'a mut W {
        self.variant(P1_01MODE_A::PULL_DOWN)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | (((value as u32) & 0x03) << 2);
        self.w
    }
}
#[doc = "Port 1 pin 4 control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum P1_04MODE_A {
    #[doc = "0: Pull-up. P1.4 pin has a pull-up resistor enabled."]
    PULL_UP = 0,
    #[doc = "1: Repeater. P1.4 pin has repeater mode enabled."]
    REPEATER = 1,
    #[doc = "2: Disabled. P1.4 pin has neither pull-up nor pull-down."]
    DISABLED = 2,
    #[doc = "3: Pull-down. P1.4 has a pull-down resistor enabled."]
    PULL_DOWN = 3,
}
impl From<P1_04MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: P1_04MODE_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `P1_04MODE`"]
pub type P1_04MODE_R = crate::R<u8, P1_04MODE_A>;
impl P1_04MODE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> P1_04MODE_A {
        match self.bits {
            0 => P1_04MODE_A::PULL_UP,
            1 => P1_04MODE_A::REPEATER,
            2 => P1_04MODE_A::DISABLED,
            3 => P1_04MODE_A::PULL_DOWN,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PULL_UP`"]
    #[inline(always)]
    pub fn is_pull_up(&self) -> bool {
        *self == P1_04MODE_A::PULL_UP
    }
    #[doc = "Checks if the value of the field is `REPEATER`"]
    #[inline(always)]
    pub fn is_repeater(&self) -> bool {
        *self == P1_04MODE_A::REPEATER
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == P1_04MODE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `PULL_DOWN`"]
    #[inline(always)]
    pub fn is_pull_down(&self) -> bool {
        *self == P1_04MODE_A::PULL_DOWN
    }
}
#[doc = "Write proxy for field `P1_04MODE`"]
pub struct P1_04MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> P1_04MODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: P1_04MODE_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Pull-up. P1.4 pin has a pull-up resistor enabled."]
    #[inline(always)]
    pub fn pull_up(self) -> &'a mut W {
        self.variant(P1_04MODE_A::PULL_UP)
    }
    #[doc = "Repeater. P1.4 pin has repeater mode enabled."]
    #[inline(always)]
    pub fn repeater(self) -> &'a mut W {
        self.variant(P1_04MODE_A::REPEATER)
    }
    #[doc = "Disabled. P1.4 pin has neither pull-up nor pull-down."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(P1_04MODE_A::DISABLED)
    }
    #[doc = "Pull-down. P1.4 has a pull-down resistor enabled."]
    #[inline(always)]
    pub fn pull_down(self) -> &'a mut W {
        self.variant(P1_04MODE_A::PULL_DOWN)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | (((value as u32) & 0x03) << 8);
        self.w
    }
}
#[doc = "Port 1 pin 8 control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum P1_08MODE_A {
    #[doc = "0: Pull-up. P1.8 pin has a pull-up resistor enabled."]
    PULL_UP = 0,
    #[doc = "1: Repeater. P1.8 pin has repeater mode enabled."]
    REPEATER = 1,
    #[doc = "2: Disabled. P1.8 pin has neither pull-up nor pull-down."]
    DISABLED = 2,
    #[doc = "3: Pull-down. P1.8 has a pull-down resistor enabled."]
    PULL_DOWN = 3,
}
impl From<P1_08MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: P1_08MODE_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `P1_08MODE`"]
pub type P1_08MODE_R = crate::R<u8, P1_08MODE_A>;
impl P1_08MODE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> P1_08MODE_A {
        match self.bits {
            0 => P1_08MODE_A::PULL_UP,
            1 => P1_08MODE_A::REPEATER,
            2 => P1_08MODE_A::DISABLED,
            3 => P1_08MODE_A::PULL_DOWN,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PULL_UP`"]
    #[inline(always)]
    pub fn is_pull_up(&self) -> bool {
        *self == P1_08MODE_A::PULL_UP
    }
    #[doc = "Checks if the value of the field is `REPEATER`"]
    #[inline(always)]
    pub fn is_repeater(&self) -> bool {
        *self == P1_08MODE_A::REPEATER
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == P1_08MODE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `PULL_DOWN`"]
    #[inline(always)]
    pub fn is_pull_down(&self) -> bool {
        *self == P1_08MODE_A::PULL_DOWN
    }
}
#[doc = "Write proxy for field `P1_08MODE`"]
pub struct P1_08MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> P1_08MODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: P1_08MODE_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Pull-up. P1.8 pin has a pull-up resistor enabled."]
    #[inline(always)]
    pub fn pull_up(self) -> &'a mut W {
        self.variant(P1_08MODE_A::PULL_UP)
    }
    #[doc = "Repeater. P1.8 pin has repeater mode enabled."]
    #[inline(always)]
    pub fn repeater(self) -> &'a mut W {
        self.variant(P1_08MODE_A::REPEATER)
    }
    #[doc = "Disabled. P1.8 pin has neither pull-up nor pull-down."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(P1_08MODE_A::DISABLED)
    }
    #[doc = "Pull-down. P1.8 has a pull-down resistor enabled."]
    #[inline(always)]
    pub fn pull_down(self) -> &'a mut W {
        self.variant(P1_08MODE_A::PULL_DOWN)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | (((value as u32) & 0x03) << 16);
        self.w
    }
}
#[doc = "Port 1 pin 9 control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum P1_09MODE_A {
    #[doc = "0: Pull-up. P1.9 pin has a pull-up resistor enabled."]
    PULL_UP = 0,
    #[doc = "1: Repeater. P1.9 pin has repeater mode enabled."]
    REPEATER = 1,
    #[doc = "2: Disabled. P1.9 pin has neither pull-up nor pull-down."]
    DISABLED = 2,
    #[doc = "3: Pull-down. P1.9 has a pull-down resistor enabled."]
    PULL_DOWN = 3,
}
impl From<P1_09MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: P1_09MODE_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `P1_09MODE`"]
pub type P1_09MODE_R = crate::R<u8, P1_09MODE_A>;
impl P1_09MODE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> P1_09MODE_A {
        match self.bits {
            0 => P1_09MODE_A::PULL_UP,
            1 => P1_09MODE_A::REPEATER,
            2 => P1_09MODE_A::DISABLED,
            3 => P1_09MODE_A::PULL_DOWN,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PULL_UP`"]
    #[inline(always)]
    pub fn is_pull_up(&self) -> bool {
        *self == P1_09MODE_A::PULL_UP
    }
    #[doc = "Checks if the value of the field is `REPEATER`"]
    #[inline(always)]
    pub fn is_repeater(&self) -> bool {
        *self == P1_09MODE_A::REPEATER
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == P1_09MODE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `PULL_DOWN`"]
    #[inline(always)]
    pub fn is_pull_down(&self) -> bool {
        *self == P1_09MODE_A::PULL_DOWN
    }
}
#[doc = "Write proxy for field `P1_09MODE`"]
pub struct P1_09MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> P1_09MODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: P1_09MODE_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Pull-up. P1.9 pin has a pull-up resistor enabled."]
    #[inline(always)]
    pub fn pull_up(self) -> &'a mut W {
        self.variant(P1_09MODE_A::PULL_UP)
    }
    #[doc = "Repeater. P1.9 pin has repeater mode enabled."]
    #[inline(always)]
    pub fn repeater(self) -> &'a mut W {
        self.variant(P1_09MODE_A::REPEATER)
    }
    #[doc = "Disabled. P1.9 pin has neither pull-up nor pull-down."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(P1_09MODE_A::DISABLED)
    }
    #[doc = "Pull-down. P1.9 has a pull-down resistor enabled."]
    #[inline(always)]
    pub fn pull_down(self) -> &'a mut W {
        self.variant(P1_09MODE_A::PULL_DOWN)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 18)) | (((value as u32) & 0x03) << 18);
        self.w
    }
}
#[doc = "Port 1 pin 10 control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum P1_10MODE_A {
    #[doc = "0: Pull-up. P1.10 pin has a pull-up resistor enabled."]
    PULL_UP = 0,
    #[doc = "1: Repeater. P1.10 pin has repeater mode enabled."]
    REPEATER = 1,
    #[doc = "2: Disabled. P1.10 pin has neither pull-up nor pull-down."]
    DISABLED = 2,
    #[doc = "3: Pull-down. P1.10 has a pull-down resistor enabled."]
    PULL_DOWN = 3,
}
impl From<P1_10MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: P1_10MODE_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `P1_10MODE`"]
pub type P1_10MODE_R = crate::R<u8, P1_10MODE_A>;
impl P1_10MODE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> P1_10MODE_A {
        match self.bits {
            0 => P1_10MODE_A::PULL_UP,
            1 => P1_10MODE_A::REPEATER,
            2 => P1_10MODE_A::DISABLED,
            3 => P1_10MODE_A::PULL_DOWN,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PULL_UP`"]
    #[inline(always)]
    pub fn is_pull_up(&self) -> bool {
        *self == P1_10MODE_A::PULL_UP
    }
    #[doc = "Checks if the value of the field is `REPEATER`"]
    #[inline(always)]
    pub fn is_repeater(&self) -> bool {
        *self == P1_10MODE_A::REPEATER
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == P1_10MODE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `PULL_DOWN`"]
    #[inline(always)]
    pub fn is_pull_down(&self) -> bool {
        *self == P1_10MODE_A::PULL_DOWN
    }
}
#[doc = "Write proxy for field `P1_10MODE`"]
pub struct P1_10MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> P1_10MODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: P1_10MODE_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Pull-up. P1.10 pin has a pull-up resistor enabled."]
    #[inline(always)]
    pub fn pull_up(self) -> &'a mut W {
        self.variant(P1_10MODE_A::PULL_UP)
    }
    #[doc = "Repeater. P1.10 pin has repeater mode enabled."]
    #[inline(always)]
    pub fn repeater(self) -> &'a mut W {
        self.variant(P1_10MODE_A::REPEATER)
    }
    #[doc = "Disabled. P1.10 pin has neither pull-up nor pull-down."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(P1_10MODE_A::DISABLED)
    }
    #[doc = "Pull-down. P1.10 has a pull-down resistor enabled."]
    #[inline(always)]
    pub fn pull_down(self) -> &'a mut W {
        self.variant(P1_10MODE_A::PULL_DOWN)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 20)) | (((value as u32) & 0x03) << 20);
        self.w
    }
}
#[doc = "Port 1 pin 14 control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum P1_14MODE_A {
    #[doc = "0: Pull-up. P1.14 pin has a pull-up resistor enabled."]
    PULL_UP = 0,
    #[doc = "1: Repeater. P1.14 pin has repeater mode enabled."]
    REPEATER = 1,
    #[doc = "2: Disabled. P1.14 pin has neither pull-up nor pull-down."]
    DISABLED = 2,
    #[doc = "3: Pull-down. P1.14 has a pull-down resistor enabled."]
    PULL_DOWN = 3,
}
impl From<P1_14MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: P1_14MODE_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `P1_14MODE`"]
pub type P1_14MODE_R = crate::R<u8, P1_14MODE_A>;
impl P1_14MODE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> P1_14MODE_A {
        match self.bits {
            0 => P1_14MODE_A::PULL_UP,
            1 => P1_14MODE_A::REPEATER,
            2 => P1_14MODE_A::DISABLED,
            3 => P1_14MODE_A::PULL_DOWN,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PULL_UP`"]
    #[inline(always)]
    pub fn is_pull_up(&self) -> bool {
        *self == P1_14MODE_A::PULL_UP
    }
    #[doc = "Checks if the value of the field is `REPEATER`"]
    #[inline(always)]
    pub fn is_repeater(&self) -> bool {
        *self == P1_14MODE_A::REPEATER
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == P1_14MODE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `PULL_DOWN`"]
    #[inline(always)]
    pub fn is_pull_down(&self) -> bool {
        *self == P1_14MODE_A::PULL_DOWN
    }
}
#[doc = "Write proxy for field `P1_14MODE`"]
pub struct P1_14MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> P1_14MODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: P1_14MODE_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Pull-up. P1.14 pin has a pull-up resistor enabled."]
    #[inline(always)]
    pub fn pull_up(self) -> &'a mut W {
        self.variant(P1_14MODE_A::PULL_UP)
    }
    #[doc = "Repeater. P1.14 pin has repeater mode enabled."]
    #[inline(always)]
    pub fn repeater(self) -> &'a mut W {
        self.variant(P1_14MODE_A::REPEATER)
    }
    #[doc = "Disabled. P1.14 pin has neither pull-up nor pull-down."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(P1_14MODE_A::DISABLED)
    }
    #[doc = "Pull-down. P1.14 has a pull-down resistor enabled."]
    #[inline(always)]
    pub fn pull_down(self) -> &'a mut W {
        self.variant(P1_14MODE_A::PULL_DOWN)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 28)) | (((value as u32) & 0x03) << 28);
        self.w
    }
}
#[doc = "Port 1 pin 15 control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum P1_15MODE_A {
    #[doc = "0: Pull-up. P1.15 pin has a pull-up resistor enabled."]
    PULL_UP = 0,
    #[doc = "1: Repeater. P1.15 pin has repeater mode enabled."]
    REPEATER = 1,
    #[doc = "2: Disabled. P1.15 pin has neither pull-up nor pull-down."]
    DISABLED = 2,
    #[doc = "3: Pull-down. P1.15 has a pull-down resistor enabled."]
    PULL_DOWN = 3,
}
impl From<P1_15MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: P1_15MODE_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `P1_15MODE`"]
pub type P1_15MODE_R = crate::R<u8, P1_15MODE_A>;
impl P1_15MODE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> P1_15MODE_A {
        match self.bits {
            0 => P1_15MODE_A::PULL_UP,
            1 => P1_15MODE_A::REPEATER,
            2 => P1_15MODE_A::DISABLED,
            3 => P1_15MODE_A::PULL_DOWN,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PULL_UP`"]
    #[inline(always)]
    pub fn is_pull_up(&self) -> bool {
        *self == P1_15MODE_A::PULL_UP
    }
    #[doc = "Checks if the value of the field is `REPEATER`"]
    #[inline(always)]
    pub fn is_repeater(&self) -> bool {
        *self == P1_15MODE_A::REPEATER
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == P1_15MODE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `PULL_DOWN`"]
    #[inline(always)]
    pub fn is_pull_down(&self) -> bool {
        *self == P1_15MODE_A::PULL_DOWN
    }
}
#[doc = "Write proxy for field `P1_15MODE`"]
pub struct P1_15MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> P1_15MODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: P1_15MODE_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Pull-up. P1.15 pin has a pull-up resistor enabled."]
    #[inline(always)]
    pub fn pull_up(self) -> &'a mut W {
        self.variant(P1_15MODE_A::PULL_UP)
    }
    #[doc = "Repeater. P1.15 pin has repeater mode enabled."]
    #[inline(always)]
    pub fn repeater(self) -> &'a mut W {
        self.variant(P1_15MODE_A::REPEATER)
    }
    #[doc = "Disabled. P1.15 pin has neither pull-up nor pull-down."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(P1_15MODE_A::DISABLED)
    }
    #[doc = "Pull-down. P1.15 has a pull-down resistor enabled."]
    #[inline(always)]
    pub fn pull_down(self) -> &'a mut W {
        self.variant(P1_15MODE_A::PULL_DOWN)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 30)) | (((value as u32) & 0x03) << 30);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - Port 1 pin 0 control."]
    #[inline(always)]
    pub fn p1_00mode(&self) -> P1_00MODE_R {
        P1_00MODE_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 2:3 - Port 1 pin 1 control."]
    #[inline(always)]
    pub fn p1_01mode(&self) -> P1_01MODE_R {
        P1_01MODE_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bits 8:9 - Port 1 pin 4 control."]
    #[inline(always)]
    pub fn p1_04mode(&self) -> P1_04MODE_R {
        P1_04MODE_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bits 16:17 - Port 1 pin 8 control."]
    #[inline(always)]
    pub fn p1_08mode(&self) -> P1_08MODE_R {
        P1_08MODE_R::new(((self.bits >> 16) & 0x03) as u8)
    }
    #[doc = "Bits 18:19 - Port 1 pin 9 control."]
    #[inline(always)]
    pub fn p1_09mode(&self) -> P1_09MODE_R {
        P1_09MODE_R::new(((self.bits >> 18) & 0x03) as u8)
    }
    #[doc = "Bits 20:21 - Port 1 pin 10 control."]
    #[inline(always)]
    pub fn p1_10mode(&self) -> P1_10MODE_R {
        P1_10MODE_R::new(((self.bits >> 20) & 0x03) as u8)
    }
    #[doc = "Bits 28:29 - Port 1 pin 14 control."]
    #[inline(always)]
    pub fn p1_14mode(&self) -> P1_14MODE_R {
        P1_14MODE_R::new(((self.bits >> 28) & 0x03) as u8)
    }
    #[doc = "Bits 30:31 - Port 1 pin 15 control."]
    #[inline(always)]
    pub fn p1_15mode(&self) -> P1_15MODE_R {
        P1_15MODE_R::new(((self.bits >> 30) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Port 1 pin 0 control."]
    #[inline(always)]
    pub fn p1_00mode(&mut self) -> P1_00MODE_W {
        P1_00MODE_W { w: self }
    }
    #[doc = "Bits 2:3 - Port 1 pin 1 control."]
    #[inline(always)]
    pub fn p1_01mode(&mut self) -> P1_01MODE_W {
        P1_01MODE_W { w: self }
    }
    #[doc = "Bits 8:9 - Port 1 pin 4 control."]
    #[inline(always)]
    pub fn p1_04mode(&mut self) -> P1_04MODE_W {
        P1_04MODE_W { w: self }
    }
    #[doc = "Bits 16:17 - Port 1 pin 8 control."]
    #[inline(always)]
    pub fn p1_08mode(&mut self) -> P1_08MODE_W {
        P1_08MODE_W { w: self }
    }
    #[doc = "Bits 18:19 - Port 1 pin 9 control."]
    #[inline(always)]
    pub fn p1_09mode(&mut self) -> P1_09MODE_W {
        P1_09MODE_W { w: self }
    }
    #[doc = "Bits 20:21 - Port 1 pin 10 control."]
    #[inline(always)]
    pub fn p1_10mode(&mut self) -> P1_10MODE_W {
        P1_10MODE_W { w: self }
    }
    #[doc = "Bits 28:29 - Port 1 pin 14 control."]
    #[inline(always)]
    pub fn p1_14mode(&mut self) -> P1_14MODE_W {
        P1_14MODE_W { w: self }
    }
    #[doc = "Bits 30:31 - Port 1 pin 15 control."]
    #[inline(always)]
    pub fn p1_15mode(&mut self) -> P1_15MODE_W {
        P1_15MODE_W { w: self }
    }
}
