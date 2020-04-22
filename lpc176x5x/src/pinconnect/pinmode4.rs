#[doc = "Reader of register PINMODE4"]
pub type R = crate::R<u32, super::PINMODE4>;
#[doc = "Writer for register PINMODE4"]
pub type W = crate::W<u32, super::PINMODE4>;
#[doc = "Register PINMODE4 `reset()`'s with value 0"]
impl crate::ResetValue for super::PINMODE4 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Port 2 pin 0 control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum P2_00MODE_A {
    #[doc = "0: Pull-up. P2.0 pin has a pull-up resistor enabled."]
    PULL_UP = 0,
    #[doc = "1: Repeater. P2.0 pin has repeater mode enabled."]
    REPEATER = 1,
    #[doc = "2: Disabled. P2.0 pin has neither pull-up nor pull-down."]
    DISABLED = 2,
    #[doc = "3: Pull-down. P2.0 has a pull-down resistor enabled."]
    PULL_DOWN = 3,
}
impl From<P2_00MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: P2_00MODE_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `P2_00MODE`"]
pub type P2_00MODE_R = crate::R<u8, P2_00MODE_A>;
impl P2_00MODE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> P2_00MODE_A {
        match self.bits {
            0 => P2_00MODE_A::PULL_UP,
            1 => P2_00MODE_A::REPEATER,
            2 => P2_00MODE_A::DISABLED,
            3 => P2_00MODE_A::PULL_DOWN,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PULL_UP`"]
    #[inline(always)]
    pub fn is_pull_up(&self) -> bool {
        *self == P2_00MODE_A::PULL_UP
    }
    #[doc = "Checks if the value of the field is `REPEATER`"]
    #[inline(always)]
    pub fn is_repeater(&self) -> bool {
        *self == P2_00MODE_A::REPEATER
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == P2_00MODE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `PULL_DOWN`"]
    #[inline(always)]
    pub fn is_pull_down(&self) -> bool {
        *self == P2_00MODE_A::PULL_DOWN
    }
}
#[doc = "Write proxy for field `P2_00MODE`"]
pub struct P2_00MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> P2_00MODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: P2_00MODE_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Pull-up. P2.0 pin has a pull-up resistor enabled."]
    #[inline(always)]
    pub fn pull_up(self) -> &'a mut W {
        self.variant(P2_00MODE_A::PULL_UP)
    }
    #[doc = "Repeater. P2.0 pin has repeater mode enabled."]
    #[inline(always)]
    pub fn repeater(self) -> &'a mut W {
        self.variant(P2_00MODE_A::REPEATER)
    }
    #[doc = "Disabled. P2.0 pin has neither pull-up nor pull-down."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(P2_00MODE_A::DISABLED)
    }
    #[doc = "Pull-down. P2.0 has a pull-down resistor enabled."]
    #[inline(always)]
    pub fn pull_down(self) -> &'a mut W {
        self.variant(P2_00MODE_A::PULL_DOWN)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
#[doc = "Port 2 pin 1 control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum P2_01MODE_A {
    #[doc = "0: Pull-up. P2.1 pin has a pull-up resistor enabled."]
    PULL_UP = 0,
    #[doc = "1: Repeater. P2.1 pin has repeater mode enabled."]
    REPEATER = 1,
    #[doc = "2: Disabled. P2.1 pin has neither pull-up nor pull-down."]
    DISABLED = 2,
    #[doc = "3: Pull-down. P2.1 has a pull-down resistor enabled."]
    PULL_DOWN = 3,
}
impl From<P2_01MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: P2_01MODE_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `P2_01MODE`"]
pub type P2_01MODE_R = crate::R<u8, P2_01MODE_A>;
impl P2_01MODE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> P2_01MODE_A {
        match self.bits {
            0 => P2_01MODE_A::PULL_UP,
            1 => P2_01MODE_A::REPEATER,
            2 => P2_01MODE_A::DISABLED,
            3 => P2_01MODE_A::PULL_DOWN,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PULL_UP`"]
    #[inline(always)]
    pub fn is_pull_up(&self) -> bool {
        *self == P2_01MODE_A::PULL_UP
    }
    #[doc = "Checks if the value of the field is `REPEATER`"]
    #[inline(always)]
    pub fn is_repeater(&self) -> bool {
        *self == P2_01MODE_A::REPEATER
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == P2_01MODE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `PULL_DOWN`"]
    #[inline(always)]
    pub fn is_pull_down(&self) -> bool {
        *self == P2_01MODE_A::PULL_DOWN
    }
}
#[doc = "Write proxy for field `P2_01MODE`"]
pub struct P2_01MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> P2_01MODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: P2_01MODE_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Pull-up. P2.1 pin has a pull-up resistor enabled."]
    #[inline(always)]
    pub fn pull_up(self) -> &'a mut W {
        self.variant(P2_01MODE_A::PULL_UP)
    }
    #[doc = "Repeater. P2.1 pin has repeater mode enabled."]
    #[inline(always)]
    pub fn repeater(self) -> &'a mut W {
        self.variant(P2_01MODE_A::REPEATER)
    }
    #[doc = "Disabled. P2.1 pin has neither pull-up nor pull-down."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(P2_01MODE_A::DISABLED)
    }
    #[doc = "Pull-down. P2.1 has a pull-down resistor enabled."]
    #[inline(always)]
    pub fn pull_down(self) -> &'a mut W {
        self.variant(P2_01MODE_A::PULL_DOWN)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | (((value as u32) & 0x03) << 2);
        self.w
    }
}
#[doc = "Port 2 pin 2 control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum P2_02MODE_A {
    #[doc = "0: Pull-up. P2.2 pin has a pull-up resistor enabled."]
    PULL_UP = 0,
    #[doc = "1: Repeater. P2.2 pin has repeater mode enabled."]
    REPEATER = 1,
    #[doc = "2: Disabled. P2.2 pin has neither pull-up nor pull-down."]
    DISABLED = 2,
    #[doc = "3: Pull-down. P2.2 has a pull-down resistor enabled."]
    PULL_DOWN = 3,
}
impl From<P2_02MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: P2_02MODE_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `P2_02MODE`"]
pub type P2_02MODE_R = crate::R<u8, P2_02MODE_A>;
impl P2_02MODE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> P2_02MODE_A {
        match self.bits {
            0 => P2_02MODE_A::PULL_UP,
            1 => P2_02MODE_A::REPEATER,
            2 => P2_02MODE_A::DISABLED,
            3 => P2_02MODE_A::PULL_DOWN,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PULL_UP`"]
    #[inline(always)]
    pub fn is_pull_up(&self) -> bool {
        *self == P2_02MODE_A::PULL_UP
    }
    #[doc = "Checks if the value of the field is `REPEATER`"]
    #[inline(always)]
    pub fn is_repeater(&self) -> bool {
        *self == P2_02MODE_A::REPEATER
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == P2_02MODE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `PULL_DOWN`"]
    #[inline(always)]
    pub fn is_pull_down(&self) -> bool {
        *self == P2_02MODE_A::PULL_DOWN
    }
}
#[doc = "Write proxy for field `P2_02MODE`"]
pub struct P2_02MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> P2_02MODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: P2_02MODE_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Pull-up. P2.2 pin has a pull-up resistor enabled."]
    #[inline(always)]
    pub fn pull_up(self) -> &'a mut W {
        self.variant(P2_02MODE_A::PULL_UP)
    }
    #[doc = "Repeater. P2.2 pin has repeater mode enabled."]
    #[inline(always)]
    pub fn repeater(self) -> &'a mut W {
        self.variant(P2_02MODE_A::REPEATER)
    }
    #[doc = "Disabled. P2.2 pin has neither pull-up nor pull-down."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(P2_02MODE_A::DISABLED)
    }
    #[doc = "Pull-down. P2.2 has a pull-down resistor enabled."]
    #[inline(always)]
    pub fn pull_down(self) -> &'a mut W {
        self.variant(P2_02MODE_A::PULL_DOWN)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | (((value as u32) & 0x03) << 4);
        self.w
    }
}
#[doc = "Port 2 pin 3 control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum P2_03MODE_A {
    #[doc = "0: Pull-up. P2.3 pin has a pull-up resistor enabled."]
    PULL_UP = 0,
    #[doc = "1: Repeater. P2.3 pin has repeater mode enabled."]
    REPEATER = 1,
    #[doc = "2: Disabled. P2.3 pin has neither pull-up nor pull-down."]
    DISABLED = 2,
    #[doc = "3: Pull-down. P2.3 has a pull-down resistor enabled."]
    PULL_DOWN = 3,
}
impl From<P2_03MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: P2_03MODE_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `P2_03MODE`"]
pub type P2_03MODE_R = crate::R<u8, P2_03MODE_A>;
impl P2_03MODE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> P2_03MODE_A {
        match self.bits {
            0 => P2_03MODE_A::PULL_UP,
            1 => P2_03MODE_A::REPEATER,
            2 => P2_03MODE_A::DISABLED,
            3 => P2_03MODE_A::PULL_DOWN,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PULL_UP`"]
    #[inline(always)]
    pub fn is_pull_up(&self) -> bool {
        *self == P2_03MODE_A::PULL_UP
    }
    #[doc = "Checks if the value of the field is `REPEATER`"]
    #[inline(always)]
    pub fn is_repeater(&self) -> bool {
        *self == P2_03MODE_A::REPEATER
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == P2_03MODE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `PULL_DOWN`"]
    #[inline(always)]
    pub fn is_pull_down(&self) -> bool {
        *self == P2_03MODE_A::PULL_DOWN
    }
}
#[doc = "Write proxy for field `P2_03MODE`"]
pub struct P2_03MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> P2_03MODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: P2_03MODE_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Pull-up. P2.3 pin has a pull-up resistor enabled."]
    #[inline(always)]
    pub fn pull_up(self) -> &'a mut W {
        self.variant(P2_03MODE_A::PULL_UP)
    }
    #[doc = "Repeater. P2.3 pin has repeater mode enabled."]
    #[inline(always)]
    pub fn repeater(self) -> &'a mut W {
        self.variant(P2_03MODE_A::REPEATER)
    }
    #[doc = "Disabled. P2.3 pin has neither pull-up nor pull-down."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(P2_03MODE_A::DISABLED)
    }
    #[doc = "Pull-down. P2.3 has a pull-down resistor enabled."]
    #[inline(always)]
    pub fn pull_down(self) -> &'a mut W {
        self.variant(P2_03MODE_A::PULL_DOWN)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | (((value as u32) & 0x03) << 6);
        self.w
    }
}
#[doc = "Port 2 pin 4 control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum P2_04MODE_A {
    #[doc = "0: Pull-up. P2.4 pin has a pull-up resistor enabled."]
    PULL_UP = 0,
    #[doc = "1: Repeater. P2.4 pin has repeater mode enabled."]
    REPEATER = 1,
    #[doc = "2: Disabled. P2.4 pin has neither pull-up nor pull-down."]
    DISABLED = 2,
    #[doc = "3: Pull-down. P2.4 has a pull-down resistor enabled."]
    PULL_DOWN = 3,
}
impl From<P2_04MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: P2_04MODE_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `P2_04MODE`"]
pub type P2_04MODE_R = crate::R<u8, P2_04MODE_A>;
impl P2_04MODE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> P2_04MODE_A {
        match self.bits {
            0 => P2_04MODE_A::PULL_UP,
            1 => P2_04MODE_A::REPEATER,
            2 => P2_04MODE_A::DISABLED,
            3 => P2_04MODE_A::PULL_DOWN,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PULL_UP`"]
    #[inline(always)]
    pub fn is_pull_up(&self) -> bool {
        *self == P2_04MODE_A::PULL_UP
    }
    #[doc = "Checks if the value of the field is `REPEATER`"]
    #[inline(always)]
    pub fn is_repeater(&self) -> bool {
        *self == P2_04MODE_A::REPEATER
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == P2_04MODE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `PULL_DOWN`"]
    #[inline(always)]
    pub fn is_pull_down(&self) -> bool {
        *self == P2_04MODE_A::PULL_DOWN
    }
}
#[doc = "Write proxy for field `P2_04MODE`"]
pub struct P2_04MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> P2_04MODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: P2_04MODE_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Pull-up. P2.4 pin has a pull-up resistor enabled."]
    #[inline(always)]
    pub fn pull_up(self) -> &'a mut W {
        self.variant(P2_04MODE_A::PULL_UP)
    }
    #[doc = "Repeater. P2.4 pin has repeater mode enabled."]
    #[inline(always)]
    pub fn repeater(self) -> &'a mut W {
        self.variant(P2_04MODE_A::REPEATER)
    }
    #[doc = "Disabled. P2.4 pin has neither pull-up nor pull-down."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(P2_04MODE_A::DISABLED)
    }
    #[doc = "Pull-down. P2.4 has a pull-down resistor enabled."]
    #[inline(always)]
    pub fn pull_down(self) -> &'a mut W {
        self.variant(P2_04MODE_A::PULL_DOWN)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | (((value as u32) & 0x03) << 8);
        self.w
    }
}
#[doc = "Port 2 pin 5 control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum P2_05MODE_A {
    #[doc = "0: Pull-up. P2.5 pin has a pull-up resistor enabled."]
    PULL_UP = 0,
    #[doc = "1: Repeater. P2.5 pin has repeater mode enabled."]
    REPEATER = 1,
    #[doc = "2: Disabled. P2.5 pin has neither pull-up nor pull-down."]
    DISABLED = 2,
    #[doc = "3: Pull-down. P2.5 has a pull-down resistor enabled."]
    PULL_DOWN = 3,
}
impl From<P2_05MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: P2_05MODE_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `P2_05MODE`"]
pub type P2_05MODE_R = crate::R<u8, P2_05MODE_A>;
impl P2_05MODE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> P2_05MODE_A {
        match self.bits {
            0 => P2_05MODE_A::PULL_UP,
            1 => P2_05MODE_A::REPEATER,
            2 => P2_05MODE_A::DISABLED,
            3 => P2_05MODE_A::PULL_DOWN,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PULL_UP`"]
    #[inline(always)]
    pub fn is_pull_up(&self) -> bool {
        *self == P2_05MODE_A::PULL_UP
    }
    #[doc = "Checks if the value of the field is `REPEATER`"]
    #[inline(always)]
    pub fn is_repeater(&self) -> bool {
        *self == P2_05MODE_A::REPEATER
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == P2_05MODE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `PULL_DOWN`"]
    #[inline(always)]
    pub fn is_pull_down(&self) -> bool {
        *self == P2_05MODE_A::PULL_DOWN
    }
}
#[doc = "Write proxy for field `P2_05MODE`"]
pub struct P2_05MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> P2_05MODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: P2_05MODE_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Pull-up. P2.5 pin has a pull-up resistor enabled."]
    #[inline(always)]
    pub fn pull_up(self) -> &'a mut W {
        self.variant(P2_05MODE_A::PULL_UP)
    }
    #[doc = "Repeater. P2.5 pin has repeater mode enabled."]
    #[inline(always)]
    pub fn repeater(self) -> &'a mut W {
        self.variant(P2_05MODE_A::REPEATER)
    }
    #[doc = "Disabled. P2.5 pin has neither pull-up nor pull-down."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(P2_05MODE_A::DISABLED)
    }
    #[doc = "Pull-down. P2.5 has a pull-down resistor enabled."]
    #[inline(always)]
    pub fn pull_down(self) -> &'a mut W {
        self.variant(P2_05MODE_A::PULL_DOWN)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 10)) | (((value as u32) & 0x03) << 10);
        self.w
    }
}
#[doc = "Port 2 pin 6 control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum P2_06MODE_A {
    #[doc = "0: Pull-up. P2.6 pin has a pull-up resistor enabled."]
    PULL_UP = 0,
    #[doc = "1: Repeater. P2.6 pin has repeater mode enabled."]
    REPEATER = 1,
    #[doc = "2: Disabled. P2.6 pin has neither pull-up nor pull-down."]
    DISABLED = 2,
    #[doc = "3: Pull-down. P2.6 has a pull-down resistor enabled."]
    PULL_DOWN = 3,
}
impl From<P2_06MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: P2_06MODE_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `P2_06MODE`"]
pub type P2_06MODE_R = crate::R<u8, P2_06MODE_A>;
impl P2_06MODE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> P2_06MODE_A {
        match self.bits {
            0 => P2_06MODE_A::PULL_UP,
            1 => P2_06MODE_A::REPEATER,
            2 => P2_06MODE_A::DISABLED,
            3 => P2_06MODE_A::PULL_DOWN,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PULL_UP`"]
    #[inline(always)]
    pub fn is_pull_up(&self) -> bool {
        *self == P2_06MODE_A::PULL_UP
    }
    #[doc = "Checks if the value of the field is `REPEATER`"]
    #[inline(always)]
    pub fn is_repeater(&self) -> bool {
        *self == P2_06MODE_A::REPEATER
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == P2_06MODE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `PULL_DOWN`"]
    #[inline(always)]
    pub fn is_pull_down(&self) -> bool {
        *self == P2_06MODE_A::PULL_DOWN
    }
}
#[doc = "Write proxy for field `P2_06MODE`"]
pub struct P2_06MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> P2_06MODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: P2_06MODE_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Pull-up. P2.6 pin has a pull-up resistor enabled."]
    #[inline(always)]
    pub fn pull_up(self) -> &'a mut W {
        self.variant(P2_06MODE_A::PULL_UP)
    }
    #[doc = "Repeater. P2.6 pin has repeater mode enabled."]
    #[inline(always)]
    pub fn repeater(self) -> &'a mut W {
        self.variant(P2_06MODE_A::REPEATER)
    }
    #[doc = "Disabled. P2.6 pin has neither pull-up nor pull-down."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(P2_06MODE_A::DISABLED)
    }
    #[doc = "Pull-down. P2.6 has a pull-down resistor enabled."]
    #[inline(always)]
    pub fn pull_down(self) -> &'a mut W {
        self.variant(P2_06MODE_A::PULL_DOWN)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 12)) | (((value as u32) & 0x03) << 12);
        self.w
    }
}
#[doc = "Port 2 pin 7 control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum P2_07MODE_A {
    #[doc = "0: Pull-up. P2.7 pin has a pull-up resistor enabled."]
    PULL_UP = 0,
    #[doc = "1: Repeater. P2.7 pin has repeater mode enabled."]
    REPEATER = 1,
    #[doc = "2: Disabled. P2.7 pin has neither pull-up nor pull-down."]
    DISABLED = 2,
    #[doc = "3: Pull-down. P2.7 has a pull-down resistor enabled."]
    PULL_DOWN = 3,
}
impl From<P2_07MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: P2_07MODE_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `P2_07MODE`"]
pub type P2_07MODE_R = crate::R<u8, P2_07MODE_A>;
impl P2_07MODE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> P2_07MODE_A {
        match self.bits {
            0 => P2_07MODE_A::PULL_UP,
            1 => P2_07MODE_A::REPEATER,
            2 => P2_07MODE_A::DISABLED,
            3 => P2_07MODE_A::PULL_DOWN,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PULL_UP`"]
    #[inline(always)]
    pub fn is_pull_up(&self) -> bool {
        *self == P2_07MODE_A::PULL_UP
    }
    #[doc = "Checks if the value of the field is `REPEATER`"]
    #[inline(always)]
    pub fn is_repeater(&self) -> bool {
        *self == P2_07MODE_A::REPEATER
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == P2_07MODE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `PULL_DOWN`"]
    #[inline(always)]
    pub fn is_pull_down(&self) -> bool {
        *self == P2_07MODE_A::PULL_DOWN
    }
}
#[doc = "Write proxy for field `P2_07MODE`"]
pub struct P2_07MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> P2_07MODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: P2_07MODE_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Pull-up. P2.7 pin has a pull-up resistor enabled."]
    #[inline(always)]
    pub fn pull_up(self) -> &'a mut W {
        self.variant(P2_07MODE_A::PULL_UP)
    }
    #[doc = "Repeater. P2.7 pin has repeater mode enabled."]
    #[inline(always)]
    pub fn repeater(self) -> &'a mut W {
        self.variant(P2_07MODE_A::REPEATER)
    }
    #[doc = "Disabled. P2.7 pin has neither pull-up nor pull-down."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(P2_07MODE_A::DISABLED)
    }
    #[doc = "Pull-down. P2.7 has a pull-down resistor enabled."]
    #[inline(always)]
    pub fn pull_down(self) -> &'a mut W {
        self.variant(P2_07MODE_A::PULL_DOWN)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 14)) | (((value as u32) & 0x03) << 14);
        self.w
    }
}
#[doc = "Port 2 pin 8 control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum P2_08MODE_A {
    #[doc = "0: Pull-up. P2.8 pin has a pull-up resistor enabled."]
    PULL_UP = 0,
    #[doc = "1: Repeater. P2.8 pin has repeater mode enabled."]
    REPEATER = 1,
    #[doc = "2: Disabled. P2.8 pin has neither pull-up nor pull-down."]
    DISABLED = 2,
    #[doc = "3: Pull-down. P2.8 has a pull-down resistor enabled."]
    PULL_DOWN = 3,
}
impl From<P2_08MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: P2_08MODE_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `P2_08MODE`"]
pub type P2_08MODE_R = crate::R<u8, P2_08MODE_A>;
impl P2_08MODE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> P2_08MODE_A {
        match self.bits {
            0 => P2_08MODE_A::PULL_UP,
            1 => P2_08MODE_A::REPEATER,
            2 => P2_08MODE_A::DISABLED,
            3 => P2_08MODE_A::PULL_DOWN,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PULL_UP`"]
    #[inline(always)]
    pub fn is_pull_up(&self) -> bool {
        *self == P2_08MODE_A::PULL_UP
    }
    #[doc = "Checks if the value of the field is `REPEATER`"]
    #[inline(always)]
    pub fn is_repeater(&self) -> bool {
        *self == P2_08MODE_A::REPEATER
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == P2_08MODE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `PULL_DOWN`"]
    #[inline(always)]
    pub fn is_pull_down(&self) -> bool {
        *self == P2_08MODE_A::PULL_DOWN
    }
}
#[doc = "Write proxy for field `P2_08MODE`"]
pub struct P2_08MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> P2_08MODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: P2_08MODE_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Pull-up. P2.8 pin has a pull-up resistor enabled."]
    #[inline(always)]
    pub fn pull_up(self) -> &'a mut W {
        self.variant(P2_08MODE_A::PULL_UP)
    }
    #[doc = "Repeater. P2.8 pin has repeater mode enabled."]
    #[inline(always)]
    pub fn repeater(self) -> &'a mut W {
        self.variant(P2_08MODE_A::REPEATER)
    }
    #[doc = "Disabled. P2.8 pin has neither pull-up nor pull-down."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(P2_08MODE_A::DISABLED)
    }
    #[doc = "Pull-down. P2.8 has a pull-down resistor enabled."]
    #[inline(always)]
    pub fn pull_down(self) -> &'a mut W {
        self.variant(P2_08MODE_A::PULL_DOWN)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | (((value as u32) & 0x03) << 16);
        self.w
    }
}
#[doc = "Port 2 pin 9 control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum P2_09MODE_A {
    #[doc = "0: Pull-up. P2.9 pin has a pull-up resistor enabled."]
    PULL_UP = 0,
    #[doc = "1: Repeater. P2.9 pin has repeater mode enabled."]
    REPEATER = 1,
    #[doc = "2: Disabled. P2.9 pin has neither pull-up nor pull-down."]
    DISABLED = 2,
    #[doc = "3: Pull-down. P2.9 has a pull-down resistor enabled."]
    PULL_DOWN = 3,
}
impl From<P2_09MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: P2_09MODE_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `P2_09MODE`"]
pub type P2_09MODE_R = crate::R<u8, P2_09MODE_A>;
impl P2_09MODE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> P2_09MODE_A {
        match self.bits {
            0 => P2_09MODE_A::PULL_UP,
            1 => P2_09MODE_A::REPEATER,
            2 => P2_09MODE_A::DISABLED,
            3 => P2_09MODE_A::PULL_DOWN,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PULL_UP`"]
    #[inline(always)]
    pub fn is_pull_up(&self) -> bool {
        *self == P2_09MODE_A::PULL_UP
    }
    #[doc = "Checks if the value of the field is `REPEATER`"]
    #[inline(always)]
    pub fn is_repeater(&self) -> bool {
        *self == P2_09MODE_A::REPEATER
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == P2_09MODE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `PULL_DOWN`"]
    #[inline(always)]
    pub fn is_pull_down(&self) -> bool {
        *self == P2_09MODE_A::PULL_DOWN
    }
}
#[doc = "Write proxy for field `P2_09MODE`"]
pub struct P2_09MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> P2_09MODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: P2_09MODE_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Pull-up. P2.9 pin has a pull-up resistor enabled."]
    #[inline(always)]
    pub fn pull_up(self) -> &'a mut W {
        self.variant(P2_09MODE_A::PULL_UP)
    }
    #[doc = "Repeater. P2.9 pin has repeater mode enabled."]
    #[inline(always)]
    pub fn repeater(self) -> &'a mut W {
        self.variant(P2_09MODE_A::REPEATER)
    }
    #[doc = "Disabled. P2.9 pin has neither pull-up nor pull-down."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(P2_09MODE_A::DISABLED)
    }
    #[doc = "Pull-down. P2.9 has a pull-down resistor enabled."]
    #[inline(always)]
    pub fn pull_down(self) -> &'a mut W {
        self.variant(P2_09MODE_A::PULL_DOWN)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 18)) | (((value as u32) & 0x03) << 18);
        self.w
    }
}
#[doc = "Port 2 pin 10 control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum P2_10MODE_A {
    #[doc = "0: Pull-up. P2.10 pin has a pull-up resistor enabled."]
    PULL_UP = 0,
    #[doc = "1: Repeater. P2.10 pin has repeater mode enabled."]
    REPEATER = 1,
    #[doc = "2: Disabled. P2.10 pin has neither pull-up nor pull-down."]
    DISABLED = 2,
    #[doc = "3: Pull-down. P2.10 has a pull-down resistor enabled."]
    PULL_DOWN = 3,
}
impl From<P2_10MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: P2_10MODE_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `P2_10MODE`"]
pub type P2_10MODE_R = crate::R<u8, P2_10MODE_A>;
impl P2_10MODE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> P2_10MODE_A {
        match self.bits {
            0 => P2_10MODE_A::PULL_UP,
            1 => P2_10MODE_A::REPEATER,
            2 => P2_10MODE_A::DISABLED,
            3 => P2_10MODE_A::PULL_DOWN,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PULL_UP`"]
    #[inline(always)]
    pub fn is_pull_up(&self) -> bool {
        *self == P2_10MODE_A::PULL_UP
    }
    #[doc = "Checks if the value of the field is `REPEATER`"]
    #[inline(always)]
    pub fn is_repeater(&self) -> bool {
        *self == P2_10MODE_A::REPEATER
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == P2_10MODE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `PULL_DOWN`"]
    #[inline(always)]
    pub fn is_pull_down(&self) -> bool {
        *self == P2_10MODE_A::PULL_DOWN
    }
}
#[doc = "Write proxy for field `P2_10MODE`"]
pub struct P2_10MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> P2_10MODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: P2_10MODE_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Pull-up. P2.10 pin has a pull-up resistor enabled."]
    #[inline(always)]
    pub fn pull_up(self) -> &'a mut W {
        self.variant(P2_10MODE_A::PULL_UP)
    }
    #[doc = "Repeater. P2.10 pin has repeater mode enabled."]
    #[inline(always)]
    pub fn repeater(self) -> &'a mut W {
        self.variant(P2_10MODE_A::REPEATER)
    }
    #[doc = "Disabled. P2.10 pin has neither pull-up nor pull-down."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(P2_10MODE_A::DISABLED)
    }
    #[doc = "Pull-down. P2.10 has a pull-down resistor enabled."]
    #[inline(always)]
    pub fn pull_down(self) -> &'a mut W {
        self.variant(P2_10MODE_A::PULL_DOWN)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 20)) | (((value as u32) & 0x03) << 20);
        self.w
    }
}
#[doc = "Port 2 pin 11 control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum P2_11MODE_A {
    #[doc = "0: Pull-up. P2.11 pin has a pull-up resistor enabled."]
    PULL_UP = 0,
    #[doc = "1: Repeater. P2.11 pin has repeater mode enabled."]
    REPEATER = 1,
    #[doc = "2: Disabled. P2.11 pin has neither pull-up nor pull-down."]
    DISABLED = 2,
    #[doc = "3: Pull-down. P2.11 has a pull-down resistor enabled."]
    PULL_DOWN = 3,
}
impl From<P2_11MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: P2_11MODE_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `P2_11MODE`"]
pub type P2_11MODE_R = crate::R<u8, P2_11MODE_A>;
impl P2_11MODE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> P2_11MODE_A {
        match self.bits {
            0 => P2_11MODE_A::PULL_UP,
            1 => P2_11MODE_A::REPEATER,
            2 => P2_11MODE_A::DISABLED,
            3 => P2_11MODE_A::PULL_DOWN,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PULL_UP`"]
    #[inline(always)]
    pub fn is_pull_up(&self) -> bool {
        *self == P2_11MODE_A::PULL_UP
    }
    #[doc = "Checks if the value of the field is `REPEATER`"]
    #[inline(always)]
    pub fn is_repeater(&self) -> bool {
        *self == P2_11MODE_A::REPEATER
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == P2_11MODE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `PULL_DOWN`"]
    #[inline(always)]
    pub fn is_pull_down(&self) -> bool {
        *self == P2_11MODE_A::PULL_DOWN
    }
}
#[doc = "Write proxy for field `P2_11MODE`"]
pub struct P2_11MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> P2_11MODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: P2_11MODE_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Pull-up. P2.11 pin has a pull-up resistor enabled."]
    #[inline(always)]
    pub fn pull_up(self) -> &'a mut W {
        self.variant(P2_11MODE_A::PULL_UP)
    }
    #[doc = "Repeater. P2.11 pin has repeater mode enabled."]
    #[inline(always)]
    pub fn repeater(self) -> &'a mut W {
        self.variant(P2_11MODE_A::REPEATER)
    }
    #[doc = "Disabled. P2.11 pin has neither pull-up nor pull-down."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(P2_11MODE_A::DISABLED)
    }
    #[doc = "Pull-down. P2.11 has a pull-down resistor enabled."]
    #[inline(always)]
    pub fn pull_down(self) -> &'a mut W {
        self.variant(P2_11MODE_A::PULL_DOWN)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 22)) | (((value as u32) & 0x03) << 22);
        self.w
    }
}
#[doc = "Port 2 pin 12 control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum P2_12MODE_A {
    #[doc = "0: Pull-up. P2.12 pin has a pull-up resistor enabled."]
    PULL_UP = 0,
    #[doc = "1: Repeater. P2.12 pin has repeater mode enabled."]
    REPEATER = 1,
    #[doc = "2: Disabled. P2.12 pin has neither pull-up nor pull-down."]
    DISABLED = 2,
    #[doc = "3: Pull-down. P2.12 has a pull-down resistor enabled."]
    PULL_DOWN = 3,
}
impl From<P2_12MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: P2_12MODE_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `P2_12MODE`"]
pub type P2_12MODE_R = crate::R<u8, P2_12MODE_A>;
impl P2_12MODE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> P2_12MODE_A {
        match self.bits {
            0 => P2_12MODE_A::PULL_UP,
            1 => P2_12MODE_A::REPEATER,
            2 => P2_12MODE_A::DISABLED,
            3 => P2_12MODE_A::PULL_DOWN,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PULL_UP`"]
    #[inline(always)]
    pub fn is_pull_up(&self) -> bool {
        *self == P2_12MODE_A::PULL_UP
    }
    #[doc = "Checks if the value of the field is `REPEATER`"]
    #[inline(always)]
    pub fn is_repeater(&self) -> bool {
        *self == P2_12MODE_A::REPEATER
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == P2_12MODE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `PULL_DOWN`"]
    #[inline(always)]
    pub fn is_pull_down(&self) -> bool {
        *self == P2_12MODE_A::PULL_DOWN
    }
}
#[doc = "Write proxy for field `P2_12MODE`"]
pub struct P2_12MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> P2_12MODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: P2_12MODE_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Pull-up. P2.12 pin has a pull-up resistor enabled."]
    #[inline(always)]
    pub fn pull_up(self) -> &'a mut W {
        self.variant(P2_12MODE_A::PULL_UP)
    }
    #[doc = "Repeater. P2.12 pin has repeater mode enabled."]
    #[inline(always)]
    pub fn repeater(self) -> &'a mut W {
        self.variant(P2_12MODE_A::REPEATER)
    }
    #[doc = "Disabled. P2.12 pin has neither pull-up nor pull-down."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(P2_12MODE_A::DISABLED)
    }
    #[doc = "Pull-down. P2.12 has a pull-down resistor enabled."]
    #[inline(always)]
    pub fn pull_down(self) -> &'a mut W {
        self.variant(P2_12MODE_A::PULL_DOWN)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 24)) | (((value as u32) & 0x03) << 24);
        self.w
    }
}
#[doc = "Port 2 pin 13 control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum P2_13MODE_A {
    #[doc = "0: Pull-up. P2.13 pin has a pull-up resistor enabled."]
    PULL_UP = 0,
    #[doc = "1: Repeater. P2.13 pin has repeater mode enabled."]
    REPEATER = 1,
    #[doc = "2: Disabled. P2.13 pin has neither pull-up nor pull-down."]
    DISABLED = 2,
    #[doc = "3: Pull-down. P2.13 has a pull-down resistor enabled."]
    PULL_DOWN = 3,
}
impl From<P2_13MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: P2_13MODE_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `P2_13MODE`"]
pub type P2_13MODE_R = crate::R<u8, P2_13MODE_A>;
impl P2_13MODE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> P2_13MODE_A {
        match self.bits {
            0 => P2_13MODE_A::PULL_UP,
            1 => P2_13MODE_A::REPEATER,
            2 => P2_13MODE_A::DISABLED,
            3 => P2_13MODE_A::PULL_DOWN,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PULL_UP`"]
    #[inline(always)]
    pub fn is_pull_up(&self) -> bool {
        *self == P2_13MODE_A::PULL_UP
    }
    #[doc = "Checks if the value of the field is `REPEATER`"]
    #[inline(always)]
    pub fn is_repeater(&self) -> bool {
        *self == P2_13MODE_A::REPEATER
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == P2_13MODE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `PULL_DOWN`"]
    #[inline(always)]
    pub fn is_pull_down(&self) -> bool {
        *self == P2_13MODE_A::PULL_DOWN
    }
}
#[doc = "Write proxy for field `P2_13MODE`"]
pub struct P2_13MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> P2_13MODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: P2_13MODE_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Pull-up. P2.13 pin has a pull-up resistor enabled."]
    #[inline(always)]
    pub fn pull_up(self) -> &'a mut W {
        self.variant(P2_13MODE_A::PULL_UP)
    }
    #[doc = "Repeater. P2.13 pin has repeater mode enabled."]
    #[inline(always)]
    pub fn repeater(self) -> &'a mut W {
        self.variant(P2_13MODE_A::REPEATER)
    }
    #[doc = "Disabled. P2.13 pin has neither pull-up nor pull-down."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(P2_13MODE_A::DISABLED)
    }
    #[doc = "Pull-down. P2.13 has a pull-down resistor enabled."]
    #[inline(always)]
    pub fn pull_down(self) -> &'a mut W {
        self.variant(P2_13MODE_A::PULL_DOWN)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 26)) | (((value as u32) & 0x03) << 26);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - Port 2 pin 0 control."]
    #[inline(always)]
    pub fn p2_00mode(&self) -> P2_00MODE_R {
        P2_00MODE_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 2:3 - Port 2 pin 1 control."]
    #[inline(always)]
    pub fn p2_01mode(&self) -> P2_01MODE_R {
        P2_01MODE_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bits 4:5 - Port 2 pin 2 control."]
    #[inline(always)]
    pub fn p2_02mode(&self) -> P2_02MODE_R {
        P2_02MODE_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bits 6:7 - Port 2 pin 3 control."]
    #[inline(always)]
    pub fn p2_03mode(&self) -> P2_03MODE_R {
        P2_03MODE_R::new(((self.bits >> 6) & 0x03) as u8)
    }
    #[doc = "Bits 8:9 - Port 2 pin 4 control."]
    #[inline(always)]
    pub fn p2_04mode(&self) -> P2_04MODE_R {
        P2_04MODE_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bits 10:11 - Port 2 pin 5 control."]
    #[inline(always)]
    pub fn p2_05mode(&self) -> P2_05MODE_R {
        P2_05MODE_R::new(((self.bits >> 10) & 0x03) as u8)
    }
    #[doc = "Bits 12:13 - Port 2 pin 6 control."]
    #[inline(always)]
    pub fn p2_06mode(&self) -> P2_06MODE_R {
        P2_06MODE_R::new(((self.bits >> 12) & 0x03) as u8)
    }
    #[doc = "Bits 14:15 - Port 2 pin 7 control."]
    #[inline(always)]
    pub fn p2_07mode(&self) -> P2_07MODE_R {
        P2_07MODE_R::new(((self.bits >> 14) & 0x03) as u8)
    }
    #[doc = "Bits 16:17 - Port 2 pin 8 control."]
    #[inline(always)]
    pub fn p2_08mode(&self) -> P2_08MODE_R {
        P2_08MODE_R::new(((self.bits >> 16) & 0x03) as u8)
    }
    #[doc = "Bits 18:19 - Port 2 pin 9 control."]
    #[inline(always)]
    pub fn p2_09mode(&self) -> P2_09MODE_R {
        P2_09MODE_R::new(((self.bits >> 18) & 0x03) as u8)
    }
    #[doc = "Bits 20:21 - Port 2 pin 10 control."]
    #[inline(always)]
    pub fn p2_10mode(&self) -> P2_10MODE_R {
        P2_10MODE_R::new(((self.bits >> 20) & 0x03) as u8)
    }
    #[doc = "Bits 22:23 - Port 2 pin 11 control."]
    #[inline(always)]
    pub fn p2_11mode(&self) -> P2_11MODE_R {
        P2_11MODE_R::new(((self.bits >> 22) & 0x03) as u8)
    }
    #[doc = "Bits 24:25 - Port 2 pin 12 control."]
    #[inline(always)]
    pub fn p2_12mode(&self) -> P2_12MODE_R {
        P2_12MODE_R::new(((self.bits >> 24) & 0x03) as u8)
    }
    #[doc = "Bits 26:27 - Port 2 pin 13 control."]
    #[inline(always)]
    pub fn p2_13mode(&self) -> P2_13MODE_R {
        P2_13MODE_R::new(((self.bits >> 26) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Port 2 pin 0 control."]
    #[inline(always)]
    pub fn p2_00mode(&mut self) -> P2_00MODE_W {
        P2_00MODE_W { w: self }
    }
    #[doc = "Bits 2:3 - Port 2 pin 1 control."]
    #[inline(always)]
    pub fn p2_01mode(&mut self) -> P2_01MODE_W {
        P2_01MODE_W { w: self }
    }
    #[doc = "Bits 4:5 - Port 2 pin 2 control."]
    #[inline(always)]
    pub fn p2_02mode(&mut self) -> P2_02MODE_W {
        P2_02MODE_W { w: self }
    }
    #[doc = "Bits 6:7 - Port 2 pin 3 control."]
    #[inline(always)]
    pub fn p2_03mode(&mut self) -> P2_03MODE_W {
        P2_03MODE_W { w: self }
    }
    #[doc = "Bits 8:9 - Port 2 pin 4 control."]
    #[inline(always)]
    pub fn p2_04mode(&mut self) -> P2_04MODE_W {
        P2_04MODE_W { w: self }
    }
    #[doc = "Bits 10:11 - Port 2 pin 5 control."]
    #[inline(always)]
    pub fn p2_05mode(&mut self) -> P2_05MODE_W {
        P2_05MODE_W { w: self }
    }
    #[doc = "Bits 12:13 - Port 2 pin 6 control."]
    #[inline(always)]
    pub fn p2_06mode(&mut self) -> P2_06MODE_W {
        P2_06MODE_W { w: self }
    }
    #[doc = "Bits 14:15 - Port 2 pin 7 control."]
    #[inline(always)]
    pub fn p2_07mode(&mut self) -> P2_07MODE_W {
        P2_07MODE_W { w: self }
    }
    #[doc = "Bits 16:17 - Port 2 pin 8 control."]
    #[inline(always)]
    pub fn p2_08mode(&mut self) -> P2_08MODE_W {
        P2_08MODE_W { w: self }
    }
    #[doc = "Bits 18:19 - Port 2 pin 9 control."]
    #[inline(always)]
    pub fn p2_09mode(&mut self) -> P2_09MODE_W {
        P2_09MODE_W { w: self }
    }
    #[doc = "Bits 20:21 - Port 2 pin 10 control."]
    #[inline(always)]
    pub fn p2_10mode(&mut self) -> P2_10MODE_W {
        P2_10MODE_W { w: self }
    }
    #[doc = "Bits 22:23 - Port 2 pin 11 control."]
    #[inline(always)]
    pub fn p2_11mode(&mut self) -> P2_11MODE_W {
        P2_11MODE_W { w: self }
    }
    #[doc = "Bits 24:25 - Port 2 pin 12 control."]
    #[inline(always)]
    pub fn p2_12mode(&mut self) -> P2_12MODE_W {
        P2_12MODE_W { w: self }
    }
    #[doc = "Bits 26:27 - Port 2 pin 13 control."]
    #[inline(always)]
    pub fn p2_13mode(&mut self) -> P2_13MODE_W {
        P2_13MODE_W { w: self }
    }
}
