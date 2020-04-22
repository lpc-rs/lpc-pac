#[doc = "Reader of register PINMODE1"]
pub type R = crate::R<u32, super::PINMODE1>;
#[doc = "Writer for register PINMODE1"]
pub type W = crate::W<u32, super::PINMODE1>;
#[doc = "Register PINMODE1 `reset()`'s with value 0"]
impl crate::ResetValue for super::PINMODE1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Port 1 pin 16 control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum P0_16MODE_A {
    #[doc = "0: Pull-up. P0.16 pin has a pull-up resistor enabled."]
    PULL_UP = 0,
    #[doc = "1: Repeater. P0.16 pin has repeater mode enabled."]
    REPEATER = 1,
    #[doc = "2: Disabled. P0.16 pin has neither pull-up nor pull-down."]
    DISABLED = 2,
    #[doc = "3: Pull-down. P0.16 has a pull-down resistor enabled."]
    PULL_DOWN = 3,
}
impl From<P0_16MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: P0_16MODE_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `P0_16MODE`"]
pub type P0_16MODE_R = crate::R<u8, P0_16MODE_A>;
impl P0_16MODE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> P0_16MODE_A {
        match self.bits {
            0 => P0_16MODE_A::PULL_UP,
            1 => P0_16MODE_A::REPEATER,
            2 => P0_16MODE_A::DISABLED,
            3 => P0_16MODE_A::PULL_DOWN,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PULL_UP`"]
    #[inline(always)]
    pub fn is_pull_up(&self) -> bool {
        *self == P0_16MODE_A::PULL_UP
    }
    #[doc = "Checks if the value of the field is `REPEATER`"]
    #[inline(always)]
    pub fn is_repeater(&self) -> bool {
        *self == P0_16MODE_A::REPEATER
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == P0_16MODE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `PULL_DOWN`"]
    #[inline(always)]
    pub fn is_pull_down(&self) -> bool {
        *self == P0_16MODE_A::PULL_DOWN
    }
}
#[doc = "Write proxy for field `P0_16MODE`"]
pub struct P0_16MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> P0_16MODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: P0_16MODE_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Pull-up. P0.16 pin has a pull-up resistor enabled."]
    #[inline(always)]
    pub fn pull_up(self) -> &'a mut W {
        self.variant(P0_16MODE_A::PULL_UP)
    }
    #[doc = "Repeater. P0.16 pin has repeater mode enabled."]
    #[inline(always)]
    pub fn repeater(self) -> &'a mut W {
        self.variant(P0_16MODE_A::REPEATER)
    }
    #[doc = "Disabled. P0.16 pin has neither pull-up nor pull-down."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(P0_16MODE_A::DISABLED)
    }
    #[doc = "Pull-down. P0.16 has a pull-down resistor enabled."]
    #[inline(always)]
    pub fn pull_down(self) -> &'a mut W {
        self.variant(P0_16MODE_A::PULL_DOWN)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
#[doc = "Port 1 pin 17 control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum P0_17MODE_A {
    #[doc = "0: Pull-up. P0.17 pin has a pull-up resistor enabled."]
    PULL_UP = 0,
    #[doc = "1: Repeater. P0.17 pin has repeater mode enabled."]
    REPEATER = 1,
    #[doc = "2: Disabled. P0.17 pin has neither pull-up nor pull-down."]
    DISABLED = 2,
    #[doc = "3: Pull-down. P0.17 has a pull-down resistor enabled."]
    PULL_DOWN = 3,
}
impl From<P0_17MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: P0_17MODE_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `P0_17MODE`"]
pub type P0_17MODE_R = crate::R<u8, P0_17MODE_A>;
impl P0_17MODE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> P0_17MODE_A {
        match self.bits {
            0 => P0_17MODE_A::PULL_UP,
            1 => P0_17MODE_A::REPEATER,
            2 => P0_17MODE_A::DISABLED,
            3 => P0_17MODE_A::PULL_DOWN,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PULL_UP`"]
    #[inline(always)]
    pub fn is_pull_up(&self) -> bool {
        *self == P0_17MODE_A::PULL_UP
    }
    #[doc = "Checks if the value of the field is `REPEATER`"]
    #[inline(always)]
    pub fn is_repeater(&self) -> bool {
        *self == P0_17MODE_A::REPEATER
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == P0_17MODE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `PULL_DOWN`"]
    #[inline(always)]
    pub fn is_pull_down(&self) -> bool {
        *self == P0_17MODE_A::PULL_DOWN
    }
}
#[doc = "Write proxy for field `P0_17MODE`"]
pub struct P0_17MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> P0_17MODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: P0_17MODE_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Pull-up. P0.17 pin has a pull-up resistor enabled."]
    #[inline(always)]
    pub fn pull_up(self) -> &'a mut W {
        self.variant(P0_17MODE_A::PULL_UP)
    }
    #[doc = "Repeater. P0.17 pin has repeater mode enabled."]
    #[inline(always)]
    pub fn repeater(self) -> &'a mut W {
        self.variant(P0_17MODE_A::REPEATER)
    }
    #[doc = "Disabled. P0.17 pin has neither pull-up nor pull-down."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(P0_17MODE_A::DISABLED)
    }
    #[doc = "Pull-down. P0.17 has a pull-down resistor enabled."]
    #[inline(always)]
    pub fn pull_down(self) -> &'a mut W {
        self.variant(P0_17MODE_A::PULL_DOWN)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | (((value as u32) & 0x03) << 2);
        self.w
    }
}
#[doc = "Port 1 pin 18 control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum P0_18MODE_A {
    #[doc = "0: Pull-up. P0.18 pin has a pull-up resistor enabled."]
    PULL_UP = 0,
    #[doc = "1: Repeater. P0.18 pin has repeater mode enabled."]
    REPEATER = 1,
    #[doc = "2: Disabled. P0.18 pin has neither pull-up nor pull-down."]
    DISABLED = 2,
    #[doc = "3: Pull-down. P0.18 has a pull-down resistor enabled."]
    PULL_DOWN = 3,
}
impl From<P0_18MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: P0_18MODE_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `P0_18MODE`"]
pub type P0_18MODE_R = crate::R<u8, P0_18MODE_A>;
impl P0_18MODE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> P0_18MODE_A {
        match self.bits {
            0 => P0_18MODE_A::PULL_UP,
            1 => P0_18MODE_A::REPEATER,
            2 => P0_18MODE_A::DISABLED,
            3 => P0_18MODE_A::PULL_DOWN,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PULL_UP`"]
    #[inline(always)]
    pub fn is_pull_up(&self) -> bool {
        *self == P0_18MODE_A::PULL_UP
    }
    #[doc = "Checks if the value of the field is `REPEATER`"]
    #[inline(always)]
    pub fn is_repeater(&self) -> bool {
        *self == P0_18MODE_A::REPEATER
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == P0_18MODE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `PULL_DOWN`"]
    #[inline(always)]
    pub fn is_pull_down(&self) -> bool {
        *self == P0_18MODE_A::PULL_DOWN
    }
}
#[doc = "Write proxy for field `P0_18MODE`"]
pub struct P0_18MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> P0_18MODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: P0_18MODE_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Pull-up. P0.18 pin has a pull-up resistor enabled."]
    #[inline(always)]
    pub fn pull_up(self) -> &'a mut W {
        self.variant(P0_18MODE_A::PULL_UP)
    }
    #[doc = "Repeater. P0.18 pin has repeater mode enabled."]
    #[inline(always)]
    pub fn repeater(self) -> &'a mut W {
        self.variant(P0_18MODE_A::REPEATER)
    }
    #[doc = "Disabled. P0.18 pin has neither pull-up nor pull-down."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(P0_18MODE_A::DISABLED)
    }
    #[doc = "Pull-down. P0.18 has a pull-down resistor enabled."]
    #[inline(always)]
    pub fn pull_down(self) -> &'a mut W {
        self.variant(P0_18MODE_A::PULL_DOWN)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | (((value as u32) & 0x03) << 4);
        self.w
    }
}
#[doc = "Port 1 pin 19 control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum P0_19MODE_A {
    #[doc = "0: Pull-up. P0.19 pin has a pull-up resistor enabled."]
    PULL_UP = 0,
    #[doc = "1: Repeater. P0.19 pin has repeater mode enabled."]
    REPEATER = 1,
    #[doc = "2: Disabled. P0.19 pin has neither pull-up nor pull-down."]
    DISABLED = 2,
    #[doc = "3: Pull-down. P0.19 has a pull-down resistor enabled."]
    PULL_DOWN = 3,
}
impl From<P0_19MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: P0_19MODE_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `P0_19MODE`"]
pub type P0_19MODE_R = crate::R<u8, P0_19MODE_A>;
impl P0_19MODE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> P0_19MODE_A {
        match self.bits {
            0 => P0_19MODE_A::PULL_UP,
            1 => P0_19MODE_A::REPEATER,
            2 => P0_19MODE_A::DISABLED,
            3 => P0_19MODE_A::PULL_DOWN,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PULL_UP`"]
    #[inline(always)]
    pub fn is_pull_up(&self) -> bool {
        *self == P0_19MODE_A::PULL_UP
    }
    #[doc = "Checks if the value of the field is `REPEATER`"]
    #[inline(always)]
    pub fn is_repeater(&self) -> bool {
        *self == P0_19MODE_A::REPEATER
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == P0_19MODE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `PULL_DOWN`"]
    #[inline(always)]
    pub fn is_pull_down(&self) -> bool {
        *self == P0_19MODE_A::PULL_DOWN
    }
}
#[doc = "Write proxy for field `P0_19MODE`"]
pub struct P0_19MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> P0_19MODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: P0_19MODE_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Pull-up. P0.19 pin has a pull-up resistor enabled."]
    #[inline(always)]
    pub fn pull_up(self) -> &'a mut W {
        self.variant(P0_19MODE_A::PULL_UP)
    }
    #[doc = "Repeater. P0.19 pin has repeater mode enabled."]
    #[inline(always)]
    pub fn repeater(self) -> &'a mut W {
        self.variant(P0_19MODE_A::REPEATER)
    }
    #[doc = "Disabled. P0.19 pin has neither pull-up nor pull-down."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(P0_19MODE_A::DISABLED)
    }
    #[doc = "Pull-down. P0.19 has a pull-down resistor enabled."]
    #[inline(always)]
    pub fn pull_down(self) -> &'a mut W {
        self.variant(P0_19MODE_A::PULL_DOWN)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | (((value as u32) & 0x03) << 6);
        self.w
    }
}
#[doc = "Port 1 pin 20 control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum P0_20MODE_A {
    #[doc = "0: Pull-up. P0.20 pin has a pull-up resistor enabled."]
    PULL_UP = 0,
    #[doc = "1: Repeater. P0.20 pin has repeater mode enabled."]
    REPEATER = 1,
    #[doc = "2: Disabled. P0.20 pin has neither pull-up nor pull-down."]
    DISABLED = 2,
    #[doc = "3: Pull-down. P0.20 has a pull-down resistor enabled."]
    PULL_DOWN = 3,
}
impl From<P0_20MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: P0_20MODE_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `P0_20MODE`"]
pub type P0_20MODE_R = crate::R<u8, P0_20MODE_A>;
impl P0_20MODE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> P0_20MODE_A {
        match self.bits {
            0 => P0_20MODE_A::PULL_UP,
            1 => P0_20MODE_A::REPEATER,
            2 => P0_20MODE_A::DISABLED,
            3 => P0_20MODE_A::PULL_DOWN,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PULL_UP`"]
    #[inline(always)]
    pub fn is_pull_up(&self) -> bool {
        *self == P0_20MODE_A::PULL_UP
    }
    #[doc = "Checks if the value of the field is `REPEATER`"]
    #[inline(always)]
    pub fn is_repeater(&self) -> bool {
        *self == P0_20MODE_A::REPEATER
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == P0_20MODE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `PULL_DOWN`"]
    #[inline(always)]
    pub fn is_pull_down(&self) -> bool {
        *self == P0_20MODE_A::PULL_DOWN
    }
}
#[doc = "Write proxy for field `P0_20MODE`"]
pub struct P0_20MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> P0_20MODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: P0_20MODE_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Pull-up. P0.20 pin has a pull-up resistor enabled."]
    #[inline(always)]
    pub fn pull_up(self) -> &'a mut W {
        self.variant(P0_20MODE_A::PULL_UP)
    }
    #[doc = "Repeater. P0.20 pin has repeater mode enabled."]
    #[inline(always)]
    pub fn repeater(self) -> &'a mut W {
        self.variant(P0_20MODE_A::REPEATER)
    }
    #[doc = "Disabled. P0.20 pin has neither pull-up nor pull-down."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(P0_20MODE_A::DISABLED)
    }
    #[doc = "Pull-down. P0.20 has a pull-down resistor enabled."]
    #[inline(always)]
    pub fn pull_down(self) -> &'a mut W {
        self.variant(P0_20MODE_A::PULL_DOWN)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | (((value as u32) & 0x03) << 8);
        self.w
    }
}
#[doc = "Port 1 pin 21 control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum P0_21MODE_A {
    #[doc = "0: Pull-up. P0.21 pin has a pull-up resistor enabled."]
    PULL_UP = 0,
    #[doc = "1: Repeater. P0.21 pin has repeater mode enabled."]
    REPEATER = 1,
    #[doc = "2: Disabled. P0.21 pin has neither pull-up nor pull-down."]
    DISABLED = 2,
    #[doc = "3: Pull-down. P0.21 has a pull-down resistor enabled."]
    PULL_DOWN = 3,
}
impl From<P0_21MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: P0_21MODE_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `P0_21MODE`"]
pub type P0_21MODE_R = crate::R<u8, P0_21MODE_A>;
impl P0_21MODE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> P0_21MODE_A {
        match self.bits {
            0 => P0_21MODE_A::PULL_UP,
            1 => P0_21MODE_A::REPEATER,
            2 => P0_21MODE_A::DISABLED,
            3 => P0_21MODE_A::PULL_DOWN,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PULL_UP`"]
    #[inline(always)]
    pub fn is_pull_up(&self) -> bool {
        *self == P0_21MODE_A::PULL_UP
    }
    #[doc = "Checks if the value of the field is `REPEATER`"]
    #[inline(always)]
    pub fn is_repeater(&self) -> bool {
        *self == P0_21MODE_A::REPEATER
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == P0_21MODE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `PULL_DOWN`"]
    #[inline(always)]
    pub fn is_pull_down(&self) -> bool {
        *self == P0_21MODE_A::PULL_DOWN
    }
}
#[doc = "Write proxy for field `P0_21MODE`"]
pub struct P0_21MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> P0_21MODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: P0_21MODE_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Pull-up. P0.21 pin has a pull-up resistor enabled."]
    #[inline(always)]
    pub fn pull_up(self) -> &'a mut W {
        self.variant(P0_21MODE_A::PULL_UP)
    }
    #[doc = "Repeater. P0.21 pin has repeater mode enabled."]
    #[inline(always)]
    pub fn repeater(self) -> &'a mut W {
        self.variant(P0_21MODE_A::REPEATER)
    }
    #[doc = "Disabled. P0.21 pin has neither pull-up nor pull-down."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(P0_21MODE_A::DISABLED)
    }
    #[doc = "Pull-down. P0.21 has a pull-down resistor enabled."]
    #[inline(always)]
    pub fn pull_down(self) -> &'a mut W {
        self.variant(P0_21MODE_A::PULL_DOWN)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 10)) | (((value as u32) & 0x03) << 10);
        self.w
    }
}
#[doc = "Port 1 pin 22 control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum P0_22MODE_A {
    #[doc = "0: Pull-up. P0.22 pin has a pull-up resistor enabled."]
    PULL_UP = 0,
    #[doc = "1: Repeater. P0.22 pin has repeater mode enabled."]
    REPEATER = 1,
    #[doc = "2: Disabled. P0.22 pin has neither pull-up nor pull-down."]
    DISABLED = 2,
    #[doc = "3: Pull-down. P0.22 has a pull-down resistor enabled."]
    PULL_DOWN = 3,
}
impl From<P0_22MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: P0_22MODE_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `P0_22MODE`"]
pub type P0_22MODE_R = crate::R<u8, P0_22MODE_A>;
impl P0_22MODE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> P0_22MODE_A {
        match self.bits {
            0 => P0_22MODE_A::PULL_UP,
            1 => P0_22MODE_A::REPEATER,
            2 => P0_22MODE_A::DISABLED,
            3 => P0_22MODE_A::PULL_DOWN,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PULL_UP`"]
    #[inline(always)]
    pub fn is_pull_up(&self) -> bool {
        *self == P0_22MODE_A::PULL_UP
    }
    #[doc = "Checks if the value of the field is `REPEATER`"]
    #[inline(always)]
    pub fn is_repeater(&self) -> bool {
        *self == P0_22MODE_A::REPEATER
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == P0_22MODE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `PULL_DOWN`"]
    #[inline(always)]
    pub fn is_pull_down(&self) -> bool {
        *self == P0_22MODE_A::PULL_DOWN
    }
}
#[doc = "Write proxy for field `P0_22MODE`"]
pub struct P0_22MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> P0_22MODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: P0_22MODE_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Pull-up. P0.22 pin has a pull-up resistor enabled."]
    #[inline(always)]
    pub fn pull_up(self) -> &'a mut W {
        self.variant(P0_22MODE_A::PULL_UP)
    }
    #[doc = "Repeater. P0.22 pin has repeater mode enabled."]
    #[inline(always)]
    pub fn repeater(self) -> &'a mut W {
        self.variant(P0_22MODE_A::REPEATER)
    }
    #[doc = "Disabled. P0.22 pin has neither pull-up nor pull-down."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(P0_22MODE_A::DISABLED)
    }
    #[doc = "Pull-down. P0.22 has a pull-down resistor enabled."]
    #[inline(always)]
    pub fn pull_down(self) -> &'a mut W {
        self.variant(P0_22MODE_A::PULL_DOWN)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 12)) | (((value as u32) & 0x03) << 12);
        self.w
    }
}
#[doc = "Port 1 pin 23 control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum P0_23MODE_A {
    #[doc = "0: Pull-up. P0.23 pin has a pull-up resistor enabled."]
    PULL_UP = 0,
    #[doc = "1: Repeater. P0.23 pin has repeater mode enabled."]
    REPEATER = 1,
    #[doc = "2: Disabled. P0.23 pin has neither pull-up nor pull-down."]
    DISABLED = 2,
    #[doc = "3: Pull-down. P0.23 has a pull-down resistor enabled."]
    PULL_DOWN = 3,
}
impl From<P0_23MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: P0_23MODE_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `P0_23MODE`"]
pub type P0_23MODE_R = crate::R<u8, P0_23MODE_A>;
impl P0_23MODE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> P0_23MODE_A {
        match self.bits {
            0 => P0_23MODE_A::PULL_UP,
            1 => P0_23MODE_A::REPEATER,
            2 => P0_23MODE_A::DISABLED,
            3 => P0_23MODE_A::PULL_DOWN,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PULL_UP`"]
    #[inline(always)]
    pub fn is_pull_up(&self) -> bool {
        *self == P0_23MODE_A::PULL_UP
    }
    #[doc = "Checks if the value of the field is `REPEATER`"]
    #[inline(always)]
    pub fn is_repeater(&self) -> bool {
        *self == P0_23MODE_A::REPEATER
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == P0_23MODE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `PULL_DOWN`"]
    #[inline(always)]
    pub fn is_pull_down(&self) -> bool {
        *self == P0_23MODE_A::PULL_DOWN
    }
}
#[doc = "Write proxy for field `P0_23MODE`"]
pub struct P0_23MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> P0_23MODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: P0_23MODE_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Pull-up. P0.23 pin has a pull-up resistor enabled."]
    #[inline(always)]
    pub fn pull_up(self) -> &'a mut W {
        self.variant(P0_23MODE_A::PULL_UP)
    }
    #[doc = "Repeater. P0.23 pin has repeater mode enabled."]
    #[inline(always)]
    pub fn repeater(self) -> &'a mut W {
        self.variant(P0_23MODE_A::REPEATER)
    }
    #[doc = "Disabled. P0.23 pin has neither pull-up nor pull-down."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(P0_23MODE_A::DISABLED)
    }
    #[doc = "Pull-down. P0.23 has a pull-down resistor enabled."]
    #[inline(always)]
    pub fn pull_down(self) -> &'a mut W {
        self.variant(P0_23MODE_A::PULL_DOWN)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 14)) | (((value as u32) & 0x03) << 14);
        self.w
    }
}
#[doc = "Port 1 pin 24 control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum P0_24MODE_A {
    #[doc = "0: Pull-up. P0.24 pin has a pull-up resistor enabled."]
    PULL_UP = 0,
    #[doc = "1: Repeater. P0.24 pin has repeater mode enabled."]
    REPEATER = 1,
    #[doc = "2: Disabled. P0.24 pin has neither pull-up nor pull-down."]
    DISABLED = 2,
    #[doc = "3: Pull-down. P0.24 has a pull-down resistor enabled."]
    PULL_DOWN = 3,
}
impl From<P0_24MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: P0_24MODE_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `P0_24MODE`"]
pub type P0_24MODE_R = crate::R<u8, P0_24MODE_A>;
impl P0_24MODE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> P0_24MODE_A {
        match self.bits {
            0 => P0_24MODE_A::PULL_UP,
            1 => P0_24MODE_A::REPEATER,
            2 => P0_24MODE_A::DISABLED,
            3 => P0_24MODE_A::PULL_DOWN,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PULL_UP`"]
    #[inline(always)]
    pub fn is_pull_up(&self) -> bool {
        *self == P0_24MODE_A::PULL_UP
    }
    #[doc = "Checks if the value of the field is `REPEATER`"]
    #[inline(always)]
    pub fn is_repeater(&self) -> bool {
        *self == P0_24MODE_A::REPEATER
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == P0_24MODE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `PULL_DOWN`"]
    #[inline(always)]
    pub fn is_pull_down(&self) -> bool {
        *self == P0_24MODE_A::PULL_DOWN
    }
}
#[doc = "Write proxy for field `P0_24MODE`"]
pub struct P0_24MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> P0_24MODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: P0_24MODE_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Pull-up. P0.24 pin has a pull-up resistor enabled."]
    #[inline(always)]
    pub fn pull_up(self) -> &'a mut W {
        self.variant(P0_24MODE_A::PULL_UP)
    }
    #[doc = "Repeater. P0.24 pin has repeater mode enabled."]
    #[inline(always)]
    pub fn repeater(self) -> &'a mut W {
        self.variant(P0_24MODE_A::REPEATER)
    }
    #[doc = "Disabled. P0.24 pin has neither pull-up nor pull-down."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(P0_24MODE_A::DISABLED)
    }
    #[doc = "Pull-down. P0.24 has a pull-down resistor enabled."]
    #[inline(always)]
    pub fn pull_down(self) -> &'a mut W {
        self.variant(P0_24MODE_A::PULL_DOWN)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | (((value as u32) & 0x03) << 16);
        self.w
    }
}
#[doc = "Port 1 pin 25 control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum P0_25MODE_A {
    #[doc = "0: Pull-up. P0.25 pin has a pull-up resistor enabled."]
    PULL_UP = 0,
    #[doc = "1: Repeater. P0.25 pin has repeater mode enabled."]
    REPEATER = 1,
    #[doc = "2: Disabled. P0.25 pin has neither pull-up nor pull-down."]
    DISABLED = 2,
    #[doc = "3: Pull-down. P0.25 has a pull-down resistor enabled."]
    PULL_DOWN = 3,
}
impl From<P0_25MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: P0_25MODE_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `P0_25MODE`"]
pub type P0_25MODE_R = crate::R<u8, P0_25MODE_A>;
impl P0_25MODE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> P0_25MODE_A {
        match self.bits {
            0 => P0_25MODE_A::PULL_UP,
            1 => P0_25MODE_A::REPEATER,
            2 => P0_25MODE_A::DISABLED,
            3 => P0_25MODE_A::PULL_DOWN,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PULL_UP`"]
    #[inline(always)]
    pub fn is_pull_up(&self) -> bool {
        *self == P0_25MODE_A::PULL_UP
    }
    #[doc = "Checks if the value of the field is `REPEATER`"]
    #[inline(always)]
    pub fn is_repeater(&self) -> bool {
        *self == P0_25MODE_A::REPEATER
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == P0_25MODE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `PULL_DOWN`"]
    #[inline(always)]
    pub fn is_pull_down(&self) -> bool {
        *self == P0_25MODE_A::PULL_DOWN
    }
}
#[doc = "Write proxy for field `P0_25MODE`"]
pub struct P0_25MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> P0_25MODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: P0_25MODE_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Pull-up. P0.25 pin has a pull-up resistor enabled."]
    #[inline(always)]
    pub fn pull_up(self) -> &'a mut W {
        self.variant(P0_25MODE_A::PULL_UP)
    }
    #[doc = "Repeater. P0.25 pin has repeater mode enabled."]
    #[inline(always)]
    pub fn repeater(self) -> &'a mut W {
        self.variant(P0_25MODE_A::REPEATER)
    }
    #[doc = "Disabled. P0.25 pin has neither pull-up nor pull-down."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(P0_25MODE_A::DISABLED)
    }
    #[doc = "Pull-down. P0.25 has a pull-down resistor enabled."]
    #[inline(always)]
    pub fn pull_down(self) -> &'a mut W {
        self.variant(P0_25MODE_A::PULL_DOWN)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 18)) | (((value as u32) & 0x03) << 18);
        self.w
    }
}
#[doc = "Port 1 pin 26 control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum P0_26MODE_A {
    #[doc = "0: Pull-up. P0.26 pin has a pull-up resistor enabled."]
    PULL_UP = 0,
    #[doc = "1: Repeater. P0.26 pin has repeater mode enabled."]
    REPEATER = 1,
    #[doc = "2: Disabled. P0.26 pin has neither pull-up nor pull-down."]
    DISABLED = 2,
    #[doc = "3: Pull-down. P0.26 has a pull-down resistor enabled."]
    PULL_DOWN = 3,
}
impl From<P0_26MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: P0_26MODE_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `P0_26MODE`"]
pub type P0_26MODE_R = crate::R<u8, P0_26MODE_A>;
impl P0_26MODE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> P0_26MODE_A {
        match self.bits {
            0 => P0_26MODE_A::PULL_UP,
            1 => P0_26MODE_A::REPEATER,
            2 => P0_26MODE_A::DISABLED,
            3 => P0_26MODE_A::PULL_DOWN,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PULL_UP`"]
    #[inline(always)]
    pub fn is_pull_up(&self) -> bool {
        *self == P0_26MODE_A::PULL_UP
    }
    #[doc = "Checks if the value of the field is `REPEATER`"]
    #[inline(always)]
    pub fn is_repeater(&self) -> bool {
        *self == P0_26MODE_A::REPEATER
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == P0_26MODE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `PULL_DOWN`"]
    #[inline(always)]
    pub fn is_pull_down(&self) -> bool {
        *self == P0_26MODE_A::PULL_DOWN
    }
}
#[doc = "Write proxy for field `P0_26MODE`"]
pub struct P0_26MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> P0_26MODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: P0_26MODE_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Pull-up. P0.26 pin has a pull-up resistor enabled."]
    #[inline(always)]
    pub fn pull_up(self) -> &'a mut W {
        self.variant(P0_26MODE_A::PULL_UP)
    }
    #[doc = "Repeater. P0.26 pin has repeater mode enabled."]
    #[inline(always)]
    pub fn repeater(self) -> &'a mut W {
        self.variant(P0_26MODE_A::REPEATER)
    }
    #[doc = "Disabled. P0.26 pin has neither pull-up nor pull-down."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(P0_26MODE_A::DISABLED)
    }
    #[doc = "Pull-down. P0.26 has a pull-down resistor enabled."]
    #[inline(always)]
    pub fn pull_down(self) -> &'a mut W {
        self.variant(P0_26MODE_A::PULL_DOWN)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 20)) | (((value as u32) & 0x03) << 20);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - Port 1 pin 16 control."]
    #[inline(always)]
    pub fn p0_16mode(&self) -> P0_16MODE_R {
        P0_16MODE_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 2:3 - Port 1 pin 17 control."]
    #[inline(always)]
    pub fn p0_17mode(&self) -> P0_17MODE_R {
        P0_17MODE_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bits 4:5 - Port 1 pin 18 control."]
    #[inline(always)]
    pub fn p0_18mode(&self) -> P0_18MODE_R {
        P0_18MODE_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bits 6:7 - Port 1 pin 19 control."]
    #[inline(always)]
    pub fn p0_19mode(&self) -> P0_19MODE_R {
        P0_19MODE_R::new(((self.bits >> 6) & 0x03) as u8)
    }
    #[doc = "Bits 8:9 - Port 1 pin 20 control."]
    #[inline(always)]
    pub fn p0_20mode(&self) -> P0_20MODE_R {
        P0_20MODE_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bits 10:11 - Port 1 pin 21 control."]
    #[inline(always)]
    pub fn p0_21mode(&self) -> P0_21MODE_R {
        P0_21MODE_R::new(((self.bits >> 10) & 0x03) as u8)
    }
    #[doc = "Bits 12:13 - Port 1 pin 22 control."]
    #[inline(always)]
    pub fn p0_22mode(&self) -> P0_22MODE_R {
        P0_22MODE_R::new(((self.bits >> 12) & 0x03) as u8)
    }
    #[doc = "Bits 14:15 - Port 1 pin 23 control."]
    #[inline(always)]
    pub fn p0_23mode(&self) -> P0_23MODE_R {
        P0_23MODE_R::new(((self.bits >> 14) & 0x03) as u8)
    }
    #[doc = "Bits 16:17 - Port 1 pin 24 control."]
    #[inline(always)]
    pub fn p0_24mode(&self) -> P0_24MODE_R {
        P0_24MODE_R::new(((self.bits >> 16) & 0x03) as u8)
    }
    #[doc = "Bits 18:19 - Port 1 pin 25 control."]
    #[inline(always)]
    pub fn p0_25mode(&self) -> P0_25MODE_R {
        P0_25MODE_R::new(((self.bits >> 18) & 0x03) as u8)
    }
    #[doc = "Bits 20:21 - Port 1 pin 26 control."]
    #[inline(always)]
    pub fn p0_26mode(&self) -> P0_26MODE_R {
        P0_26MODE_R::new(((self.bits >> 20) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Port 1 pin 16 control."]
    #[inline(always)]
    pub fn p0_16mode(&mut self) -> P0_16MODE_W {
        P0_16MODE_W { w: self }
    }
    #[doc = "Bits 2:3 - Port 1 pin 17 control."]
    #[inline(always)]
    pub fn p0_17mode(&mut self) -> P0_17MODE_W {
        P0_17MODE_W { w: self }
    }
    #[doc = "Bits 4:5 - Port 1 pin 18 control."]
    #[inline(always)]
    pub fn p0_18mode(&mut self) -> P0_18MODE_W {
        P0_18MODE_W { w: self }
    }
    #[doc = "Bits 6:7 - Port 1 pin 19 control."]
    #[inline(always)]
    pub fn p0_19mode(&mut self) -> P0_19MODE_W {
        P0_19MODE_W { w: self }
    }
    #[doc = "Bits 8:9 - Port 1 pin 20 control."]
    #[inline(always)]
    pub fn p0_20mode(&mut self) -> P0_20MODE_W {
        P0_20MODE_W { w: self }
    }
    #[doc = "Bits 10:11 - Port 1 pin 21 control."]
    #[inline(always)]
    pub fn p0_21mode(&mut self) -> P0_21MODE_W {
        P0_21MODE_W { w: self }
    }
    #[doc = "Bits 12:13 - Port 1 pin 22 control."]
    #[inline(always)]
    pub fn p0_22mode(&mut self) -> P0_22MODE_W {
        P0_22MODE_W { w: self }
    }
    #[doc = "Bits 14:15 - Port 1 pin 23 control."]
    #[inline(always)]
    pub fn p0_23mode(&mut self) -> P0_23MODE_W {
        P0_23MODE_W { w: self }
    }
    #[doc = "Bits 16:17 - Port 1 pin 24 control."]
    #[inline(always)]
    pub fn p0_24mode(&mut self) -> P0_24MODE_W {
        P0_24MODE_W { w: self }
    }
    #[doc = "Bits 18:19 - Port 1 pin 25 control."]
    #[inline(always)]
    pub fn p0_25mode(&mut self) -> P0_25MODE_W {
        P0_25MODE_W { w: self }
    }
    #[doc = "Bits 20:21 - Port 1 pin 26 control."]
    #[inline(always)]
    pub fn p0_26mode(&mut self) -> P0_26MODE_W {
        P0_26MODE_W { w: self }
    }
}
