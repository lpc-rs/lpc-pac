#[doc = "Reader of register PINMODE_OD1"]
pub type R = crate::R<u32, super::PINMODE_OD1>;
#[doc = "Writer for register PINMODE_OD1"]
pub type W = crate::W<u32, super::PINMODE_OD1>;
#[doc = "Register PINMODE_OD1 `reset()`'s with value 0"]
impl crate::ResetValue for super::PINMODE_OD1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Port 1 pin 0 open drain mode control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P1_00OD_A {
    #[doc = "0: Normal. P1.0 pin is in the normal (not open drain) mode."]
    NORMAL = 0,
    #[doc = "1: Open-drain. P1.0 pin is in the open drain mode."]
    OPEN_DRAIN = 1,
}
impl From<P1_00OD_A> for bool {
    #[inline(always)]
    fn from(variant: P1_00OD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `P1_00OD`"]
pub type P1_00OD_R = crate::R<bool, P1_00OD_A>;
impl P1_00OD_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> P1_00OD_A {
        match self.bits {
            false => P1_00OD_A::NORMAL,
            true => P1_00OD_A::OPEN_DRAIN,
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == P1_00OD_A::NORMAL
    }
    #[doc = "Checks if the value of the field is `OPEN_DRAIN`"]
    #[inline(always)]
    pub fn is_open_drain(&self) -> bool {
        *self == P1_00OD_A::OPEN_DRAIN
    }
}
#[doc = "Write proxy for field `P1_00OD`"]
pub struct P1_00OD_W<'a> {
    w: &'a mut W,
}
impl<'a> P1_00OD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: P1_00OD_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Normal. P1.0 pin is in the normal (not open drain) mode."]
    #[inline(always)]
    pub fn normal(self) -> &'a mut W {
        self.variant(P1_00OD_A::NORMAL)
    }
    #[doc = "Open-drain. P1.0 pin is in the open drain mode."]
    #[inline(always)]
    pub fn open_drain(self) -> &'a mut W {
        self.variant(P1_00OD_A::OPEN_DRAIN)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
#[doc = "Port 1 pin 1 open drain mode control, see P1.00OD\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P1_01OD_A {
    #[doc = "0: Normal. P1.1 pin is in the normal (not open drain) mode."]
    NORMAL = 0,
    #[doc = "1: Open-drain. P1.1 pin is in the open drain mode."]
    OPEN_DRAIN = 1,
}
impl From<P1_01OD_A> for bool {
    #[inline(always)]
    fn from(variant: P1_01OD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `P1_01OD`"]
pub type P1_01OD_R = crate::R<bool, P1_01OD_A>;
impl P1_01OD_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> P1_01OD_A {
        match self.bits {
            false => P1_01OD_A::NORMAL,
            true => P1_01OD_A::OPEN_DRAIN,
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == P1_01OD_A::NORMAL
    }
    #[doc = "Checks if the value of the field is `OPEN_DRAIN`"]
    #[inline(always)]
    pub fn is_open_drain(&self) -> bool {
        *self == P1_01OD_A::OPEN_DRAIN
    }
}
#[doc = "Write proxy for field `P1_01OD`"]
pub struct P1_01OD_W<'a> {
    w: &'a mut W,
}
impl<'a> P1_01OD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: P1_01OD_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Normal. P1.1 pin is in the normal (not open drain) mode."]
    #[inline(always)]
    pub fn normal(self) -> &'a mut W {
        self.variant(P1_01OD_A::NORMAL)
    }
    #[doc = "Open-drain. P1.1 pin is in the open drain mode."]
    #[inline(always)]
    pub fn open_drain(self) -> &'a mut W {
        self.variant(P1_01OD_A::OPEN_DRAIN)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "Port 1 pin 4 open drain mode control, see P1.00OD\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P1_04OD_A {
    #[doc = "0: Normal. P1.4 pin is in the normal (not open drain) mode."]
    NORMAL = 0,
    #[doc = "1: Open-drain. P1.4 pin is in the open drain mode."]
    OPEN_DRAIN = 1,
}
impl From<P1_04OD_A> for bool {
    #[inline(always)]
    fn from(variant: P1_04OD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `P1_04OD`"]
pub type P1_04OD_R = crate::R<bool, P1_04OD_A>;
impl P1_04OD_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> P1_04OD_A {
        match self.bits {
            false => P1_04OD_A::NORMAL,
            true => P1_04OD_A::OPEN_DRAIN,
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == P1_04OD_A::NORMAL
    }
    #[doc = "Checks if the value of the field is `OPEN_DRAIN`"]
    #[inline(always)]
    pub fn is_open_drain(&self) -> bool {
        *self == P1_04OD_A::OPEN_DRAIN
    }
}
#[doc = "Write proxy for field `P1_04OD`"]
pub struct P1_04OD_W<'a> {
    w: &'a mut W,
}
impl<'a> P1_04OD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: P1_04OD_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Normal. P1.4 pin is in the normal (not open drain) mode."]
    #[inline(always)]
    pub fn normal(self) -> &'a mut W {
        self.variant(P1_04OD_A::NORMAL)
    }
    #[doc = "Open-drain. P1.4 pin is in the open drain mode."]
    #[inline(always)]
    pub fn open_drain(self) -> &'a mut W {
        self.variant(P1_04OD_A::OPEN_DRAIN)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
#[doc = "Port 1 pin 8 open drain mode control, see P1.00OD\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P1_08OD_A {
    #[doc = "0: Normal. P1.8 pin is in the normal (not open drain) mode."]
    NORMAL = 0,
    #[doc = "1: Open-drain. P1.8 pin is in the open drain mode."]
    OPEN_DRAIN = 1,
}
impl From<P1_08OD_A> for bool {
    #[inline(always)]
    fn from(variant: P1_08OD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `P1_08OD`"]
pub type P1_08OD_R = crate::R<bool, P1_08OD_A>;
impl P1_08OD_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> P1_08OD_A {
        match self.bits {
            false => P1_08OD_A::NORMAL,
            true => P1_08OD_A::OPEN_DRAIN,
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == P1_08OD_A::NORMAL
    }
    #[doc = "Checks if the value of the field is `OPEN_DRAIN`"]
    #[inline(always)]
    pub fn is_open_drain(&self) -> bool {
        *self == P1_08OD_A::OPEN_DRAIN
    }
}
#[doc = "Write proxy for field `P1_08OD`"]
pub struct P1_08OD_W<'a> {
    w: &'a mut W,
}
impl<'a> P1_08OD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: P1_08OD_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Normal. P1.8 pin is in the normal (not open drain) mode."]
    #[inline(always)]
    pub fn normal(self) -> &'a mut W {
        self.variant(P1_08OD_A::NORMAL)
    }
    #[doc = "Open-drain. P1.8 pin is in the open drain mode."]
    #[inline(always)]
    pub fn open_drain(self) -> &'a mut W {
        self.variant(P1_08OD_A::OPEN_DRAIN)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
