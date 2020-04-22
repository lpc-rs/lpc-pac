#[doc = "Reader of register PINMODE0"]
pub type R = crate::R<u32, super::PINMODE0>;
#[doc = "Writer for register PINMODE0"]
pub type W = crate::W<u32, super::PINMODE0>;
#[doc = "Register PINMODE0 `reset()`'s with value 0"]
impl crate::ResetValue for super::PINMODE0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Port 0 pin 0 on-chip pull-up/down resistor control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum P0_00MODE_A {
    #[doc = "0: Pull-up. P0.0 pin has a pull-up resistor enabled."]
    PULL_UP = 0,
    #[doc = "1: Repeater. P0.0 pin has repeater mode enabled."]
    REPEATER = 1,
    #[doc = "2: Disabled. P0.0 pin has neither pull-up nor pull-down."]
    DISABLED = 2,
    #[doc = "3: Pull-down. P0.0 has a pull-down resistor enabled."]
    PULL_DOWN = 3,
}
impl From<P0_00MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: P0_00MODE_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `P0_00MODE`"]
pub type P0_00MODE_R = crate::R<u8, P0_00MODE_A>;
impl P0_00MODE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> P0_00MODE_A {
        match self.bits {
            0 => P0_00MODE_A::PULL_UP,
            1 => P0_00MODE_A::REPEATER,
            2 => P0_00MODE_A::DISABLED,
            3 => P0_00MODE_A::PULL_DOWN,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PULL_UP`"]
    #[inline(always)]
    pub fn is_pull_up(&self) -> bool {
        *self == P0_00MODE_A::PULL_UP
    }
    #[doc = "Checks if the value of the field is `REPEATER`"]
    #[inline(always)]
    pub fn is_repeater(&self) -> bool {
        *self == P0_00MODE_A::REPEATER
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == P0_00MODE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `PULL_DOWN`"]
    #[inline(always)]
    pub fn is_pull_down(&self) -> bool {
        *self == P0_00MODE_A::PULL_DOWN
    }
}
#[doc = "Write proxy for field `P0_00MODE`"]
pub struct P0_00MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> P0_00MODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: P0_00MODE_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Pull-up. P0.0 pin has a pull-up resistor enabled."]
    #[inline(always)]
    pub fn pull_up(self) -> &'a mut W {
        self.variant(P0_00MODE_A::PULL_UP)
    }
    #[doc = "Repeater. P0.0 pin has repeater mode enabled."]
    #[inline(always)]
    pub fn repeater(self) -> &'a mut W {
        self.variant(P0_00MODE_A::REPEATER)
    }
    #[doc = "Disabled. P0.0 pin has neither pull-up nor pull-down."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(P0_00MODE_A::DISABLED)
    }
    #[doc = "Pull-down. P0.0 has a pull-down resistor enabled."]
    #[inline(always)]
    pub fn pull_down(self) -> &'a mut W {
        self.variant(P0_00MODE_A::PULL_DOWN)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
#[doc = "Port 0 pin 1 control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum P0_01MODE_A {
    #[doc = "0: Pull-up. P0.1 pin has a pull-up resistor enabled."]
    PULL_UP = 0,
    #[doc = "1: Repeater. P0.1 pin has repeater mode enabled."]
    REPEATER = 1,
    #[doc = "2: Disabled. P0.1 pin has neither pull-up nor pull-down."]
    DISABLED = 2,
    #[doc = "3: Pull-down. P0.1 has a pull-down resistor enabled."]
    PULL_DOWN = 3,
}
impl From<P0_01MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: P0_01MODE_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `P0_01MODE`"]
pub type P0_01MODE_R = crate::R<u8, P0_01MODE_A>;
impl P0_01MODE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> P0_01MODE_A {
        match self.bits {
            0 => P0_01MODE_A::PULL_UP,
            1 => P0_01MODE_A::REPEATER,
            2 => P0_01MODE_A::DISABLED,
            3 => P0_01MODE_A::PULL_DOWN,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PULL_UP`"]
    #[inline(always)]
    pub fn is_pull_up(&self) -> bool {
        *self == P0_01MODE_A::PULL_UP
    }
    #[doc = "Checks if the value of the field is `REPEATER`"]
    #[inline(always)]
    pub fn is_repeater(&self) -> bool {
        *self == P0_01MODE_A::REPEATER
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == P0_01MODE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `PULL_DOWN`"]
    #[inline(always)]
    pub fn is_pull_down(&self) -> bool {
        *self == P0_01MODE_A::PULL_DOWN
    }
}
#[doc = "Write proxy for field `P0_01MODE`"]
pub struct P0_01MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> P0_01MODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: P0_01MODE_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Pull-up. P0.1 pin has a pull-up resistor enabled."]
    #[inline(always)]
    pub fn pull_up(self) -> &'a mut W {
        self.variant(P0_01MODE_A::PULL_UP)
    }
    #[doc = "Repeater. P0.1 pin has repeater mode enabled."]
    #[inline(always)]
    pub fn repeater(self) -> &'a mut W {
        self.variant(P0_01MODE_A::REPEATER)
    }
    #[doc = "Disabled. P0.1 pin has neither pull-up nor pull-down."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(P0_01MODE_A::DISABLED)
    }
    #[doc = "Pull-down. P0.1 has a pull-down resistor enabled."]
    #[inline(always)]
    pub fn pull_down(self) -> &'a mut W {
        self.variant(P0_01MODE_A::PULL_DOWN)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | (((value as u32) & 0x03) << 2);
        self.w
    }
}
#[doc = "Port 0 pin 2 control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum P0_02MODE_A {
    #[doc = "0: Pull-up. P0.2 pin has a pull-up resistor enabled."]
    PULL_UP = 0,
    #[doc = "1: Repeater. P0.2 pin has repeater mode enabled."]
    REPEATER = 1,
    #[doc = "2: Disabled. P0.2 pin has neither pull-up nor pull-down."]
    DISABLED = 2,
    #[doc = "3: Pull-down. P0.2 has a pull-down resistor enabled."]
    PULL_DOWN = 3,
}
impl From<P0_02MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: P0_02MODE_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `P0_02MODE`"]
pub type P0_02MODE_R = crate::R<u8, P0_02MODE_A>;
impl P0_02MODE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> P0_02MODE_A {
        match self.bits {
            0 => P0_02MODE_A::PULL_UP,
            1 => P0_02MODE_A::REPEATER,
            2 => P0_02MODE_A::DISABLED,
            3 => P0_02MODE_A::PULL_DOWN,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PULL_UP`"]
    #[inline(always)]
    pub fn is_pull_up(&self) -> bool {
        *self == P0_02MODE_A::PULL_UP
    }
    #[doc = "Checks if the value of the field is `REPEATER`"]
    #[inline(always)]
    pub fn is_repeater(&self) -> bool {
        *self == P0_02MODE_A::REPEATER
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == P0_02MODE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `PULL_DOWN`"]
    #[inline(always)]
    pub fn is_pull_down(&self) -> bool {
        *self == P0_02MODE_A::PULL_DOWN
    }
}
#[doc = "Write proxy for field `P0_02MODE`"]
pub struct P0_02MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> P0_02MODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: P0_02MODE_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Pull-up. P0.2 pin has a pull-up resistor enabled."]
    #[inline(always)]
    pub fn pull_up(self) -> &'a mut W {
        self.variant(P0_02MODE_A::PULL_UP)
    }
    #[doc = "Repeater. P0.2 pin has repeater mode enabled."]
    #[inline(always)]
    pub fn repeater(self) -> &'a mut W {
        self.variant(P0_02MODE_A::REPEATER)
    }
    #[doc = "Disabled. P0.2 pin has neither pull-up nor pull-down."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(P0_02MODE_A::DISABLED)
    }
    #[doc = "Pull-down. P0.2 has a pull-down resistor enabled."]
    #[inline(always)]
    pub fn pull_down(self) -> &'a mut W {
        self.variant(P0_02MODE_A::PULL_DOWN)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | (((value as u32) & 0x03) << 4);
        self.w
    }
}
#[doc = "Port 0 pin 3 control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum P0_03MODE_A {
    #[doc = "0: Pull-up. P0.3 pin has a pull-up resistor enabled."]
    PULL_UP = 0,
    #[doc = "1: Repeater. P0.3 pin has repeater mode enabled."]
    REPEATER = 1,
    #[doc = "2: Disabled. P0.3 pin has neither pull-up nor pull-down."]
    DISABLED = 2,
    #[doc = "3: Pull-down. P0.3 has a pull-down resistor enabled."]
    PULL_DOWN = 3,
}
impl From<P0_03MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: P0_03MODE_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `P0_03MODE`"]
pub type P0_03MODE_R = crate::R<u8, P0_03MODE_A>;
impl P0_03MODE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> P0_03MODE_A {
        match self.bits {
            0 => P0_03MODE_A::PULL_UP,
            1 => P0_03MODE_A::REPEATER,
            2 => P0_03MODE_A::DISABLED,
            3 => P0_03MODE_A::PULL_DOWN,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PULL_UP`"]
    #[inline(always)]
    pub fn is_pull_up(&self) -> bool {
        *self == P0_03MODE_A::PULL_UP
    }
    #[doc = "Checks if the value of the field is `REPEATER`"]
    #[inline(always)]
    pub fn is_repeater(&self) -> bool {
        *self == P0_03MODE_A::REPEATER
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == P0_03MODE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `PULL_DOWN`"]
    #[inline(always)]
    pub fn is_pull_down(&self) -> bool {
        *self == P0_03MODE_A::PULL_DOWN
    }
}
#[doc = "Write proxy for field `P0_03MODE`"]
pub struct P0_03MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> P0_03MODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: P0_03MODE_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Pull-up. P0.3 pin has a pull-up resistor enabled."]
    #[inline(always)]
    pub fn pull_up(self) -> &'a mut W {
        self.variant(P0_03MODE_A::PULL_UP)
    }
    #[doc = "Repeater. P0.3 pin has repeater mode enabled."]
    #[inline(always)]
    pub fn repeater(self) -> &'a mut W {
        self.variant(P0_03MODE_A::REPEATER)
    }
    #[doc = "Disabled. P0.3 pin has neither pull-up nor pull-down."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(P0_03MODE_A::DISABLED)
    }
    #[doc = "Pull-down. P0.3 has a pull-down resistor enabled."]
    #[inline(always)]
    pub fn pull_down(self) -> &'a mut W {
        self.variant(P0_03MODE_A::PULL_DOWN)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | (((value as u32) & 0x03) << 6);
        self.w
    }
}
#[doc = "Port 0 pin 4 control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum P0_04MODE_A {
    #[doc = "0: Pull-up. P0.4 pin has a pull-up resistor enabled."]
    PULL_UP = 0,
    #[doc = "1: Repeater. P0.4 pin has repeater mode enabled."]
    REPEATER = 1,
    #[doc = "2: Disabled. P0.4 pin has neither pull-up nor pull-down."]
    DISABLED = 2,
    #[doc = "3: Pull-down. P0.4 has a pull-down resistor enabled."]
    PULL_DOWN = 3,
}
impl From<P0_04MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: P0_04MODE_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `P0_04MODE`"]
pub type P0_04MODE_R = crate::R<u8, P0_04MODE_A>;
impl P0_04MODE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> P0_04MODE_A {
        match self.bits {
            0 => P0_04MODE_A::PULL_UP,
            1 => P0_04MODE_A::REPEATER,
            2 => P0_04MODE_A::DISABLED,
            3 => P0_04MODE_A::PULL_DOWN,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PULL_UP`"]
    #[inline(always)]
    pub fn is_pull_up(&self) -> bool {
        *self == P0_04MODE_A::PULL_UP
    }
    #[doc = "Checks if the value of the field is `REPEATER`"]
    #[inline(always)]
    pub fn is_repeater(&self) -> bool {
        *self == P0_04MODE_A::REPEATER
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == P0_04MODE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `PULL_DOWN`"]
    #[inline(always)]
    pub fn is_pull_down(&self) -> bool {
        *self == P0_04MODE_A::PULL_DOWN
    }
}
#[doc = "Write proxy for field `P0_04MODE`"]
pub struct P0_04MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> P0_04MODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: P0_04MODE_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Pull-up. P0.4 pin has a pull-up resistor enabled."]
    #[inline(always)]
    pub fn pull_up(self) -> &'a mut W {
        self.variant(P0_04MODE_A::PULL_UP)
    }
    #[doc = "Repeater. P0.4 pin has repeater mode enabled."]
    #[inline(always)]
    pub fn repeater(self) -> &'a mut W {
        self.variant(P0_04MODE_A::REPEATER)
    }
    #[doc = "Disabled. P0.4 pin has neither pull-up nor pull-down."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(P0_04MODE_A::DISABLED)
    }
    #[doc = "Pull-down. P0.4 has a pull-down resistor enabled."]
    #[inline(always)]
    pub fn pull_down(self) -> &'a mut W {
        self.variant(P0_04MODE_A::PULL_DOWN)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | (((value as u32) & 0x03) << 8);
        self.w
    }
}
#[doc = "Port 0 pin 5 control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum P0_05MODE_A {
    #[doc = "0: Pull-up. P0.5 pin has a pull-up resistor enabled."]
    PULL_UP = 0,
    #[doc = "1: Repeater. P0.5 pin has repeater mode enabled."]
    REPEATER = 1,
    #[doc = "2: Disabled. P0.5 pin has neither pull-up nor pull-down."]
    DISABLED = 2,
    #[doc = "3: Pull-down. P0.5 has a pull-down resistor enabled."]
    PULL_DOWN = 3,
}
impl From<P0_05MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: P0_05MODE_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `P0_05MODE`"]
pub type P0_05MODE_R = crate::R<u8, P0_05MODE_A>;
impl P0_05MODE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> P0_05MODE_A {
        match self.bits {
            0 => P0_05MODE_A::PULL_UP,
            1 => P0_05MODE_A::REPEATER,
            2 => P0_05MODE_A::DISABLED,
            3 => P0_05MODE_A::PULL_DOWN,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PULL_UP`"]
    #[inline(always)]
    pub fn is_pull_up(&self) -> bool {
        *self == P0_05MODE_A::PULL_UP
    }
    #[doc = "Checks if the value of the field is `REPEATER`"]
    #[inline(always)]
    pub fn is_repeater(&self) -> bool {
        *self == P0_05MODE_A::REPEATER
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == P0_05MODE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `PULL_DOWN`"]
    #[inline(always)]
    pub fn is_pull_down(&self) -> bool {
        *self == P0_05MODE_A::PULL_DOWN
    }
}
#[doc = "Write proxy for field `P0_05MODE`"]
pub struct P0_05MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> P0_05MODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: P0_05MODE_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Pull-up. P0.5 pin has a pull-up resistor enabled."]
    #[inline(always)]
    pub fn pull_up(self) -> &'a mut W {
        self.variant(P0_05MODE_A::PULL_UP)
    }
    #[doc = "Repeater. P0.5 pin has repeater mode enabled."]
    #[inline(always)]
    pub fn repeater(self) -> &'a mut W {
        self.variant(P0_05MODE_A::REPEATER)
    }
    #[doc = "Disabled. P0.5 pin has neither pull-up nor pull-down."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(P0_05MODE_A::DISABLED)
    }
    #[doc = "Pull-down. P0.5 has a pull-down resistor enabled."]
    #[inline(always)]
    pub fn pull_down(self) -> &'a mut W {
        self.variant(P0_05MODE_A::PULL_DOWN)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 10)) | (((value as u32) & 0x03) << 10);
        self.w
    }
}
#[doc = "Port 0 pin 6 control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum P0_06MODE_A {
    #[doc = "0: Pull-up. P0.6 pin has a pull-up resistor enabled."]
    PULL_UP = 0,
    #[doc = "1: Disabled. Repeater. P0.6 pin has repeater mode enabled."]
    DISABLED = 1,
    #[doc = "2: Disabled. P0.6 pin has neither pull-up nor pull-down."]
    DISABLED = 2,
    #[doc = "3: Pull-down. P0.6 has a pull-down resistor enabled."]
    PULL_DOWN = 3,
}
impl From<P0_06MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: P0_06MODE_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `P0_06MODE`"]
pub type P0_06MODE_R = crate::R<u8, P0_06MODE_A>;
impl P0_06MODE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> P0_06MODE_A {
        match self.bits {
            0 => P0_06MODE_A::PULL_UP,
            1 => P0_06MODE_A::DISABLED,
            2 => P0_06MODE_A::DISABLED,
            3 => P0_06MODE_A::PULL_DOWN,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PULL_UP`"]
    #[inline(always)]
    pub fn is_pull_up(&self) -> bool {
        *self == P0_06MODE_A::PULL_UP
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == P0_06MODE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == P0_06MODE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `PULL_DOWN`"]
    #[inline(always)]
    pub fn is_pull_down(&self) -> bool {
        *self == P0_06MODE_A::PULL_DOWN
    }
}
#[doc = "Write proxy for field `P0_06MODE`"]
pub struct P0_06MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> P0_06MODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: P0_06MODE_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Pull-up. P0.6 pin has a pull-up resistor enabled."]
    #[inline(always)]
    pub fn pull_up(self) -> &'a mut W {
        self.variant(P0_06MODE_A::PULL_UP)
    }
    #[doc = "Disabled. Repeater. P0.6 pin has repeater mode enabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(P0_06MODE_A::DISABLED)
    }
    #[doc = "Disabled. P0.6 pin has neither pull-up nor pull-down."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(P0_06MODE_A::DISABLED)
    }
    #[doc = "Pull-down. P0.6 has a pull-down resistor enabled."]
    #[inline(always)]
    pub fn pull_down(self) -> &'a mut W {
        self.variant(P0_06MODE_A::PULL_DOWN)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 12)) | (((value as u32) & 0x03) << 12);
        self.w
    }
}
#[doc = "Port 0 pin 7 control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum P0_07MODE_A {
    #[doc = "0: Pull-up. P0.7 pin has a pull-up resistor enabled."]
    PULL_UP = 0,
    #[doc = "1: Repeater. P0.7 pin has repeater mode enabled."]
    REPEATER = 1,
    #[doc = "2: Disabled. P0.7 pin has neither pull-up nor pull-down."]
    DISABLED = 2,
    #[doc = "3: Pull-down. P0.7 has a pull-down resistor enabled."]
    PULL_DOWN = 3,
}
impl From<P0_07MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: P0_07MODE_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `P0_07MODE`"]
pub type P0_07MODE_R = crate::R<u8, P0_07MODE_A>;
impl P0_07MODE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> P0_07MODE_A {
        match self.bits {
            0 => P0_07MODE_A::PULL_UP,
            1 => P0_07MODE_A::REPEATER,
            2 => P0_07MODE_A::DISABLED,
            3 => P0_07MODE_A::PULL_DOWN,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PULL_UP`"]
    #[inline(always)]
    pub fn is_pull_up(&self) -> bool {
        *self == P0_07MODE_A::PULL_UP
    }
    #[doc = "Checks if the value of the field is `REPEATER`"]
    #[inline(always)]
    pub fn is_repeater(&self) -> bool {
        *self == P0_07MODE_A::REPEATER
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == P0_07MODE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `PULL_DOWN`"]
    #[inline(always)]
    pub fn is_pull_down(&self) -> bool {
        *self == P0_07MODE_A::PULL_DOWN
    }
}
#[doc = "Write proxy for field `P0_07MODE`"]
pub struct P0_07MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> P0_07MODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: P0_07MODE_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Pull-up. P0.7 pin has a pull-up resistor enabled."]
    #[inline(always)]
    pub fn pull_up(self) -> &'a mut W {
        self.variant(P0_07MODE_A::PULL_UP)
    }
    #[doc = "Repeater. P0.7 pin has repeater mode enabled."]
    #[inline(always)]
    pub fn repeater(self) -> &'a mut W {
        self.variant(P0_07MODE_A::REPEATER)
    }
    #[doc = "Disabled. P0.7 pin has neither pull-up nor pull-down."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(P0_07MODE_A::DISABLED)
    }
    #[doc = "Pull-down. P0.7 has a pull-down resistor enabled."]
    #[inline(always)]
    pub fn pull_down(self) -> &'a mut W {
        self.variant(P0_07MODE_A::PULL_DOWN)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 14)) | (((value as u32) & 0x03) << 14);
        self.w
    }
}
#[doc = "Port 0 pin 8 control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum P0_08MODE_A {
    #[doc = "0: Pull-up. P0.8 pin has a pull-up resistor enabled."]
    PULL_UP = 0,
    #[doc = "1: Repeater. P0.8 pin has repeater mode enabled."]
    REPEATER = 1,
    #[doc = "2: Disabled. P0.8 pin has neither pull-up nor pull-down."]
    DISABLED = 2,
    #[doc = "3: Pull-down. P0.8 has a pull-down resistor enabled."]
    PULL_DOWN = 3,
}
impl From<P0_08MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: P0_08MODE_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `P0_08MODE`"]
pub type P0_08MODE_R = crate::R<u8, P0_08MODE_A>;
impl P0_08MODE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> P0_08MODE_A {
        match self.bits {
            0 => P0_08MODE_A::PULL_UP,
            1 => P0_08MODE_A::REPEATER,
            2 => P0_08MODE_A::DISABLED,
            3 => P0_08MODE_A::PULL_DOWN,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PULL_UP`"]
    #[inline(always)]
    pub fn is_pull_up(&self) -> bool {
        *self == P0_08MODE_A::PULL_UP
    }
    #[doc = "Checks if the value of the field is `REPEATER`"]
    #[inline(always)]
    pub fn is_repeater(&self) -> bool {
        *self == P0_08MODE_A::REPEATER
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == P0_08MODE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `PULL_DOWN`"]
    #[inline(always)]
    pub fn is_pull_down(&self) -> bool {
        *self == P0_08MODE_A::PULL_DOWN
    }
}
#[doc = "Write proxy for field `P0_08MODE`"]
pub struct P0_08MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> P0_08MODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: P0_08MODE_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Pull-up. P0.8 pin has a pull-up resistor enabled."]
    #[inline(always)]
    pub fn pull_up(self) -> &'a mut W {
        self.variant(P0_08MODE_A::PULL_UP)
    }
    #[doc = "Repeater. P0.8 pin has repeater mode enabled."]
    #[inline(always)]
    pub fn repeater(self) -> &'a mut W {
        self.variant(P0_08MODE_A::REPEATER)
    }
    #[doc = "Disabled. P0.8 pin has neither pull-up nor pull-down."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(P0_08MODE_A::DISABLED)
    }
    #[doc = "Pull-down. P0.8 has a pull-down resistor enabled."]
    #[inline(always)]
    pub fn pull_down(self) -> &'a mut W {
        self.variant(P0_08MODE_A::PULL_DOWN)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | (((value as u32) & 0x03) << 16);
        self.w
    }
}
#[doc = "Port 0 pin 9 control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum P0_09MODE_A {
    #[doc = "0: Pull-up. P0.9 pin has a pull-up resistor enabled."]
    PULL_UP = 0,
    #[doc = "1: Repeater. P0.9 pin has repeater mode enabled."]
    REPEATER = 1,
    #[doc = "2: Disabled. P0.9 pin has neither pull-up nor pull-down."]
    DISABLED = 2,
    #[doc = "3: Pull-down. P0.9 has a pull-down resistor enabled."]
    PULL_DOWN = 3,
}
impl From<P0_09MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: P0_09MODE_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `P0_09MODE`"]
pub type P0_09MODE_R = crate::R<u8, P0_09MODE_A>;
impl P0_09MODE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> P0_09MODE_A {
        match self.bits {
            0 => P0_09MODE_A::PULL_UP,
            1 => P0_09MODE_A::REPEATER,
            2 => P0_09MODE_A::DISABLED,
            3 => P0_09MODE_A::PULL_DOWN,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PULL_UP`"]
    #[inline(always)]
    pub fn is_pull_up(&self) -> bool {
        *self == P0_09MODE_A::PULL_UP
    }
    #[doc = "Checks if the value of the field is `REPEATER`"]
    #[inline(always)]
    pub fn is_repeater(&self) -> bool {
        *self == P0_09MODE_A::REPEATER
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == P0_09MODE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `PULL_DOWN`"]
    #[inline(always)]
    pub fn is_pull_down(&self) -> bool {
        *self == P0_09MODE_A::PULL_DOWN
    }
}
#[doc = "Write proxy for field `P0_09MODE`"]
pub struct P0_09MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> P0_09MODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: P0_09MODE_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Pull-up. P0.9 pin has a pull-up resistor enabled."]
    #[inline(always)]
    pub fn pull_up(self) -> &'a mut W {
        self.variant(P0_09MODE_A::PULL_UP)
    }
    #[doc = "Repeater. P0.9 pin has repeater mode enabled."]
    #[inline(always)]
    pub fn repeater(self) -> &'a mut W {
        self.variant(P0_09MODE_A::REPEATER)
    }
    #[doc = "Disabled. P0.9 pin has neither pull-up nor pull-down."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(P0_09MODE_A::DISABLED)
    }
    #[doc = "Pull-down. P0.9 has a pull-down resistor enabled."]
    #[inline(always)]
    pub fn pull_down(self) -> &'a mut W {
        self.variant(P0_09MODE_A::PULL_DOWN)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 18)) | (((value as u32) & 0x03) << 18);
        self.w
    }
}
#[doc = "Port 0 pin 10 control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum P0_10MODE_A {
    #[doc = "0: Pull-up. P0.10 pin has a pull-up resistor enabled."]
    PULL_UP = 0,
    #[doc = "1: Repeater. P0.10 pin has repeater mode enabled."]
    REPEATER = 1,
    #[doc = "2: Disabled. P0.10 pin has neither pull-up nor pull-down."]
    DISABLED = 2,
    #[doc = "3: Pull-down. P0.10 has a pull-down resistor enabled."]
    PULL_DOWN = 3,
}
impl From<P0_10MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: P0_10MODE_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `P0_10MODE`"]
pub type P0_10MODE_R = crate::R<u8, P0_10MODE_A>;
impl P0_10MODE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> P0_10MODE_A {
        match self.bits {
            0 => P0_10MODE_A::PULL_UP,
            1 => P0_10MODE_A::REPEATER,
            2 => P0_10MODE_A::DISABLED,
            3 => P0_10MODE_A::PULL_DOWN,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PULL_UP`"]
    #[inline(always)]
    pub fn is_pull_up(&self) -> bool {
        *self == P0_10MODE_A::PULL_UP
    }
    #[doc = "Checks if the value of the field is `REPEATER`"]
    #[inline(always)]
    pub fn is_repeater(&self) -> bool {
        *self == P0_10MODE_A::REPEATER
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == P0_10MODE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `PULL_DOWN`"]
    #[inline(always)]
    pub fn is_pull_down(&self) -> bool {
        *self == P0_10MODE_A::PULL_DOWN
    }
}
#[doc = "Write proxy for field `P0_10MODE`"]
pub struct P0_10MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> P0_10MODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: P0_10MODE_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Pull-up. P0.10 pin has a pull-up resistor enabled."]
    #[inline(always)]
    pub fn pull_up(self) -> &'a mut W {
        self.variant(P0_10MODE_A::PULL_UP)
    }
    #[doc = "Repeater. P0.10 pin has repeater mode enabled."]
    #[inline(always)]
    pub fn repeater(self) -> &'a mut W {
        self.variant(P0_10MODE_A::REPEATER)
    }
    #[doc = "Disabled. P0.10 pin has neither pull-up nor pull-down."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(P0_10MODE_A::DISABLED)
    }
    #[doc = "Pull-down. P0.10 has a pull-down resistor enabled."]
    #[inline(always)]
    pub fn pull_down(self) -> &'a mut W {
        self.variant(P0_10MODE_A::PULL_DOWN)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 20)) | (((value as u32) & 0x03) << 20);
        self.w
    }
}
#[doc = "Port 0 pin 11 control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum P0_11MODE_A {
    #[doc = "0: Pull-up. P0.11 pin has a pull-up resistor enabled."]
    PULL_UP = 0,
    #[doc = "1: Repeater. P0.11 pin has repeater mode enabled."]
    REPEATER = 1,
    #[doc = "2: Disabled. P0.11 pin has neither pull-up nor pull-down."]
    DISABLED = 2,
    #[doc = "3: Pull-down. P0.11 has a pull-down resistor enabled."]
    PULL_DOWN = 3,
}
impl From<P0_11MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: P0_11MODE_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `P0_11MODE`"]
pub type P0_11MODE_R = crate::R<u8, P0_11MODE_A>;
impl P0_11MODE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> P0_11MODE_A {
        match self.bits {
            0 => P0_11MODE_A::PULL_UP,
            1 => P0_11MODE_A::REPEATER,
            2 => P0_11MODE_A::DISABLED,
            3 => P0_11MODE_A::PULL_DOWN,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PULL_UP`"]
    #[inline(always)]
    pub fn is_pull_up(&self) -> bool {
        *self == P0_11MODE_A::PULL_UP
    }
    #[doc = "Checks if the value of the field is `REPEATER`"]
    #[inline(always)]
    pub fn is_repeater(&self) -> bool {
        *self == P0_11MODE_A::REPEATER
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == P0_11MODE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `PULL_DOWN`"]
    #[inline(always)]
    pub fn is_pull_down(&self) -> bool {
        *self == P0_11MODE_A::PULL_DOWN
    }
}
#[doc = "Write proxy for field `P0_11MODE`"]
pub struct P0_11MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> P0_11MODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: P0_11MODE_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Pull-up. P0.11 pin has a pull-up resistor enabled."]
    #[inline(always)]
    pub fn pull_up(self) -> &'a mut W {
        self.variant(P0_11MODE_A::PULL_UP)
    }
    #[doc = "Repeater. P0.11 pin has repeater mode enabled."]
    #[inline(always)]
    pub fn repeater(self) -> &'a mut W {
        self.variant(P0_11MODE_A::REPEATER)
    }
    #[doc = "Disabled. P0.11 pin has neither pull-up nor pull-down."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(P0_11MODE_A::DISABLED)
    }
    #[doc = "Pull-down. P0.11 has a pull-down resistor enabled."]
    #[inline(always)]
    pub fn pull_down(self) -> &'a mut W {
        self.variant(P0_11MODE_A::PULL_DOWN)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 22)) | (((value as u32) & 0x03) << 22);
        self.w
    }
}
#[doc = "Port 0 pin 15 control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum P0_15MODE_A {
    #[doc = "0: Pull-up. P0.15 pin has a pull-up resistor enabled."]
    PULL_UP = 0,
    #[doc = "1: Repeater. P0.15 pin has repeater mode enabled."]
    REPEATER = 1,
    #[doc = "2: Disabled. P0.15 pin has neither pull-up nor pull-down."]
    DISABLED = 2,
    #[doc = "3: Pull-down. P0.15 has a pull-down resistor enabled."]
    PULL_DOWN = 3,
}
impl From<P0_15MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: P0_15MODE_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `P0_15MODE`"]
pub type P0_15MODE_R = crate::R<u8, P0_15MODE_A>;
impl P0_15MODE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> P0_15MODE_A {
        match self.bits {
            0 => P0_15MODE_A::PULL_UP,
            1 => P0_15MODE_A::REPEATER,
            2 => P0_15MODE_A::DISABLED,
            3 => P0_15MODE_A::PULL_DOWN,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PULL_UP`"]
    #[inline(always)]
    pub fn is_pull_up(&self) -> bool {
        *self == P0_15MODE_A::PULL_UP
    }
    #[doc = "Checks if the value of the field is `REPEATER`"]
    #[inline(always)]
    pub fn is_repeater(&self) -> bool {
        *self == P0_15MODE_A::REPEATER
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == P0_15MODE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `PULL_DOWN`"]
    #[inline(always)]
    pub fn is_pull_down(&self) -> bool {
        *self == P0_15MODE_A::PULL_DOWN
    }
}
#[doc = "Write proxy for field `P0_15MODE`"]
pub struct P0_15MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> P0_15MODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: P0_15MODE_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Pull-up. P0.15 pin has a pull-up resistor enabled."]
    #[inline(always)]
    pub fn pull_up(self) -> &'a mut W {
        self.variant(P0_15MODE_A::PULL_UP)
    }
    #[doc = "Repeater. P0.15 pin has repeater mode enabled."]
    #[inline(always)]
    pub fn repeater(self) -> &'a mut W {
        self.variant(P0_15MODE_A::REPEATER)
    }
    #[doc = "Disabled. P0.15 pin has neither pull-up nor pull-down."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(P0_15MODE_A::DISABLED)
    }
    #[doc = "Pull-down. P0.15 has a pull-down resistor enabled."]
    #[inline(always)]
    pub fn pull_down(self) -> &'a mut W {
        self.variant(P0_15MODE_A::PULL_DOWN)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 30)) | (((value as u32) & 0x03) << 30);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - Port 0 pin 0 on-chip pull-up/down resistor control."]
    #[inline(always)]
    pub fn p0_00mode(&self) -> P0_00MODE_R {
        P0_00MODE_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 2:3 - Port 0 pin 1 control."]
    #[inline(always)]
    pub fn p0_01mode(&self) -> P0_01MODE_R {
        P0_01MODE_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bits 4:5 - Port 0 pin 2 control."]
    #[inline(always)]
    pub fn p0_02mode(&self) -> P0_02MODE_R {
        P0_02MODE_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bits 6:7 - Port 0 pin 3 control."]
    #[inline(always)]
    pub fn p0_03mode(&self) -> P0_03MODE_R {
        P0_03MODE_R::new(((self.bits >> 6) & 0x03) as u8)
    }
    #[doc = "Bits 8:9 - Port 0 pin 4 control."]
    #[inline(always)]
    pub fn p0_04mode(&self) -> P0_04MODE_R {
        P0_04MODE_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bits 10:11 - Port 0 pin 5 control."]
    #[inline(always)]
    pub fn p0_05mode(&self) -> P0_05MODE_R {
        P0_05MODE_R::new(((self.bits >> 10) & 0x03) as u8)
    }
    #[doc = "Bits 12:13 - Port 0 pin 6 control."]
    #[inline(always)]
    pub fn p0_06mode(&self) -> P0_06MODE_R {
        P0_06MODE_R::new(((self.bits >> 12) & 0x03) as u8)
    }
    #[doc = "Bits 14:15 - Port 0 pin 7 control."]
    #[inline(always)]
    pub fn p0_07mode(&self) -> P0_07MODE_R {
        P0_07MODE_R::new(((self.bits >> 14) & 0x03) as u8)
    }
    #[doc = "Bits 16:17 - Port 0 pin 8 control."]
    #[inline(always)]
    pub fn p0_08mode(&self) -> P0_08MODE_R {
        P0_08MODE_R::new(((self.bits >> 16) & 0x03) as u8)
    }
    #[doc = "Bits 18:19 - Port 0 pin 9 control."]
    #[inline(always)]
    pub fn p0_09mode(&self) -> P0_09MODE_R {
        P0_09MODE_R::new(((self.bits >> 18) & 0x03) as u8)
    }
    #[doc = "Bits 20:21 - Port 0 pin 10 control."]
    #[inline(always)]
    pub fn p0_10mode(&self) -> P0_10MODE_R {
        P0_10MODE_R::new(((self.bits >> 20) & 0x03) as u8)
    }
    #[doc = "Bits 22:23 - Port 0 pin 11 control."]
    #[inline(always)]
    pub fn p0_11mode(&self) -> P0_11MODE_R {
        P0_11MODE_R::new(((self.bits >> 22) & 0x03) as u8)
    }
    #[doc = "Bits 30:31 - Port 0 pin 15 control."]
    #[inline(always)]
    pub fn p0_15mode(&self) -> P0_15MODE_R {
        P0_15MODE_R::new(((self.bits >> 30) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Port 0 pin 0 on-chip pull-up/down resistor control."]
    #[inline(always)]
    pub fn p0_00mode(&mut self) -> P0_00MODE_W {
        P0_00MODE_W { w: self }
    }
    #[doc = "Bits 2:3 - Port 0 pin 1 control."]
    #[inline(always)]
    pub fn p0_01mode(&mut self) -> P0_01MODE_W {
        P0_01MODE_W { w: self }
    }
    #[doc = "Bits 4:5 - Port 0 pin 2 control."]
    #[inline(always)]
    pub fn p0_02mode(&mut self) -> P0_02MODE_W {
        P0_02MODE_W { w: self }
    }
    #[doc = "Bits 6:7 - Port 0 pin 3 control."]
    #[inline(always)]
    pub fn p0_03mode(&mut self) -> P0_03MODE_W {
        P0_03MODE_W { w: self }
    }
    #[doc = "Bits 8:9 - Port 0 pin 4 control."]
    #[inline(always)]
    pub fn p0_04mode(&mut self) -> P0_04MODE_W {
        P0_04MODE_W { w: self }
    }
    #[doc = "Bits 10:11 - Port 0 pin 5 control."]
    #[inline(always)]
    pub fn p0_05mode(&mut self) -> P0_05MODE_W {
        P0_05MODE_W { w: self }
    }
    #[doc = "Bits 12:13 - Port 0 pin 6 control."]
    #[inline(always)]
    pub fn p0_06mode(&mut self) -> P0_06MODE_W {
        P0_06MODE_W { w: self }
    }
    #[doc = "Bits 14:15 - Port 0 pin 7 control."]
    #[inline(always)]
    pub fn p0_07mode(&mut self) -> P0_07MODE_W {
        P0_07MODE_W { w: self }
    }
    #[doc = "Bits 16:17 - Port 0 pin 8 control."]
    #[inline(always)]
    pub fn p0_08mode(&mut self) -> P0_08MODE_W {
        P0_08MODE_W { w: self }
    }
    #[doc = "Bits 18:19 - Port 0 pin 9 control."]
    #[inline(always)]
    pub fn p0_09mode(&mut self) -> P0_09MODE_W {
        P0_09MODE_W { w: self }
    }
    #[doc = "Bits 20:21 - Port 0 pin 10 control."]
    #[inline(always)]
    pub fn p0_10mode(&mut self) -> P0_10MODE_W {
        P0_10MODE_W { w: self }
    }
    #[doc = "Bits 22:23 - Port 0 pin 11 control."]
    #[inline(always)]
    pub fn p0_11mode(&mut self) -> P0_11MODE_W {
        P0_11MODE_W { w: self }
    }
    #[doc = "Bits 30:31 - Port 0 pin 15 control."]
    #[inline(always)]
    pub fn p0_15mode(&mut self) -> P0_15MODE_W {
        P0_15MODE_W { w: self }
    }
}
