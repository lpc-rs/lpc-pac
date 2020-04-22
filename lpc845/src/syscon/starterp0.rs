#[doc = "Reader of register STARTERP0"]
pub type R = crate::R<u32, super::STARTERP0>;
#[doc = "Writer for register STARTERP0"]
pub type W = crate::W<u32, super::STARTERP0>;
#[doc = "Register STARTERP0 `reset()`'s with value 0"]
impl crate::ResetValue for super::STARTERP0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "GPIO pin interrupt 0 wake-up\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PINT0_A {
    #[doc = "0: Disabled"]
    DISABLED = 0,
    #[doc = "1: Enabled"]
    ENABLED = 1,
}
impl From<PINT0_A> for bool {
    #[inline(always)]
    fn from(variant: PINT0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PINT0`"]
pub type PINT0_R = crate::R<bool, PINT0_A>;
impl PINT0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PINT0_A {
        match self.bits {
            false => PINT0_A::DISABLED,
            true => PINT0_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PINT0_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == PINT0_A::ENABLED
    }
}
#[doc = "Write proxy for field `PINT0`"]
pub struct PINT0_W<'a> {
    w: &'a mut W,
}
impl<'a> PINT0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PINT0_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(PINT0_A::DISABLED)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(PINT0_A::ENABLED)
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
#[doc = "GPIO pin interrupt 1 wake-up\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PINT1_A {
    #[doc = "0: Disabled"]
    DISABLED = 0,
    #[doc = "1: Enabled"]
    ENABLED = 1,
}
impl From<PINT1_A> for bool {
    #[inline(always)]
    fn from(variant: PINT1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PINT1`"]
pub type PINT1_R = crate::R<bool, PINT1_A>;
impl PINT1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PINT1_A {
        match self.bits {
            false => PINT1_A::DISABLED,
            true => PINT1_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PINT1_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == PINT1_A::ENABLED
    }
}
#[doc = "Write proxy for field `PINT1`"]
pub struct PINT1_W<'a> {
    w: &'a mut W,
}
impl<'a> PINT1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PINT1_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(PINT1_A::DISABLED)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(PINT1_A::ENABLED)
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
#[doc = "GPIO pin interrupt 2 wake-up\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PINT2_A {
    #[doc = "0: Disabled"]
    DISABLED = 0,
    #[doc = "1: Enabled"]
    ENABLED = 1,
}
impl From<PINT2_A> for bool {
    #[inline(always)]
    fn from(variant: PINT2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PINT2`"]
pub type PINT2_R = crate::R<bool, PINT2_A>;
impl PINT2_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PINT2_A {
        match self.bits {
            false => PINT2_A::DISABLED,
            true => PINT2_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PINT2_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == PINT2_A::ENABLED
    }
}
#[doc = "Write proxy for field `PINT2`"]
pub struct PINT2_W<'a> {
    w: &'a mut W,
}
impl<'a> PINT2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PINT2_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(PINT2_A::DISABLED)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(PINT2_A::ENABLED)
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
#[doc = "GPIO pin interrupt 3 wake-up\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PINT3_A {
    #[doc = "0: Disabled"]
    DISABLED = 0,
    #[doc = "1: Enabled"]
    ENABLED = 1,
}
impl From<PINT3_A> for bool {
    #[inline(always)]
    fn from(variant: PINT3_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PINT3`"]
pub type PINT3_R = crate::R<bool, PINT3_A>;
impl PINT3_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PINT3_A {
        match self.bits {
            false => PINT3_A::DISABLED,
            true => PINT3_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PINT3_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == PINT3_A::ENABLED
    }
}
#[doc = "Write proxy for field `PINT3`"]
pub struct PINT3_W<'a> {
    w: &'a mut W,
}
impl<'a> PINT3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PINT3_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(PINT3_A::DISABLED)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(PINT3_A::ENABLED)
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
#[doc = "GPIO pin interrupt 4 wake-up\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PINT4_A {
    #[doc = "0: Disabled"]
    DISABLED = 0,
    #[doc = "1: Enabled"]
    ENABLED = 1,
}
impl From<PINT4_A> for bool {
    #[inline(always)]
    fn from(variant: PINT4_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PINT4`"]
pub type PINT4_R = crate::R<bool, PINT4_A>;
impl PINT4_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PINT4_A {
        match self.bits {
            false => PINT4_A::DISABLED,
            true => PINT4_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PINT4_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == PINT4_A::ENABLED
    }
}
#[doc = "Write proxy for field `PINT4`"]
pub struct PINT4_W<'a> {
    w: &'a mut W,
}
impl<'a> PINT4_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PINT4_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(PINT4_A::DISABLED)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(PINT4_A::ENABLED)
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
#[doc = "GPIO pin interrupt 5 wake-up\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PINT5_A {
    #[doc = "0: Disabled"]
    DISABLED = 0,
    #[doc = "1: Enabled"]
    ENABLED = 1,
}
impl From<PINT5_A> for bool {
    #[inline(always)]
    fn from(variant: PINT5_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PINT5`"]
pub type PINT5_R = crate::R<bool, PINT5_A>;
impl PINT5_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PINT5_A {
        match self.bits {
            false => PINT5_A::DISABLED,
            true => PINT5_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PINT5_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == PINT5_A::ENABLED
    }
}
#[doc = "Write proxy for field `PINT5`"]
pub struct PINT5_W<'a> {
    w: &'a mut W,
}
impl<'a> PINT5_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PINT5_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(PINT5_A::DISABLED)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(PINT5_A::ENABLED)
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
#[doc = "GPIO pin interrupt 6 wake-up\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PINT6_A {
    #[doc = "0: Disabled"]
    DISABLED = 0,
    #[doc = "1: Enabled"]
    ENABLED = 1,
}
impl From<PINT6_A> for bool {
    #[inline(always)]
    fn from(variant: PINT6_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PINT6`"]
pub type PINT6_R = crate::R<bool, PINT6_A>;
impl PINT6_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PINT6_A {
        match self.bits {
            false => PINT6_A::DISABLED,
            true => PINT6_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PINT6_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == PINT6_A::ENABLED
    }
}
#[doc = "Write proxy for field `PINT6`"]
pub struct PINT6_W<'a> {
    w: &'a mut W,
}
impl<'a> PINT6_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PINT6_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(PINT6_A::DISABLED)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(PINT6_A::ENABLED)
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
#[doc = "GPIO pin interrupt 7 wake-up\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PINT7_A {
    #[doc = "0: Disabled"]
    DISABLED = 0,
    #[doc = "1: Enabled"]
    ENABLED = 1,
}
impl From<PINT7_A> for bool {
    #[inline(always)]
    fn from(variant: PINT7_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PINT7`"]
pub type PINT7_R = crate::R<bool, PINT7_A>;
impl PINT7_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PINT7_A {
        match self.bits {
            false => PINT7_A::DISABLED,
            true => PINT7_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PINT7_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == PINT7_A::ENABLED
    }
}
#[doc = "Write proxy for field `PINT7`"]
pub struct PINT7_W<'a> {
    w: &'a mut W,
}
impl<'a> PINT7_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PINT7_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(PINT7_A::DISABLED)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(PINT7_A::ENABLED)
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
impl R {
    #[doc = "Bit 0 - GPIO pin interrupt 0 wake-up"]
    #[inline(always)]
    pub fn pint0(&self) -> PINT0_R {
        PINT0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - GPIO pin interrupt 1 wake-up"]
    #[inline(always)]
    pub fn pint1(&self) -> PINT1_R {
        PINT1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - GPIO pin interrupt 2 wake-up"]
    #[inline(always)]
    pub fn pint2(&self) -> PINT2_R {
        PINT2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - GPIO pin interrupt 3 wake-up"]
    #[inline(always)]
    pub fn pint3(&self) -> PINT3_R {
        PINT3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - GPIO pin interrupt 4 wake-up"]
    #[inline(always)]
    pub fn pint4(&self) -> PINT4_R {
        PINT4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - GPIO pin interrupt 5 wake-up"]
    #[inline(always)]
    pub fn pint5(&self) -> PINT5_R {
        PINT5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - GPIO pin interrupt 6 wake-up"]
    #[inline(always)]
    pub fn pint6(&self) -> PINT6_R {
        PINT6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - GPIO pin interrupt 7 wake-up"]
    #[inline(always)]
    pub fn pint7(&self) -> PINT7_R {
        PINT7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - GPIO pin interrupt 0 wake-up"]
    #[inline(always)]
    pub fn pint0(&mut self) -> PINT0_W {
        PINT0_W { w: self }
    }
    #[doc = "Bit 1 - GPIO pin interrupt 1 wake-up"]
    #[inline(always)]
    pub fn pint1(&mut self) -> PINT1_W {
        PINT1_W { w: self }
    }
    #[doc = "Bit 2 - GPIO pin interrupt 2 wake-up"]
    #[inline(always)]
    pub fn pint2(&mut self) -> PINT2_W {
        PINT2_W { w: self }
    }
    #[doc = "Bit 3 - GPIO pin interrupt 3 wake-up"]
    #[inline(always)]
    pub fn pint3(&mut self) -> PINT3_W {
        PINT3_W { w: self }
    }
    #[doc = "Bit 4 - GPIO pin interrupt 4 wake-up"]
    #[inline(always)]
    pub fn pint4(&mut self) -> PINT4_W {
        PINT4_W { w: self }
    }
    #[doc = "Bit 5 - GPIO pin interrupt 5 wake-up"]
    #[inline(always)]
    pub fn pint5(&mut self) -> PINT5_W {
        PINT5_W { w: self }
    }
    #[doc = "Bit 6 - GPIO pin interrupt 6 wake-up"]
    #[inline(always)]
    pub fn pint6(&mut self) -> PINT6_W {
        PINT6_W { w: self }
    }
    #[doc = "Bit 7 - GPIO pin interrupt 7 wake-up"]
    #[inline(always)]
    pub fn pint7(&mut self) -> PINT7_W {
        PINT7_W { w: self }
    }
}