#[doc = "Port 1 pin 9 open drain mode control, see P1.00OD\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P1_09OD_A {
    #[doc = "0: Normal. P1.9 pin is in the normal (not open drain) mode."]
    NORMAL = 0,
    #[doc = "1: Open-drain. P1.9 pin is in the open drain mode."]
    OPEN_DRAIN = 1,
}
impl From<P1_09OD_A> for bool {
    #[inline(always)]
    fn from(variant: P1_09OD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `P1_09OD`"]
pub type P1_09OD_R = crate::R<bool, P1_09OD_A>;
impl P1_09OD_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> P1_09OD_A {
        match self.bits {
            false => P1_09OD_A::NORMAL,
            true => P1_09OD_A::OPEN_DRAIN,
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == P1_09OD_A::NORMAL
    }
    #[doc = "Checks if the value of the field is `OPEN_DRAIN`"]
    #[inline(always)]
    pub fn is_open_drain(&self) -> bool {
        *self == P1_09OD_A::OPEN_DRAIN
    }
}
#[doc = "Write proxy for field `P1_09OD`"]
pub struct P1_09OD_W<'a> {
    w: &'a mut W,
}
impl<'a> P1_09OD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: P1_09OD_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Normal. P1.9 pin is in the normal (not open drain) mode."]
    #[inline(always)]
    pub fn normal(self) -> &'a mut W {
        self.variant(P1_09OD_A::NORMAL)
    }
    #[doc = "Open-drain. P1.9 pin is in the open drain mode."]
    #[inline(always)]
    pub fn open_drain(self) -> &'a mut W {
        self.variant(P1_09OD_A::OPEN_DRAIN)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
        self.w
    }
}
#[doc = "Port 1 pin 10 open drain mode control, see P1.00OD\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P1_10OD_A {
    #[doc = "0: Normal. P1.10 pin is in the normal (not open drain) mode."]
    NORMAL = 0,
    #[doc = "1: Open-drain. P1.10 pin is in the open drain mode."]
    OPEN_DRAIN = 1,
}
impl From<P1_10OD_A> for bool {
    #[inline(always)]
    fn from(variant: P1_10OD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `P1_10OD`"]
pub type P1_10OD_R = crate::R<bool, P1_10OD_A>;
impl P1_10OD_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> P1_10OD_A {
        match self.bits {
            false => P1_10OD_A::NORMAL,
            true => P1_10OD_A::OPEN_DRAIN,
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == P1_10OD_A::NORMAL
    }
    #[doc = "Checks if the value of the field is `OPEN_DRAIN`"]
    #[inline(always)]
    pub fn is_open_drain(&self) -> bool {
        *self == P1_10OD_A::OPEN_DRAIN
    }
}
#[doc = "Write proxy for field `P1_10OD`"]
pub struct P1_10OD_W<'a> {
    w: &'a mut W,
}
impl<'a> P1_10OD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: P1_10OD_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Normal. P1.10 pin is in the normal (not open drain) mode."]
    #[inline(always)]
    pub fn normal(self) -> &'a mut W {
        self.variant(P1_10OD_A::NORMAL)
    }
    #[doc = "Open-drain. P1.10 pin is in the open drain mode."]
    #[inline(always)]
    pub fn open_drain(self) -> &'a mut W {
        self.variant(P1_10OD_A::OPEN_DRAIN)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
        self.w
    }
}
#[doc = "Port 1 pin 14 open drain mode control, see P1.00OD\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P1_14OD_A {
    #[doc = "0: Normal. P1.14 pin is in the normal (not open drain) mode."]
    NORMAL = 0,
    #[doc = "1: Open-drain. P1.14 pin is in the open drain mode."]
    OPEN_DRAIN = 1,
}
impl From<P1_14OD_A> for bool {
    #[inline(always)]
    fn from(variant: P1_14OD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `P1_14OD`"]
pub type P1_14OD_R = crate::R<bool, P1_14OD_A>;
impl P1_14OD_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> P1_14OD_A {
        match self.bits {
            false => P1_14OD_A::NORMAL,
            true => P1_14OD_A::OPEN_DRAIN,
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == P1_14OD_A::NORMAL
    }
    #[doc = "Checks if the value of the field is `OPEN_DRAIN`"]
    #[inline(always)]
    pub fn is_open_drain(&self) -> bool {
        *self == P1_14OD_A::OPEN_DRAIN
    }
}
#[doc = "Write proxy for field `P1_14OD`"]
pub struct P1_14OD_W<'a> {
    w: &'a mut W,
}
impl<'a> P1_14OD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: P1_14OD_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Normal. P1.14 pin is in the normal (not open drain) mode."]
    #[inline(always)]
    pub fn normal(self) -> &'a mut W {
        self.variant(P1_14OD_A::NORMAL)
    }
    #[doc = "Open-drain. P1.14 pin is in the open drain mode."]
    #[inline(always)]
    pub fn open_drain(self) -> &'a mut W {
        self.variant(P1_14OD_A::OPEN_DRAIN)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 14)) | (((value as u32) & 0x01) << 14);
        self.w
    }
}
#[doc = "Port 1 pin 15 open drain mode control, see P1.00OD\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P1_15OD_A {
    #[doc = "0: Normal. P1.15 pin is in the normal (not open drain) mode."]
    NORMAL = 0,
    #[doc = "1: Open-drain. P1.15 pin is in the open drain mode."]
    OPEN_DRAIN = 1,
}
impl From<P1_15OD_A> for bool {
    #[inline(always)]
    fn from(variant: P1_15OD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `P1_15OD`"]
pub type P1_15OD_R = crate::R<bool, P1_15OD_A>;
impl P1_15OD_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> P1_15OD_A {
        match self.bits {
            false => P1_15OD_A::NORMAL,
            true => P1_15OD_A::OPEN_DRAIN,
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == P1_15OD_A::NORMAL
    }
    #[doc = "Checks if the value of the field is `OPEN_DRAIN`"]
    #[inline(always)]
    pub fn is_open_drain(&self) -> bool {
        *self == P1_15OD_A::OPEN_DRAIN
    }
}
#[doc = "Write proxy for field `P1_15OD`"]
pub struct P1_15OD_W<'a> {
    w: &'a mut W,
}
impl<'a> P1_15OD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: P1_15OD_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Normal. P1.15 pin is in the normal (not open drain) mode."]
    #[inline(always)]
    pub fn normal(self) -> &'a mut W {
        self.variant(P1_15OD_A::NORMAL)
    }
    #[doc = "Open-drain. P1.15 pin is in the open drain mode."]
    #[inline(always)]
    pub fn open_drain(self) -> &'a mut W {
        self.variant(P1_15OD_A::OPEN_DRAIN)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u32) & 0x01) << 15);
        self.w
    }
}
#[doc = "Port 1 pin 16 open drain mode control, see P1.00OD\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P1_16OD_A {
    #[doc = "0: Normal. P1.16 pin is in the normal (not open drain) mode."]
    NORMAL = 0,
    #[doc = "1: Open-drain. P1.16 pin is in the open drain mode."]
    OPEN_DRAIN = 1,
}
impl From<P1_16OD_A> for bool {
    #[inline(always)]
    fn from(variant: P1_16OD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `P1_16OD`"]
pub type P1_16OD_R = crate::R<bool, P1_16OD_A>;
impl P1_16OD_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> P1_16OD_A {
        match self.bits {
            false => P1_16OD_A::NORMAL,
            true => P1_16OD_A::OPEN_DRAIN,
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == P1_16OD_A::NORMAL
    }
    #[doc = "Checks if the value of the field is `OPEN_DRAIN`"]
    #[inline(always)]
    pub fn is_open_drain(&self) -> bool {
        *self == P1_16OD_A::OPEN_DRAIN
    }
}
#[doc = "Write proxy for field `P1_16OD`"]
pub struct P1_16OD_W<'a> {
    w: &'a mut W,
}
impl<'a> P1_16OD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: P1_16OD_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Normal. P1.16 pin is in the normal (not open drain) mode."]
    #[inline(always)]
    pub fn normal(self) -> &'a mut W {
        self.variant(P1_16OD_A::NORMAL)
    }
    #[doc = "Open-drain. P1.16 pin is in the open drain mode."]
    #[inline(always)]
    pub fn open_drain(self) -> &'a mut W {
        self.variant(P1_16OD_A::OPEN_DRAIN)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
        self.w
    }
}
#[doc = "Port 1 pin 17 open drain mode control, see P1.00OD\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P1_17OD_A {
    #[doc = "0: Normal. P1.17 pin is in the normal (not open drain) mode."]
    NORMAL = 0,
    #[doc = "1: Open-drain. P1.17 pin is in the open drain mode."]
    OPEN_DRAIN = 1,
}
impl From<P1_17OD_A> for bool {
    #[inline(always)]
    fn from(variant: P1_17OD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `P1_17OD`"]
pub type P1_17OD_R = crate::R<bool, P1_17OD_A>;
impl P1_17OD_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> P1_17OD_A {
        match self.bits {
            false => P1_17OD_A::NORMAL,
            true => P1_17OD_A::OPEN_DRAIN,
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == P1_17OD_A::NORMAL
    }
    #[doc = "Checks if the value of the field is `OPEN_DRAIN`"]
    #[inline(always)]
    pub fn is_open_drain(&self) -> bool {
        *self == P1_17OD_A::OPEN_DRAIN
    }
}
#[doc = "Write proxy for field `P1_17OD`"]
pub struct P1_17OD_W<'a> {
    w: &'a mut W,
}
impl<'a> P1_17OD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: P1_17OD_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Normal. P1.17 pin is in the normal (not open drain) mode."]
    #[inline(always)]
    pub fn normal(self) -> &'a mut W {
        self.variant(P1_17OD_A::NORMAL)
    }
    #[doc = "Open-drain. P1.17 pin is in the open drain mode."]
    #[inline(always)]
    pub fn open_drain(self) -> &'a mut W {
        self.variant(P1_17OD_A::OPEN_DRAIN)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 17)) | (((value as u32) & 0x01) << 17);
        self.w
    }
}
#[doc = "Port 1 pin 18 open drain mode control, see P1.00OD\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P1_18OD_A {
    #[doc = "0: Normal. P1.18 pin is in the normal (not open drain) mode."]
    NORMAL = 0,
    #[doc = "1: Open-drain. P1.18 pin is in the open drain mode."]
    OPEN_DRAIN = 1,
}
impl From<P1_18OD_A> for bool {
    #[inline(always)]
    fn from(variant: P1_18OD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `P1_18OD`"]
pub type P1_18OD_R = crate::R<bool, P1_18OD_A>;
impl P1_18OD_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> P1_18OD_A {
        match self.bits {
            false => P1_18OD_A::NORMAL,
            true => P1_18OD_A::OPEN_DRAIN,
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == P1_18OD_A::NORMAL
    }
    #[doc = "Checks if the value of the field is `OPEN_DRAIN`"]
    #[inline(always)]
    pub fn is_open_drain(&self) -> bool {
        *self == P1_18OD_A::OPEN_DRAIN
    }
}
#[doc = "Write proxy for field `P1_18OD`"]
pub struct P1_18OD_W<'a> {
    w: &'a mut W,
}
impl<'a> P1_18OD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: P1_18OD_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Normal. P1.18 pin is in the normal (not open drain) mode."]
    #[inline(always)]
    pub fn normal(self) -> &'a mut W {
        self.variant(P1_18OD_A::NORMAL)
    }
    #[doc = "Open-drain. P1.18 pin is in the open drain mode."]
    #[inline(always)]
    pub fn open_drain(self) -> &'a mut W {
        self.variant(P1_18OD_A::OPEN_DRAIN)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 18)) | (((value as u32) & 0x01) << 18);
        self.w
    }
}
#[doc = "Port 1 pin 19 open drain mode control, see P1.00OD\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P1_19OD_A {
    #[doc = "0: Normal. P1.19 pin is in the normal (not open drain) mode."]
    NORMAL = 0,
    #[doc = "1: Open-drain. P1.19 pin is in the open drain mode."]
    OPEN_DRAIN = 1,
}
impl From<P1_19OD_A> for bool {
    #[inline(always)]
    fn from(variant: P1_19OD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `P1_19OD`"]
pub type P1_19OD_R = crate::R<bool, P1_19OD_A>;
impl P1_19OD_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> P1_19OD_A {
        match self.bits {
            false => P1_19OD_A::NORMAL,
            true => P1_19OD_A::OPEN_DRAIN,
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == P1_19OD_A::NORMAL
    }
    #[doc = "Checks if the value of the field is `OPEN_DRAIN`"]
    #[inline(always)]
    pub fn is_open_drain(&self) -> bool {
        *self == P1_19OD_A::OPEN_DRAIN
    }
}
#[doc = "Write proxy for field `P1_19OD`"]
pub struct P1_19OD_W<'a> {
    w: &'a mut W,
}
impl<'a> P1_19OD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: P1_19OD_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Normal. P1.19 pin is in the normal (not open drain) mode."]
    #[inline(always)]
    pub fn normal(self) -> &'a mut W {
        self.variant(P1_19OD_A::NORMAL)
    }
    #[doc = "Open-drain. P1.19 pin is in the open drain mode."]
    #[inline(always)]
    pub fn open_drain(self) -> &'a mut W {
        self.variant(P1_19OD_A::OPEN_DRAIN)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 19)) | (((value as u32) & 0x01) << 19);
        self.w
    }
}
#[doc = "Port 1 pin 20open drain mode control, see P1.00OD\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P1_20OD_A {
    #[doc = "0: Normal. P1.20 pin is in the normal (not open drain) mode."]
    NORMAL = 0,
    #[doc = "1: Open-drain. P1.20 pin is in the open drain mode."]
    OPEN_DRAIN = 1,
}
impl From<P1_20OD_A> for bool {
    #[inline(always)]
    fn from(variant: P1_20OD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `P1_20OD`"]
pub type P1_20OD_R = crate::R<bool, P1_20OD_A>;
impl P1_20OD_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> P1_20OD_A {
        match self.bits {
            false => P1_20OD_A::NORMAL,
            true => P1_20OD_A::OPEN_DRAIN,
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == P1_20OD_A::NORMAL
    }
    #[doc = "Checks if the value of the field is `OPEN_DRAIN`"]
    #[inline(always)]
    pub fn is_open_drain(&self) -> bool {
        *self == P1_20OD_A::OPEN_DRAIN
    }
}
#[doc = "Write proxy for field `P1_20OD`"]
pub struct P1_20OD_W<'a> {
    w: &'a mut W,
}
impl<'a> P1_20OD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: P1_20OD_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Normal. P1.20 pin is in the normal (not open drain) mode."]
    #[inline(always)]
    pub fn normal(self) -> &'a mut W {
        self.variant(P1_20OD_A::NORMAL)
    }
    #[doc = "Open-drain. P1.20 pin is in the open drain mode."]
    #[inline(always)]
    pub fn open_drain(self) -> &'a mut W {
        self.variant(P1_20OD_A::OPEN_DRAIN)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 20)) | (((value as u32) & 0x01) << 20);
        self.w
    }
}
#[doc = "Port 1 pin 21 open drain mode control, see P1.00OD\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P1_21OD_A {
    #[doc = "0: Normal. P1.21 pin is in the normal (not open drain) mode."]
    NORMAL = 0,
    #[doc = "1: Open-drain. P1.21 pin is in the open drain mode."]
    OPEN_DRAIN = 1,
}
impl From<P1_21OD_A> for bool {
    #[inline(always)]
    fn from(variant: P1_21OD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `P1_21OD`"]
pub type P1_21OD_R = crate::R<bool, P1_21OD_A>;
impl P1_21OD_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> P1_21OD_A {
        match self.bits {
            false => P1_21OD_A::NORMAL,
            true => P1_21OD_A::OPEN_DRAIN,
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == P1_21OD_A::NORMAL
    }
    #[doc = "Checks if the value of the field is `OPEN_DRAIN`"]
    #[inline(always)]
    pub fn is_open_drain(&self) -> bool {
        *self == P1_21OD_A::OPEN_DRAIN
    }
}
#[doc = "Write proxy for field `P1_21OD`"]
pub struct P1_21OD_W<'a> {
    w: &'a mut W,
}
impl<'a> P1_21OD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: P1_21OD_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Normal. P1.21 pin is in the normal (not open drain) mode."]
    #[inline(always)]
    pub fn normal(self) -> &'a mut W {
        self.variant(P1_21OD_A::NORMAL)
    }
    #[doc = "Open-drain. P1.21 pin is in the open drain mode."]
    #[inline(always)]
    pub fn open_drain(self) -> &'a mut W {
        self.variant(P1_21OD_A::OPEN_DRAIN)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 21)) | (((value as u32) & 0x01) << 21);
        self.w
    }
}
#[doc = "Port 1 pin 22 open drain mode control, see P1.00OD\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P1_22OD_A {
    #[doc = "0: Normal. P1.22 pin is in the normal (not open drain) mode."]
    NORMAL = 0,
    #[doc = "1: Open-drain. P1.22 pin is in the open drain mode."]
    OPEN_DRAIN = 1,
}
impl From<P1_22OD_A> for bool {
    #[inline(always)]
    fn from(variant: P1_22OD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `P1_22OD`"]
pub type P1_22OD_R = crate::R<bool, P1_22OD_A>;
impl P1_22OD_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> P1_22OD_A {
        match self.bits {
            false => P1_22OD_A::NORMAL,
            true => P1_22OD_A::OPEN_DRAIN,
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == P1_22OD_A::NORMAL
    }
    #[doc = "Checks if the value of the field is `OPEN_DRAIN`"]
    #[inline(always)]
    pub fn is_open_drain(&self) -> bool {
        *self == P1_22OD_A::OPEN_DRAIN
    }
}
#[doc = "Write proxy for field `P1_22OD`"]
pub struct P1_22OD_W<'a> {
    w: &'a mut W,
}
impl<'a> P1_22OD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: P1_22OD_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Normal. P1.22 pin is in the normal (not open drain) mode."]
    #[inline(always)]
    pub fn normal(self) -> &'a mut W {
        self.variant(P1_22OD_A::NORMAL)
    }
    #[doc = "Open-drain. P1.22 pin is in the open drain mode."]
    #[inline(always)]
    pub fn open_drain(self) -> &'a mut W {
        self.variant(P1_22OD_A::OPEN_DRAIN)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 22)) | (((value as u32) & 0x01) << 22);
        self.w
    }
}
#[doc = "Port 1 pin 23 open drain mode control, see P1.00OD\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P1_23OD_A {
    #[doc = "0: Normal. P1.23 pin is in the normal (not open drain) mode."]
    NORMAL = 0,
    #[doc = "1: Open-drain. P1.23 pin is in the open drain mode."]
    OPEN_DRAIN = 1,
}
impl From<P1_23OD_A> for bool {
    #[inline(always)]
    fn from(variant: P1_23OD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `P1_23OD`"]
pub type P1_23OD_R = crate::R<bool, P1_23OD_A>;
impl P1_23OD_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> P1_23OD_A {
        match self.bits {
            false => P1_23OD_A::NORMAL,
            true => P1_23OD_A::OPEN_DRAIN,
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == P1_23OD_A::NORMAL
    }
    #[doc = "Checks if the value of the field is `OPEN_DRAIN`"]
    #[inline(always)]
    pub fn is_open_drain(&self) -> bool {
        *self == P1_23OD_A::OPEN_DRAIN
    }
}
#[doc = "Write proxy for field `P1_23OD`"]
pub struct P1_23OD_W<'a> {
    w: &'a mut W,
}
impl<'a> P1_23OD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: P1_23OD_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Normal. P1.23 pin is in the normal (not open drain) mode."]
    #[inline(always)]
    pub fn normal(self) -> &'a mut W {
        self.variant(P1_23OD_A::NORMAL)
    }
    #[doc = "Open-drain. P1.23 pin is in the open drain mode."]
    #[inline(always)]
    pub fn open_drain(self) -> &'a mut W {
        self.variant(P1_23OD_A::OPEN_DRAIN)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 23)) | (((value as u32) & 0x01) << 23);
        self.w
    }
}
#[doc = "Port 1 pin 24open drain mode control, see P1.00OD\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P1_24OD_A {
    #[doc = "0: Normal. P1.24 pin is in the normal (not open drain) mode."]
    NORMAL = 0,
    #[doc = "1: Open-drain. P1.24 pin is in the open drain mode."]
    OPEN_DRAIN = 1,
}
impl From<P1_24OD_A> for bool {
    #[inline(always)]
    fn from(variant: P1_24OD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `P1_24OD`"]
pub type P1_24OD_R = crate::R<bool, P1_24OD_A>;
impl P1_24OD_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> P1_24OD_A {
        match self.bits {
            false => P1_24OD_A::NORMAL,
            true => P1_24OD_A::OPEN_DRAIN,
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == P1_24OD_A::NORMAL
    }
    #[doc = "Checks if the value of the field is `OPEN_DRAIN`"]
    #[inline(always)]
    pub fn is_open_drain(&self) -> bool {
        *self == P1_24OD_A::OPEN_DRAIN
    }
}
#[doc = "Write proxy for field `P1_24OD`"]
pub struct P1_24OD_W<'a> {
    w: &'a mut W,
}
impl<'a> P1_24OD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: P1_24OD_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Normal. P1.24 pin is in the normal (not open drain) mode."]
    #[inline(always)]
    pub fn normal(self) -> &'a mut W {
        self.variant(P1_24OD_A::NORMAL)
    }
    #[doc = "Open-drain. P1.24 pin is in the open drain mode."]
    #[inline(always)]
    pub fn open_drain(self) -> &'a mut W {
        self.variant(P1_24OD_A::OPEN_DRAIN)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 24)) | (((value as u32) & 0x01) << 24);
        self.w
    }
}
#[doc = "Port 1 pin 25 open drain mode control, see P1.00OD\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P1_25OD_A {
    #[doc = "0: Normal. P1.25 pin is in the normal (not open drain) mode."]
    NORMAL = 0,
    #[doc = "1: Open-drain. P1.25 pin is in the open drain mode."]
    OPEN_DRAIN = 1,
}
impl From<P1_25OD_A> for bool {
    #[inline(always)]
    fn from(variant: P1_25OD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `P1_25OD`"]
pub type P1_25OD_R = crate::R<bool, P1_25OD_A>;
impl P1_25OD_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> P1_25OD_A {
        match self.bits {
            false => P1_25OD_A::NORMAL,
            true => P1_25OD_A::OPEN_DRAIN,
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == P1_25OD_A::NORMAL
    }
    #[doc = "Checks if the value of the field is `OPEN_DRAIN`"]
    #[inline(always)]
    pub fn is_open_drain(&self) -> bool {
        *self == P1_25OD_A::OPEN_DRAIN
    }
}
#[doc = "Write proxy for field `P1_25OD`"]
pub struct P1_25OD_W<'a> {
    w: &'a mut W,
}
impl<'a> P1_25OD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: P1_25OD_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Normal. P1.25 pin is in the normal (not open drain) mode."]
    #[inline(always)]
    pub fn normal(self) -> &'a mut W {
        self.variant(P1_25OD_A::NORMAL)
    }
    #[doc = "Open-drain. P1.25 pin is in the open drain mode."]
    #[inline(always)]
    pub fn open_drain(self) -> &'a mut W {
        self.variant(P1_25OD_A::OPEN_DRAIN)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 25)) | (((value as u32) & 0x01) << 25);
        self.w
    }
}
#[doc = "Port 1 pin 26 open drain mode control, see P1.00OD\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P1_26OD_A {
    #[doc = "0: Normal. P1.26 pin is in the normal (not open drain) mode."]
    NORMAL = 0,
    #[doc = "1: Open-drain. P1.26 pin is in the open drain mode."]
    OPEN_DRAIN = 1,
}
impl From<P1_26OD_A> for bool {
    #[inline(always)]
    fn from(variant: P1_26OD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `P1_26OD`"]
pub type P1_26OD_R = crate::R<bool, P1_26OD_A>;
impl P1_26OD_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> P1_26OD_A {
        match self.bits {
            false => P1_26OD_A::NORMAL,
            true => P1_26OD_A::OPEN_DRAIN,
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == P1_26OD_A::NORMAL
    }
    #[doc = "Checks if the value of the field is `OPEN_DRAIN`"]
    #[inline(always)]
    pub fn is_open_drain(&self) -> bool {
        *self == P1_26OD_A::OPEN_DRAIN
    }
}
#[doc = "Write proxy for field `P1_26OD`"]
pub struct P1_26OD_W<'a> {
    w: &'a mut W,
}
impl<'a> P1_26OD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: P1_26OD_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Normal. P1.26 pin is in the normal (not open drain) mode."]
    #[inline(always)]
    pub fn normal(self) -> &'a mut W {
        self.variant(P1_26OD_A::NORMAL)
    }
    #[doc = "Open-drain. P1.26 pin is in the open drain mode."]
    #[inline(always)]
    pub fn open_drain(self) -> &'a mut W {
        self.variant(P1_26OD_A::OPEN_DRAIN)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 26)) | (((value as u32) & 0x01) << 26);
        self.w
    }
}
#[doc = "Port 1 pin 27 open drain mode control, see P1.00OD\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P1_27OD_A {
    #[doc = "0: Normal. P1.27 pin is in the normal (not open drain) mode."]
    NORMAL = 0,
    #[doc = "1: Open-drain. P1.27 pin is in the open drain mode."]
    OPEN_DRAIN = 1,
}
impl From<P1_27OD_A> for bool {
    #[inline(always)]
    fn from(variant: P1_27OD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `P1_27OD`"]
pub type P1_27OD_R = crate::R<bool, P1_27OD_A>;
impl P1_27OD_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> P1_27OD_A {
        match self.bits {
            false => P1_27OD_A::NORMAL,
            true => P1_27OD_A::OPEN_DRAIN,
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == P1_27OD_A::NORMAL
    }
    #[doc = "Checks if the value of the field is `OPEN_DRAIN`"]
    #[inline(always)]
    pub fn is_open_drain(&self) -> bool {
        *self == P1_27OD_A::OPEN_DRAIN
    }
}
#[doc = "Write proxy for field `P1_27OD`"]
pub struct P1_27OD_W<'a> {
    w: &'a mut W,
}
impl<'a> P1_27OD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: P1_27OD_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Normal. P1.27 pin is in the normal (not open drain) mode."]
    #[inline(always)]
    pub fn normal(self) -> &'a mut W {
        self.variant(P1_27OD_A::NORMAL)
    }
    #[doc = "Open-drain. P1.27 pin is in the open drain mode."]
    #[inline(always)]
    pub fn open_drain(self) -> &'a mut W {
        self.variant(P1_27OD_A::OPEN_DRAIN)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 27)) | (((value as u32) & 0x01) << 27);
        self.w
    }
}
#[doc = "Port 1 pin 28 open drain mode control, see P1.00OD\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P1_28OD_A {
    #[doc = "0: Normal. P1.28 pin is in the normal (not open drain) mode."]
    NORMAL = 0,
    #[doc = "1: Open-drain. P1.28 pin is in the open drain mode."]
    OPEN_DRAIN = 1,
}
impl From<P1_28OD_A> for bool {
    #[inline(always)]
    fn from(variant: P1_28OD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `P1_28OD`"]
pub type P1_28OD_R = crate::R<bool, P1_28OD_A>;
impl P1_28OD_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> P1_28OD_A {
        match self.bits {
            false => P1_28OD_A::NORMAL,
            true => P1_28OD_A::OPEN_DRAIN,
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == P1_28OD_A::NORMAL
    }
    #[doc = "Checks if the value of the field is `OPEN_DRAIN`"]
    #[inline(always)]
    pub fn is_open_drain(&self) -> bool {
        *self == P1_28OD_A::OPEN_DRAIN
    }
}
#[doc = "Write proxy for field `P1_28OD`"]
pub struct P1_28OD_W<'a> {
    w: &'a mut W,
}
impl<'a> P1_28OD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: P1_28OD_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Normal. P1.28 pin is in the normal (not open drain) mode."]
    #[inline(always)]
    pub fn normal(self) -> &'a mut W {
        self.variant(P1_28OD_A::NORMAL)
    }
    #[doc = "Open-drain. P1.28 pin is in the open drain mode."]
    #[inline(always)]
    pub fn open_drain(self) -> &'a mut W {
        self.variant(P1_28OD_A::OPEN_DRAIN)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 28)) | (((value as u32) & 0x01) << 28);
        self.w
    }
}
#[doc = "Port 1 pin 29 open drain mode control, see P1.00OD\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P1_29OD_A {
    #[doc = "0: Normal. P1.29 pin is in the normal (not open drain) mode."]
    NORMAL = 0,
    #[doc = "1: Open-drain. P1.29 pin is in the open drain mode."]
    OPEN_DRAIN = 1,
}
impl From<P1_29OD_A> for bool {
    #[inline(always)]
    fn from(variant: P1_29OD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `P1_29OD`"]
pub type P1_29OD_R = crate::R<bool, P1_29OD_A>;
impl P1_29OD_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> P1_29OD_A {
        match self.bits {
            false => P1_29OD_A::NORMAL,
            true => P1_29OD_A::OPEN_DRAIN,
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == P1_29OD_A::NORMAL
    }
    #[doc = "Checks if the value of the field is `OPEN_DRAIN`"]
    #[inline(always)]
    pub fn is_open_drain(&self) -> bool {
        *self == P1_29OD_A::OPEN_DRAIN
    }
}
#[doc = "Write proxy for field `P1_29OD`"]
pub struct P1_29OD_W<'a> {
    w: &'a mut W,
}
impl<'a> P1_29OD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: P1_29OD_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Normal. P1.29 pin is in the normal (not open drain) mode."]
    #[inline(always)]
    pub fn normal(self) -> &'a mut W {
        self.variant(P1_29OD_A::NORMAL)
    }
    #[doc = "Open-drain. P1.29 pin is in the open drain mode."]
    #[inline(always)]
    pub fn open_drain(self) -> &'a mut W {
        self.variant(P1_29OD_A::OPEN_DRAIN)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 29)) | (((value as u32) & 0x01) << 29);
        self.w
    }
}
#[doc = "Port 1 pin 30 open drain mode control, see P1.00OD\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P1_30OD_A {
    #[doc = "0: Normal. P1.30 pin is in the normal (not open drain) mode."]
    NORMAL = 0,
    #[doc = "1: Open-drain. P1.30 pin is in the open drain mode."]
    OPEN_DRAIN = 1,
}
impl From<P1_30OD_A> for bool {
    #[inline(always)]
    fn from(variant: P1_30OD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `P1_30OD`"]
pub type P1_30OD_R = crate::R<bool, P1_30OD_A>;
impl P1_30OD_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> P1_30OD_A {
        match self.bits {
            false => P1_30OD_A::NORMAL,
            true => P1_30OD_A::OPEN_DRAIN,
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == P1_30OD_A::NORMAL
    }
    #[doc = "Checks if the value of the field is `OPEN_DRAIN`"]
    #[inline(always)]
    pub fn is_open_drain(&self) -> bool {
        *self == P1_30OD_A::OPEN_DRAIN
    }
}
#[doc = "Write proxy for field `P1_30OD`"]
pub struct P1_30OD_W<'a> {
    w: &'a mut W,
}
impl<'a> P1_30OD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: P1_30OD_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Normal. P1.30 pin is in the normal (not open drain) mode."]
    #[inline(always)]
    pub fn normal(self) -> &'a mut W {
        self.variant(P1_30OD_A::NORMAL)
    }
    #[doc = "Open-drain. P1.30 pin is in the open drain mode."]
    #[inline(always)]
    pub fn open_drain(self) -> &'a mut W {
        self.variant(P1_30OD_A::OPEN_DRAIN)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 30)) | (((value as u32) & 0x01) << 30);
        self.w
    }
}
#[doc = "Port 1 pin 31 open drain mode control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P1_31OD_A {
    #[doc = "0: Normal. P1.31 pin is in the normal (not open drain) mode."]
    NORMAL = 0,
    #[doc = "1: Open-drain. P1.31 pin is in the open drain mode."]
    OPEN_DRAIN = 1,
}
impl From<P1_31OD_A> for bool {
    #[inline(always)]
    fn from(variant: P1_31OD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `P1_31OD`"]
pub type P1_31OD_R = crate::R<bool, P1_31OD_A>;
impl P1_31OD_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> P1_31OD_A {
        match self.bits {
            false => P1_31OD_A::NORMAL,
            true => P1_31OD_A::OPEN_DRAIN,
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == P1_31OD_A::NORMAL
    }
    #[doc = "Checks if the value of the field is `OPEN_DRAIN`"]
    #[inline(always)]
    pub fn is_open_drain(&self) -> bool {
        *self == P1_31OD_A::OPEN_DRAIN
    }
}
#[doc = "Write proxy for field `P1_31OD`"]
pub struct P1_31OD_W<'a> {
    w: &'a mut W,
}
impl<'a> P1_31OD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: P1_31OD_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Normal. P1.31 pin is in the normal (not open drain) mode."]
    #[inline(always)]
    pub fn normal(self) -> &'a mut W {
        self.variant(P1_31OD_A::NORMAL)
    }
    #[doc = "Open-drain. P1.31 pin is in the open drain mode."]
    #[inline(always)]
    pub fn open_drain(self) -> &'a mut W {
        self.variant(P1_31OD_A::OPEN_DRAIN)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 31)) | (((value as u32) & 0x01) << 31);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Port 1 pin 0 open drain mode control."]
    #[inline(always)]
    pub fn p1_00od(&self) -> P1_00OD_R {
        P1_00OD_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Port 1 pin 1 open drain mode control, see P1.00OD"]
    #[inline(always)]
    pub fn p1_01od(&self) -> P1_01OD_R {
        P1_01OD_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Port 1 pin 4 open drain mode control, see P1.00OD"]
    #[inline(always)]
    pub fn p1_04od(&self) -> P1_04OD_R {
        P1_04OD_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Port 1 pin 8 open drain mode control, see P1.00OD"]
    #[inline(always)]
    pub fn p1_08od(&self) -> P1_08OD_R {
        P1_08OD_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Port 1 pin 9 open drain mode control, see P1.00OD"]
    #[inline(always)]
    pub fn p1_09od(&self) -> P1_09OD_R {
        P1_09OD_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Port 1 pin 10 open drain mode control, see P1.00OD"]
    #[inline(always)]
    pub fn p1_10od(&self) -> P1_10OD_R {
        P1_10OD_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Port 1 pin 14 open drain mode control, see P1.00OD"]
    #[inline(always)]
    pub fn p1_14od(&self) -> P1_14OD_R {
        P1_14OD_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Port 1 pin 15 open drain mode control, see P1.00OD"]
    #[inline(always)]
    pub fn p1_15od(&self) -> P1_15OD_R {
        P1_15OD_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Port 1 pin 16 open drain mode control, see P1.00OD"]
    #[inline(always)]
    pub fn p1_16od(&self) -> P1_16OD_R {
        P1_16OD_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Port 1 pin 17 open drain mode control, see P1.00OD"]
    #[inline(always)]
    pub fn p1_17od(&self) -> P1_17OD_R {
        P1_17OD_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Port 1 pin 18 open drain mode control, see P1.00OD"]
    #[inline(always)]
    pub fn p1_18od(&self) -> P1_18OD_R {
        P1_18OD_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Port 1 pin 19 open drain mode control, see P1.00OD"]
    #[inline(always)]
    pub fn p1_19od(&self) -> P1_19OD_R {
        P1_19OD_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Port 1 pin 20open drain mode control, see P1.00OD"]
    #[inline(always)]
    pub fn p1_20od(&self) -> P1_20OD_R {
        P1_20OD_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Port 1 pin 21 open drain mode control, see P1.00OD"]
    #[inline(always)]
    pub fn p1_21od(&self) -> P1_21OD_R {
        P1_21OD_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Port 1 pin 22 open drain mode control, see P1.00OD"]
    #[inline(always)]
    pub fn p1_22od(&self) -> P1_22OD_R {
        P1_22OD_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - Port 1 pin 23 open drain mode control, see P1.00OD"]
    #[inline(always)]
    pub fn p1_23od(&self) -> P1_23OD_R {
        P1_23OD_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Port 1 pin 24open drain mode control, see P1.00OD"]
    #[inline(always)]
    pub fn p1_24od(&self) -> P1_24OD_R {
        P1_24OD_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Port 1 pin 25 open drain mode control, see P1.00OD"]
    #[inline(always)]
    pub fn p1_25od(&self) -> P1_25OD_R {
        P1_25OD_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - Port 1 pin 26 open drain mode control, see P1.00OD"]
    #[inline(always)]
    pub fn p1_26od(&self) -> P1_26OD_R {
        P1_26OD_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - Port 1 pin 27 open drain mode control, see P1.00OD"]
    #[inline(always)]
    pub fn p1_27od(&self) -> P1_27OD_R {
        P1_27OD_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - Port 1 pin 28 open drain mode control, see P1.00OD"]
    #[inline(always)]
    pub fn p1_28od(&self) -> P1_28OD_R {
        P1_28OD_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - Port 1 pin 29 open drain mode control, see P1.00OD"]
    #[inline(always)]
    pub fn p1_29od(&self) -> P1_29OD_R {
        P1_29OD_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - Port 1 pin 30 open drain mode control, see P1.00OD"]
    #[inline(always)]
    pub fn p1_30od(&self) -> P1_30OD_R {
        P1_30OD_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Port 1 pin 31 open drain mode control."]
    #[inline(always)]
    pub fn p1_31od(&self) -> P1_31OD_R {
        P1_31OD_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Port 1 pin 0 open drain mode control."]
    #[inline(always)]
    pub fn p1_00od(&mut self) -> P1_00OD_W {
        P1_00OD_W { w: self }
    }
    #[doc = "Bit 1 - Port 1 pin 1 open drain mode control, see P1.00OD"]
    #[inline(always)]
    pub fn p1_01od(&mut self) -> P1_01OD_W {
        P1_01OD_W { w: self }
    }
    #[doc = "Bit 4 - Port 1 pin 4 open drain mode control, see P1.00OD"]
    #[inline(always)]
    pub fn p1_04od(&mut self) -> P1_04OD_W {
        P1_04OD_W { w: self }
    }
    #[doc = "Bit 8 - Port 1 pin 8 open drain mode control, see P1.00OD"]
    #[inline(always)]
    pub fn p1_08od(&mut self) -> P1_08OD_W {
        P1_08OD_W { w: self }
    }
    #[doc = "Bit 9 - Port 1 pin 9 open drain mode control, see P1.00OD"]
    #[inline(always)]
    pub fn p1_09od(&mut self) -> P1_09OD_W {
        P1_09OD_W { w: self }
    }
    #[doc = "Bit 10 - Port 1 pin 10 open drain mode control, see P1.00OD"]
    #[inline(always)]
    pub fn p1_10od(&mut self) -> P1_10OD_W {
        P1_10OD_W { w: self }
    }
    #[doc = "Bit 14 - Port 1 pin 14 open drain mode control, see P1.00OD"]
    #[inline(always)]
    pub fn p1_14od(&mut self) -> P1_14OD_W {
        P1_14OD_W { w: self }
    }
    #[doc = "Bit 15 - Port 1 pin 15 open drain mode control, see P1.00OD"]
    #[inline(always)]
    pub fn p1_15od(&mut self) -> P1_15OD_W {
        P1_15OD_W { w: self }
    }
    #[doc = "Bit 16 - Port 1 pin 16 open drain mode control, see P1.00OD"]
    #[inline(always)]
    pub fn p1_16od(&mut self) -> P1_16OD_W {
        P1_16OD_W { w: self }
    }
    #[doc = "Bit 17 - Port 1 pin 17 open drain mode control, see P1.00OD"]
    #[inline(always)]
    pub fn p1_17od(&mut self) -> P1_17OD_W {
        P1_17OD_W { w: self }
    }
    #[doc = "Bit 18 - Port 1 pin 18 open drain mode control, see P1.00OD"]
    #[inline(always)]
    pub fn p1_18od(&mut self) -> P1_18OD_W {
        P1_18OD_W { w: self }
    }
    #[doc = "Bit 19 - Port 1 pin 19 open drain mode control, see P1.00OD"]
    #[inline(always)]
    pub fn p1_19od(&mut self) -> P1_19OD_W {
        P1_19OD_W { w: self }
    }
    #[doc = "Bit 20 - Port 1 pin 20open drain mode control, see P1.00OD"]
    #[inline(always)]
    pub fn p1_20od(&mut self) -> P1_20OD_W {
        P1_20OD_W { w: self }
    }
    #[doc = "Bit 21 - Port 1 pin 21 open drain mode control, see P1.00OD"]
    #[inline(always)]
    pub fn p1_21od(&mut self) -> P1_21OD_W {
        P1_21OD_W { w: self }
    }
    #[doc = "Bit 22 - Port 1 pin 22 open drain mode control, see P1.00OD"]
    #[inline(always)]
    pub fn p1_22od(&mut self) -> P1_22OD_W {
        P1_22OD_W { w: self }
    }
    #[doc = "Bit 23 - Port 1 pin 23 open drain mode control, see P1.00OD"]
    #[inline(always)]
    pub fn p1_23od(&mut self) -> P1_23OD_W {
        P1_23OD_W { w: self }
    }
    #[doc = "Bit 24 - Port 1 pin 24open drain mode control, see P1.00OD"]
    #[inline(always)]
    pub fn p1_24od(&mut self) -> P1_24OD_W {
        P1_24OD_W { w: self }
    }
    #[doc = "Bit 25 - Port 1 pin 25 open drain mode control, see P1.00OD"]
    #[inline(always)]
    pub fn p1_25od(&mut self) -> P1_25OD_W {
        P1_25OD_W { w: self }
    }
    #[doc = "Bit 26 - Port 1 pin 26 open drain mode control, see P1.00OD"]
    #[inline(always)]
    pub fn p1_26od(&mut self) -> P1_26OD_W {
        P1_26OD_W { w: self }
    }
    #[doc = "Bit 27 - Port 1 pin 27 open drain mode control, see P1.00OD"]
    #[inline(always)]
    pub fn p1_27od(&mut self) -> P1_27OD_W {
        P1_27OD_W { w: self }
    }
    #[doc = "Bit 28 - Port 1 pin 28 open drain mode control, see P1.00OD"]
    #[inline(always)]
    pub fn p1_28od(&mut self) -> P1_28OD_W {
        P1_28OD_W { w: self }
    }
    #[doc = "Bit 29 - Port 1 pin 29 open drain mode control, see P1.00OD"]
    #[inline(always)]
    pub fn p1_29od(&mut self) -> P1_29OD_W {
        P1_29OD_W { w: self }
    }
    #[doc = "Bit 30 - Port 1 pin 30 open drain mode control, see P1.00OD"]
    #[inline(always)]
    pub fn p1_30od(&mut self) -> P1_30OD_W {
        P1_30OD_W { w: self }
    }
    #[doc = "Bit 31 - Port 1 pin 31 open drain mode control."]
    #[inline(always)]
    pub fn p1_31od(&mut self) -> P1_31OD_W {
        P1_31OD_W { w: self }
    }
}
