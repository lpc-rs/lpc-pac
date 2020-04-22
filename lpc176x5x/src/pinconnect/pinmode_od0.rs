#[doc = "Reader of register PINMODE_OD0"]
pub type R = crate::R<u32, super::PINMODE_OD0>;
#[doc = "Writer for register PINMODE_OD0"]
pub type W = crate::W<u32, super::PINMODE_OD0>;
#[doc = "Register PINMODE_OD0 `reset()`'s with value 0"]
impl crate::ResetValue for super::PINMODE_OD0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Port 0 pin 0 open drain mode control. Pins may potentially be used for I2C-buses using standard port pins. If so, they should be configured for open drain mode via the related bits in PINMODE_OD0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P0_00OD_A {
    #[doc = "0: Normal. P0.0 pin is in the normal (not open drain) mode."]
    NORMAL = 0,
    #[doc = "1: Open-drain. P0.0 pin is in the open drain mode."]
    OPEN_DRAIN = 1,
}
impl From<P0_00OD_A> for bool {
    #[inline(always)]
    fn from(variant: P0_00OD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `P0_00OD`"]
pub type P0_00OD_R = crate::R<bool, P0_00OD_A>;
impl P0_00OD_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> P0_00OD_A {
        match self.bits {
            false => P0_00OD_A::NORMAL,
            true => P0_00OD_A::OPEN_DRAIN,
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == P0_00OD_A::NORMAL
    }
    #[doc = "Checks if the value of the field is `OPEN_DRAIN`"]
    #[inline(always)]
    pub fn is_open_drain(&self) -> bool {
        *self == P0_00OD_A::OPEN_DRAIN
    }
}
#[doc = "Write proxy for field `P0_00OD`"]
pub struct P0_00OD_W<'a> {
    w: &'a mut W,
}
impl<'a> P0_00OD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: P0_00OD_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Normal. P0.0 pin is in the normal (not open drain) mode."]
    #[inline(always)]
    pub fn normal(self) -> &'a mut W {
        self.variant(P0_00OD_A::NORMAL)
    }
    #[doc = "Open-drain. P0.0 pin is in the open drain mode."]
    #[inline(always)]
    pub fn open_drain(self) -> &'a mut W {
        self.variant(P0_00OD_A::OPEN_DRAIN)
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
#[doc = "Port 0 pin 1 open drain mode control. Pins may potentially be used for I2C-buses using standard port pins. If so, they should be configured for open drain mode via the related bits in PINMODE_OD0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P0_01OD_A {
    #[doc = "0: Normal. P0.1 pin is in the normal (not open drain) mode."]
    NORMAL = 0,
    #[doc = "1: Open-drain. P0.1 pin is in the open drain mode."]
    OPEN_DRAIN = 1,
}
impl From<P0_01OD_A> for bool {
    #[inline(always)]
    fn from(variant: P0_01OD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `P0_01OD`"]
pub type P0_01OD_R = crate::R<bool, P0_01OD_A>;
impl P0_01OD_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> P0_01OD_A {
        match self.bits {
            false => P0_01OD_A::NORMAL,
            true => P0_01OD_A::OPEN_DRAIN,
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == P0_01OD_A::NORMAL
    }
    #[doc = "Checks if the value of the field is `OPEN_DRAIN`"]
    #[inline(always)]
    pub fn is_open_drain(&self) -> bool {
        *self == P0_01OD_A::OPEN_DRAIN
    }
}
#[doc = "Write proxy for field `P0_01OD`"]
pub struct P0_01OD_W<'a> {
    w: &'a mut W,
}
impl<'a> P0_01OD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: P0_01OD_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Normal. P0.1 pin is in the normal (not open drain) mode."]
    #[inline(always)]
    pub fn normal(self) -> &'a mut W {
        self.variant(P0_01OD_A::NORMAL)
    }
    #[doc = "Open-drain. P0.1 pin is in the open drain mode."]
    #[inline(always)]
    pub fn open_drain(self) -> &'a mut W {
        self.variant(P0_01OD_A::OPEN_DRAIN)
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
#[doc = "Port 0 pin 2 open drain mode control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P0_02OD_A {
    #[doc = "0: Normal. P0.2 pin is in the normal (not open drain) mode."]
    NORMAL = 0,
    #[doc = "1: Open-drain. P0.2 pin is in the open drain mode."]
    OPEN_DRAIN = 1,
}
impl From<P0_02OD_A> for bool {
    #[inline(always)]
    fn from(variant: P0_02OD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `P0_02OD`"]
pub type P0_02OD_R = crate::R<bool, P0_02OD_A>;
impl P0_02OD_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> P0_02OD_A {
        match self.bits {
            false => P0_02OD_A::NORMAL,
            true => P0_02OD_A::OPEN_DRAIN,
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == P0_02OD_A::NORMAL
    }
    #[doc = "Checks if the value of the field is `OPEN_DRAIN`"]
    #[inline(always)]
    pub fn is_open_drain(&self) -> bool {
        *self == P0_02OD_A::OPEN_DRAIN
    }
}
#[doc = "Write proxy for field `P0_02OD`"]
pub struct P0_02OD_W<'a> {
    w: &'a mut W,
}
impl<'a> P0_02OD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: P0_02OD_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Normal. P0.2 pin is in the normal (not open drain) mode."]
    #[inline(always)]
    pub fn normal(self) -> &'a mut W {
        self.variant(P0_02OD_A::NORMAL)
    }
    #[doc = "Open-drain. P0.2 pin is in the open drain mode."]
    #[inline(always)]
    pub fn open_drain(self) -> &'a mut W {
        self.variant(P0_02OD_A::OPEN_DRAIN)
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
#[doc = "Port 0 pin 3 open drain mode control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P0_03OD_A {
    #[doc = "0: Normal. P0.3 pin is in the normal (not open drain) mode."]
    NORMAL = 0,
    #[doc = "1: Open-drain. P0.3 pin is in the open drain mode."]
    OPEN_DRAIN = 1,
}
impl From<P0_03OD_A> for bool {
    #[inline(always)]
    fn from(variant: P0_03OD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `P0_03OD`"]
pub type P0_03OD_R = crate::R<bool, P0_03OD_A>;
impl P0_03OD_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> P0_03OD_A {
        match self.bits {
            false => P0_03OD_A::NORMAL,
            true => P0_03OD_A::OPEN_DRAIN,
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == P0_03OD_A::NORMAL
    }
    #[doc = "Checks if the value of the field is `OPEN_DRAIN`"]
    #[inline(always)]
    pub fn is_open_drain(&self) -> bool {
        *self == P0_03OD_A::OPEN_DRAIN
    }
}
#[doc = "Write proxy for field `P0_03OD`"]
pub struct P0_03OD_W<'a> {
    w: &'a mut W,
}
impl<'a> P0_03OD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: P0_03OD_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Normal. P0.3 pin is in the normal (not open drain) mode."]
    #[inline(always)]
    pub fn normal(self) -> &'a mut W {
        self.variant(P0_03OD_A::NORMAL)
    }
    #[doc = "Open-drain. P0.3 pin is in the open drain mode."]
    #[inline(always)]
    pub fn open_drain(self) -> &'a mut W {
        self.variant(P0_03OD_A::OPEN_DRAIN)
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
#[doc = "Port 0 pin 4 open drain mode control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P0_04OD_A {
    #[doc = "0: Normal. P0.4 pin is in the normal (not open drain) mode."]
    NORMAL = 0,
    #[doc = "1: Open-drain. P0.4 pin is in the open drain mode."]
    OPEN_DRAIN = 1,
}
impl From<P0_04OD_A> for bool {
    #[inline(always)]
    fn from(variant: P0_04OD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `P0_04OD`"]
pub type P0_04OD_R = crate::R<bool, P0_04OD_A>;
impl P0_04OD_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> P0_04OD_A {
        match self.bits {
            false => P0_04OD_A::NORMAL,
            true => P0_04OD_A::OPEN_DRAIN,
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == P0_04OD_A::NORMAL
    }
    #[doc = "Checks if the value of the field is `OPEN_DRAIN`"]
    #[inline(always)]
    pub fn is_open_drain(&self) -> bool {
        *self == P0_04OD_A::OPEN_DRAIN
    }
}
#[doc = "Write proxy for field `P0_04OD`"]
pub struct P0_04OD_W<'a> {
    w: &'a mut W,
}
impl<'a> P0_04OD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: P0_04OD_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Normal. P0.4 pin is in the normal (not open drain) mode."]
    #[inline(always)]
    pub fn normal(self) -> &'a mut W {
        self.variant(P0_04OD_A::NORMAL)
    }
    #[doc = "Open-drain. P0.4 pin is in the open drain mode."]
    #[inline(always)]
    pub fn open_drain(self) -> &'a mut W {
        self.variant(P0_04OD_A::OPEN_DRAIN)
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
#[doc = "Port 0 pin 5 open drain mode control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P0_05OD_A {
    #[doc = "0: Normal. P0.5 pin is in the normal (not open drain) mode."]
    NORMAL = 0,
    #[doc = "1: Open-drain. P0.5 pin is in the open drain mode."]
    OPEN_DRAIN = 1,
}
impl From<P0_05OD_A> for bool {
    #[inline(always)]
    fn from(variant: P0_05OD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `P0_05OD`"]
pub type P0_05OD_R = crate::R<bool, P0_05OD_A>;
impl P0_05OD_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> P0_05OD_A {
        match self.bits {
            false => P0_05OD_A::NORMAL,
            true => P0_05OD_A::OPEN_DRAIN,
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == P0_05OD_A::NORMAL
    }
    #[doc = "Checks if the value of the field is `OPEN_DRAIN`"]
    #[inline(always)]
    pub fn is_open_drain(&self) -> bool {
        *self == P0_05OD_A::OPEN_DRAIN
    }
}
#[doc = "Write proxy for field `P0_05OD`"]
pub struct P0_05OD_W<'a> {
    w: &'a mut W,
}
impl<'a> P0_05OD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: P0_05OD_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Normal. P0.5 pin is in the normal (not open drain) mode."]
    #[inline(always)]
    pub fn normal(self) -> &'a mut W {
        self.variant(P0_05OD_A::NORMAL)
    }
    #[doc = "Open-drain. P0.5 pin is in the open drain mode."]
    #[inline(always)]
    pub fn open_drain(self) -> &'a mut W {
        self.variant(P0_05OD_A::OPEN_DRAIN)
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
#[doc = "Port 0 pin 6 open drain mode control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P0_06OD_A {
    #[doc = "0: Normal. P0.6 pin is in the normal (not open drain) mode."]
    NORMAL = 0,
    #[doc = "1: Open-drain. P0.6 pin is in the open drain mode."]
    OPEN_DRAIN = 1,
}
impl From<P0_06OD_A> for bool {
    #[inline(always)]
    fn from(variant: P0_06OD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `P0_06OD`"]
pub type P0_06OD_R = crate::R<bool, P0_06OD_A>;
impl P0_06OD_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> P0_06OD_A {
        match self.bits {
            false => P0_06OD_A::NORMAL,
            true => P0_06OD_A::OPEN_DRAIN,
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == P0_06OD_A::NORMAL
    }
    #[doc = "Checks if the value of the field is `OPEN_DRAIN`"]
    #[inline(always)]
    pub fn is_open_drain(&self) -> bool {
        *self == P0_06OD_A::OPEN_DRAIN
    }
}
#[doc = "Write proxy for field `P0_06OD`"]
pub struct P0_06OD_W<'a> {
    w: &'a mut W,
}
impl<'a> P0_06OD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: P0_06OD_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Normal. P0.6 pin is in the normal (not open drain) mode."]
    #[inline(always)]
    pub fn normal(self) -> &'a mut W {
        self.variant(P0_06OD_A::NORMAL)
    }
    #[doc = "Open-drain. P0.6 pin is in the open drain mode."]
    #[inline(always)]
    pub fn open_drain(self) -> &'a mut W {
        self.variant(P0_06OD_A::OPEN_DRAIN)
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
#[doc = "Port 0 pin 7 open drain mode control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P0_07OD_A {
    #[doc = "0: Normal. P0.7 pin is in the normal (not open drain) mode."]
    NORMAL = 0,
    #[doc = "1: Open-drain. P0.7 pin is in the open drain mode."]
    OPEN_DRAIN = 1,
}
impl From<P0_07OD_A> for bool {
    #[inline(always)]
    fn from(variant: P0_07OD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `P0_07OD`"]
pub type P0_07OD_R = crate::R<bool, P0_07OD_A>;
impl P0_07OD_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> P0_07OD_A {
        match self.bits {
            false => P0_07OD_A::NORMAL,
            true => P0_07OD_A::OPEN_DRAIN,
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == P0_07OD_A::NORMAL
    }
    #[doc = "Checks if the value of the field is `OPEN_DRAIN`"]
    #[inline(always)]
    pub fn is_open_drain(&self) -> bool {
        *self == P0_07OD_A::OPEN_DRAIN
    }
}
#[doc = "Write proxy for field `P0_07OD`"]
pub struct P0_07OD_W<'a> {
    w: &'a mut W,
}
impl<'a> P0_07OD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: P0_07OD_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Normal. P0.7 pin is in the normal (not open drain) mode."]
    #[inline(always)]
    pub fn normal(self) -> &'a mut W {
        self.variant(P0_07OD_A::NORMAL)
    }
    #[doc = "Open-drain. P0.7 pin is in the open drain mode."]
    #[inline(always)]
    pub fn open_drain(self) -> &'a mut W {
        self.variant(P0_07OD_A::OPEN_DRAIN)
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
#[doc = "Port 0 pin 8 open drain mode control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P0_08OD_A {
    #[doc = "0: Normal. P0.8 pin is in the normal (not open drain) mode."]
    NORMAL = 0,
    #[doc = "1: Open-drain. P0.8 pin is in the open drain mode."]
    OPEN_DRAIN = 1,
}
impl From<P0_08OD_A> for bool {
    #[inline(always)]
    fn from(variant: P0_08OD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `P0_08OD`"]
pub type P0_08OD_R = crate::R<bool, P0_08OD_A>;
impl P0_08OD_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> P0_08OD_A {
        match self.bits {
            false => P0_08OD_A::NORMAL,
            true => P0_08OD_A::OPEN_DRAIN,
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == P0_08OD_A::NORMAL
    }
    #[doc = "Checks if the value of the field is `OPEN_DRAIN`"]
    #[inline(always)]
    pub fn is_open_drain(&self) -> bool {
        *self == P0_08OD_A::OPEN_DRAIN
    }
}
#[doc = "Write proxy for field `P0_08OD`"]
pub struct P0_08OD_W<'a> {
    w: &'a mut W,
}
impl<'a> P0_08OD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: P0_08OD_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Normal. P0.8 pin is in the normal (not open drain) mode."]
    #[inline(always)]
    pub fn normal(self) -> &'a mut W {
        self.variant(P0_08OD_A::NORMAL)
    }
    #[doc = "Open-drain. P0.8 pin is in the open drain mode."]
    #[inline(always)]
    pub fn open_drain(self) -> &'a mut W {
        self.variant(P0_08OD_A::OPEN_DRAIN)
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
#[doc = "Port 0 pin 9 open drain mode control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P0_09OD_A {
    #[doc = "0: Normal. P0.9 pin is in the normal (not open drain) mode."]
    NORMAL = 0,
    #[doc = "1: Open-drain. P0.9 pin is in the open drain mode."]
    OPEN_DRAIN = 1,
}
impl From<P0_09OD_A> for bool {
    #[inline(always)]
    fn from(variant: P0_09OD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `P0_09OD`"]
pub type P0_09OD_R = crate::R<bool, P0_09OD_A>;
impl P0_09OD_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> P0_09OD_A {
        match self.bits {
            false => P0_09OD_A::NORMAL,
            true => P0_09OD_A::OPEN_DRAIN,
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == P0_09OD_A::NORMAL
    }
    #[doc = "Checks if the value of the field is `OPEN_DRAIN`"]
    #[inline(always)]
    pub fn is_open_drain(&self) -> bool {
        *self == P0_09OD_A::OPEN_DRAIN
    }
}
#[doc = "Write proxy for field `P0_09OD`"]
pub struct P0_09OD_W<'a> {
    w: &'a mut W,
}
impl<'a> P0_09OD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: P0_09OD_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Normal. P0.9 pin is in the normal (not open drain) mode."]
    #[inline(always)]
    pub fn normal(self) -> &'a mut W {
        self.variant(P0_09OD_A::NORMAL)
    }
    #[doc = "Open-drain. P0.9 pin is in the open drain mode."]
    #[inline(always)]
    pub fn open_drain(self) -> &'a mut W {
        self.variant(P0_09OD_A::OPEN_DRAIN)
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
#[doc = "Port 0 pin 10 open drain mode control. Pins may potentially be used for I2C-buses using standard port pins. If so, they should be configured for open drain mode via the related bits in PINMODE_OD0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P0_10OD_A {
    #[doc = "0: Normal. P0.10 pin is in the normal (not open drain) mode."]
    NORMAL = 0,
    #[doc = "1: Open-drain. P0.10 pin is in the open drain mode."]
    OPEN_DRAIN = 1,
}
impl From<P0_10OD_A> for bool {
    #[inline(always)]
    fn from(variant: P0_10OD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `P0_10OD`"]
pub type P0_10OD_R = crate::R<bool, P0_10OD_A>;
impl P0_10OD_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> P0_10OD_A {
        match self.bits {
            false => P0_10OD_A::NORMAL,
            true => P0_10OD_A::OPEN_DRAIN,
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == P0_10OD_A::NORMAL
    }
    #[doc = "Checks if the value of the field is `OPEN_DRAIN`"]
    #[inline(always)]
    pub fn is_open_drain(&self) -> bool {
        *self == P0_10OD_A::OPEN_DRAIN
    }
}
#[doc = "Write proxy for field `P0_10OD`"]
pub struct P0_10OD_W<'a> {
    w: &'a mut W,
}
impl<'a> P0_10OD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: P0_10OD_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Normal. P0.10 pin is in the normal (not open drain) mode."]
    #[inline(always)]
    pub fn normal(self) -> &'a mut W {
        self.variant(P0_10OD_A::NORMAL)
    }
    #[doc = "Open-drain. P0.10 pin is in the open drain mode."]
    #[inline(always)]
    pub fn open_drain(self) -> &'a mut W {
        self.variant(P0_10OD_A::OPEN_DRAIN)
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
#[doc = "Port 0 pin 11 open drain mode control. Pins may potentially be used for I2C-buses using standard port pins. If so, they should be configured for open drain mode via the related bits in PINMODE_OD0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P0_11OD_A {
    #[doc = "0: Normal. P0.11 pin is in the normal (not open drain) mode."]
    NORMAL = 0,
    #[doc = "1: Open-drain. P0.11 pin is in the open drain mode."]
    OPEN_DRAIN = 1,
}
impl From<P0_11OD_A> for bool {
    #[inline(always)]
    fn from(variant: P0_11OD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `P0_11OD`"]
pub type P0_11OD_R = crate::R<bool, P0_11OD_A>;
impl P0_11OD_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> P0_11OD_A {
        match self.bits {
            false => P0_11OD_A::NORMAL,
            true => P0_11OD_A::OPEN_DRAIN,
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == P0_11OD_A::NORMAL
    }
    #[doc = "Checks if the value of the field is `OPEN_DRAIN`"]
    #[inline(always)]
    pub fn is_open_drain(&self) -> bool {
        *self == P0_11OD_A::OPEN_DRAIN
    }
}
#[doc = "Write proxy for field `P0_11OD`"]
pub struct P0_11OD_W<'a> {
    w: &'a mut W,
}
impl<'a> P0_11OD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: P0_11OD_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Normal. P0.11 pin is in the normal (not open drain) mode."]
    #[inline(always)]
    pub fn normal(self) -> &'a mut W {
        self.variant(P0_11OD_A::NORMAL)
    }
    #[doc = "Open-drain. P0.11 pin is in the open drain mode."]
    #[inline(always)]
    pub fn open_drain(self) -> &'a mut W {
        self.variant(P0_11OD_A::OPEN_DRAIN)
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
#[doc = "Port 0 pin 15 open drain mode control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P0_15OD_A {
    #[doc = "0: Normal. P0.15 pin is in the normal (not open drain) mode."]
    NORMAL = 0,
    #[doc = "1: Open-drain. P0.15 pin is in the open drain mode."]
    OPEN_DRAIN = 1,
}
impl From<P0_15OD_A> for bool {
    #[inline(always)]
    fn from(variant: P0_15OD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `P0_15OD`"]
pub type P0_15OD_R = crate::R<bool, P0_15OD_A>;
impl P0_15OD_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> P0_15OD_A {
        match self.bits {
            false => P0_15OD_A::NORMAL,
            true => P0_15OD_A::OPEN_DRAIN,
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == P0_15OD_A::NORMAL
    }
    #[doc = "Checks if the value of the field is `OPEN_DRAIN`"]
    #[inline(always)]
    pub fn is_open_drain(&self) -> bool {
        *self == P0_15OD_A::OPEN_DRAIN
    }
}
#[doc = "Write proxy for field `P0_15OD`"]
pub struct P0_15OD_W<'a> {
    w: &'a mut W,
}
impl<'a> P0_15OD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: P0_15OD_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Normal. P0.15 pin is in the normal (not open drain) mode."]
    #[inline(always)]
    pub fn normal(self) -> &'a mut W {
        self.variant(P0_15OD_A::NORMAL)
    }
    #[doc = "Open-drain. P0.15 pin is in the open drain mode."]
    #[inline(always)]
    pub fn open_drain(self) -> &'a mut W {
        self.variant(P0_15OD_A::OPEN_DRAIN)
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
#[doc = "Port 0 pin 16 open drain mode control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P0_16OD_A {
    #[doc = "0: Normal. P0.16 pin is in the normal (not open drain) mode."]
    NORMAL = 0,
    #[doc = "1: Open-drain. P0.16 pin is in the open drain mode."]
    OPEN_DRAIN = 1,
}
impl From<P0_16OD_A> for bool {
    #[inline(always)]
    fn from(variant: P0_16OD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `P0_16OD`"]
pub type P0_16OD_R = crate::R<bool, P0_16OD_A>;
impl P0_16OD_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> P0_16OD_A {
        match self.bits {
            false => P0_16OD_A::NORMAL,
            true => P0_16OD_A::OPEN_DRAIN,
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == P0_16OD_A::NORMAL
    }
    #[doc = "Checks if the value of the field is `OPEN_DRAIN`"]
    #[inline(always)]
    pub fn is_open_drain(&self) -> bool {
        *self == P0_16OD_A::OPEN_DRAIN
    }
}
#[doc = "Write proxy for field `P0_16OD`"]
pub struct P0_16OD_W<'a> {
    w: &'a mut W,
}
impl<'a> P0_16OD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: P0_16OD_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Normal. P0.16 pin is in the normal (not open drain) mode."]
    #[inline(always)]
    pub fn normal(self) -> &'a mut W {
        self.variant(P0_16OD_A::NORMAL)
    }
    #[doc = "Open-drain. P0.16 pin is in the open drain mode."]
    #[inline(always)]
    pub fn open_drain(self) -> &'a mut W {
        self.variant(P0_16OD_A::OPEN_DRAIN)
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
#[doc = "Port 0 pin 17 open drain mode control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P0_17OD_A {
    #[doc = "0: Normal. P0.17 pin is in the normal (not open drain) mode."]
    NORMAL = 0,
    #[doc = "1: Open-drain. P0.17 pin is in the open drain mode."]
    OPEN_DRAIN = 1,
}
impl From<P0_17OD_A> for bool {
    #[inline(always)]
    fn from(variant: P0_17OD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `P0_17OD`"]
pub type P0_17OD_R = crate::R<bool, P0_17OD_A>;
impl P0_17OD_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> P0_17OD_A {
        match self.bits {
            false => P0_17OD_A::NORMAL,
            true => P0_17OD_A::OPEN_DRAIN,
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == P0_17OD_A::NORMAL
    }
    #[doc = "Checks if the value of the field is `OPEN_DRAIN`"]
    #[inline(always)]
    pub fn is_open_drain(&self) -> bool {
        *self == P0_17OD_A::OPEN_DRAIN
    }
}
#[doc = "Write proxy for field `P0_17OD`"]
pub struct P0_17OD_W<'a> {
    w: &'a mut W,
}
impl<'a> P0_17OD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: P0_17OD_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Normal. P0.17 pin is in the normal (not open drain) mode."]
    #[inline(always)]
    pub fn normal(self) -> &'a mut W {
        self.variant(P0_17OD_A::NORMAL)
    }
    #[doc = "Open-drain. P0.17 pin is in the open drain mode."]
    #[inline(always)]
    pub fn open_drain(self) -> &'a mut W {
        self.variant(P0_17OD_A::OPEN_DRAIN)
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
#[doc = "Port 0 pin 18 open drain mode control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P0_18OD_A {
    #[doc = "0: Normal. P0.18 pin is in the normal (not open drain) mode."]
    NORMAL = 0,
    #[doc = "1: Open-drain. P0.18 pin is in the open drain mode."]
    OPEN_DRAIN = 1,
}
impl From<P0_18OD_A> for bool {
    #[inline(always)]
    fn from(variant: P0_18OD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `P0_18OD`"]
pub type P0_18OD_R = crate::R<bool, P0_18OD_A>;
impl P0_18OD_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> P0_18OD_A {
        match self.bits {
            false => P0_18OD_A::NORMAL,
            true => P0_18OD_A::OPEN_DRAIN,
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == P0_18OD_A::NORMAL
    }
    #[doc = "Checks if the value of the field is `OPEN_DRAIN`"]
    #[inline(always)]
    pub fn is_open_drain(&self) -> bool {
        *self == P0_18OD_A::OPEN_DRAIN
    }
}
#[doc = "Write proxy for field `P0_18OD`"]
pub struct P0_18OD_W<'a> {
    w: &'a mut W,
}
impl<'a> P0_18OD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: P0_18OD_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Normal. P0.18 pin is in the normal (not open drain) mode."]
    #[inline(always)]
    pub fn normal(self) -> &'a mut W {
        self.variant(P0_18OD_A::NORMAL)
    }
    #[doc = "Open-drain. P0.18 pin is in the open drain mode."]
    #[inline(always)]
    pub fn open_drain(self) -> &'a mut W {
        self.variant(P0_18OD_A::OPEN_DRAIN)
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
#[doc = "Port 0 pin 19 open drain mode control. Pins may potentially be used for I2C-buses using standard port pins. If so, they should be configured for open drain mode via the related bits in PINMODE_OD0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P0_19OD_A {
    #[doc = "0: Normal. P0.19 pin is in the normal (not open drain) mode."]
    NORMAL = 0,
    #[doc = "1: Open-drain. P0.19 pin is in the open drain mode."]
    OPEN_DRAIN = 1,
}
impl From<P0_19OD_A> for bool {
    #[inline(always)]
    fn from(variant: P0_19OD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `P0_19OD`"]
pub type P0_19OD_R = crate::R<bool, P0_19OD_A>;
impl P0_19OD_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> P0_19OD_A {
        match self.bits {
            false => P0_19OD_A::NORMAL,
            true => P0_19OD_A::OPEN_DRAIN,
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == P0_19OD_A::NORMAL
    }
    #[doc = "Checks if the value of the field is `OPEN_DRAIN`"]
    #[inline(always)]
    pub fn is_open_drain(&self) -> bool {
        *self == P0_19OD_A::OPEN_DRAIN
    }
}
#[doc = "Write proxy for field `P0_19OD`"]
pub struct P0_19OD_W<'a> {
    w: &'a mut W,
}
impl<'a> P0_19OD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: P0_19OD_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Normal. P0.19 pin is in the normal (not open drain) mode."]
    #[inline(always)]
    pub fn normal(self) -> &'a mut W {
        self.variant(P0_19OD_A::NORMAL)
    }
    #[doc = "Open-drain. P0.19 pin is in the open drain mode."]
    #[inline(always)]
    pub fn open_drain(self) -> &'a mut W {
        self.variant(P0_19OD_A::OPEN_DRAIN)
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
#[doc = "Port 0 pin 20open drain mode control. Pins may potentially be used for I2C-buses using standard port pins. If so, they should be configured for open drain mode via the related bits in PINMODE_OD0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P0_20OD_A {
    #[doc = "0: Normal. P0.20 pin is in the normal (not open drain) mode."]
    NORMAL = 0,
    #[doc = "1: Open-drain. P0.20 pin is in the open drain mode."]
    OPEN_DRAIN = 1,
}
impl From<P0_20OD_A> for bool {
    #[inline(always)]
    fn from(variant: P0_20OD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `P0_20OD`"]
pub type P0_20OD_R = crate::R<bool, P0_20OD_A>;
impl P0_20OD_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> P0_20OD_A {
        match self.bits {
            false => P0_20OD_A::NORMAL,
            true => P0_20OD_A::OPEN_DRAIN,
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == P0_20OD_A::NORMAL
    }
    #[doc = "Checks if the value of the field is `OPEN_DRAIN`"]
    #[inline(always)]
    pub fn is_open_drain(&self) -> bool {
        *self == P0_20OD_A::OPEN_DRAIN
    }
}
#[doc = "Write proxy for field `P0_20OD`"]
pub struct P0_20OD_W<'a> {
    w: &'a mut W,
}
impl<'a> P0_20OD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: P0_20OD_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Normal. P0.20 pin is in the normal (not open drain) mode."]
    #[inline(always)]
    pub fn normal(self) -> &'a mut W {
        self.variant(P0_20OD_A::NORMAL)
    }
    #[doc = "Open-drain. P0.20 pin is in the open drain mode."]
    #[inline(always)]
    pub fn open_drain(self) -> &'a mut W {
        self.variant(P0_20OD_A::OPEN_DRAIN)
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
#[doc = "Port 0 pin 21 open drain mode control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P0_21OD_A {
    #[doc = "0: Normal. P0.21 pin is in the normal (not open drain) mode."]
    NORMAL = 0,
    #[doc = "1: Open-drain. P0.21 pin is in the open drain mode."]
    OPEN_DRAIN = 1,
}
impl From<P0_21OD_A> for bool {
    #[inline(always)]
    fn from(variant: P0_21OD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `P0_21OD`"]
pub type P0_21OD_R = crate::R<bool, P0_21OD_A>;
impl P0_21OD_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> P0_21OD_A {
        match self.bits {
            false => P0_21OD_A::NORMAL,
            true => P0_21OD_A::OPEN_DRAIN,
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == P0_21OD_A::NORMAL
    }
    #[doc = "Checks if the value of the field is `OPEN_DRAIN`"]
    #[inline(always)]
    pub fn is_open_drain(&self) -> bool {
        *self == P0_21OD_A::OPEN_DRAIN
    }
}
#[doc = "Write proxy for field `P0_21OD`"]
pub struct P0_21OD_W<'a> {
    w: &'a mut W,
}
impl<'a> P0_21OD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: P0_21OD_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Normal. P0.21 pin is in the normal (not open drain) mode."]
    #[inline(always)]
    pub fn normal(self) -> &'a mut W {
        self.variant(P0_21OD_A::NORMAL)
    }
    #[doc = "Open-drain. P0.21 pin is in the open drain mode."]
    #[inline(always)]
    pub fn open_drain(self) -> &'a mut W {
        self.variant(P0_21OD_A::OPEN_DRAIN)
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
#[doc = "Port 0 pin 22 open drain mode control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P0_22OD_A {
    #[doc = "0: Normal. P0.22 pin is in the normal (not open drain) mode."]
    NORMAL = 0,
    #[doc = "1: Open-drain. P0.22 pin is in the open drain mode."]
    OPEN_DRAIN = 1,
}
impl From<P0_22OD_A> for bool {
    #[inline(always)]
    fn from(variant: P0_22OD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `P0_22OD`"]
pub type P0_22OD_R = crate::R<bool, P0_22OD_A>;
impl P0_22OD_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> P0_22OD_A {
        match self.bits {
            false => P0_22OD_A::NORMAL,
            true => P0_22OD_A::OPEN_DRAIN,
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == P0_22OD_A::NORMAL
    }
    #[doc = "Checks if the value of the field is `OPEN_DRAIN`"]
    #[inline(always)]
    pub fn is_open_drain(&self) -> bool {
        *self == P0_22OD_A::OPEN_DRAIN
    }
}
#[doc = "Write proxy for field `P0_22OD`"]
pub struct P0_22OD_W<'a> {
    w: &'a mut W,
}
impl<'a> P0_22OD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: P0_22OD_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Normal. P0.22 pin is in the normal (not open drain) mode."]
    #[inline(always)]
    pub fn normal(self) -> &'a mut W {
        self.variant(P0_22OD_A::NORMAL)
    }
    #[doc = "Open-drain. P0.22 pin is in the open drain mode."]
    #[inline(always)]
    pub fn open_drain(self) -> &'a mut W {
        self.variant(P0_22OD_A::OPEN_DRAIN)
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
#[doc = "Port 0 pin 23 open drain mode control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P0_23OD_A {
    #[doc = "0: Normal. P0.23 pin is in the normal (not open drain) mode."]
    NORMAL = 0,
    #[doc = "1: Open-drain. P0.23 pin is in the open drain mode."]
    OPEN_DRAIN = 1,
}
impl From<P0_23OD_A> for bool {
    #[inline(always)]
    fn from(variant: P0_23OD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `P0_23OD`"]
pub type P0_23OD_R = crate::R<bool, P0_23OD_A>;
impl P0_23OD_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> P0_23OD_A {
        match self.bits {
            false => P0_23OD_A::NORMAL,
            true => P0_23OD_A::OPEN_DRAIN,
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == P0_23OD_A::NORMAL
    }
    #[doc = "Checks if the value of the field is `OPEN_DRAIN`"]
    #[inline(always)]
    pub fn is_open_drain(&self) -> bool {
        *self == P0_23OD_A::OPEN_DRAIN
    }
}
#[doc = "Write proxy for field `P0_23OD`"]
pub struct P0_23OD_W<'a> {
    w: &'a mut W,
}
impl<'a> P0_23OD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: P0_23OD_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Normal. P0.23 pin is in the normal (not open drain) mode."]
    #[inline(always)]
    pub fn normal(self) -> &'a mut W {
        self.variant(P0_23OD_A::NORMAL)
    }
    #[doc = "Open-drain. P0.23 pin is in the open drain mode."]
    #[inline(always)]
    pub fn open_drain(self) -> &'a mut W {
        self.variant(P0_23OD_A::OPEN_DRAIN)
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
#[doc = "Port 0 pin 24open drain mode control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P0_24OD_A {
    #[doc = "0: Normal. P0.23 pin is in the normal (not open drain) mode."]
    NORMAL = 0,
    #[doc = "1: Open-drain. P0.23 pin is in the open drain mode."]
    OPEN_DRAIN = 1,
}
impl From<P0_24OD_A> for bool {
    #[inline(always)]
    fn from(variant: P0_24OD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `P0_24OD`"]
pub type P0_24OD_R = crate::R<bool, P0_24OD_A>;
impl P0_24OD_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> P0_24OD_A {
        match self.bits {
            false => P0_24OD_A::NORMAL,
            true => P0_24OD_A::OPEN_DRAIN,
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == P0_24OD_A::NORMAL
    }
    #[doc = "Checks if the value of the field is `OPEN_DRAIN`"]
    #[inline(always)]
    pub fn is_open_drain(&self) -> bool {
        *self == P0_24OD_A::OPEN_DRAIN
    }
}
#[doc = "Write proxy for field `P0_24OD`"]
pub struct P0_24OD_W<'a> {
    w: &'a mut W,
}
impl<'a> P0_24OD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: P0_24OD_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Normal. P0.23 pin is in the normal (not open drain) mode."]
    #[inline(always)]
    pub fn normal(self) -> &'a mut W {
        self.variant(P0_24OD_A::NORMAL)
    }
    #[doc = "Open-drain. P0.23 pin is in the open drain mode."]
    #[inline(always)]
    pub fn open_drain(self) -> &'a mut W {
        self.variant(P0_24OD_A::OPEN_DRAIN)
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
#[doc = "Port 0 pin 25 open drain mode control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P0_25OD_A {
    #[doc = "0: Normal. P0.25 pin is in the normal (not open drain) mode."]
    NORMAL = 0,
    #[doc = "1: Open-drain. P0.25 pin is in the open drain mode."]
    OPEN_DRAIN = 1,
}
impl From<P0_25OD_A> for bool {
    #[inline(always)]
    fn from(variant: P0_25OD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `P0_25OD`"]
pub type P0_25OD_R = crate::R<bool, P0_25OD_A>;
impl P0_25OD_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> P0_25OD_A {
        match self.bits {
            false => P0_25OD_A::NORMAL,
            true => P0_25OD_A::OPEN_DRAIN,
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == P0_25OD_A::NORMAL
    }
    #[doc = "Checks if the value of the field is `OPEN_DRAIN`"]
    #[inline(always)]
    pub fn is_open_drain(&self) -> bool {
        *self == P0_25OD_A::OPEN_DRAIN
    }
}
#[doc = "Write proxy for field `P0_25OD`"]
pub struct P0_25OD_W<'a> {
    w: &'a mut W,
}
impl<'a> P0_25OD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: P0_25OD_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Normal. P0.25 pin is in the normal (not open drain) mode."]
    #[inline(always)]
    pub fn normal(self) -> &'a mut W {
        self.variant(P0_25OD_A::NORMAL)
    }
    #[doc = "Open-drain. P0.25 pin is in the open drain mode."]
    #[inline(always)]
    pub fn open_drain(self) -> &'a mut W {
        self.variant(P0_25OD_A::OPEN_DRAIN)
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
#[doc = "Port 0 pin 26 open drain mode control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P0_26OD_A {
    #[doc = "0: Normal. P0.26 pin is in the normal (not open drain) mode."]
    NORMAL = 0,
    #[doc = "1: Open-drain. P0.26 pin is in the open drain mode."]
    OPEN_DRAIN = 1,
}
impl From<P0_26OD_A> for bool {
    #[inline(always)]
    fn from(variant: P0_26OD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `P0_26OD`"]
pub type P0_26OD_R = crate::R<bool, P0_26OD_A>;
impl P0_26OD_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> P0_26OD_A {
        match self.bits {
            false => P0_26OD_A::NORMAL,
            true => P0_26OD_A::OPEN_DRAIN,
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == P0_26OD_A::NORMAL
    }
    #[doc = "Checks if the value of the field is `OPEN_DRAIN`"]
    #[inline(always)]
    pub fn is_open_drain(&self) -> bool {
        *self == P0_26OD_A::OPEN_DRAIN
    }
}
#[doc = "Write proxy for field `P0_26OD`"]
pub struct P0_26OD_W<'a> {
    w: &'a mut W,
}
impl<'a> P0_26OD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: P0_26OD_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Normal. P0.26 pin is in the normal (not open drain) mode."]
    #[inline(always)]
    pub fn normal(self) -> &'a mut W {
        self.variant(P0_26OD_A::NORMAL)
    }
    #[doc = "Open-drain. P0.26 pin is in the open drain mode."]
    #[inline(always)]
    pub fn open_drain(self) -> &'a mut W {
        self.variant(P0_26OD_A::OPEN_DRAIN)
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
#[doc = "Port 0 pin 29 open drain mode control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P0_29OD_A {
    #[doc = "0: Normal. P0.29 pin is in the normal (not open drain) mode."]
    NORMAL = 0,
    #[doc = "1: Open-drain. P0.29 pin is in the open drain mode."]
    OPEN_DRAIN = 1,
}
impl From<P0_29OD_A> for bool {
    #[inline(always)]
    fn from(variant: P0_29OD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `P0_29OD`"]
pub type P0_29OD_R = crate::R<bool, P0_29OD_A>;
impl P0_29OD_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> P0_29OD_A {
        match self.bits {
            false => P0_29OD_A::NORMAL,
            true => P0_29OD_A::OPEN_DRAIN,
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == P0_29OD_A::NORMAL
    }
    #[doc = "Checks if the value of the field is `OPEN_DRAIN`"]
    #[inline(always)]
    pub fn is_open_drain(&self) -> bool {
        *self == P0_29OD_A::OPEN_DRAIN
    }
}
#[doc = "Write proxy for field `P0_29OD`"]
pub struct P0_29OD_W<'a> {
    w: &'a mut W,
}
impl<'a> P0_29OD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: P0_29OD_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Normal. P0.29 pin is in the normal (not open drain) mode."]
    #[inline(always)]
    pub fn normal(self) -> &'a mut W {
        self.variant(P0_29OD_A::NORMAL)
    }
    #[doc = "Open-drain. P0.29 pin is in the open drain mode."]
    #[inline(always)]
    pub fn open_drain(self) -> &'a mut W {
        self.variant(P0_29OD_A::OPEN_DRAIN)
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
#[doc = "Port 0 pin 30 open drain mode control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P0_30OD_A {
    #[doc = "0: Normal. P0.30 pin is in the normal (not open drain) mode."]
    NORMAL = 0,
    #[doc = "1: Open-drain. P0.30 pin is in the open drain mode."]
    OPEN_DRAIN = 1,
}
impl From<P0_30OD_A> for bool {
    #[inline(always)]
    fn from(variant: P0_30OD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `P0_30OD`"]
pub type P0_30OD_R = crate::R<bool, P0_30OD_A>;
impl P0_30OD_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> P0_30OD_A {
        match self.bits {
            false => P0_30OD_A::NORMAL,
            true => P0_30OD_A::OPEN_DRAIN,
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == P0_30OD_A::NORMAL
    }
    #[doc = "Checks if the value of the field is `OPEN_DRAIN`"]
    #[inline(always)]
    pub fn is_open_drain(&self) -> bool {
        *self == P0_30OD_A::OPEN_DRAIN
    }
}
#[doc = "Write proxy for field `P0_30OD`"]
pub struct P0_30OD_W<'a> {
    w: &'a mut W,
}
impl<'a> P0_30OD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: P0_30OD_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Normal. P0.30 pin is in the normal (not open drain) mode."]
    #[inline(always)]
    pub fn normal(self) -> &'a mut W {
        self.variant(P0_30OD_A::NORMAL)
    }
    #[doc = "Open-drain. P0.30 pin is in the open drain mode."]
    #[inline(always)]
    pub fn open_drain(self) -> &'a mut W {
        self.variant(P0_30OD_A::OPEN_DRAIN)
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
impl R {
    #[doc = "Bit 0 - Port 0 pin 0 open drain mode control. Pins may potentially be used for I2C-buses using standard port pins. If so, they should be configured for open drain mode via the related bits in PINMODE_OD0."]
    #[inline(always)]
    pub fn p0_00od(&self) -> P0_00OD_R {
        P0_00OD_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Port 0 pin 1 open drain mode control. Pins may potentially be used for I2C-buses using standard port pins. If so, they should be configured for open drain mode via the related bits in PINMODE_OD0."]
    #[inline(always)]
    pub fn p0_01od(&self) -> P0_01OD_R {
        P0_01OD_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Port 0 pin 2 open drain mode control"]
    #[inline(always)]
    pub fn p0_02od(&self) -> P0_02OD_R {
        P0_02OD_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Port 0 pin 3 open drain mode control"]
    #[inline(always)]
    pub fn p0_03od(&self) -> P0_03OD_R {
        P0_03OD_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Port 0 pin 4 open drain mode control"]
    #[inline(always)]
    pub fn p0_04od(&self) -> P0_04OD_R {
        P0_04OD_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Port 0 pin 5 open drain mode control"]
    #[inline(always)]
    pub fn p0_05od(&self) -> P0_05OD_R {
        P0_05OD_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Port 0 pin 6 open drain mode control"]
    #[inline(always)]
    pub fn p0_06od(&self) -> P0_06OD_R {
        P0_06OD_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Port 0 pin 7 open drain mode control"]
    #[inline(always)]
    pub fn p0_07od(&self) -> P0_07OD_R {
        P0_07OD_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Port 0 pin 8 open drain mode control"]
    #[inline(always)]
    pub fn p0_08od(&self) -> P0_08OD_R {
        P0_08OD_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Port 0 pin 9 open drain mode control"]
    #[inline(always)]
    pub fn p0_09od(&self) -> P0_09OD_R {
        P0_09OD_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Port 0 pin 10 open drain mode control. Pins may potentially be used for I2C-buses using standard port pins. If so, they should be configured for open drain mode via the related bits in PINMODE_OD0."]
    #[inline(always)]
    pub fn p0_10od(&self) -> P0_10OD_R {
        P0_10OD_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Port 0 pin 11 open drain mode control. Pins may potentially be used for I2C-buses using standard port pins. If so, they should be configured for open drain mode via the related bits in PINMODE_OD0."]
    #[inline(always)]
    pub fn p0_11od(&self) -> P0_11OD_R {
        P0_11OD_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Port 0 pin 15 open drain mode control"]
    #[inline(always)]
    pub fn p0_15od(&self) -> P0_15OD_R {
        P0_15OD_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Port 0 pin 16 open drain mode control"]
    #[inline(always)]
    pub fn p0_16od(&self) -> P0_16OD_R {
        P0_16OD_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Port 0 pin 17 open drain mode control"]
    #[inline(always)]
    pub fn p0_17od(&self) -> P0_17OD_R {
        P0_17OD_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Port 0 pin 18 open drain mode control"]
    #[inline(always)]
    pub fn p0_18od(&self) -> P0_18OD_R {
        P0_18OD_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Port 0 pin 19 open drain mode control. Pins may potentially be used for I2C-buses using standard port pins. If so, they should be configured for open drain mode via the related bits in PINMODE_OD0."]
    #[inline(always)]
    pub fn p0_19od(&self) -> P0_19OD_R {
        P0_19OD_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Port 0 pin 20open drain mode control. Pins may potentially be used for I2C-buses using standard port pins. If so, they should be configured for open drain mode via the related bits in PINMODE_OD0."]
    #[inline(always)]
    pub fn p0_20od(&self) -> P0_20OD_R {
        P0_20OD_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Port 0 pin 21 open drain mode control"]
    #[inline(always)]
    pub fn p0_21od(&self) -> P0_21OD_R {
        P0_21OD_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Port 0 pin 22 open drain mode control"]
    #[inline(always)]
    pub fn p0_22od(&self) -> P0_22OD_R {
        P0_22OD_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - Port 0 pin 23 open drain mode control"]
    #[inline(always)]
    pub fn p0_23od(&self) -> P0_23OD_R {
        P0_23OD_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Port 0 pin 24open drain mode control"]
    #[inline(always)]
    pub fn p0_24od(&self) -> P0_24OD_R {
        P0_24OD_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Port 0 pin 25 open drain mode control"]
    #[inline(always)]
    pub fn p0_25od(&self) -> P0_25OD_R {
        P0_25OD_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - Port 0 pin 26 open drain mode control"]
    #[inline(always)]
    pub fn p0_26od(&self) -> P0_26OD_R {
        P0_26OD_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 29 - Port 0 pin 29 open drain mode control"]
    #[inline(always)]
    pub fn p0_29od(&self) -> P0_29OD_R {
        P0_29OD_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - Port 0 pin 30 open drain mode control"]
    #[inline(always)]
    pub fn p0_30od(&self) -> P0_30OD_R {
        P0_30OD_R::new(((self.bits >> 30) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Port 0 pin 0 open drain mode control. Pins may potentially be used for I2C-buses using standard port pins. If so, they should be configured for open drain mode via the related bits in PINMODE_OD0."]
    #[inline(always)]
    pub fn p0_00od(&mut self) -> P0_00OD_W {
        P0_00OD_W { w: self }
    }
    #[doc = "Bit 1 - Port 0 pin 1 open drain mode control. Pins may potentially be used for I2C-buses using standard port pins. If so, they should be configured for open drain mode via the related bits in PINMODE_OD0."]
    #[inline(always)]
    pub fn p0_01od(&mut self) -> P0_01OD_W {
        P0_01OD_W { w: self }
    }
    #[doc = "Bit 2 - Port 0 pin 2 open drain mode control"]
    #[inline(always)]
    pub fn p0_02od(&mut self) -> P0_02OD_W {
        P0_02OD_W { w: self }
    }
    #[doc = "Bit 3 - Port 0 pin 3 open drain mode control"]
    #[inline(always)]
    pub fn p0_03od(&mut self) -> P0_03OD_W {
        P0_03OD_W { w: self }
    }
    #[doc = "Bit 4 - Port 0 pin 4 open drain mode control"]
    #[inline(always)]
    pub fn p0_04od(&mut self) -> P0_04OD_W {
        P0_04OD_W { w: self }
    }
    #[doc = "Bit 5 - Port 0 pin 5 open drain mode control"]
    #[inline(always)]
    pub fn p0_05od(&mut self) -> P0_05OD_W {
        P0_05OD_W { w: self }
    }
    #[doc = "Bit 6 - Port 0 pin 6 open drain mode control"]
    #[inline(always)]
    pub fn p0_06od(&mut self) -> P0_06OD_W {
        P0_06OD_W { w: self }
    }
    #[doc = "Bit 7 - Port 0 pin 7 open drain mode control"]
    #[inline(always)]
    pub fn p0_07od(&mut self) -> P0_07OD_W {
        P0_07OD_W { w: self }
    }
    #[doc = "Bit 8 - Port 0 pin 8 open drain mode control"]
    #[inline(always)]
    pub fn p0_08od(&mut self) -> P0_08OD_W {
        P0_08OD_W { w: self }
    }
    #[doc = "Bit 9 - Port 0 pin 9 open drain mode control"]
    #[inline(always)]
    pub fn p0_09od(&mut self) -> P0_09OD_W {
        P0_09OD_W { w: self }
    }
    #[doc = "Bit 10 - Port 0 pin 10 open drain mode control. Pins may potentially be used for I2C-buses using standard port pins. If so, they should be configured for open drain mode via the related bits in PINMODE_OD0."]
    #[inline(always)]
    pub fn p0_10od(&mut self) -> P0_10OD_W {
        P0_10OD_W { w: self }
    }
    #[doc = "Bit 11 - Port 0 pin 11 open drain mode control. Pins may potentially be used for I2C-buses using standard port pins. If so, they should be configured for open drain mode via the related bits in PINMODE_OD0."]
    #[inline(always)]
    pub fn p0_11od(&mut self) -> P0_11OD_W {
        P0_11OD_W { w: self }
    }
    #[doc = "Bit 15 - Port 0 pin 15 open drain mode control"]
    #[inline(always)]
    pub fn p0_15od(&mut self) -> P0_15OD_W {
        P0_15OD_W { w: self }
    }
    #[doc = "Bit 16 - Port 0 pin 16 open drain mode control"]
    #[inline(always)]
    pub fn p0_16od(&mut self) -> P0_16OD_W {
        P0_16OD_W { w: self }
    }
    #[doc = "Bit 17 - Port 0 pin 17 open drain mode control"]
    #[inline(always)]
    pub fn p0_17od(&mut self) -> P0_17OD_W {
        P0_17OD_W { w: self }
    }
    #[doc = "Bit 18 - Port 0 pin 18 open drain mode control"]
    #[inline(always)]
    pub fn p0_18od(&mut self) -> P0_18OD_W {
        P0_18OD_W { w: self }
    }
    #[doc = "Bit 19 - Port 0 pin 19 open drain mode control. Pins may potentially be used for I2C-buses using standard port pins. If so, they should be configured for open drain mode via the related bits in PINMODE_OD0."]
    #[inline(always)]
    pub fn p0_19od(&mut self) -> P0_19OD_W {
        P0_19OD_W { w: self }
    }
    #[doc = "Bit 20 - Port 0 pin 20open drain mode control. Pins may potentially be used for I2C-buses using standard port pins. If so, they should be configured for open drain mode via the related bits in PINMODE_OD0."]
    #[inline(always)]
    pub fn p0_20od(&mut self) -> P0_20OD_W {
        P0_20OD_W { w: self }
    }
    #[doc = "Bit 21 - Port 0 pin 21 open drain mode control"]
    #[inline(always)]
    pub fn p0_21od(&mut self) -> P0_21OD_W {
        P0_21OD_W { w: self }
    }
    #[doc = "Bit 22 - Port 0 pin 22 open drain mode control"]
    #[inline(always)]
    pub fn p0_22od(&mut self) -> P0_22OD_W {
        P0_22OD_W { w: self }
    }
    #[doc = "Bit 23 - Port 0 pin 23 open drain mode control"]
    #[inline(always)]
    pub fn p0_23od(&mut self) -> P0_23OD_W {
        P0_23OD_W { w: self }
    }
    #[doc = "Bit 24 - Port 0 pin 24open drain mode control"]
    #[inline(always)]
    pub fn p0_24od(&mut self) -> P0_24OD_W {
        P0_24OD_W { w: self }
    }
    #[doc = "Bit 25 - Port 0 pin 25 open drain mode control"]
    #[inline(always)]
    pub fn p0_25od(&mut self) -> P0_25OD_W {
        P0_25OD_W { w: self }
    }
    #[doc = "Bit 26 - Port 0 pin 26 open drain mode control"]
    #[inline(always)]
    pub fn p0_26od(&mut self) -> P0_26OD_W {
        P0_26OD_W { w: self }
    }
    #[doc = "Bit 29 - Port 0 pin 29 open drain mode control"]
    #[inline(always)]
    pub fn p0_29od(&mut self) -> P0_29OD_W {
        P0_29OD_W { w: self }
    }
    #[doc = "Bit 30 - Port 0 pin 30 open drain mode control"]
    #[inline(always)]
    pub fn p0_30od(&mut self) -> P0_30OD_W {
        P0_30OD_W { w: self }
    }
}
