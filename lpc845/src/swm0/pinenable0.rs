#[doc = "Reader of register PINENABLE0"]
pub type R = crate::R<u32, super::PINENABLE0>;
#[doc = "Writer for register PINENABLE0"]
pub type W = crate::W<u32, super::PINENABLE0>;
#[doc = "Register PINENABLE0 `reset()`'s with value 0xffff_fd9f"]
impl crate::ResetValue for super::PINENABLE0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0xffff_fd9f
    }
}
#[doc = "ACMP_I1 function select.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ACMP_I1_A {
    #[doc = "0: ACMP_I1 enabled on pin PIO0_00."]
    ENABLED = 0,
    #[doc = "1: ACMP_I1 disabled."]
    DISABLED = 1,
}
impl From<ACMP_I1_A> for bool {
    #[inline(always)]
    fn from(variant: ACMP_I1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ACMP_I1`"]
pub type ACMP_I1_R = crate::R<bool, ACMP_I1_A>;
impl ACMP_I1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ACMP_I1_A {
        match self.bits {
            false => ACMP_I1_A::ENABLED,
            true => ACMP_I1_A::DISABLED,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ACMP_I1_A::ENABLED
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ACMP_I1_A::DISABLED
    }
}
#[doc = "Write proxy for field `ACMP_I1`"]
pub struct ACMP_I1_W<'a> {
    w: &'a mut W,
}
impl<'a> ACMP_I1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ACMP_I1_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "ACMP_I1 enabled on pin PIO0_00."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ACMP_I1_A::ENABLED)
    }
    #[doc = "ACMP_I1 disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ACMP_I1_A::DISABLED)
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
#[doc = "ACMP_I2 function select.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ACMP_I2_A {
    #[doc = "0: ACMP_I2 enabled on pin PIO0_1."]
    ACMP_I2_0 = 0,
    #[doc = "1: ACMP_I2 disabled."]
    ACMP_I2_1 = 1,
}
impl From<ACMP_I2_A> for bool {
    #[inline(always)]
    fn from(variant: ACMP_I2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ACMP_I2`"]
pub type ACMP_I2_R = crate::R<bool, ACMP_I2_A>;
impl ACMP_I2_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ACMP_I2_A {
        match self.bits {
            false => ACMP_I2_A::ACMP_I2_0,
            true => ACMP_I2_A::ACMP_I2_1,
        }
    }
    #[doc = "Checks if the value of the field is `ACMP_I2_0`"]
    #[inline(always)]
    pub fn is_acmp_i2_0(&self) -> bool {
        *self == ACMP_I2_A::ACMP_I2_0
    }
    #[doc = "Checks if the value of the field is `ACMP_I2_1`"]
    #[inline(always)]
    pub fn is_acmp_i2_1(&self) -> bool {
        *self == ACMP_I2_A::ACMP_I2_1
    }
}
#[doc = "Write proxy for field `ACMP_I2`"]
pub struct ACMP_I2_W<'a> {
    w: &'a mut W,
}
impl<'a> ACMP_I2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ACMP_I2_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "ACMP_I2 enabled on pin PIO0_1."]
    #[inline(always)]
    pub fn acmp_i2_0(self) -> &'a mut W {
        self.variant(ACMP_I2_A::ACMP_I2_0)
    }
    #[doc = "ACMP_I2 disabled."]
    #[inline(always)]
    pub fn acmp_i2_1(self) -> &'a mut W {
        self.variant(ACMP_I2_A::ACMP_I2_1)
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
#[doc = "ACMP_I3 function select.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ACMP_I3_A {
    #[doc = "0: ACMP_I3 enabled on pin PIO0_14."]
    ENABLED = 0,
    #[doc = "1: ACMP_I3 disabled."]
    DISABLED = 1,
}
impl From<ACMP_I3_A> for bool {
    #[inline(always)]
    fn from(variant: ACMP_I3_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ACMP_I3`"]
pub type ACMP_I3_R = crate::R<bool, ACMP_I3_A>;
impl ACMP_I3_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ACMP_I3_A {
        match self.bits {
            false => ACMP_I3_A::ENABLED,
            true => ACMP_I3_A::DISABLED,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ACMP_I3_A::ENABLED
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ACMP_I3_A::DISABLED
    }
}
#[doc = "Write proxy for field `ACMP_I3`"]
pub struct ACMP_I3_W<'a> {
    w: &'a mut W,
}
impl<'a> ACMP_I3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ACMP_I3_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "ACMP_I3 enabled on pin PIO0_14."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ACMP_I3_A::ENABLED)
    }
    #[doc = "ACMP_I3 disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ACMP_I3_A::DISABLED)
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
#[doc = "ACMP_I4 function select.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ACMP_I4_A {
    #[doc = "0: ACMP_I4 enabled on pin PIO0_23."]
    ENABLED = 0,
    #[doc = "1: ACMP_I4 disabled."]
    DISABLED = 1,
}
impl From<ACMP_I4_A> for bool {
    #[inline(always)]
    fn from(variant: ACMP_I4_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ACMP_I4`"]
pub type ACMP_I4_R = crate::R<bool, ACMP_I4_A>;
impl ACMP_I4_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ACMP_I4_A {
        match self.bits {
            false => ACMP_I4_A::ENABLED,
            true => ACMP_I4_A::DISABLED,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ACMP_I4_A::ENABLED
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ACMP_I4_A::DISABLED
    }
}
#[doc = "Write proxy for field `ACMP_I4`"]
pub struct ACMP_I4_W<'a> {
    w: &'a mut W,
}
impl<'a> ACMP_I4_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ACMP_I4_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "ACMP_I4 enabled on pin PIO0_23."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ACMP_I4_A::ENABLED)
    }
    #[doc = "ACMP_I4 disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ACMP_I4_A::DISABLED)
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
#[doc = "ACMP_I5 function select.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ACMP_I5_A {
    #[doc = "0: ACMP_I5 enabled on pin PIO0_30."]
    ENABLED = 0,
    #[doc = "1: ACMP_I5 disabled."]
    DISABLED = 1,
}
impl From<ACMP_I5_A> for bool {
    #[inline(always)]
    fn from(variant: ACMP_I5_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ACMP_I5`"]
pub type ACMP_I5_R = crate::R<bool, ACMP_I5_A>;
impl ACMP_I5_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ACMP_I5_A {
        match self.bits {
            false => ACMP_I5_A::ENABLED,
            true => ACMP_I5_A::DISABLED,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ACMP_I5_A::ENABLED
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ACMP_I5_A::DISABLED
    }
}
#[doc = "Write proxy for field `ACMP_I5`"]
pub struct ACMP_I5_W<'a> {
    w: &'a mut W,
}
impl<'a> ACMP_I5_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ACMP_I5_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "ACMP_I5 enabled on pin PIO0_30."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ACMP_I5_A::ENABLED)
    }
    #[doc = "ACMP_I5 disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ACMP_I5_A::DISABLED)
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
#[doc = "SWCLK function select.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SWCLK_A {
    #[doc = "0: SWCLK enabled on pin PIO0_3."]
    ENABLED = 0,
    #[doc = "1: SWCLK disabled."]
    DISABLED = 1,
}
impl From<SWCLK_A> for bool {
    #[inline(always)]
    fn from(variant: SWCLK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SWCLK`"]
pub type SWCLK_R = crate::R<bool, SWCLK_A>;
impl SWCLK_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SWCLK_A {
        match self.bits {
            false => SWCLK_A::ENABLED,
            true => SWCLK_A::DISABLED,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SWCLK_A::ENABLED
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SWCLK_A::DISABLED
    }
}
#[doc = "Write proxy for field `SWCLK`"]
pub struct SWCLK_W<'a> {
    w: &'a mut W,
}
impl<'a> SWCLK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SWCLK_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "SWCLK enabled on pin PIO0_3."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(SWCLK_A::ENABLED)
    }
    #[doc = "SWCLK disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(SWCLK_A::DISABLED)
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
#[doc = "SWDIO function select.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SWDIO_A {
    #[doc = "0: SWDIO enabled on pin PIO0_2."]
    ENABLED = 0,
    #[doc = "1: SWDIO disabled."]
    DISABLED = 1,
}
impl From<SWDIO_A> for bool {
    #[inline(always)]
    fn from(variant: SWDIO_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SWDIO`"]
pub type SWDIO_R = crate::R<bool, SWDIO_A>;
impl SWDIO_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SWDIO_A {
        match self.bits {
            false => SWDIO_A::ENABLED,
            true => SWDIO_A::DISABLED,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SWDIO_A::ENABLED
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SWDIO_A::DISABLED
    }
}
#[doc = "Write proxy for field `SWDIO`"]
pub struct SWDIO_W<'a> {
    w: &'a mut W,
}
impl<'a> SWDIO_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SWDIO_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "SWDIO enabled on pin PIO0_2."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(SWDIO_A::ENABLED)
    }
    #[doc = "SWDIO disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(SWDIO_A::DISABLED)
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
#[doc = "XTALIN function select.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum XTALIN_A {
    #[doc = "0: XTALIN enabled on pin PIO0_8."]
    ENABLED = 0,
    #[doc = "1: XTALIN disabled."]
    DISABLED = 1,
}
impl From<XTALIN_A> for bool {
    #[inline(always)]
    fn from(variant: XTALIN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `XTALIN`"]
pub type XTALIN_R = crate::R<bool, XTALIN_A>;
impl XTALIN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> XTALIN_A {
        match self.bits {
            false => XTALIN_A::ENABLED,
            true => XTALIN_A::DISABLED,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == XTALIN_A::ENABLED
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == XTALIN_A::DISABLED
    }
}
#[doc = "Write proxy for field `XTALIN`"]
pub struct XTALIN_W<'a> {
    w: &'a mut W,
}
impl<'a> XTALIN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: XTALIN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "XTALIN enabled on pin PIO0_8."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(XTALIN_A::ENABLED)
    }
    #[doc = "XTALIN disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(XTALIN_A::DISABLED)
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
#[doc = "XTALOUT function select.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum XTALOUT_A {
    #[doc = "0: XTALOUT enabled on pin PIO0_9."]
    ENABLED = 0,
    #[doc = "1: XTALOUT disabled."]
    DISABLED = 1,
}
impl From<XTALOUT_A> for bool {
    #[inline(always)]
    fn from(variant: XTALOUT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `XTALOUT`"]
pub type XTALOUT_R = crate::R<bool, XTALOUT_A>;
impl XTALOUT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> XTALOUT_A {
        match self.bits {
            false => XTALOUT_A::ENABLED,
            true => XTALOUT_A::DISABLED,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == XTALOUT_A::ENABLED
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == XTALOUT_A::DISABLED
    }
}
#[doc = "Write proxy for field `XTALOUT`"]
pub struct XTALOUT_W<'a> {
    w: &'a mut W,
}
impl<'a> XTALOUT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: XTALOUT_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "XTALOUT enabled on pin PIO0_9."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(XTALOUT_A::ENABLED)
    }
    #[doc = "XTALOUT disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(XTALOUT_A::DISABLED)
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
#[doc = "RESETN function select.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RESETN_A {
    #[doc = "0: RESETN enabled on pin PIO0_5."]
    ENABLED = 0,
    #[doc = "1: RESETN disabled."]
    DISABLED = 1,
}
impl From<RESETN_A> for bool {
    #[inline(always)]
    fn from(variant: RESETN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `RESETN`"]
pub type RESETN_R = crate::R<bool, RESETN_A>;
impl RESETN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RESETN_A {
        match self.bits {
            false => RESETN_A::ENABLED,
            true => RESETN_A::DISABLED,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == RESETN_A::ENABLED
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == RESETN_A::DISABLED
    }
}
#[doc = "Write proxy for field `RESETN`"]
pub struct RESETN_W<'a> {
    w: &'a mut W,
}
impl<'a> RESETN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RESETN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "RESETN enabled on pin PIO0_5."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(RESETN_A::ENABLED)
    }
    #[doc = "RESETN disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(RESETN_A::DISABLED)
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
#[doc = "CLKIN function select.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CLKIN_A {
    #[doc = "0: CLKIN enabled on pin PIO0_1."]
    ENABLED = 0,
    #[doc = "1: CLKIN disabled."]
    DISABLED = 1,
}
impl From<CLKIN_A> for bool {
    #[inline(always)]
    fn from(variant: CLKIN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CLKIN`"]
pub type CLKIN_R = crate::R<bool, CLKIN_A>;
impl CLKIN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CLKIN_A {
        match self.bits {
            false => CLKIN_A::ENABLED,
            true => CLKIN_A::DISABLED,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CLKIN_A::ENABLED
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CLKIN_A::DISABLED
    }
}
#[doc = "Write proxy for field `CLKIN`"]
pub struct CLKIN_W<'a> {
    w: &'a mut W,
}
impl<'a> CLKIN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CLKIN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "CLKIN enabled on pin PIO0_1."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(CLKIN_A::ENABLED)
    }
    #[doc = "CLKIN disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(CLKIN_A::DISABLED)
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
#[doc = "VDDCMP function select.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VDDCMP_A {
    #[doc = "0: VDDCMP enabled on pin PIO0_6."]
    ENABLED = 0,
    #[doc = "1: VDDCMP disabled."]
    DISABLED = 1,
}
impl From<VDDCMP_A> for bool {
    #[inline(always)]
    fn from(variant: VDDCMP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `VDDCMP`"]
pub type VDDCMP_R = crate::R<bool, VDDCMP_A>;
impl VDDCMP_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VDDCMP_A {
        match self.bits {
            false => VDDCMP_A::ENABLED,
            true => VDDCMP_A::DISABLED,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == VDDCMP_A::ENABLED
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == VDDCMP_A::DISABLED
    }
}
#[doc = "Write proxy for field `VDDCMP`"]
pub struct VDDCMP_W<'a> {
    w: &'a mut W,
}
impl<'a> VDDCMP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: VDDCMP_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "VDDCMP enabled on pin PIO0_6."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(VDDCMP_A::ENABLED)
    }
    #[doc = "VDDCMP disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(VDDCMP_A::DISABLED)
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
#[doc = "I2C0_SDA function select.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum I2C0_SDA_A {
    #[doc = "0: I2C0_SDA enabled on pin PIO0_11."]
    ENABLED = 0,
    #[doc = "1: I2C0_SDA disabled."]
    DISABLED = 1,
}
impl From<I2C0_SDA_A> for bool {
    #[inline(always)]
    fn from(variant: I2C0_SDA_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `I2C0_SDA`"]
pub type I2C0_SDA_R = crate::R<bool, I2C0_SDA_A>;
impl I2C0_SDA_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> I2C0_SDA_A {
        match self.bits {
            false => I2C0_SDA_A::ENABLED,
            true => I2C0_SDA_A::DISABLED,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == I2C0_SDA_A::ENABLED
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == I2C0_SDA_A::DISABLED
    }
}
#[doc = "Write proxy for field `I2C0_SDA`"]
pub struct I2C0_SDA_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C0_SDA_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: I2C0_SDA_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "I2C0_SDA enabled on pin PIO0_11."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(I2C0_SDA_A::ENABLED)
    }
    #[doc = "I2C0_SDA disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(I2C0_SDA_A::DISABLED)
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
#[doc = "I2C0_SCL function select.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum I2C0_SCL_A {
    #[doc = "0: I2C0_SCL enabled on pin PIO0_10."]
    ENABLED = 0,
    #[doc = "1: I2C0_SCL disabled."]
    DISABLED = 1,
}
impl From<I2C0_SCL_A> for bool {
    #[inline(always)]
    fn from(variant: I2C0_SCL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `I2C0_SCL`"]
pub type I2C0_SCL_R = crate::R<bool, I2C0_SCL_A>;
impl I2C0_SCL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> I2C0_SCL_A {
        match self.bits {
            false => I2C0_SCL_A::ENABLED,
            true => I2C0_SCL_A::DISABLED,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == I2C0_SCL_A::ENABLED
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == I2C0_SCL_A::DISABLED
    }
}
#[doc = "Write proxy for field `I2C0_SCL`"]
pub struct I2C0_SCL_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C0_SCL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: I2C0_SCL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "I2C0_SCL enabled on pin PIO0_10."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(I2C0_SCL_A::ENABLED)
    }
    #[doc = "I2C0_SCL disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(I2C0_SCL_A::DISABLED)
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
#[doc = "ADC_0 function select.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADC_0_A {
    #[doc = "0: ADC_0 enabled on pin PIO0_7."]
    ENABLED = 0,
    #[doc = "1: ADC_0 disabled."]
    DISABLED = 1,
}
impl From<ADC_0_A> for bool {
    #[inline(always)]
    fn from(variant: ADC_0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ADC_0`"]
pub type ADC_0_R = crate::R<bool, ADC_0_A>;
impl ADC_0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADC_0_A {
        match self.bits {
            false => ADC_0_A::ENABLED,
            true => ADC_0_A::DISABLED,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ADC_0_A::ENABLED
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ADC_0_A::DISABLED
    }
}
#[doc = "Write proxy for field `ADC_0`"]
pub struct ADC_0_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC_0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADC_0_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "ADC_0 enabled on pin PIO0_7."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ADC_0_A::ENABLED)
    }
    #[doc = "ADC_0 disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ADC_0_A::DISABLED)
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
#[doc = "ADC_1 function select.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADC_1_A {
    #[doc = "0: ADC_1 enabled on pin PIO0_6."]
    ENABLED = 0,
    #[doc = "1: ADC_1 disabled."]
    DISABLED = 1,
}
impl From<ADC_1_A> for bool {
    #[inline(always)]
    fn from(variant: ADC_1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ADC_1`"]
pub type ADC_1_R = crate::R<bool, ADC_1_A>;
impl ADC_1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADC_1_A {
        match self.bits {
            false => ADC_1_A::ENABLED,
            true => ADC_1_A::DISABLED,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ADC_1_A::ENABLED
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ADC_1_A::DISABLED
    }
}
#[doc = "Write proxy for field `ADC_1`"]
pub struct ADC_1_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC_1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADC_1_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "ADC_1 enabled on pin PIO0_6."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ADC_1_A::ENABLED)
    }
    #[doc = "ADC_1 disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ADC_1_A::DISABLED)
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
#[doc = "ADC_2 function select.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADC_2_A {
    #[doc = "0: ADC_2 enabled on pin PIO0_14."]
    ENABLED = 0,
    #[doc = "1: ADC_2 disabled."]
    DISABLED = 1,
}
impl From<ADC_2_A> for bool {
    #[inline(always)]
    fn from(variant: ADC_2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ADC_2`"]
pub type ADC_2_R = crate::R<bool, ADC_2_A>;
impl ADC_2_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADC_2_A {
        match self.bits {
            false => ADC_2_A::ENABLED,
            true => ADC_2_A::DISABLED,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ADC_2_A::ENABLED
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ADC_2_A::DISABLED
    }
}
#[doc = "Write proxy for field `ADC_2`"]
pub struct ADC_2_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC_2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADC_2_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "ADC_2 enabled on pin PIO0_14."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ADC_2_A::ENABLED)
    }
    #[doc = "ADC_2 disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ADC_2_A::DISABLED)
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
#[doc = "ADC_3 function select.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADC_3_A {
    #[doc = "0: ADC_3 enabled on pin PIO0_23."]
    ENABLED = 0,
    #[doc = "1: ADC_3 disabled."]
    DISABLED = 1,
}
impl From<ADC_3_A> for bool {
    #[inline(always)]
    fn from(variant: ADC_3_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ADC_3`"]
pub type ADC_3_R = crate::R<bool, ADC_3_A>;
impl ADC_3_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADC_3_A {
        match self.bits {
            false => ADC_3_A::ENABLED,
            true => ADC_3_A::DISABLED,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ADC_3_A::ENABLED
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ADC_3_A::DISABLED
    }
}
#[doc = "Write proxy for field `ADC_3`"]
pub struct ADC_3_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC_3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADC_3_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "ADC_3 enabled on pin PIO0_23."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ADC_3_A::ENABLED)
    }
    #[doc = "ADC_3 disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ADC_3_A::DISABLED)
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
#[doc = "ADC_4 function select.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADC_4_A {
    #[doc = "0: ADC_4 enabled on pin PIO0_22."]
    ENABLED = 0,
    #[doc = "1: ADC_4 disabled."]
    DISABLED = 1,
}
impl From<ADC_4_A> for bool {
    #[inline(always)]
    fn from(variant: ADC_4_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ADC_4`"]
pub type ADC_4_R = crate::R<bool, ADC_4_A>;
impl ADC_4_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADC_4_A {
        match self.bits {
            false => ADC_4_A::ENABLED,
            true => ADC_4_A::DISABLED,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ADC_4_A::ENABLED
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ADC_4_A::DISABLED
    }
}
#[doc = "Write proxy for field `ADC_4`"]
pub struct ADC_4_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC_4_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADC_4_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "ADC_4 enabled on pin PIO0_22."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ADC_4_A::ENABLED)
    }
    #[doc = "ADC_4 disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ADC_4_A::DISABLED)
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
#[doc = "ADC_5 function select.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADC_5_A {
    #[doc = "0: ADC_5 enabled on pin PIO0_21."]
    ENABLED = 0,
    #[doc = "1: ADC_5 disabled."]
    DISABLED = 1,
}
impl From<ADC_5_A> for bool {
    #[inline(always)]
    fn from(variant: ADC_5_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ADC_5`"]
pub type ADC_5_R = crate::R<bool, ADC_5_A>;
impl ADC_5_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADC_5_A {
        match self.bits {
            false => ADC_5_A::ENABLED,
            true => ADC_5_A::DISABLED,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ADC_5_A::ENABLED
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ADC_5_A::DISABLED
    }
}
#[doc = "Write proxy for field `ADC_5`"]
pub struct ADC_5_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC_5_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADC_5_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "ADC_5 enabled on pin PIO0_21."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ADC_5_A::ENABLED)
    }
    #[doc = "ADC_5 disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ADC_5_A::DISABLED)
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
#[doc = "ADC_6 function select.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADC_6_A {
    #[doc = "0: ADC_6 enabled on pin PIO0_20."]
    ENABLED = 0,
    #[doc = "1: ADC_6 disabled."]
    DISABLED = 1,
}
impl From<ADC_6_A> for bool {
    #[inline(always)]
    fn from(variant: ADC_6_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ADC_6`"]
pub type ADC_6_R = crate::R<bool, ADC_6_A>;
impl ADC_6_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADC_6_A {
        match self.bits {
            false => ADC_6_A::ENABLED,
            true => ADC_6_A::DISABLED,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ADC_6_A::ENABLED
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ADC_6_A::DISABLED
    }
}
#[doc = "Write proxy for field `ADC_6`"]
pub struct ADC_6_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC_6_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADC_6_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "ADC_6 enabled on pin PIO0_20."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ADC_6_A::ENABLED)
    }
    #[doc = "ADC_6 disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ADC_6_A::DISABLED)
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
#[doc = "ADC_7 function select.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADC_7_A {
    #[doc = "0: ADC_7 enabled on pin PIO0_19."]
    ENABLED = 0,
    #[doc = "1: ADC_7 disabled."]
    DISABLED = 1,
}
impl From<ADC_7_A> for bool {
    #[inline(always)]
    fn from(variant: ADC_7_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ADC_7`"]
pub type ADC_7_R = crate::R<bool, ADC_7_A>;
impl ADC_7_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADC_7_A {
        match self.bits {
            false => ADC_7_A::ENABLED,
            true => ADC_7_A::DISABLED,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ADC_7_A::ENABLED
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ADC_7_A::DISABLED
    }
}
#[doc = "Write proxy for field `ADC_7`"]
pub struct ADC_7_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC_7_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADC_7_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "ADC_7 enabled on pin PIO0_19."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ADC_7_A::ENABLED)
    }
    #[doc = "ADC_7 disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ADC_7_A::DISABLED)
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
#[doc = "ADC_8 function select.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADC_8_A {
    #[doc = "0: ADC_8 enabled on pin PIO0_18."]
    ENABLED = 0,
    #[doc = "1: ADC_8 disabled."]
    DISABLED = 1,
}
impl From<ADC_8_A> for bool {
    #[inline(always)]
    fn from(variant: ADC_8_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ADC_8`"]
pub type ADC_8_R = crate::R<bool, ADC_8_A>;
impl ADC_8_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADC_8_A {
        match self.bits {
            false => ADC_8_A::ENABLED,
            true => ADC_8_A::DISABLED,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ADC_8_A::ENABLED
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ADC_8_A::DISABLED
    }
}
#[doc = "Write proxy for field `ADC_8`"]
pub struct ADC_8_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC_8_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADC_8_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "ADC_8 enabled on pin PIO0_18."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ADC_8_A::ENABLED)
    }
    #[doc = "ADC_8 disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ADC_8_A::DISABLED)
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
#[doc = "ADC_9 function select.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADC_9_A {
    #[doc = "0: ADC_9 enabled on pin PIO0_17."]
    ENABLED = 0,
    #[doc = "1: ADC_9 disabled."]
    DISABLED = 1,
}
impl From<ADC_9_A> for bool {
    #[inline(always)]
    fn from(variant: ADC_9_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ADC_9`"]
pub type ADC_9_R = crate::R<bool, ADC_9_A>;
impl ADC_9_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADC_9_A {
        match self.bits {
            false => ADC_9_A::ENABLED,
            true => ADC_9_A::DISABLED,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ADC_9_A::ENABLED
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ADC_9_A::DISABLED
    }
}
#[doc = "Write proxy for field `ADC_9`"]
pub struct ADC_9_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC_9_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADC_9_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "ADC_9 enabled on pin PIO0_17."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ADC_9_A::ENABLED)
    }
    #[doc = "ADC_9 disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ADC_9_A::DISABLED)
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
#[doc = "ADC_10 function select.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADC_10_A {
    #[doc = "0: ADC_10 enabled on pin PIO0_13."]
    ENABLED = 0,
    #[doc = "1: ADC_10 disabled."]
    DISABLED = 1,
}
impl From<ADC_10_A> for bool {
    #[inline(always)]
    fn from(variant: ADC_10_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ADC_10`"]
pub type ADC_10_R = crate::R<bool, ADC_10_A>;
impl ADC_10_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADC_10_A {
        match self.bits {
            false => ADC_10_A::ENABLED,
            true => ADC_10_A::DISABLED,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ADC_10_A::ENABLED
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ADC_10_A::DISABLED
    }
}
#[doc = "Write proxy for field `ADC_10`"]
pub struct ADC_10_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC_10_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADC_10_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "ADC_10 enabled on pin PIO0_13."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ADC_10_A::ENABLED)
    }
    #[doc = "ADC_10 disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ADC_10_A::DISABLED)
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
#[doc = "ADC_11 function select.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADC_11_A {
    #[doc = "0: ADC_11 enabled on pin PIO0_4."]
    ENABLED = 0,
    #[doc = "1: ADC_11 disabled."]
    DISABLED = 1,
}
impl From<ADC_11_A> for bool {
    #[inline(always)]
    fn from(variant: ADC_11_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ADC_11`"]
pub type ADC_11_R = crate::R<bool, ADC_11_A>;
impl ADC_11_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADC_11_A {
        match self.bits {
            false => ADC_11_A::ENABLED,
            true => ADC_11_A::DISABLED,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ADC_11_A::ENABLED
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ADC_11_A::DISABLED
    }
}
#[doc = "Write proxy for field `ADC_11`"]
pub struct ADC_11_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC_11_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADC_11_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "ADC_11 enabled on pin PIO0_4."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ADC_11_A::ENABLED)
    }
    #[doc = "ADC_11 disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ADC_11_A::DISABLED)
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
#[doc = "DACOUT0 function select.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DACOUT0_A {
    #[doc = "0: DACOUT0 enabled on pin PIO0_17."]
    ENABLED = 0,
    #[doc = "1: DACOUT0 disabled."]
    DISABLED = 1,
}
impl From<DACOUT0_A> for bool {
    #[inline(always)]
    fn from(variant: DACOUT0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DACOUT0`"]
pub type DACOUT0_R = crate::R<bool, DACOUT0_A>;
impl DACOUT0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DACOUT0_A {
        match self.bits {
            false => DACOUT0_A::ENABLED,
            true => DACOUT0_A::DISABLED,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DACOUT0_A::ENABLED
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DACOUT0_A::DISABLED
    }
}
#[doc = "Write proxy for field `DACOUT0`"]
pub struct DACOUT0_W<'a> {
    w: &'a mut W,
}
impl<'a> DACOUT0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DACOUT0_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "DACOUT0 enabled on pin PIO0_17."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(DACOUT0_A::ENABLED)
    }
    #[doc = "DACOUT0 disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(DACOUT0_A::DISABLED)
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
#[doc = "DACOUT1 function select.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DACOUT1_A {
    #[doc = "0: DACOUT1 enabled on pin PIO0_29."]
    ENABLED = 0,
    #[doc = "1: DACOUT1 disabled."]
    DISABLED = 1,
}
impl From<DACOUT1_A> for bool {
    #[inline(always)]
    fn from(variant: DACOUT1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DACOUT1`"]
pub type DACOUT1_R = crate::R<bool, DACOUT1_A>;
impl DACOUT1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DACOUT1_A {
        match self.bits {
            false => DACOUT1_A::ENABLED,
            true => DACOUT1_A::DISABLED,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DACOUT1_A::ENABLED
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DACOUT1_A::DISABLED
    }
}
#[doc = "Write proxy for field `DACOUT1`"]
pub struct DACOUT1_W<'a> {
    w: &'a mut W,
}
impl<'a> DACOUT1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DACOUT1_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "DACOUT1 enabled on pin PIO0_29."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(DACOUT1_A::ENABLED)
    }
    #[doc = "DACOUT1 disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(DACOUT1_A::DISABLED)
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
#[doc = "CAPT_X0 function select.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CAPT_X0_A {
    #[doc = "0: CAPT_X0 enabled on pin PIO0_31."]
    ENABLED = 0,
    #[doc = "1: CAPT_X0 disabled."]
    DISABLED = 1,
}
impl From<CAPT_X0_A> for bool {
    #[inline(always)]
    fn from(variant: CAPT_X0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CAPT_X0`"]
pub type CAPT_X0_R = crate::R<bool, CAPT_X0_A>;
impl CAPT_X0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CAPT_X0_A {
        match self.bits {
            false => CAPT_X0_A::ENABLED,
            true => CAPT_X0_A::DISABLED,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CAPT_X0_A::ENABLED
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CAPT_X0_A::DISABLED
    }
}
#[doc = "Write proxy for field `CAPT_X0`"]
pub struct CAPT_X0_W<'a> {
    w: &'a mut W,
}
impl<'a> CAPT_X0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CAPT_X0_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "CAPT_X0 enabled on pin PIO0_31."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(CAPT_X0_A::ENABLED)
    }
    #[doc = "CAPT_X0 disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(CAPT_X0_A::DISABLED)
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
#[doc = "CAPT_X1 function select.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CAPT_X1_A {
    #[doc = "0: CAPT_X1 enabled on pin PIO1_0."]
    ENABLED = 0,
    #[doc = "1: CAPT_X1 disabled."]
    DISABLED = 1,
}
impl From<CAPT_X1_A> for bool {
    #[inline(always)]
    fn from(variant: CAPT_X1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CAPT_X1`"]
pub type CAPT_X1_R = crate::R<bool, CAPT_X1_A>;
impl CAPT_X1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CAPT_X1_A {
        match self.bits {
            false => CAPT_X1_A::ENABLED,
            true => CAPT_X1_A::DISABLED,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CAPT_X1_A::ENABLED
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CAPT_X1_A::DISABLED
    }
}
#[doc = "Write proxy for field `CAPT_X1`"]
pub struct CAPT_X1_W<'a> {
    w: &'a mut W,
}
impl<'a> CAPT_X1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CAPT_X1_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "CAPT_X1 enabled on pin PIO1_0."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(CAPT_X1_A::ENABLED)
    }
    #[doc = "CAPT_X1 disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(CAPT_X1_A::DISABLED)
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
#[doc = "CAPT_X2 function select.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CAPT_X2_A {
    #[doc = "0: CAPT_X2 enabled on pin PIO1_1."]
    ENABLED = 0,
    #[doc = "1: CAPT_X2 disabled."]
    DISABLED = 1,
}
impl From<CAPT_X2_A> for bool {
    #[inline(always)]
    fn from(variant: CAPT_X2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CAPT_X2`"]
pub type CAPT_X2_R = crate::R<bool, CAPT_X2_A>;
impl CAPT_X2_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CAPT_X2_A {
        match self.bits {
            false => CAPT_X2_A::ENABLED,
            true => CAPT_X2_A::DISABLED,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CAPT_X2_A::ENABLED
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CAPT_X2_A::DISABLED
    }
}
#[doc = "Write proxy for field `CAPT_X2`"]
pub struct CAPT_X2_W<'a> {
    w: &'a mut W,
}
impl<'a> CAPT_X2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CAPT_X2_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "CAPT_X2 enabled on pin PIO1_1."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(CAPT_X2_A::ENABLED)
    }
    #[doc = "CAPT_X2 disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(CAPT_X2_A::DISABLED)
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
#[doc = "CAPT_X3 function select.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CAPT_X3_A {
    #[doc = "0: CAPT_X3 enabled on pin PIO1_2."]
    ENABLED = 0,
    #[doc = "1: CAPT_X3 disabled."]
    DISABLED = 1,
}
impl From<CAPT_X3_A> for bool {
    #[inline(always)]
    fn from(variant: CAPT_X3_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CAPT_X3`"]
pub type CAPT_X3_R = crate::R<bool, CAPT_X3_A>;
impl CAPT_X3_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CAPT_X3_A {
        match self.bits {
            false => CAPT_X3_A::ENABLED,
            true => CAPT_X3_A::DISABLED,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CAPT_X3_A::ENABLED
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CAPT_X3_A::DISABLED
    }
}
#[doc = "Write proxy for field `CAPT_X3`"]
pub struct CAPT_X3_W<'a> {
    w: &'a mut W,
}
impl<'a> CAPT_X3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CAPT_X3_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "CAPT_X3 enabled on pin PIO1_2."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(CAPT_X3_A::ENABLED)
    }
    #[doc = "CAPT_X3 disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(CAPT_X3_A::DISABLED)
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
    #[doc = "Bit 0 - ACMP_I1 function select."]
    #[inline(always)]
    pub fn acmp_i1(&self) -> ACMP_I1_R {
        ACMP_I1_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - ACMP_I2 function select."]
    #[inline(always)]
    pub fn acmp_i2(&self) -> ACMP_I2_R {
        ACMP_I2_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - ACMP_I3 function select."]
    #[inline(always)]
    pub fn acmp_i3(&self) -> ACMP_I3_R {
        ACMP_I3_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - ACMP_I4 function select."]
    #[inline(always)]
    pub fn acmp_i4(&self) -> ACMP_I4_R {
        ACMP_I4_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - ACMP_I5 function select."]
    #[inline(always)]
    pub fn acmp_i5(&self) -> ACMP_I5_R {
        ACMP_I5_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - SWCLK function select."]
    #[inline(always)]
    pub fn swclk(&self) -> SWCLK_R {
        SWCLK_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - SWDIO function select."]
    #[inline(always)]
    pub fn swdio(&self) -> SWDIO_R {
        SWDIO_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - XTALIN function select."]
    #[inline(always)]
    pub fn xtalin(&self) -> XTALIN_R {
        XTALIN_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - XTALOUT function select."]
    #[inline(always)]
    pub fn xtalout(&self) -> XTALOUT_R {
        XTALOUT_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - RESETN function select."]
    #[inline(always)]
    pub fn resetn(&self) -> RESETN_R {
        RESETN_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - CLKIN function select."]
    #[inline(always)]
    pub fn clkin(&self) -> CLKIN_R {
        CLKIN_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - VDDCMP function select."]
    #[inline(always)]
    pub fn vddcmp(&self) -> VDDCMP_R {
        VDDCMP_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - I2C0_SDA function select."]
    #[inline(always)]
    pub fn i2c0_sda(&self) -> I2C0_SDA_R {
        I2C0_SDA_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - I2C0_SCL function select."]
    #[inline(always)]
    pub fn i2c0_scl(&self) -> I2C0_SCL_R {
        I2C0_SCL_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - ADC_0 function select."]
    #[inline(always)]
    pub fn adc_0(&self) -> ADC_0_R {
        ADC_0_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - ADC_1 function select."]
    #[inline(always)]
    pub fn adc_1(&self) -> ADC_1_R {
        ADC_1_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - ADC_2 function select."]
    #[inline(always)]
    pub fn adc_2(&self) -> ADC_2_R {
        ADC_2_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - ADC_3 function select."]
    #[inline(always)]
    pub fn adc_3(&self) -> ADC_3_R {
        ADC_3_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - ADC_4 function select."]
    #[inline(always)]
    pub fn adc_4(&self) -> ADC_4_R {
        ADC_4_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - ADC_5 function select."]
    #[inline(always)]
    pub fn adc_5(&self) -> ADC_5_R {
        ADC_5_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - ADC_6 function select."]
    #[inline(always)]
    pub fn adc_6(&self) -> ADC_6_R {
        ADC_6_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - ADC_7 function select."]
    #[inline(always)]
    pub fn adc_7(&self) -> ADC_7_R {
        ADC_7_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - ADC_8 function select."]
    #[inline(always)]
    pub fn adc_8(&self) -> ADC_8_R {
        ADC_8_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - ADC_9 function select."]
    #[inline(always)]
    pub fn adc_9(&self) -> ADC_9_R {
        ADC_9_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - ADC_10 function select."]
    #[inline(always)]
    pub fn adc_10(&self) -> ADC_10_R {
        ADC_10_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - ADC_11 function select."]
    #[inline(always)]
    pub fn adc_11(&self) -> ADC_11_R {
        ADC_11_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - DACOUT0 function select."]
    #[inline(always)]
    pub fn dacout0(&self) -> DACOUT0_R {
        DACOUT0_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - DACOUT1 function select."]
    #[inline(always)]
    pub fn dacout1(&self) -> DACOUT1_R {
        DACOUT1_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - CAPT_X0 function select."]
    #[inline(always)]
    pub fn capt_x0(&self) -> CAPT_X0_R {
        CAPT_X0_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - CAPT_X1 function select."]
    #[inline(always)]
    pub fn capt_x1(&self) -> CAPT_X1_R {
        CAPT_X1_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - CAPT_X2 function select."]
    #[inline(always)]
    pub fn capt_x2(&self) -> CAPT_X2_R {
        CAPT_X2_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - CAPT_X3 function select."]
    #[inline(always)]
    pub fn capt_x3(&self) -> CAPT_X3_R {
        CAPT_X3_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - ACMP_I1 function select."]
    #[inline(always)]
    pub fn acmp_i1(&mut self) -> ACMP_I1_W {
        ACMP_I1_W { w: self }
    }
    #[doc = "Bit 1 - ACMP_I2 function select."]
    #[inline(always)]
    pub fn acmp_i2(&mut self) -> ACMP_I2_W {
        ACMP_I2_W { w: self }
    }
    #[doc = "Bit 2 - ACMP_I3 function select."]
    #[inline(always)]
    pub fn acmp_i3(&mut self) -> ACMP_I3_W {
        ACMP_I3_W { w: self }
    }
    #[doc = "Bit 3 - ACMP_I4 function select."]
    #[inline(always)]
    pub fn acmp_i4(&mut self) -> ACMP_I4_W {
        ACMP_I4_W { w: self }
    }
    #[doc = "Bit 4 - ACMP_I5 function select."]
    #[inline(always)]
    pub fn acmp_i5(&mut self) -> ACMP_I5_W {
        ACMP_I5_W { w: self }
    }
    #[doc = "Bit 5 - SWCLK function select."]
    #[inline(always)]
    pub fn swclk(&mut self) -> SWCLK_W {
        SWCLK_W { w: self }
    }
    #[doc = "Bit 6 - SWDIO function select."]
    #[inline(always)]
    pub fn swdio(&mut self) -> SWDIO_W {
        SWDIO_W { w: self }
    }
    #[doc = "Bit 7 - XTALIN function select."]
    #[inline(always)]
    pub fn xtalin(&mut self) -> XTALIN_W {
        XTALIN_W { w: self }
    }
    #[doc = "Bit 8 - XTALOUT function select."]
    #[inline(always)]
    pub fn xtalout(&mut self) -> XTALOUT_W {
        XTALOUT_W { w: self }
    }
    #[doc = "Bit 9 - RESETN function select."]
    #[inline(always)]
    pub fn resetn(&mut self) -> RESETN_W {
        RESETN_W { w: self }
    }
    #[doc = "Bit 10 - CLKIN function select."]
    #[inline(always)]
    pub fn clkin(&mut self) -> CLKIN_W {
        CLKIN_W { w: self }
    }
    #[doc = "Bit 11 - VDDCMP function select."]
    #[inline(always)]
    pub fn vddcmp(&mut self) -> VDDCMP_W {
        VDDCMP_W { w: self }
    }
    #[doc = "Bit 12 - I2C0_SDA function select."]
    #[inline(always)]
    pub fn i2c0_sda(&mut self) -> I2C0_SDA_W {
        I2C0_SDA_W { w: self }
    }
    #[doc = "Bit 13 - I2C0_SCL function select."]
    #[inline(always)]
    pub fn i2c0_scl(&mut self) -> I2C0_SCL_W {
        I2C0_SCL_W { w: self }
    }
    #[doc = "Bit 14 - ADC_0 function select."]
    #[inline(always)]
    pub fn adc_0(&mut self) -> ADC_0_W {
        ADC_0_W { w: self }
    }
    #[doc = "Bit 15 - ADC_1 function select."]
    #[inline(always)]
    pub fn adc_1(&mut self) -> ADC_1_W {
        ADC_1_W { w: self }
    }
    #[doc = "Bit 16 - ADC_2 function select."]
    #[inline(always)]
    pub fn adc_2(&mut self) -> ADC_2_W {
        ADC_2_W { w: self }
    }
    #[doc = "Bit 17 - ADC_3 function select."]
    #[inline(always)]
    pub fn adc_3(&mut self) -> ADC_3_W {
        ADC_3_W { w: self }
    }
    #[doc = "Bit 18 - ADC_4 function select."]
    #[inline(always)]
    pub fn adc_4(&mut self) -> ADC_4_W {
        ADC_4_W { w: self }
    }
    #[doc = "Bit 19 - ADC_5 function select."]
    #[inline(always)]
    pub fn adc_5(&mut self) -> ADC_5_W {
        ADC_5_W { w: self }
    }
    #[doc = "Bit 20 - ADC_6 function select."]
    #[inline(always)]
    pub fn adc_6(&mut self) -> ADC_6_W {
        ADC_6_W { w: self }
    }
    #[doc = "Bit 21 - ADC_7 function select."]
    #[inline(always)]
    pub fn adc_7(&mut self) -> ADC_7_W {
        ADC_7_W { w: self }
    }
    #[doc = "Bit 22 - ADC_8 function select."]
    #[inline(always)]
    pub fn adc_8(&mut self) -> ADC_8_W {
        ADC_8_W { w: self }
    }
    #[doc = "Bit 23 - ADC_9 function select."]
    #[inline(always)]
    pub fn adc_9(&mut self) -> ADC_9_W {
        ADC_9_W { w: self }
    }
    #[doc = "Bit 24 - ADC_10 function select."]
    #[inline(always)]
    pub fn adc_10(&mut self) -> ADC_10_W {
        ADC_10_W { w: self }
    }
    #[doc = "Bit 25 - ADC_11 function select."]
    #[inline(always)]
    pub fn adc_11(&mut self) -> ADC_11_W {
        ADC_11_W { w: self }
    }
    #[doc = "Bit 26 - DACOUT0 function select."]
    #[inline(always)]
    pub fn dacout0(&mut self) -> DACOUT0_W {
        DACOUT0_W { w: self }
    }
    #[doc = "Bit 27 - DACOUT1 function select."]
    #[inline(always)]
    pub fn dacout1(&mut self) -> DACOUT1_W {
        DACOUT1_W { w: self }
    }
    #[doc = "Bit 28 - CAPT_X0 function select."]
    #[inline(always)]
    pub fn capt_x0(&mut self) -> CAPT_X0_W {
        CAPT_X0_W { w: self }
    }
    #[doc = "Bit 29 - CAPT_X1 function select."]
    #[inline(always)]
    pub fn capt_x1(&mut self) -> CAPT_X1_W {
        CAPT_X1_W { w: self }
    }
    #[doc = "Bit 30 - CAPT_X2 function select."]
    #[inline(always)]
    pub fn capt_x2(&mut self) -> CAPT_X2_W {
        CAPT_X2_W { w: self }
    }
    #[doc = "Bit 31 - CAPT_X3 function select."]
    #[inline(always)]
    pub fn capt_x3(&mut self) -> CAPT_X3_W {
        CAPT_X3_W { w: self }
    }
}
