#[doc = "Reader of register PINMODE3"]
pub type R = crate::R<u32, super::PINMODE3>;
#[doc = "Writer for register PINMODE3"]
pub type W = crate::W<u32, super::PINMODE3>;
#[doc = "Register PINMODE3 `reset()`'s with value 0"]
impl crate::ResetValue for super::PINMODE3 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Port 1 pin 16 control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum P1_16MODE_A {
    #[doc = "0: Pull-up. P1.16 pin has a pull-up resistor enabled."]
    PULL_UP = 0,
    #[doc = "1: Repeater. P1.16 pin has repeater mode enabled."]
    REPEATER = 1,
    #[doc = "2: Disabled. P1.16 pin has neither pull-up nor pull-down."]
    DISABLED = 2,
    #[doc = "3: Pull-down. P1.16 has a pull-down resistor enabled."]
    PULL_DOWN = 3,
}
impl From<P1_16MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: P1_16MODE_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `P1_16MODE`"]
pub type P1_16MODE_R = crate::R<u8, P1_16MODE_A>;
impl P1_16MODE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> P1_16MODE_A {
        match self.bits {
            0 => P1_16MODE_A::PULL_UP,
            1 => P1_16MODE_A::REPEATER,
            2 => P1_16MODE_A::DISABLED,
            3 => P1_16MODE_A::PULL_DOWN,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PULL_UP`"]
    #[inline(always)]
    pub fn is_pull_up(&self) -> bool {
        *self == P1_16MODE_A::PULL_UP
    }
    #[doc = "Checks if the value of the field is `REPEATER`"]
    #[inline(always)]
    pub fn is_repeater(&self) -> bool {
        *self == P1_16MODE_A::REPEATER
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == P1_16MODE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `PULL_DOWN`"]
    #[inline(always)]
    pub fn is_pull_down(&self) -> bool {
        *self == P1_16MODE_A::PULL_DOWN
    }
}
#[doc = "Write proxy for field `P1_16MODE`"]
pub struct P1_16MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> P1_16MODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: P1_16MODE_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Pull-up. P1.16 pin has a pull-up resistor enabled."]
    #[inline(always)]
    pub fn pull_up(self) -> &'a mut W {
        self.variant(P1_16MODE_A::PULL_UP)
    }
    #[doc = "Repeater. P1.16 pin has repeater mode enabled."]
    #[inline(always)]
    pub fn repeater(self) -> &'a mut W {
        self.variant(P1_16MODE_A::REPEATER)
    }
    #[doc = "Disabled. P1.16 pin has neither pull-up nor pull-down."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(P1_16MODE_A::DISABLED)
    }
    #[doc = "Pull-down. P1.16 has a pull-down resistor enabled."]
    #[inline(always)]
    pub fn pull_down(self) -> &'a mut W {
        self.variant(P1_16MODE_A::PULL_DOWN)
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
pub enum P1_17MODE_A {
    #[doc = "0: Pull-up. P1.17 pin has a pull-up resistor enabled."]
    PULL_UP = 0,
    #[doc = "1: Repeater. P1.17 pin has repeater mode enabled."]
    REPEATER = 1,
    #[doc = "2: Disabled. P1.17 pin has neither pull-up nor pull-down."]
    DISABLED = 2,
    #[doc = "3: Pull-down. P1.17 has a pull-down resistor enabled."]
    PULL_DOWN = 3,
}
impl From<P1_17MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: P1_17MODE_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `P1_17MODE`"]
pub type P1_17MODE_R = crate::R<u8, P1_17MODE_A>;
impl P1_17MODE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> P1_17MODE_A {
        match self.bits {
            0 => P1_17MODE_A::PULL_UP,
            1 => P1_17MODE_A::REPEATER,
            2 => P1_17MODE_A::DISABLED,
            3 => P1_17MODE_A::PULL_DOWN,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PULL_UP`"]
    #[inline(always)]
    pub fn is_pull_up(&self) -> bool {
        *self == P1_17MODE_A::PULL_UP
    }
    #[doc = "Checks if the value of the field is `REPEATER`"]
    #[inline(always)]
    pub fn is_repeater(&self) -> bool {
        *self == P1_17MODE_A::REPEATER
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == P1_17MODE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `PULL_DOWN`"]
    #[inline(always)]
    pub fn is_pull_down(&self) -> bool {
        *self == P1_17MODE_A::PULL_DOWN
    }
}
#[doc = "Write proxy for field `P1_17MODE`"]
pub struct P1_17MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> P1_17MODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: P1_17MODE_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Pull-up. P1.17 pin has a pull-up resistor enabled."]
    #[inline(always)]
    pub fn pull_up(self) -> &'a mut W {
        self.variant(P1_17MODE_A::PULL_UP)
    }
    #[doc = "Repeater. P1.17 pin has repeater mode enabled."]
    #[inline(always)]
    pub fn repeater(self) -> &'a mut W {
        self.variant(P1_17MODE_A::REPEATER)
    }
    #[doc = "Disabled. P1.17 pin has neither pull-up nor pull-down."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(P1_17MODE_A::DISABLED)
    }
    #[doc = "Pull-down. P1.17 has a pull-down resistor enabled."]
    #[inline(always)]
    pub fn pull_down(self) -> &'a mut W {
        self.variant(P1_17MODE_A::PULL_DOWN)
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
pub enum P1_18MODE_A {
    #[doc = "0: Pull-up. P1.18 pin has a pull-up resistor enabled."]
    PULL_UP = 0,
    #[doc = "1: Repeater. P1.18 pin has repeater mode enabled."]
    REPEATER = 1,
    #[doc = "2: Disabled. P1.18 pin has neither pull-up nor pull-down."]
    DISABLED = 2,
    #[doc = "3: Pull-down. P1.18 has a pull-down resistor enabled."]
    PULL_DOWN = 3,
}
impl From<P1_18MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: P1_18MODE_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `P1_18MODE`"]
pub type P1_18MODE_R = crate::R<u8, P1_18MODE_A>;
impl P1_18MODE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> P1_18MODE_A {
        match self.bits {
            0 => P1_18MODE_A::PULL_UP,
            1 => P1_18MODE_A::REPEATER,
            2 => P1_18MODE_A::DISABLED,
            3 => P1_18MODE_A::PULL_DOWN,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PULL_UP`"]
    #[inline(always)]
    pub fn is_pull_up(&self) -> bool {
        *self == P1_18MODE_A::PULL_UP
    }
    #[doc = "Checks if the value of the field is `REPEATER`"]
    #[inline(always)]
    pub fn is_repeater(&self) -> bool {
        *self == P1_18MODE_A::REPEATER
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == P1_18MODE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `PULL_DOWN`"]
    #[inline(always)]
    pub fn is_pull_down(&self) -> bool {
        *self == P1_18MODE_A::PULL_DOWN
    }
}
#[doc = "Write proxy for field `P1_18MODE`"]
pub struct P1_18MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> P1_18MODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: P1_18MODE_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Pull-up. P1.18 pin has a pull-up resistor enabled."]
    #[inline(always)]
    pub fn pull_up(self) -> &'a mut W {
        self.variant(P1_18MODE_A::PULL_UP)
    }
    #[doc = "Repeater. P1.18 pin has repeater mode enabled."]
    #[inline(always)]
    pub fn repeater(self) -> &'a mut W {
        self.variant(P1_18MODE_A::REPEATER)
    }
    #[doc = "Disabled. P1.18 pin has neither pull-up nor pull-down."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(P1_18MODE_A::DISABLED)
    }
    #[doc = "Pull-down. P1.18 has a pull-down resistor enabled."]
    #[inline(always)]
    pub fn pull_down(self) -> &'a mut W {
        self.variant(P1_18MODE_A::PULL_DOWN)
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
pub enum P1_19MODE_A {
    #[doc = "0: Pull-up. P1.19 pin has a pull-up resistor enabled."]
    PULL_UP = 0,
    #[doc = "1: Repeater. P1.19 pin has repeater mode enabled."]
    REPEATER = 1,
    #[doc = "2: Disabled. P1.19 pin has neither pull-up nor pull-down."]
    DISABLED = 2,
    #[doc = "3: Pull-down. P1.19 has a pull-down resistor enabled."]
    PULL_DOWN = 3,
}
impl From<P1_19MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: P1_19MODE_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `P1_19MODE`"]
pub type P1_19MODE_R = crate::R<u8, P1_19MODE_A>;
impl P1_19MODE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> P1_19MODE_A {
        match self.bits {
            0 => P1_19MODE_A::PULL_UP,
            1 => P1_19MODE_A::REPEATER,
            2 => P1_19MODE_A::DISABLED,
            3 => P1_19MODE_A::PULL_DOWN,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PULL_UP`"]
    #[inline(always)]
    pub fn is_pull_up(&self) -> bool {
        *self == P1_19MODE_A::PULL_UP
    }
    #[doc = "Checks if the value of the field is `REPEATER`"]
    #[inline(always)]
    pub fn is_repeater(&self) -> bool {
        *self == P1_19MODE_A::REPEATER
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == P1_19MODE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `PULL_DOWN`"]
    #[inline(always)]
    pub fn is_pull_down(&self) -> bool {
        *self == P1_19MODE_A::PULL_DOWN
    }
}
#[doc = "Write proxy for field `P1_19MODE`"]
pub struct P1_19MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> P1_19MODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: P1_19MODE_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Pull-up. P1.19 pin has a pull-up resistor enabled."]
    #[inline(always)]
    pub fn pull_up(self) -> &'a mut W {
        self.variant(P1_19MODE_A::PULL_UP)
    }
    #[doc = "Repeater. P1.19 pin has repeater mode enabled."]
    #[inline(always)]
    pub fn repeater(self) -> &'a mut W {
        self.variant(P1_19MODE_A::REPEATER)
    }
    #[doc = "Disabled. P1.19 pin has neither pull-up nor pull-down."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(P1_19MODE_A::DISABLED)
    }
    #[doc = "Pull-down. P1.19 has a pull-down resistor enabled."]
    #[inline(always)]
    pub fn pull_down(self) -> &'a mut W {
        self.variant(P1_19MODE_A::PULL_DOWN)
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
pub enum P1_20MODE_A {
    #[doc = "0: Pull-up. P1.20 pin has a pull-up resistor enabled."]
    PULL_UP = 0,
    #[doc = "1: Repeater. P1.20 pin has repeater mode enabled."]
    REPEATER = 1,
    #[doc = "2: Disabled. P1.20 pin has neither pull-up nor pull-down."]
    DISABLED = 2,
    #[doc = "3: Pull-down. P1.20 has a pull-down resistor enabled."]
    PULL_DOWN = 3,
}
impl From<P1_20MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: P1_20MODE_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `P1_20MODE`"]
pub type P1_20MODE_R = crate::R<u8, P1_20MODE_A>;
impl P1_20MODE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> P1_20MODE_A {
        match self.bits {
            0 => P1_20MODE_A::PULL_UP,
            1 => P1_20MODE_A::REPEATER,
            2 => P1_20MODE_A::DISABLED,
            3 => P1_20MODE_A::PULL_DOWN,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PULL_UP`"]
    #[inline(always)]
    pub fn is_pull_up(&self) -> bool {
        *self == P1_20MODE_A::PULL_UP
    }
    #[doc = "Checks if the value of the field is `REPEATER`"]
    #[inline(always)]
    pub fn is_repeater(&self) -> bool {
        *self == P1_20MODE_A::REPEATER
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == P1_20MODE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `PULL_DOWN`"]
    #[inline(always)]
    pub fn is_pull_down(&self) -> bool {
        *self == P1_20MODE_A::PULL_DOWN
    }
}
#[doc = "Write proxy for field `P1_20MODE`"]
pub struct P1_20MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> P1_20MODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: P1_20MODE_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Pull-up. P1.20 pin has a pull-up resistor enabled."]
    #[inline(always)]
    pub fn pull_up(self) -> &'a mut W {
        self.variant(P1_20MODE_A::PULL_UP)
    }
    #[doc = "Repeater. P1.20 pin has repeater mode enabled."]
    #[inline(always)]
    pub fn repeater(self) -> &'a mut W {
        self.variant(P1_20MODE_A::REPEATER)
    }
    #[doc = "Disabled. P1.20 pin has neither pull-up nor pull-down."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(P1_20MODE_A::DISABLED)
    }
    #[doc = "Pull-down. P1.20 has a pull-down resistor enabled."]
    #[inline(always)]
    pub fn pull_down(self) -> &'a mut W {
        self.variant(P1_20MODE_A::PULL_DOWN)
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
pub enum P1_21MODE_A {
    #[doc = "0: Pull-up. P1.21 pin has a pull-up resistor enabled."]
    PULL_UP = 0,
    #[doc = "1: Repeater. P1.21 pin has repeater mode enabled."]
    REPEATER = 1,
    #[doc = "2: Disabled. P1.21 pin has neither pull-up nor pull-down."]
    DISABLED = 2,
    #[doc = "3: Pull-down. P1.21 has a pull-down resistor enabled."]
    PULL_DOWN = 3,
}
impl From<P1_21MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: P1_21MODE_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `P1_21MODE`"]
pub type P1_21MODE_R = crate::R<u8, P1_21MODE_A>;
impl P1_21MODE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> P1_21MODE_A {
        match self.bits {
            0 => P1_21MODE_A::PULL_UP,
            1 => P1_21MODE_A::REPEATER,
            2 => P1_21MODE_A::DISABLED,
            3 => P1_21MODE_A::PULL_DOWN,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PULL_UP`"]
    #[inline(always)]
    pub fn is_pull_up(&self) -> bool {
        *self == P1_21MODE_A::PULL_UP
    }
    #[doc = "Checks if the value of the field is `REPEATER`"]
    #[inline(always)]
    pub fn is_repeater(&self) -> bool {
        *self == P1_21MODE_A::REPEATER
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == P1_21MODE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `PULL_DOWN`"]
    #[inline(always)]
    pub fn is_pull_down(&self) -> bool {
        *self == P1_21MODE_A::PULL_DOWN
    }
}
#[doc = "Write proxy for field `P1_21MODE`"]
pub struct P1_21MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> P1_21MODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: P1_21MODE_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Pull-up. P1.21 pin has a pull-up resistor enabled."]
    #[inline(always)]
    pub fn pull_up(self) -> &'a mut W {
        self.variant(P1_21MODE_A::PULL_UP)
    }
    #[doc = "Repeater. P1.21 pin has repeater mode enabled."]
    #[inline(always)]
    pub fn repeater(self) -> &'a mut W {
        self.variant(P1_21MODE_A::REPEATER)
    }
    #[doc = "Disabled. P1.21 pin has neither pull-up nor pull-down."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(P1_21MODE_A::DISABLED)
    }
    #[doc = "Pull-down. P1.21 has a pull-down resistor enabled."]
    #[inline(always)]
    pub fn pull_down(self) -> &'a mut W {
        self.variant(P1_21MODE_A::PULL_DOWN)
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
pub enum P1_22MODE_A {
    #[doc = "0: Pull-up. P1.22 pin has a pull-up resistor enabled."]
    PULL_UP = 0,
    #[doc = "1: Repeater. P1.22 pin has repeater mode enabled."]
    REPEATER = 1,
    #[doc = "2: Disabled. P1.22 pin has neither pull-up nor pull-down."]
    DISABLED = 2,
    #[doc = "3: Pull-down. P1.22 has a pull-down resistor enabled."]
    PULL_DOWN = 3,
}
impl From<P1_22MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: P1_22MODE_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `P1_22MODE`"]
pub type P1_22MODE_R = crate::R<u8, P1_22MODE_A>;
impl P1_22MODE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> P1_22MODE_A {
        match self.bits {
            0 => P1_22MODE_A::PULL_UP,
            1 => P1_22MODE_A::REPEATER,
            2 => P1_22MODE_A::DISABLED,
            3 => P1_22MODE_A::PULL_DOWN,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PULL_UP`"]
    #[inline(always)]
    pub fn is_pull_up(&self) -> bool {
        *self == P1_22MODE_A::PULL_UP
    }
    #[doc = "Checks if the value of the field is `REPEATER`"]
    #[inline(always)]
    pub fn is_repeater(&self) -> bool {
        *self == P1_22MODE_A::REPEATER
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == P1_22MODE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `PULL_DOWN`"]
    #[inline(always)]
    pub fn is_pull_down(&self) -> bool {
        *self == P1_22MODE_A::PULL_DOWN
    }
}
#[doc = "Write proxy for field `P1_22MODE`"]
pub struct P1_22MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> P1_22MODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: P1_22MODE_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Pull-up. P1.22 pin has a pull-up resistor enabled."]
    #[inline(always)]
    pub fn pull_up(self) -> &'a mut W {
        self.variant(P1_22MODE_A::PULL_UP)
    }
    #[doc = "Repeater. P1.22 pin has repeater mode enabled."]
    #[inline(always)]
    pub fn repeater(self) -> &'a mut W {
        self.variant(P1_22MODE_A::REPEATER)
    }
    #[doc = "Disabled. P1.22 pin has neither pull-up nor pull-down."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(P1_22MODE_A::DISABLED)
    }
    #[doc = "Pull-down. P1.22 has a pull-down resistor enabled."]
    #[inline(always)]
    pub fn pull_down(self) -> &'a mut W {
        self.variant(P1_22MODE_A::PULL_DOWN)
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
pub enum P1_23MODE_A {
    #[doc = "0: Pull-up. P1.23 pin has a pull-up resistor enabled."]
    PULL_UP = 0,
    #[doc = "1: Repeater. P1.23 pin has repeater mode enabled."]
    REPEATER = 1,
    #[doc = "2: Disabled. P1.23 pin has neither pull-up nor pull-down."]
    DISABLED = 2,
    #[doc = "3: Pull-down. P1.23 has a pull-down resistor enabled."]
    PULL_DOWN = 3,
}
impl From<P1_23MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: P1_23MODE_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `P1_23MODE`"]
pub type P1_23MODE_R = crate::R<u8, P1_23MODE_A>;
impl P1_23MODE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> P1_23MODE_A {
        match self.bits {
            0 => P1_23MODE_A::PULL_UP,
            1 => P1_23MODE_A::REPEATER,
            2 => P1_23MODE_A::DISABLED,
            3 => P1_23MODE_A::PULL_DOWN,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PULL_UP`"]
    #[inline(always)]
    pub fn is_pull_up(&self) -> bool {
        *self == P1_23MODE_A::PULL_UP
    }
    #[doc = "Checks if the value of the field is `REPEATER`"]
    #[inline(always)]
    pub fn is_repeater(&self) -> bool {
        *self == P1_23MODE_A::REPEATER
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == P1_23MODE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `PULL_DOWN`"]
    #[inline(always)]
    pub fn is_pull_down(&self) -> bool {
        *self == P1_23MODE_A::PULL_DOWN
    }
}
#[doc = "Write proxy for field `P1_23MODE`"]
pub struct P1_23MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> P1_23MODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: P1_23MODE_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Pull-up. P1.23 pin has a pull-up resistor enabled."]
    #[inline(always)]
    pub fn pull_up(self) -> &'a mut W {
        self.variant(P1_23MODE_A::PULL_UP)
    }
    #[doc = "Repeater. P1.23 pin has repeater mode enabled."]
    #[inline(always)]
    pub fn repeater(self) -> &'a mut W {
        self.variant(P1_23MODE_A::REPEATER)
    }
    #[doc = "Disabled. P1.23 pin has neither pull-up nor pull-down."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(P1_23MODE_A::DISABLED)
    }
    #[doc = "Pull-down. P1.23 has a pull-down resistor enabled."]
    #[inline(always)]
    pub fn pull_down(self) -> &'a mut W {
        self.variant(P1_23MODE_A::PULL_DOWN)
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
pub enum P1_24MODE_A {
    #[doc = "0: Pull-up. P1.24 pin has a pull-up resistor enabled."]
    PULL_UP = 0,
    #[doc = "1: Repeater. P1.24 pin has repeater mode enabled."]
    REPEATER = 1,
    #[doc = "2: Disabled. P1.24 pin has neither pull-up nor pull-down."]
    DISABLED = 2,
    #[doc = "3: Pull-down. P1.24 has a pull-down resistor enabled."]
    PULL_DOWN = 3,
}
impl From<P1_24MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: P1_24MODE_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `P1_24MODE`"]
pub type P1_24MODE_R = crate::R<u8, P1_24MODE_A>;
impl P1_24MODE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> P1_24MODE_A {
        match self.bits {
            0 => P1_24MODE_A::PULL_UP,
            1 => P1_24MODE_A::REPEATER,
            2 => P1_24MODE_A::DISABLED,
            3 => P1_24MODE_A::PULL_DOWN,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PULL_UP`"]
    #[inline(always)]
    pub fn is_pull_up(&self) -> bool {
        *self == P1_24MODE_A::PULL_UP
    }
    #[doc = "Checks if the value of the field is `REPEATER`"]
    #[inline(always)]
    pub fn is_repeater(&self) -> bool {
        *self == P1_24MODE_A::REPEATER
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == P1_24MODE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `PULL_DOWN`"]
    #[inline(always)]
    pub fn is_pull_down(&self) -> bool {
        *self == P1_24MODE_A::PULL_DOWN
    }
}
#[doc = "Write proxy for field `P1_24MODE`"]
pub struct P1_24MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> P1_24MODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: P1_24MODE_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Pull-up. P1.24 pin has a pull-up resistor enabled."]
    #[inline(always)]
    pub fn pull_up(self) -> &'a mut W {
        self.variant(P1_24MODE_A::PULL_UP)
    }
    #[doc = "Repeater. P1.24 pin has repeater mode enabled."]
    #[inline(always)]
    pub fn repeater(self) -> &'a mut W {
        self.variant(P1_24MODE_A::REPEATER)
    }
    #[doc = "Disabled. P1.24 pin has neither pull-up nor pull-down."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(P1_24MODE_A::DISABLED)
    }
    #[doc = "Pull-down. P1.24 has a pull-down resistor enabled."]
    #[inline(always)]
    pub fn pull_down(self) -> &'a mut W {
        self.variant(P1_24MODE_A::PULL_DOWN)
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
pub enum P1_25MODE_A {
    #[doc = "0: Pull-up. P1.25 pin has a pull-up resistor enabled."]
    PULL_UP = 0,
    #[doc = "1: Repeater. P1.25 pin has repeater mode enabled."]
    REPEATER = 1,
    #[doc = "2: Disabled. P1.25 pin has neither pull-up nor pull-down."]
    DISABLED = 2,
    #[doc = "3: Pull-down. P1.25 has a pull-down resistor enabled."]
    PULL_DOWN = 3,
}
impl From<P1_25MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: P1_25MODE_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `P1_25MODE`"]
pub type P1_25MODE_R = crate::R<u8, P1_25MODE_A>;
impl P1_25MODE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> P1_25MODE_A {
        match self.bits {
            0 => P1_25MODE_A::PULL_UP,
            1 => P1_25MODE_A::REPEATER,
            2 => P1_25MODE_A::DISABLED,
            3 => P1_25MODE_A::PULL_DOWN,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PULL_UP`"]
    #[inline(always)]
    pub fn is_pull_up(&self) -> bool {
        *self == P1_25MODE_A::PULL_UP
    }
    #[doc = "Checks if the value of the field is `REPEATER`"]
    #[inline(always)]
    pub fn is_repeater(&self) -> bool {
        *self == P1_25MODE_A::REPEATER
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == P1_25MODE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `PULL_DOWN`"]
    #[inline(always)]
    pub fn is_pull_down(&self) -> bool {
        *self == P1_25MODE_A::PULL_DOWN
    }
}
#[doc = "Write proxy for field `P1_25MODE`"]
pub struct P1_25MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> P1_25MODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: P1_25MODE_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Pull-up. P1.25 pin has a pull-up resistor enabled."]
    #[inline(always)]
    pub fn pull_up(self) -> &'a mut W {
        self.variant(P1_25MODE_A::PULL_UP)
    }
    #[doc = "Repeater. P1.25 pin has repeater mode enabled."]
    #[inline(always)]
    pub fn repeater(self) -> &'a mut W {
        self.variant(P1_25MODE_A::REPEATER)
    }
    #[doc = "Disabled. P1.25 pin has neither pull-up nor pull-down."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(P1_25MODE_A::DISABLED)
    }
    #[doc = "Pull-down. P1.25 has a pull-down resistor enabled."]
    #[inline(always)]
    pub fn pull_down(self) -> &'a mut W {
        self.variant(P1_25MODE_A::PULL_DOWN)
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
pub enum P1_26MODE_A {
    #[doc = "0: Pull-up. P1.26 pin has a pull-up resistor enabled."]
    PULL_UP = 0,
    #[doc = "1: Repeater. P1.26 pin has repeater mode enabled."]
    REPEATER = 1,
    #[doc = "2: Disabled. P1.26 pin has neither pull-up nor pull-down."]
    DISABLED = 2,
    #[doc = "3: Pull-down. P1.26 has a pull-down resistor enabled."]
    PULL_DOWN = 3,
}
impl From<P1_26MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: P1_26MODE_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `P1_26MODE`"]
pub type P1_26MODE_R = crate::R<u8, P1_26MODE_A>;
impl P1_26MODE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> P1_26MODE_A {
        match self.bits {
            0 => P1_26MODE_A::PULL_UP,
            1 => P1_26MODE_A::REPEATER,
            2 => P1_26MODE_A::DISABLED,
            3 => P1_26MODE_A::PULL_DOWN,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PULL_UP`"]
    #[inline(always)]
    pub fn is_pull_up(&self) -> bool {
        *self == P1_26MODE_A::PULL_UP
    }
    #[doc = "Checks if the value of the field is `REPEATER`"]
    #[inline(always)]
    pub fn is_repeater(&self) -> bool {
        *self == P1_26MODE_A::REPEATER
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == P1_26MODE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `PULL_DOWN`"]
    #[inline(always)]
    pub fn is_pull_down(&self) -> bool {
        *self == P1_26MODE_A::PULL_DOWN
    }
}
#[doc = "Write proxy for field `P1_26MODE`"]
pub struct P1_26MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> P1_26MODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: P1_26MODE_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Pull-up. P1.26 pin has a pull-up resistor enabled."]
    #[inline(always)]
    pub fn pull_up(self) -> &'a mut W {
        self.variant(P1_26MODE_A::PULL_UP)
    }
    #[doc = "Repeater. P1.26 pin has repeater mode enabled."]
    #[inline(always)]
    pub fn repeater(self) -> &'a mut W {
        self.variant(P1_26MODE_A::REPEATER)
    }
    #[doc = "Disabled. P1.26 pin has neither pull-up nor pull-down."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(P1_26MODE_A::DISABLED)
    }
    #[doc = "Pull-down. P1.26 has a pull-down resistor enabled."]
    #[inline(always)]
    pub fn pull_down(self) -> &'a mut W {
        self.variant(P1_26MODE_A::PULL_DOWN)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 20)) | (((value as u32) & 0x03) << 20);
        self.w
    }
}
#[doc = "Port 1 pin 27 control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum P1_27MODE_A {
    #[doc = "0: Pull-up. P1.27 pin has a pull-up resistor enabled."]
    PULL_UP = 0,
    #[doc = "1: Repeater. P1.27 pin has repeater mode enabled."]
    REPEATER = 1,
    #[doc = "2: Disabled. P1.27 pin has neither pull-up nor pull-down."]
    DISABLED = 2,
    #[doc = "3: Pull-down. P1.27 has a pull-down resistor enabled."]
    PULL_DOWN = 3,
}
impl From<P1_27MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: P1_27MODE_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `P1_27MODE`"]
pub type P1_27MODE_R = crate::R<u8, P1_27MODE_A>;
impl P1_27MODE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> P1_27MODE_A {
        match self.bits {
            0 => P1_27MODE_A::PULL_UP,
            1 => P1_27MODE_A::REPEATER,
            2 => P1_27MODE_A::DISABLED,
            3 => P1_27MODE_A::PULL_DOWN,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PULL_UP`"]
    #[inline(always)]
    pub fn is_pull_up(&self) -> bool {
        *self == P1_27MODE_A::PULL_UP
    }
    #[doc = "Checks if the value of the field is `REPEATER`"]
    #[inline(always)]
    pub fn is_repeater(&self) -> bool {
        *self == P1_27MODE_A::REPEATER
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == P1_27MODE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `PULL_DOWN`"]
    #[inline(always)]
    pub fn is_pull_down(&self) -> bool {
        *self == P1_27MODE_A::PULL_DOWN
    }
}
#[doc = "Write proxy for field `P1_27MODE`"]
pub struct P1_27MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> P1_27MODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: P1_27MODE_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Pull-up. P1.27 pin has a pull-up resistor enabled."]
    #[inline(always)]
    pub fn pull_up(self) -> &'a mut W {
        self.variant(P1_27MODE_A::PULL_UP)
    }
    #[doc = "Repeater. P1.27 pin has repeater mode enabled."]
    #[inline(always)]
    pub fn repeater(self) -> &'a mut W {
        self.variant(P1_27MODE_A::REPEATER)
    }
    #[doc = "Disabled. P1.27 pin has neither pull-up nor pull-down."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(P1_27MODE_A::DISABLED)
    }
    #[doc = "Pull-down. P1.27 has a pull-down resistor enabled."]
    #[inline(always)]
    pub fn pull_down(self) -> &'a mut W {
        self.variant(P1_27MODE_A::PULL_DOWN)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 22)) | (((value as u32) & 0x03) << 22);
        self.w
    }
}
#[doc = "Port 1 pin 28 control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum P1_28MODE_A {
    #[doc = "0: Pull-up. P1.28 pin has a pull-up resistor enabled."]
    PULL_UP = 0,
    #[doc = "1: Repeater. P1.28 pin has repeater mode enabled."]
    REPEATER = 1,
    #[doc = "2: Disabled. P1.28 pin has neither pull-up nor pull-down."]
    DISABLED = 2,
    #[doc = "3: Pull-down. P1.28 has a pull-down resistor enabled."]
    PULL_DOWN = 3,
}
impl From<P1_28MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: P1_28MODE_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `P1_28MODE`"]
pub type P1_28MODE_R = crate::R<u8, P1_28MODE_A>;
impl P1_28MODE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> P1_28MODE_A {
        match self.bits {
            0 => P1_28MODE_A::PULL_UP,
            1 => P1_28MODE_A::REPEATER,
            2 => P1_28MODE_A::DISABLED,
            3 => P1_28MODE_A::PULL_DOWN,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PULL_UP`"]
    #[inline(always)]
    pub fn is_pull_up(&self) -> bool {
        *self == P1_28MODE_A::PULL_UP
    }
    #[doc = "Checks if the value of the field is `REPEATER`"]
    #[inline(always)]
    pub fn is_repeater(&self) -> bool {
        *self == P1_28MODE_A::REPEATER
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == P1_28MODE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `PULL_DOWN`"]
    #[inline(always)]
    pub fn is_pull_down(&self) -> bool {
        *self == P1_28MODE_A::PULL_DOWN
    }
}
#[doc = "Write proxy for field `P1_28MODE`"]
pub struct P1_28MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> P1_28MODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: P1_28MODE_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Pull-up. P1.28 pin has a pull-up resistor enabled."]
    #[inline(always)]
    pub fn pull_up(self) -> &'a mut W {
        self.variant(P1_28MODE_A::PULL_UP)
    }
    #[doc = "Repeater. P1.28 pin has repeater mode enabled."]
    #[inline(always)]
    pub fn repeater(self) -> &'a mut W {
        self.variant(P1_28MODE_A::REPEATER)
    }
    #[doc = "Disabled. P1.28 pin has neither pull-up nor pull-down."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(P1_28MODE_A::DISABLED)
    }
    #[doc = "Pull-down. P1.28 has a pull-down resistor enabled."]
    #[inline(always)]
    pub fn pull_down(self) -> &'a mut W {
        self.variant(P1_28MODE_A::PULL_DOWN)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 24)) | (((value as u32) & 0x03) << 24);
        self.w
    }
}
#[doc = "Port 1 pin 29 control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum P1_29MODE_A {
    #[doc = "0: Pull-up. P1.29 pin has a pull-up resistor enabled."]
    PULL_UP = 0,
    #[doc = "1: Repeater. P1.29 pin has repeater mode enabled."]
    REPEATER = 1,
    #[doc = "2: Disabled. P1.29 pin has neither pull-up nor pull-down."]
    DISABLED = 2,
    #[doc = "3: Pull-down. P1.29 has a pull-down resistor enabled."]
    PULL_DOWN = 3,
}
impl From<P1_29MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: P1_29MODE_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `P1_29MODE`"]
pub type P1_29MODE_R = crate::R<u8, P1_29MODE_A>;
impl P1_29MODE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> P1_29MODE_A {
        match self.bits {
            0 => P1_29MODE_A::PULL_UP,
            1 => P1_29MODE_A::REPEATER,
            2 => P1_29MODE_A::DISABLED,
            3 => P1_29MODE_A::PULL_DOWN,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PULL_UP`"]
    #[inline(always)]
    pub fn is_pull_up(&self) -> bool {
        *self == P1_29MODE_A::PULL_UP
    }
    #[doc = "Checks if the value of the field is `REPEATER`"]
    #[inline(always)]
    pub fn is_repeater(&self) -> bool {
        *self == P1_29MODE_A::REPEATER
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == P1_29MODE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `PULL_DOWN`"]
    #[inline(always)]
    pub fn is_pull_down(&self) -> bool {
        *self == P1_29MODE_A::PULL_DOWN
    }
}
#[doc = "Write proxy for field `P1_29MODE`"]
pub struct P1_29MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> P1_29MODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: P1_29MODE_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Pull-up. P1.29 pin has a pull-up resistor enabled."]
    #[inline(always)]
    pub fn pull_up(self) -> &'a mut W {
        self.variant(P1_29MODE_A::PULL_UP)
    }
    #[doc = "Repeater. P1.29 pin has repeater mode enabled."]
    #[inline(always)]
    pub fn repeater(self) -> &'a mut W {
        self.variant(P1_29MODE_A::REPEATER)
    }
    #[doc = "Disabled. P1.29 pin has neither pull-up nor pull-down."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(P1_29MODE_A::DISABLED)
    }
    #[doc = "Pull-down. P1.29 has a pull-down resistor enabled."]
    #[inline(always)]
    pub fn pull_down(self) -> &'a mut W {
        self.variant(P1_29MODE_A::PULL_DOWN)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 26)) | (((value as u32) & 0x03) << 26);
        self.w
    }
}
#[doc = "Port 1 pin 30 control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum P1_30MODE_A {
    #[doc = "0: Pull-up. P1.30 pin has a pull-up resistor enabled."]
    PULL_UP = 0,
    #[doc = "1: Repeater. P1.30 pin has repeater mode enabled."]
    REPEATER = 1,
    #[doc = "2: Disabled. P1.30 pin has neither pull-up nor pull-down."]
    DISABLED = 2,
    #[doc = "3: Pull-down. P1.30 has a pull-down resistor enabled."]
    PULL_DOWN = 3,
}
impl From<P1_30MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: P1_30MODE_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `P1_30MODE`"]
pub type P1_30MODE_R = crate::R<u8, P1_30MODE_A>;
impl P1_30MODE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> P1_30MODE_A {
        match self.bits {
            0 => P1_30MODE_A::PULL_UP,
            1 => P1_30MODE_A::REPEATER,
            2 => P1_30MODE_A::DISABLED,
            3 => P1_30MODE_A::PULL_DOWN,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PULL_UP`"]
    #[inline(always)]
    pub fn is_pull_up(&self) -> bool {
        *self == P1_30MODE_A::PULL_UP
    }
    #[doc = "Checks if the value of the field is `REPEATER`"]
    #[inline(always)]
    pub fn is_repeater(&self) -> bool {
        *self == P1_30MODE_A::REPEATER
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == P1_30MODE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `PULL_DOWN`"]
    #[inline(always)]
    pub fn is_pull_down(&self) -> bool {
        *self == P1_30MODE_A::PULL_DOWN
    }
}
#[doc = "Write proxy for field `P1_30MODE`"]
pub struct P1_30MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> P1_30MODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: P1_30MODE_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Pull-up. P1.30 pin has a pull-up resistor enabled."]
    #[inline(always)]
    pub fn pull_up(self) -> &'a mut W {
        self.variant(P1_30MODE_A::PULL_UP)
    }
    #[doc = "Repeater. P1.30 pin has repeater mode enabled."]
    #[inline(always)]
    pub fn repeater(self) -> &'a mut W {
        self.variant(P1_30MODE_A::REPEATER)
    }
    #[doc = "Disabled. P1.30 pin has neither pull-up nor pull-down."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(P1_30MODE_A::DISABLED)
    }
    #[doc = "Pull-down. P1.30 has a pull-down resistor enabled."]
    #[inline(always)]
    pub fn pull_down(self) -> &'a mut W {
        self.variant(P1_30MODE_A::PULL_DOWN)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 28)) | (((value as u32) & 0x03) << 28);
        self.w
    }
}
#[doc = "Port 1 pin 31 control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum P1_31MODE_A {
    #[doc = "0: Pull-up. P1.31 pin has a pull-up resistor enabled."]
    PULL_UP = 0,
    #[doc = "1: Repeater. P1.31 pin has repeater mode enabled."]
    REPEATER = 1,
    #[doc = "2: Disabled. P1.31 pin has neither pull-up nor pull-down."]
    DISABLED = 2,
    #[doc = "3: Pull-down. P1.31 has a pull-down resistor enabled."]
    PULL_DOWN = 3,
}
impl From<P1_31MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: P1_31MODE_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `P1_31MODE`"]
pub type P1_31MODE_R = crate::R<u8, P1_31MODE_A>;
impl P1_31MODE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> P1_31MODE_A {
        match self.bits {
            0 => P1_31MODE_A::PULL_UP,
            1 => P1_31MODE_A::REPEATER,
            2 => P1_31MODE_A::DISABLED,
            3 => P1_31MODE_A::PULL_DOWN,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PULL_UP`"]
    #[inline(always)]
    pub fn is_pull_up(&self) -> bool {
        *self == P1_31MODE_A::PULL_UP
    }
    #[doc = "Checks if the value of the field is `REPEATER`"]
    #[inline(always)]
    pub fn is_repeater(&self) -> bool {
        *self == P1_31MODE_A::REPEATER
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == P1_31MODE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `PULL_DOWN`"]
    #[inline(always)]
    pub fn is_pull_down(&self) -> bool {
        *self == P1_31MODE_A::PULL_DOWN
    }
}
#[doc = "Write proxy for field `P1_31MODE`"]
pub struct P1_31MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> P1_31MODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: P1_31MODE_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Pull-up. P1.31 pin has a pull-up resistor enabled."]
    #[inline(always)]
    pub fn pull_up(self) -> &'a mut W {
        self.variant(P1_31MODE_A::PULL_UP)
    }
    #[doc = "Repeater. P1.31 pin has repeater mode enabled."]
    #[inline(always)]
    pub fn repeater(self) -> &'a mut W {
        self.variant(P1_31MODE_A::REPEATER)
    }
    #[doc = "Disabled. P1.31 pin has neither pull-up nor pull-down."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(P1_31MODE_A::DISABLED)
    }
    #[doc = "Pull-down. P1.31 has a pull-down resistor enabled."]
    #[inline(always)]
    pub fn pull_down(self) -> &'a mut W {
        self.variant(P1_31MODE_A::PULL_DOWN)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 30)) | (((value as u32) & 0x03) << 30);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - Port 1 pin 16 control."]
    #[inline(always)]
    pub fn p1_16mode(&self) -> P1_16MODE_R {
        P1_16MODE_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 2:3 - Port 1 pin 17 control."]
    #[inline(always)]
    pub fn p1_17mode(&self) -> P1_17MODE_R {
        P1_17MODE_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bits 4:5 - Port 1 pin 18 control."]
    #[inline(always)]
    pub fn p1_18mode(&self) -> P1_18MODE_R {
        P1_18MODE_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bits 6:7 - Port 1 pin 19 control."]
    #[inline(always)]
    pub fn p1_19mode(&self) -> P1_19MODE_R {
        P1_19MODE_R::new(((self.bits >> 6) & 0x03) as u8)
    }
    #[doc = "Bits 8:9 - Port 1 pin 20 control."]
    #[inline(always)]
    pub fn p1_20mode(&self) -> P1_20MODE_R {
        P1_20MODE_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bits 10:11 - Port 1 pin 21 control."]
    #[inline(always)]
    pub fn p1_21mode(&self) -> P1_21MODE_R {
        P1_21MODE_R::new(((self.bits >> 10) & 0x03) as u8)
    }
    #[doc = "Bits 12:13 - Port 1 pin 22 control."]
    #[inline(always)]
    pub fn p1_22mode(&self) -> P1_22MODE_R {
        P1_22MODE_R::new(((self.bits >> 12) & 0x03) as u8)
    }
    #[doc = "Bits 14:15 - Port 1 pin 23 control."]
    #[inline(always)]
    pub fn p1_23mode(&self) -> P1_23MODE_R {
        P1_23MODE_R::new(((self.bits >> 14) & 0x03) as u8)
    }
    #[doc = "Bits 16:17 - Port 1 pin 24 control."]
    #[inline(always)]
    pub fn p1_24mode(&self) -> P1_24MODE_R {
        P1_24MODE_R::new(((self.bits >> 16) & 0x03) as u8)
    }
    #[doc = "Bits 18:19 - Port 1 pin 25 control."]
    #[inline(always)]
    pub fn p1_25mode(&self) -> P1_25MODE_R {
        P1_25MODE_R::new(((self.bits >> 18) & 0x03) as u8)
    }
    #[doc = "Bits 20:21 - Port 1 pin 26 control."]
    #[inline(always)]
    pub fn p1_26mode(&self) -> P1_26MODE_R {
        P1_26MODE_R::new(((self.bits >> 20) & 0x03) as u8)
    }
    #[doc = "Bits 22:23 - Port 1 pin 27 control."]
    #[inline(always)]
    pub fn p1_27mode(&self) -> P1_27MODE_R {
        P1_27MODE_R::new(((self.bits >> 22) & 0x03) as u8)
    }
    #[doc = "Bits 24:25 - Port 1 pin 28 control."]
    #[inline(always)]
    pub fn p1_28mode(&self) -> P1_28MODE_R {
        P1_28MODE_R::new(((self.bits >> 24) & 0x03) as u8)
    }
    #[doc = "Bits 26:27 - Port 1 pin 29 control."]
    #[inline(always)]
    pub fn p1_29mode(&self) -> P1_29MODE_R {
        P1_29MODE_R::new(((self.bits >> 26) & 0x03) as u8)
    }
    #[doc = "Bits 28:29 - Port 1 pin 30 control."]
    #[inline(always)]
    pub fn p1_30mode(&self) -> P1_30MODE_R {
        P1_30MODE_R::new(((self.bits >> 28) & 0x03) as u8)
    }
    #[doc = "Bits 30:31 - Port 1 pin 31 control."]
    #[inline(always)]
    pub fn p1_31mode(&self) -> P1_31MODE_R {
        P1_31MODE_R::new(((self.bits >> 30) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Port 1 pin 16 control."]
    #[inline(always)]
    pub fn p1_16mode(&mut self) -> P1_16MODE_W {
        P1_16MODE_W { w: self }
    }
    #[doc = "Bits 2:3 - Port 1 pin 17 control."]
    #[inline(always)]
    pub fn p1_17mode(&mut self) -> P1_17MODE_W {
        P1_17MODE_W { w: self }
    }
    #[doc = "Bits 4:5 - Port 1 pin 18 control."]
    #[inline(always)]
    pub fn p1_18mode(&mut self) -> P1_18MODE_W {
        P1_18MODE_W { w: self }
    }
    #[doc = "Bits 6:7 - Port 1 pin 19 control."]
    #[inline(always)]
    pub fn p1_19mode(&mut self) -> P1_19MODE_W {
        P1_19MODE_W { w: self }
    }
    #[doc = "Bits 8:9 - Port 1 pin 20 control."]
    #[inline(always)]
    pub fn p1_20mode(&mut self) -> P1_20MODE_W {
        P1_20MODE_W { w: self }
    }
    #[doc = "Bits 10:11 - Port 1 pin 21 control."]
    #[inline(always)]
    pub fn p1_21mode(&mut self) -> P1_21MODE_W {
        P1_21MODE_W { w: self }
    }
    #[doc = "Bits 12:13 - Port 1 pin 22 control."]
    #[inline(always)]
    pub fn p1_22mode(&mut self) -> P1_22MODE_W {
        P1_22MODE_W { w: self }
    }
    #[doc = "Bits 14:15 - Port 1 pin 23 control."]
    #[inline(always)]
    pub fn p1_23mode(&mut self) -> P1_23MODE_W {
        P1_23MODE_W { w: self }
    }
    #[doc = "Bits 16:17 - Port 1 pin 24 control."]
    #[inline(always)]
    pub fn p1_24mode(&mut self) -> P1_24MODE_W {
        P1_24MODE_W { w: self }
    }
    #[doc = "Bits 18:19 - Port 1 pin 25 control."]
    #[inline(always)]
    pub fn p1_25mode(&mut self) -> P1_25MODE_W {
        P1_25MODE_W { w: self }
    }
    #[doc = "Bits 20:21 - Port 1 pin 26 control."]
    #[inline(always)]
    pub fn p1_26mode(&mut self) -> P1_26MODE_W {
        P1_26MODE_W { w: self }
    }
    #[doc = "Bits 22:23 - Port 1 pin 27 control."]
    #[inline(always)]
    pub fn p1_27mode(&mut self) -> P1_27MODE_W {
        P1_27MODE_W { w: self }
    }
    #[doc = "Bits 24:25 - Port 1 pin 28 control."]
    #[inline(always)]
    pub fn p1_28mode(&mut self) -> P1_28MODE_W {
        P1_28MODE_W { w: self }
    }
    #[doc = "Bits 26:27 - Port 1 pin 29 control."]
    #[inline(always)]
    pub fn p1_29mode(&mut self) -> P1_29MODE_W {
        P1_29MODE_W { w: self }
    }
    #[doc = "Bits 28:29 - Port 1 pin 30 control."]
    #[inline(always)]
    pub fn p1_30mode(&mut self) -> P1_30MODE_W {
        P1_30MODE_W { w: self }
    }
    #[doc = "Bits 30:31 - Port 1 pin 31 control."]
    #[inline(always)]
    pub fn p1_31mode(&mut self) -> P1_31MODE_W {
        P1_31MODE_W { w: self }
    }
}
