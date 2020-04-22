#[doc = "Reader of register PINMODE_OD2"]
pub type R = crate::R<u32, super::PINMODE_OD2>;
#[doc = "Writer for register PINMODE_OD2"]
pub type W = crate::W<u32, super::PINMODE_OD2>;
#[doc = "Register PINMODE_OD2 `reset()`'s with value 0"]
impl crate::ResetValue for super::PINMODE_OD2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Port 2 pin 0 open drain mode control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P2_00OD_A {
    #[doc = "0: Normal. P2.0 pin is in the normal (not open drain) mode."]
    NORMAL = 0,
    #[doc = "1: Open-drain. P2.0 pin is in the open drain mode."]
    OPEN_DRAIN = 1,
}
impl From<P2_00OD_A> for bool {
    #[inline(always)]
    fn from(variant: P2_00OD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `P2_00OD`"]
pub type P2_00OD_R = crate::R<bool, P2_00OD_A>;
impl P2_00OD_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> P2_00OD_A {
        match self.bits {
            false => P2_00OD_A::NORMAL,
            true => P2_00OD_A::OPEN_DRAIN,
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == P2_00OD_A::NORMAL
    }
    #[doc = "Checks if the value of the field is `OPEN_DRAIN`"]
    #[inline(always)]
    pub fn is_open_drain(&self) -> bool {
        *self == P2_00OD_A::OPEN_DRAIN
    }
}
#[doc = "Write proxy for field `P2_00OD`"]
pub struct P2_00OD_W<'a> {
    w: &'a mut W,
}
impl<'a> P2_00OD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: P2_00OD_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Normal. P2.0 pin is in the normal (not open drain) mode."]
    #[inline(always)]
    pub fn normal(self) -> &'a mut W {
        self.variant(P2_00OD_A::NORMAL)
    }
    #[doc = "Open-drain. P2.0 pin is in the open drain mode."]
    #[inline(always)]
    pub fn open_drain(self) -> &'a mut W {
        self.variant(P2_00OD_A::OPEN_DRAIN)
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
#[doc = "Port 2 pin 1 open drain mode control, see P2.00OD\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P2_01OD_A {
    #[doc = "0: Normal. P2.1 pin is in the normal (not open drain) mode."]
    NORMAL = 0,
    #[doc = "1: Open-drain. P2.1p in is in the open drain mode."]
    OPEN_DRAIN = 1,
}
impl From<P2_01OD_A> for bool {
    #[inline(always)]
    fn from(variant: P2_01OD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `P2_01OD`"]
pub type P2_01OD_R = crate::R<bool, P2_01OD_A>;
impl P2_01OD_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> P2_01OD_A {
        match self.bits {
            false => P2_01OD_A::NORMAL,
            true => P2_01OD_A::OPEN_DRAIN,
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == P2_01OD_A::NORMAL
    }
    #[doc = "Checks if the value of the field is `OPEN_DRAIN`"]
    #[inline(always)]
    pub fn is_open_drain(&self) -> bool {
        *self == P2_01OD_A::OPEN_DRAIN
    }
}
#[doc = "Write proxy for field `P2_01OD`"]
pub struct P2_01OD_W<'a> {
    w: &'a mut W,
}
impl<'a> P2_01OD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: P2_01OD_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Normal. P2.1 pin is in the normal (not open drain) mode."]
    #[inline(always)]
    pub fn normal(self) -> &'a mut W {
        self.variant(P2_01OD_A::NORMAL)
    }
    #[doc = "Open-drain. P2.1p in is in the open drain mode."]
    #[inline(always)]
    pub fn open_drain(self) -> &'a mut W {
        self.variant(P2_01OD_A::OPEN_DRAIN)
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
#[doc = "Port 2 pin 2 open drain mode control, see P2.00OD\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P2_02OD_A {
    #[doc = "0: Normal. P2.2 pin is in the normal (not open drain) mode."]
    NORMAL = 0,
    #[doc = "1: Open-drain. P2.2 pin is in the open drain mode."]
    OPEN_DRAIN = 1,
}
impl From<P2_02OD_A> for bool {
    #[inline(always)]
    fn from(variant: P2_02OD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `P2_02OD`"]
pub type P2_02OD_R = crate::R<bool, P2_02OD_A>;
impl P2_02OD_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> P2_02OD_A {
        match self.bits {
            false => P2_02OD_A::NORMAL,
            true => P2_02OD_A::OPEN_DRAIN,
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == P2_02OD_A::NORMAL
    }
    #[doc = "Checks if the value of the field is `OPEN_DRAIN`"]
    #[inline(always)]
    pub fn is_open_drain(&self) -> bool {
        *self == P2_02OD_A::OPEN_DRAIN
    }
}
#[doc = "Write proxy for field `P2_02OD`"]
pub struct P2_02OD_W<'a> {
    w: &'a mut W,
}
impl<'a> P2_02OD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: P2_02OD_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Normal. P2.2 pin is in the normal (not open drain) mode."]
    #[inline(always)]
    pub fn normal(self) -> &'a mut W {
        self.variant(P2_02OD_A::NORMAL)
    }
    #[doc = "Open-drain. P2.2 pin is in the open drain mode."]
    #[inline(always)]
    pub fn open_drain(self) -> &'a mut W {
        self.variant(P2_02OD_A::OPEN_DRAIN)
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "Port 2 pin 3 open drain mode control, see P2.00OD\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P2_03OD_A {
    #[doc = "0: Normal. P2.3 pin is in the normal (not open drain) mode."]
    NORMAL = 0,
    #[doc = "1: Open-drain. P2.3 pin is in the open drain mode."]
    OPEN_DRAIN = 1,
}
impl From<P2_03OD_A> for bool {
    #[inline(always)]
    fn from(variant: P2_03OD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `P2_03OD`"]
pub type P2_03OD_R = crate::R<bool, P2_03OD_A>;
impl P2_03OD_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> P2_03OD_A {
        match self.bits {
            false => P2_03OD_A::NORMAL,
            true => P2_03OD_A::OPEN_DRAIN,
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == P2_03OD_A::NORMAL
    }
    #[doc = "Checks if the value of the field is `OPEN_DRAIN`"]
    #[inline(always)]
    pub fn is_open_drain(&self) -> bool {
        *self == P2_03OD_A::OPEN_DRAIN
    }
}
#[doc = "Write proxy for field `P2_03OD`"]
pub struct P2_03OD_W<'a> {
    w: &'a mut W,
}
impl<'a> P2_03OD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: P2_03OD_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Normal. P2.3 pin is in the normal (not open drain) mode."]
    #[inline(always)]
    pub fn normal(self) -> &'a mut W {
        self.variant(P2_03OD_A::NORMAL)
    }
    #[doc = "Open-drain. P2.3 pin is in the open drain mode."]
    #[inline(always)]
    pub fn open_drain(self) -> &'a mut W {
        self.variant(P2_03OD_A::OPEN_DRAIN)
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
#[doc = "Port 2 pin 4 open drain mode control, see P2.00OD\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P2_04OD_A {
    #[doc = "0: Normal. P2.4 pin is in the normal (not open drain) mode."]
    NORMAL = 0,
    #[doc = "1: Open-drain. P2.4 pin is in the open drain mode."]
    OPEN_DRAIN = 1,
}
impl From<P2_04OD_A> for bool {
    #[inline(always)]
    fn from(variant: P2_04OD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `P2_04OD`"]
pub type P2_04OD_R = crate::R<bool, P2_04OD_A>;
impl P2_04OD_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> P2_04OD_A {
        match self.bits {
            false => P2_04OD_A::NORMAL,
            true => P2_04OD_A::OPEN_DRAIN,
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == P2_04OD_A::NORMAL
    }
    #[doc = "Checks if the value of the field is `OPEN_DRAIN`"]
    #[inline(always)]
    pub fn is_open_drain(&self) -> bool {
        *self == P2_04OD_A::OPEN_DRAIN
    }
}
#[doc = "Write proxy for field `P2_04OD`"]
pub struct P2_04OD_W<'a> {
    w: &'a mut W,
}
impl<'a> P2_04OD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: P2_04OD_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Normal. P2.4 pin is in the normal (not open drain) mode."]
    #[inline(always)]
    pub fn normal(self) -> &'a mut W {
        self.variant(P2_04OD_A::NORMAL)
    }
    #[doc = "Open-drain. P2.4 pin is in the open drain mode."]
    #[inline(always)]
    pub fn open_drain(self) -> &'a mut W {
        self.variant(P2_04OD_A::OPEN_DRAIN)
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
#[doc = "Port 2 pin 5 open drain mode control, see P2.00OD\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P2_05OD_A {
    #[doc = "0: Normal. P2.5 pin is in the normal (not open drain) mode."]
    NORMAL = 0,
    #[doc = "1: Open-drain. P2.5 pin is in the open drain mode."]
    OPEN_DRAIN = 1,
}
impl From<P2_05OD_A> for bool {
    #[inline(always)]
    fn from(variant: P2_05OD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `P2_05OD`"]
pub type P2_05OD_R = crate::R<bool, P2_05OD_A>;
impl P2_05OD_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> P2_05OD_A {
        match self.bits {
            false => P2_05OD_A::NORMAL,
            true => P2_05OD_A::OPEN_DRAIN,
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == P2_05OD_A::NORMAL
    }
    #[doc = "Checks if the value of the field is `OPEN_DRAIN`"]
    #[inline(always)]
    pub fn is_open_drain(&self) -> bool {
        *self == P2_05OD_A::OPEN_DRAIN
    }
}
#[doc = "Write proxy for field `P2_05OD`"]
pub struct P2_05OD_W<'a> {
    w: &'a mut W,
}
impl<'a> P2_05OD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: P2_05OD_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Normal. P2.5 pin is in the normal (not open drain) mode."]
    #[inline(always)]
    pub fn normal(self) -> &'a mut W {
        self.variant(P2_05OD_A::NORMAL)
    }
    #[doc = "Open-drain. P2.5 pin is in the open drain mode."]
    #[inline(always)]
    pub fn open_drain(self) -> &'a mut W {
        self.variant(P2_05OD_A::OPEN_DRAIN)
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
#[doc = "Port 2 pin 6 open drain mode control, see P2.00OD\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P2_06OD_A {
    #[doc = "0: Normal. P2.6 pin is in the normal (not open drain) mode."]
    NORMAL = 0,
    #[doc = "1: Open-drain. P2.6 pin is in the open drain mode."]
    OPEN_DRAIN = 1,
}
impl From<P2_06OD_A> for bool {
    #[inline(always)]
    fn from(variant: P2_06OD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `P2_06OD`"]
pub type P2_06OD_R = crate::R<bool, P2_06OD_A>;
impl P2_06OD_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> P2_06OD_A {
        match self.bits {
            false => P2_06OD_A::NORMAL,
            true => P2_06OD_A::OPEN_DRAIN,
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == P2_06OD_A::NORMAL
    }
    #[doc = "Checks if the value of the field is `OPEN_DRAIN`"]
    #[inline(always)]
    pub fn is_open_drain(&self) -> bool {
        *self == P2_06OD_A::OPEN_DRAIN
    }
}
#[doc = "Write proxy for field `P2_06OD`"]
pub struct P2_06OD_W<'a> {
    w: &'a mut W,
}
impl<'a> P2_06OD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: P2_06OD_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Normal. P2.6 pin is in the normal (not open drain) mode."]
    #[inline(always)]
    pub fn normal(self) -> &'a mut W {
        self.variant(P2_06OD_A::NORMAL)
    }
    #[doc = "Open-drain. P2.6 pin is in the open drain mode."]
    #[inline(always)]
    pub fn open_drain(self) -> &'a mut W {
        self.variant(P2_06OD_A::OPEN_DRAIN)
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
#[doc = "Port 2 pin 7 open drain mode control, see P2.00OD\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P2_07OD_A {
    #[doc = "0: Normal. P2.7 pin is in the normal (not open drain) mode."]
    NORMAL = 0,
    #[doc = "1: Open-drain. P2.7 pin is in the open drain mode."]
    OPEN_DRAIN = 1,
}
impl From<P2_07OD_A> for bool {
    #[inline(always)]
    fn from(variant: P2_07OD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `P2_07OD`"]
pub type P2_07OD_R = crate::R<bool, P2_07OD_A>;
impl P2_07OD_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> P2_07OD_A {
        match self.bits {
            false => P2_07OD_A::NORMAL,
            true => P2_07OD_A::OPEN_DRAIN,
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == P2_07OD_A::NORMAL
    }
    #[doc = "Checks if the value of the field is `OPEN_DRAIN`"]
    #[inline(always)]
    pub fn is_open_drain(&self) -> bool {
        *self == P2_07OD_A::OPEN_DRAIN
    }
}
#[doc = "Write proxy for field `P2_07OD`"]
pub struct P2_07OD_W<'a> {
    w: &'a mut W,
}
impl<'a> P2_07OD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: P2_07OD_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Normal. P2.7 pin is in the normal (not open drain) mode."]
    #[inline(always)]
    pub fn normal(self) -> &'a mut W {
        self.variant(P2_07OD_A::NORMAL)
    }
    #[doc = "Open-drain. P2.7 pin is in the open drain mode."]
    #[inline(always)]
    pub fn open_drain(self) -> &'a mut W {
        self.variant(P2_07OD_A::OPEN_DRAIN)
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
#[doc = "Port 2 pin 8 open drain mode control, see P2.00OD\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P2_08OD_A {
    #[doc = "0: Normal. P2.8 pin is in the normal (not open drain) mode."]
    NORMAL = 0,
    #[doc = "1: Open-drain. P2.8 pin is in the open drain mode."]
    OPEN_DRAIN = 1,
}
impl From<P2_08OD_A> for bool {
    #[inline(always)]
    fn from(variant: P2_08OD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `P2_08OD`"]
pub type P2_08OD_R = crate::R<bool, P2_08OD_A>;
impl P2_08OD_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> P2_08OD_A {
        match self.bits {
            false => P2_08OD_A::NORMAL,
            true => P2_08OD_A::OPEN_DRAIN,
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == P2_08OD_A::NORMAL
    }
    #[doc = "Checks if the value of the field is `OPEN_DRAIN`"]
    #[inline(always)]
    pub fn is_open_drain(&self) -> bool {
        *self == P2_08OD_A::OPEN_DRAIN
    }
}
#[doc = "Write proxy for field `P2_08OD`"]
pub struct P2_08OD_W<'a> {
    w: &'a mut W,
}
impl<'a> P2_08OD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: P2_08OD_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Normal. P2.8 pin is in the normal (not open drain) mode."]
    #[inline(always)]
    pub fn normal(self) -> &'a mut W {
        self.variant(P2_08OD_A::NORMAL)
    }
    #[doc = "Open-drain. P2.8 pin is in the open drain mode."]
    #[inline(always)]
    pub fn open_drain(self) -> &'a mut W {
        self.variant(P2_08OD_A::OPEN_DRAIN)
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
#[doc = "Port 2 pin 9 open drain mode control, see P2.00OD\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P2_09OD_A {
    #[doc = "0: Normal. P2.9 pin is in the normal (not open drain) mode."]
    NORMAL = 0,
    #[doc = "1: Open-drain. P2.9 pin is in the open drain mode."]
    OPEN_DRAIN = 1,
}
impl From<P2_09OD_A> for bool {
    #[inline(always)]
    fn from(variant: P2_09OD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `P2_09OD`"]
pub type P2_09OD_R = crate::R<bool, P2_09OD_A>;
impl P2_09OD_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> P2_09OD_A {
        match self.bits {
            false => P2_09OD_A::NORMAL,
            true => P2_09OD_A::OPEN_DRAIN,
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == P2_09OD_A::NORMAL
    }
    #[doc = "Checks if the value of the field is `OPEN_DRAIN`"]
    #[inline(always)]
    pub fn is_open_drain(&self) -> bool {
        *self == P2_09OD_A::OPEN_DRAIN
    }
}
#[doc = "Write proxy for field `P2_09OD`"]
pub struct P2_09OD_W<'a> {
    w: &'a mut W,
}
impl<'a> P2_09OD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: P2_09OD_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Normal. P2.9 pin is in the normal (not open drain) mode."]
    #[inline(always)]
    pub fn normal(self) -> &'a mut W {
        self.variant(P2_09OD_A::NORMAL)
    }
    #[doc = "Open-drain. P2.9 pin is in the open drain mode."]
    #[inline(always)]
    pub fn open_drain(self) -> &'a mut W {
        self.variant(P2_09OD_A::OPEN_DRAIN)
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
#[doc = "Port 2 pin 10 open drain mode control, see P2.00OD\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P2_10OD_A {
    #[doc = "0: Normal. P2.10 pin is in the normal (not open drain) mode."]
    NORMAL = 0,
    #[doc = "1: Open-drain. P2.10 pin is in the open drain mode."]
    OPEN_DRAIN = 1,
}
impl From<P2_10OD_A> for bool {
    #[inline(always)]
    fn from(variant: P2_10OD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `P2_10OD`"]
pub type P2_10OD_R = crate::R<bool, P2_10OD_A>;
impl P2_10OD_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> P2_10OD_A {
        match self.bits {
            false => P2_10OD_A::NORMAL,
            true => P2_10OD_A::OPEN_DRAIN,
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == P2_10OD_A::NORMAL
    }
    #[doc = "Checks if the value of the field is `OPEN_DRAIN`"]
    #[inline(always)]
    pub fn is_open_drain(&self) -> bool {
        *self == P2_10OD_A::OPEN_DRAIN
    }
}
#[doc = "Write proxy for field `P2_10OD`"]
pub struct P2_10OD_W<'a> {
    w: &'a mut W,
}
impl<'a> P2_10OD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: P2_10OD_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Normal. P2.10 pin is in the normal (not open drain) mode."]
    #[inline(always)]
    pub fn normal(self) -> &'a mut W {
        self.variant(P2_10OD_A::NORMAL)
    }
    #[doc = "Open-drain. P2.10 pin is in the open drain mode."]
    #[inline(always)]
    pub fn open_drain(self) -> &'a mut W {
        self.variant(P2_10OD_A::OPEN_DRAIN)
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
#[doc = "Port 2 pin 11 open drain mode control, see P2.00OD\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P2_11OD_A {
    #[doc = "0: Normal. P2.11 pin is in the normal (not open drain) mode."]
    NORMAL = 0,
    #[doc = "1: Open-drain. P2.11 pin is in the open drain mode."]
    OPEN_DRAIN = 1,
}
impl From<P2_11OD_A> for bool {
    #[inline(always)]
    fn from(variant: P2_11OD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `P2_11OD`"]
pub type P2_11OD_R = crate::R<bool, P2_11OD_A>;
impl P2_11OD_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> P2_11OD_A {
        match self.bits {
            false => P2_11OD_A::NORMAL,
            true => P2_11OD_A::OPEN_DRAIN,
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == P2_11OD_A::NORMAL
    }
    #[doc = "Checks if the value of the field is `OPEN_DRAIN`"]
    #[inline(always)]
    pub fn is_open_drain(&self) -> bool {
        *self == P2_11OD_A::OPEN_DRAIN
    }
}
#[doc = "Write proxy for field `P2_11OD`"]
pub struct P2_11OD_W<'a> {
    w: &'a mut W,
}
impl<'a> P2_11OD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: P2_11OD_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Normal. P2.11 pin is in the normal (not open drain) mode."]
    #[inline(always)]
    pub fn normal(self) -> &'a mut W {
        self.variant(P2_11OD_A::NORMAL)
    }
    #[doc = "Open-drain. P2.11 pin is in the open drain mode."]
    #[inline(always)]
    pub fn open_drain(self) -> &'a mut W {
        self.variant(P2_11OD_A::OPEN_DRAIN)
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u32) & 0x01) << 11);
        self.w
    }
}
#[doc = "Port 2 pin 12 open drain mode control, see P2.00OD\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P2_12OD_A {
    #[doc = "0: Normal. P2.12 pin is in the normal (not open drain) mode."]
    NORMAL = 0,
    #[doc = "1: Open-drain. P2.12 pin is in the open drain mode."]
    OPEN_DRAIN = 1,
}
impl From<P2_12OD_A> for bool {
    #[inline(always)]
    fn from(variant: P2_12OD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `P2_12OD`"]
pub type P2_12OD_R = crate::R<bool, P2_12OD_A>;
impl P2_12OD_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> P2_12OD_A {
        match self.bits {
            false => P2_12OD_A::NORMAL,
            true => P2_12OD_A::OPEN_DRAIN,
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == P2_12OD_A::NORMAL
    }
    #[doc = "Checks if the value of the field is `OPEN_DRAIN`"]
    #[inline(always)]
    pub fn is_open_drain(&self) -> bool {
        *self == P2_12OD_A::OPEN_DRAIN
    }
}
#[doc = "Write proxy for field `P2_12OD`"]
pub struct P2_12OD_W<'a> {
    w: &'a mut W,
}
impl<'a> P2_12OD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: P2_12OD_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Normal. P2.12 pin is in the normal (not open drain) mode."]
    #[inline(always)]
    pub fn normal(self) -> &'a mut W {
        self.variant(P2_12OD_A::NORMAL)
    }
    #[doc = "Open-drain. P2.12 pin is in the open drain mode."]
    #[inline(always)]
    pub fn open_drain(self) -> &'a mut W {
        self.variant(P2_12OD_A::OPEN_DRAIN)
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u32) & 0x01) << 12);
        self.w
    }
}
#[doc = "Port 2 pin 13 open drain mode control, see P2.00OD\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P2_13OD_A {
    #[doc = "0: Normal. P2.13 pin is in the normal (not open drain) mode."]
    NORMAL = 0,
    #[doc = "1: Open-drain. P2.13 pin is in the open drain mode."]
    OPEN_DRAIN = 1,
}
impl From<P2_13OD_A> for bool {
    #[inline(always)]
    fn from(variant: P2_13OD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `P2_13OD`"]
pub type P2_13OD_R = crate::R<bool, P2_13OD_A>;
impl P2_13OD_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> P2_13OD_A {
        match self.bits {
            false => P2_13OD_A::NORMAL,
            true => P2_13OD_A::OPEN_DRAIN,
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == P2_13OD_A::NORMAL
    }
    #[doc = "Checks if the value of the field is `OPEN_DRAIN`"]
    #[inline(always)]
    pub fn is_open_drain(&self) -> bool {
        *self == P2_13OD_A::OPEN_DRAIN
    }
}
#[doc = "Write proxy for field `P2_13OD`"]
pub struct P2_13OD_W<'a> {
    w: &'a mut W,
}
impl<'a> P2_13OD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: P2_13OD_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Normal. P2.13 pin is in the normal (not open drain) mode."]
    #[inline(always)]
    pub fn normal(self) -> &'a mut W {
        self.variant(P2_13OD_A::NORMAL)
    }
    #[doc = "Open-drain. P2.13 pin is in the open drain mode."]
    #[inline(always)]
    pub fn open_drain(self) -> &'a mut W {
        self.variant(P2_13OD_A::OPEN_DRAIN)
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | (((value as u32) & 0x01) << 13);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Port 2 pin 0 open drain mode control."]
    #[inline(always)]
    pub fn p2_00od(&self) -> P2_00OD_R {
        P2_00OD_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Port 2 pin 1 open drain mode control, see P2.00OD"]
    #[inline(always)]
    pub fn p2_01od(&self) -> P2_01OD_R {
        P2_01OD_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Port 2 pin 2 open drain mode control, see P2.00OD"]
    #[inline(always)]
    pub fn p2_02od(&self) -> P2_02OD_R {
        P2_02OD_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Port 2 pin 3 open drain mode control, see P2.00OD"]
    #[inline(always)]
    pub fn p2_03od(&self) -> P2_03OD_R {
        P2_03OD_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Port 2 pin 4 open drain mode control, see P2.00OD"]
    #[inline(always)]
    pub fn p2_04od(&self) -> P2_04OD_R {
        P2_04OD_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Port 2 pin 5 open drain mode control, see P2.00OD"]
    #[inline(always)]
    pub fn p2_05od(&self) -> P2_05OD_R {
        P2_05OD_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Port 2 pin 6 open drain mode control, see P2.00OD"]
    #[inline(always)]
    pub fn p2_06od(&self) -> P2_06OD_R {
        P2_06OD_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Port 2 pin 7 open drain mode control, see P2.00OD"]
    #[inline(always)]
    pub fn p2_07od(&self) -> P2_07OD_R {
        P2_07OD_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Port 2 pin 8 open drain mode control, see P2.00OD"]
    #[inline(always)]
    pub fn p2_08od(&self) -> P2_08OD_R {
        P2_08OD_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Port 2 pin 9 open drain mode control, see P2.00OD"]
    #[inline(always)]
    pub fn p2_09od(&self) -> P2_09OD_R {
        P2_09OD_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Port 2 pin 10 open drain mode control, see P2.00OD"]
    #[inline(always)]
    pub fn p2_10od(&self) -> P2_10OD_R {
        P2_10OD_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Port 2 pin 11 open drain mode control, see P2.00OD"]
    #[inline(always)]
    pub fn p2_11od(&self) -> P2_11OD_R {
        P2_11OD_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Port 2 pin 12 open drain mode control, see P2.00OD"]
    #[inline(always)]
    pub fn p2_12od(&self) -> P2_12OD_R {
        P2_12OD_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Port 2 pin 13 open drain mode control, see P2.00OD"]
    #[inline(always)]
    pub fn p2_13od(&self) -> P2_13OD_R {
        P2_13OD_R::new(((self.bits >> 13) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Port 2 pin 0 open drain mode control."]
    #[inline(always)]
    pub fn p2_00od(&mut self) -> P2_00OD_W {
        P2_00OD_W { w: self }
    }
    #[doc = "Bit 1 - Port 2 pin 1 open drain mode control, see P2.00OD"]
    #[inline(always)]
    pub fn p2_01od(&mut self) -> P2_01OD_W {
        P2_01OD_W { w: self }
    }
    #[doc = "Bit 2 - Port 2 pin 2 open drain mode control, see P2.00OD"]
    #[inline(always)]
    pub fn p2_02od(&mut self) -> P2_02OD_W {
        P2_02OD_W { w: self }
    }
    #[doc = "Bit 3 - Port 2 pin 3 open drain mode control, see P2.00OD"]
    #[inline(always)]
    pub fn p2_03od(&mut self) -> P2_03OD_W {
        P2_03OD_W { w: self }
    }
    #[doc = "Bit 4 - Port 2 pin 4 open drain mode control, see P2.00OD"]
    #[inline(always)]
    pub fn p2_04od(&mut self) -> P2_04OD_W {
        P2_04OD_W { w: self }
    }
    #[doc = "Bit 5 - Port 2 pin 5 open drain mode control, see P2.00OD"]
    #[inline(always)]
    pub fn p2_05od(&mut self) -> P2_05OD_W {
        P2_05OD_W { w: self }
    }
    #[doc = "Bit 6 - Port 2 pin 6 open drain mode control, see P2.00OD"]
    #[inline(always)]
    pub fn p2_06od(&mut self) -> P2_06OD_W {
        P2_06OD_W { w: self }
    }
    #[doc = "Bit 7 - Port 2 pin 7 open drain mode control, see P2.00OD"]
    #[inline(always)]
    pub fn p2_07od(&mut self) -> P2_07OD_W {
        P2_07OD_W { w: self }
    }
    #[doc = "Bit 8 - Port 2 pin 8 open drain mode control, see P2.00OD"]
    #[inline(always)]
    pub fn p2_08od(&mut self) -> P2_08OD_W {
        P2_08OD_W { w: self }
    }
    #[doc = "Bit 9 - Port 2 pin 9 open drain mode control, see P2.00OD"]
    #[inline(always)]
    pub fn p2_09od(&mut self) -> P2_09OD_W {
        P2_09OD_W { w: self }
    }
    #[doc = "Bit 10 - Port 2 pin 10 open drain mode control, see P2.00OD"]
    #[inline(always)]
    pub fn p2_10od(&mut self) -> P2_10OD_W {
        P2_10OD_W { w: self }
    }
    #[doc = "Bit 11 - Port 2 pin 11 open drain mode control, see P2.00OD"]
    #[inline(always)]
    pub fn p2_11od(&mut self) -> P2_11OD_W {
        P2_11OD_W { w: self }
    }
    #[doc = "Bit 12 - Port 2 pin 12 open drain mode control, see P2.00OD"]
    #[inline(always)]
    pub fn p2_12od(&mut self) -> P2_12OD_W {
        P2_12OD_W { w: self }
    }
    #[doc = "Bit 13 - Port 2 pin 13 open drain mode control, see P2.00OD"]
    #[inline(always)]
    pub fn p2_13od(&mut self) -> P2_13OD_W {
        P2_13OD_W { w: self }
    }
}
