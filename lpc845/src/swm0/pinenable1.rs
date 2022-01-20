#[doc = "Register `PINENABLE1` reader"]
pub struct R(crate::R<PINENABLE1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PINENABLE1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PINENABLE1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PINENABLE1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PINENABLE1` writer"]
pub struct W(crate::W<PINENABLE1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PINENABLE1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<PINENABLE1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PINENABLE1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "CAPT_X4 function select.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CAPT_X4_A {
    #[doc = "0: CAPT_X4 enabled on pin PIO1_3."]
    ENABLED = 0,
    #[doc = "1: CAPT_X4 disabled."]
    DISABLED = 1,
}
impl From<CAPT_X4_A> for bool {
    #[inline(always)]
    fn from(variant: CAPT_X4_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CAPT_X4` reader - CAPT_X4 function select."]
pub struct CAPT_X4_R(crate::FieldReader<bool, CAPT_X4_A>);
impl CAPT_X4_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CAPT_X4_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CAPT_X4_A {
        match self.bits {
            false => CAPT_X4_A::ENABLED,
            true => CAPT_X4_A::DISABLED,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == CAPT_X4_A::ENABLED
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == CAPT_X4_A::DISABLED
    }
}
impl core::ops::Deref for CAPT_X4_R {
    type Target = crate::FieldReader<bool, CAPT_X4_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CAPT_X4` writer - CAPT_X4 function select."]
pub struct CAPT_X4_W<'a> {
    w: &'a mut W,
}
impl<'a> CAPT_X4_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CAPT_X4_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "CAPT_X4 enabled on pin PIO1_3."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(CAPT_X4_A::ENABLED)
    }
    #[doc = "CAPT_X4 disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(CAPT_X4_A::DISABLED)
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
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
#[doc = "CAPT_X5 function select.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CAPT_X5_A {
    #[doc = "0: CAPT_X5 enabled on pin PIO1_4."]
    ENABLED = 0,
    #[doc = "1: CAPT_X5 disabled."]
    DISABLED = 1,
}
impl From<CAPT_X5_A> for bool {
    #[inline(always)]
    fn from(variant: CAPT_X5_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CAPT_X5` reader - CAPT_X5 function select."]
pub struct CAPT_X5_R(crate::FieldReader<bool, CAPT_X5_A>);
impl CAPT_X5_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CAPT_X5_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CAPT_X5_A {
        match self.bits {
            false => CAPT_X5_A::ENABLED,
            true => CAPT_X5_A::DISABLED,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == CAPT_X5_A::ENABLED
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == CAPT_X5_A::DISABLED
    }
}
impl core::ops::Deref for CAPT_X5_R {
    type Target = crate::FieldReader<bool, CAPT_X5_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CAPT_X5` writer - CAPT_X5 function select."]
pub struct CAPT_X5_W<'a> {
    w: &'a mut W,
}
impl<'a> CAPT_X5_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CAPT_X5_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "CAPT_X5 enabled on pin PIO1_4."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(CAPT_X5_A::ENABLED)
    }
    #[doc = "CAPT_X5 disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(CAPT_X5_A::DISABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
        self.w
    }
}
#[doc = "CAPT_X6 function select.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CAPT_X6_A {
    #[doc = "0: CAPT_X6 enabled on pin PIO1_5."]
    ENABLED = 0,
    #[doc = "1: CAPT_X6 disabled."]
    DISABLED = 1,
}
impl From<CAPT_X6_A> for bool {
    #[inline(always)]
    fn from(variant: CAPT_X6_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CAPT_X6` reader - CAPT_X6 function select."]
pub struct CAPT_X6_R(crate::FieldReader<bool, CAPT_X6_A>);
impl CAPT_X6_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CAPT_X6_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CAPT_X6_A {
        match self.bits {
            false => CAPT_X6_A::ENABLED,
            true => CAPT_X6_A::DISABLED,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == CAPT_X6_A::ENABLED
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == CAPT_X6_A::DISABLED
    }
}
impl core::ops::Deref for CAPT_X6_R {
    type Target = crate::FieldReader<bool, CAPT_X6_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CAPT_X6` writer - CAPT_X6 function select."]
pub struct CAPT_X6_W<'a> {
    w: &'a mut W,
}
impl<'a> CAPT_X6_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CAPT_X6_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "CAPT_X6 enabled on pin PIO1_5."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(CAPT_X6_A::ENABLED)
    }
    #[doc = "CAPT_X6 disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(CAPT_X6_A::DISABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u32 & 0x01) << 2);
        self.w
    }
}
#[doc = "CAPT_X7 function select.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CAPT_X7_A {
    #[doc = "0: CAPT_X7 enabled on pin PIO1_6."]
    ENABLED = 0,
    #[doc = "1: CAPT_X7 disabled."]
    DISABLED = 1,
}
impl From<CAPT_X7_A> for bool {
    #[inline(always)]
    fn from(variant: CAPT_X7_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CAPT_X7` reader - CAPT_X7 function select."]
pub struct CAPT_X7_R(crate::FieldReader<bool, CAPT_X7_A>);
impl CAPT_X7_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CAPT_X7_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CAPT_X7_A {
        match self.bits {
            false => CAPT_X7_A::ENABLED,
            true => CAPT_X7_A::DISABLED,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == CAPT_X7_A::ENABLED
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == CAPT_X7_A::DISABLED
    }
}
impl core::ops::Deref for CAPT_X7_R {
    type Target = crate::FieldReader<bool, CAPT_X7_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CAPT_X7` writer - CAPT_X7 function select."]
pub struct CAPT_X7_W<'a> {
    w: &'a mut W,
}
impl<'a> CAPT_X7_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CAPT_X7_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "CAPT_X7 enabled on pin PIO1_6."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(CAPT_X7_A::ENABLED)
    }
    #[doc = "CAPT_X7 disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(CAPT_X7_A::DISABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | ((value as u32 & 0x01) << 3);
        self.w
    }
}
#[doc = "CAPT_X8 function select.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CAPT_X8_A {
    #[doc = "0: CAPT_X8 enabled on pin PIO1_7."]
    ENABLED = 0,
    #[doc = "1: CAPT_X8 disabled."]
    DISABLED = 1,
}
impl From<CAPT_X8_A> for bool {
    #[inline(always)]
    fn from(variant: CAPT_X8_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CAPT_X8` reader - CAPT_X8 function select."]
pub struct CAPT_X8_R(crate::FieldReader<bool, CAPT_X8_A>);
impl CAPT_X8_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CAPT_X8_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CAPT_X8_A {
        match self.bits {
            false => CAPT_X8_A::ENABLED,
            true => CAPT_X8_A::DISABLED,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == CAPT_X8_A::ENABLED
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == CAPT_X8_A::DISABLED
    }
}
impl core::ops::Deref for CAPT_X8_R {
    type Target = crate::FieldReader<bool, CAPT_X8_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CAPT_X8` writer - CAPT_X8 function select."]
pub struct CAPT_X8_W<'a> {
    w: &'a mut W,
}
impl<'a> CAPT_X8_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CAPT_X8_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "CAPT_X8 enabled on pin PIO1_7."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(CAPT_X8_A::ENABLED)
    }
    #[doc = "CAPT_X8 disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(CAPT_X8_A::DISABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | ((value as u32 & 0x01) << 4);
        self.w
    }
}
#[doc = "CAPT_YL function select.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CAPT_YL_A {
    #[doc = "0: CAPT_YL enabled on pin PIO1_8."]
    ENABLED = 0,
    #[doc = "1: CAPT_YL disabled."]
    DISABLED = 1,
}
impl From<CAPT_YL_A> for bool {
    #[inline(always)]
    fn from(variant: CAPT_YL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CAPT_YL` reader - CAPT_YL function select."]
pub struct CAPT_YL_R(crate::FieldReader<bool, CAPT_YL_A>);
impl CAPT_YL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CAPT_YL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CAPT_YL_A {
        match self.bits {
            false => CAPT_YL_A::ENABLED,
            true => CAPT_YL_A::DISABLED,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == CAPT_YL_A::ENABLED
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == CAPT_YL_A::DISABLED
    }
}
impl core::ops::Deref for CAPT_YL_R {
    type Target = crate::FieldReader<bool, CAPT_YL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CAPT_YL` writer - CAPT_YL function select."]
pub struct CAPT_YL_W<'a> {
    w: &'a mut W,
}
impl<'a> CAPT_YL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CAPT_YL_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "CAPT_YL enabled on pin PIO1_8."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(CAPT_YL_A::ENABLED)
    }
    #[doc = "CAPT_YL disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(CAPT_YL_A::DISABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | ((value as u32 & 0x01) << 5);
        self.w
    }
}
#[doc = "CAPT_YH function select.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CAPT_YH_A {
    #[doc = "0: CAPT_YH enabled on pin PIO1_9."]
    ENABLED = 0,
    #[doc = "1: CAPT_YH disabled."]
    DISABLED = 1,
}
impl From<CAPT_YH_A> for bool {
    #[inline(always)]
    fn from(variant: CAPT_YH_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CAPT_YH` reader - CAPT_YH function select."]
pub struct CAPT_YH_R(crate::FieldReader<bool, CAPT_YH_A>);
impl CAPT_YH_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CAPT_YH_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CAPT_YH_A {
        match self.bits {
            false => CAPT_YH_A::ENABLED,
            true => CAPT_YH_A::DISABLED,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == CAPT_YH_A::ENABLED
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == CAPT_YH_A::DISABLED
    }
}
impl core::ops::Deref for CAPT_YH_R {
    type Target = crate::FieldReader<bool, CAPT_YH_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CAPT_YH` writer - CAPT_YH function select."]
pub struct CAPT_YH_W<'a> {
    w: &'a mut W,
}
impl<'a> CAPT_YH_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CAPT_YH_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "CAPT_YH enabled on pin PIO1_9."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(CAPT_YH_A::ENABLED)
    }
    #[doc = "CAPT_YH disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(CAPT_YH_A::DISABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | ((value as u32 & 0x01) << 6);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - CAPT_X4 function select."]
    #[inline(always)]
    pub fn capt_x4(&self) -> CAPT_X4_R {
        CAPT_X4_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - CAPT_X5 function select."]
    #[inline(always)]
    pub fn capt_x5(&self) -> CAPT_X5_R {
        CAPT_X5_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - CAPT_X6 function select."]
    #[inline(always)]
    pub fn capt_x6(&self) -> CAPT_X6_R {
        CAPT_X6_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - CAPT_X7 function select."]
    #[inline(always)]
    pub fn capt_x7(&self) -> CAPT_X7_R {
        CAPT_X7_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - CAPT_X8 function select."]
    #[inline(always)]
    pub fn capt_x8(&self) -> CAPT_X8_R {
        CAPT_X8_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - CAPT_YL function select."]
    #[inline(always)]
    pub fn capt_yl(&self) -> CAPT_YL_R {
        CAPT_YL_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - CAPT_YH function select."]
    #[inline(always)]
    pub fn capt_yh(&self) -> CAPT_YH_R {
        CAPT_YH_R::new(((self.bits >> 6) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - CAPT_X4 function select."]
    #[inline(always)]
    pub fn capt_x4(&mut self) -> CAPT_X4_W {
        CAPT_X4_W { w: self }
    }
    #[doc = "Bit 1 - CAPT_X5 function select."]
    #[inline(always)]
    pub fn capt_x5(&mut self) -> CAPT_X5_W {
        CAPT_X5_W { w: self }
    }
    #[doc = "Bit 2 - CAPT_X6 function select."]
    #[inline(always)]
    pub fn capt_x6(&mut self) -> CAPT_X6_W {
        CAPT_X6_W { w: self }
    }
    #[doc = "Bit 3 - CAPT_X7 function select."]
    #[inline(always)]
    pub fn capt_x7(&mut self) -> CAPT_X7_W {
        CAPT_X7_W { w: self }
    }
    #[doc = "Bit 4 - CAPT_X8 function select."]
    #[inline(always)]
    pub fn capt_x8(&mut self) -> CAPT_X8_W {
        CAPT_X8_W { w: self }
    }
    #[doc = "Bit 5 - CAPT_YL function select."]
    #[inline(always)]
    pub fn capt_yl(&mut self) -> CAPT_YL_W {
        CAPT_YL_W { w: self }
    }
    #[doc = "Bit 6 - CAPT_YH function select."]
    #[inline(always)]
    pub fn capt_yh(&mut self) -> CAPT_YH_W {
        CAPT_YH_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Pin enable register 1. Enables fixed-pin functions CAPT_X4, CAPT_X5, CAPT_X6, CAPT_X7, CAPT_X8, CAPT_X4, CAPT_YL and CAPT_YH.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pinenable1](index.html) module"]
pub struct PINENABLE1_SPEC;
impl crate::RegisterSpec for PINENABLE1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pinenable1::R](R) reader structure"]
impl crate::Readable for PINENABLE1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pinenable1::W](W) writer structure"]
impl crate::Writable for PINENABLE1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PINENABLE1 to value 0x1f"]
impl crate::Resettable for PINENABLE1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x1f
    }
}
