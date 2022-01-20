#[doc = "Register `PINENABLE0` reader"]
pub struct R(crate::R<PINENABLE0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PINENABLE0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PINENABLE0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PINENABLE0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PINENABLE0` writer"]
pub struct W(crate::W<PINENABLE0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PINENABLE0_SPEC>;
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
impl From<crate::W<PINENABLE0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PINENABLE0_SPEC>) -> Self {
        W(writer)
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
#[doc = "Field `ACMP_I1` reader - ACMP_I1 function select."]
pub struct ACMP_I1_R(crate::FieldReader<bool, ACMP_I1_A>);
impl ACMP_I1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ACMP_I1_R(crate::FieldReader::new(bits))
    }
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
        **self == ACMP_I1_A::ENABLED
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == ACMP_I1_A::DISABLED
    }
}
impl core::ops::Deref for ACMP_I1_R {
    type Target = crate::FieldReader<bool, ACMP_I1_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ACMP_I1` writer - ACMP_I1 function select."]
pub struct ACMP_I1_W<'a> {
    w: &'a mut W,
}
impl<'a> ACMP_I1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ACMP_I1_A) -> &'a mut W {
        self.bit(variant.into())
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
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
#[doc = "ACMP_I2 function select.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ACMP_I2_A {
    #[doc = "0: ACMP_I2 enabled on pin PIO0_1."]
    ENABLED = 0,
    #[doc = "1: ACMP_I2 disabled."]
    DISABLED = 1,
}
impl From<ACMP_I2_A> for bool {
    #[inline(always)]
    fn from(variant: ACMP_I2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ACMP_I2` reader - ACMP_I2 function select."]
pub struct ACMP_I2_R(crate::FieldReader<bool, ACMP_I2_A>);
impl ACMP_I2_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ACMP_I2_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ACMP_I2_A {
        match self.bits {
            false => ACMP_I2_A::ENABLED,
            true => ACMP_I2_A::DISABLED,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == ACMP_I2_A::ENABLED
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == ACMP_I2_A::DISABLED
    }
}
impl core::ops::Deref for ACMP_I2_R {
    type Target = crate::FieldReader<bool, ACMP_I2_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ACMP_I2` writer - ACMP_I2 function select."]
pub struct ACMP_I2_W<'a> {
    w: &'a mut W,
}
impl<'a> ACMP_I2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ACMP_I2_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "ACMP_I2 enabled on pin PIO0_1."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ACMP_I2_A::ENABLED)
    }
    #[doc = "ACMP_I2 disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ACMP_I2_A::DISABLED)
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
#[doc = "Field `ACMP_I3` reader - ACMP_I3 function select."]
pub struct ACMP_I3_R(crate::FieldReader<bool, ACMP_I3_A>);
impl ACMP_I3_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ACMP_I3_R(crate::FieldReader::new(bits))
    }
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
        **self == ACMP_I3_A::ENABLED
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == ACMP_I3_A::DISABLED
    }
}
impl core::ops::Deref for ACMP_I3_R {
    type Target = crate::FieldReader<bool, ACMP_I3_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ACMP_I3` writer - ACMP_I3 function select."]
pub struct ACMP_I3_W<'a> {
    w: &'a mut W,
}
impl<'a> ACMP_I3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ACMP_I3_A) -> &'a mut W {
        self.bit(variant.into())
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u32 & 0x01) << 2);
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
#[doc = "Field `ACMP_I4` reader - ACMP_I4 function select."]
pub struct ACMP_I4_R(crate::FieldReader<bool, ACMP_I4_A>);
impl ACMP_I4_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ACMP_I4_R(crate::FieldReader::new(bits))
    }
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
        **self == ACMP_I4_A::ENABLED
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == ACMP_I4_A::DISABLED
    }
}
impl core::ops::Deref for ACMP_I4_R {
    type Target = crate::FieldReader<bool, ACMP_I4_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ACMP_I4` writer - ACMP_I4 function select."]
pub struct ACMP_I4_W<'a> {
    w: &'a mut W,
}
impl<'a> ACMP_I4_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ACMP_I4_A) -> &'a mut W {
        self.bit(variant.into())
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | ((value as u32 & 0x01) << 3);
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
#[doc = "Field `SWCLK` reader - SWCLK function select."]
pub struct SWCLK_R(crate::FieldReader<bool, SWCLK_A>);
impl SWCLK_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SWCLK_R(crate::FieldReader::new(bits))
    }
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
        **self == SWCLK_A::ENABLED
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == SWCLK_A::DISABLED
    }
}
impl core::ops::Deref for SWCLK_R {
    type Target = crate::FieldReader<bool, SWCLK_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SWCLK` writer - SWCLK function select."]
pub struct SWCLK_W<'a> {
    w: &'a mut W,
}
impl<'a> SWCLK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SWCLK_A) -> &'a mut W {
        self.bit(variant.into())
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | ((value as u32 & 0x01) << 4);
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
#[doc = "Field `SWDIO` reader - SWDIO function select."]
pub struct SWDIO_R(crate::FieldReader<bool, SWDIO_A>);
impl SWDIO_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SWDIO_R(crate::FieldReader::new(bits))
    }
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
        **self == SWDIO_A::ENABLED
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == SWDIO_A::DISABLED
    }
}
impl core::ops::Deref for SWDIO_R {
    type Target = crate::FieldReader<bool, SWDIO_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SWDIO` writer - SWDIO function select."]
pub struct SWDIO_W<'a> {
    w: &'a mut W,
}
impl<'a> SWDIO_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SWDIO_A) -> &'a mut W {
        self.bit(variant.into())
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | ((value as u32 & 0x01) << 5);
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
#[doc = "Field `XTALIN` reader - XTALIN function select."]
pub struct XTALIN_R(crate::FieldReader<bool, XTALIN_A>);
impl XTALIN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        XTALIN_R(crate::FieldReader::new(bits))
    }
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
        **self == XTALIN_A::ENABLED
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == XTALIN_A::DISABLED
    }
}
impl core::ops::Deref for XTALIN_R {
    type Target = crate::FieldReader<bool, XTALIN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `XTALIN` writer - XTALIN function select."]
pub struct XTALIN_W<'a> {
    w: &'a mut W,
}
impl<'a> XTALIN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: XTALIN_A) -> &'a mut W {
        self.bit(variant.into())
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | ((value as u32 & 0x01) << 6);
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
#[doc = "Field `XTALOUT` reader - XTALOUT function select."]
pub struct XTALOUT_R(crate::FieldReader<bool, XTALOUT_A>);
impl XTALOUT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        XTALOUT_R(crate::FieldReader::new(bits))
    }
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
        **self == XTALOUT_A::ENABLED
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == XTALOUT_A::DISABLED
    }
}
impl core::ops::Deref for XTALOUT_R {
    type Target = crate::FieldReader<bool, XTALOUT_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `XTALOUT` writer - XTALOUT function select."]
pub struct XTALOUT_W<'a> {
    w: &'a mut W,
}
impl<'a> XTALOUT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: XTALOUT_A) -> &'a mut W {
        self.bit(variant.into())
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | ((value as u32 & 0x01) << 7);
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
#[doc = "Field `RESETN` reader - RESETN function select."]
pub struct RESETN_R(crate::FieldReader<bool, RESETN_A>);
impl RESETN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RESETN_R(crate::FieldReader::new(bits))
    }
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
        **self == RESETN_A::ENABLED
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == RESETN_A::DISABLED
    }
}
impl core::ops::Deref for RESETN_R {
    type Target = crate::FieldReader<bool, RESETN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RESETN` writer - RESETN function select."]
pub struct RESETN_W<'a> {
    w: &'a mut W,
}
impl<'a> RESETN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RESETN_A) -> &'a mut W {
        self.bit(variant.into())
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | ((value as u32 & 0x01) << 8);
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
#[doc = "Field `CLKIN` reader - CLKIN function select."]
pub struct CLKIN_R(crate::FieldReader<bool, CLKIN_A>);
impl CLKIN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CLKIN_R(crate::FieldReader::new(bits))
    }
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
        **self == CLKIN_A::ENABLED
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == CLKIN_A::DISABLED
    }
}
impl core::ops::Deref for CLKIN_R {
    type Target = crate::FieldReader<bool, CLKIN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CLKIN` writer - CLKIN function select."]
pub struct CLKIN_W<'a> {
    w: &'a mut W,
}
impl<'a> CLKIN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CLKIN_A) -> &'a mut W {
        self.bit(variant.into())
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | ((value as u32 & 0x01) << 9);
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
#[doc = "Field `VDDCMP` reader - VDDCMP function select."]
pub struct VDDCMP_R(crate::FieldReader<bool, VDDCMP_A>);
impl VDDCMP_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        VDDCMP_R(crate::FieldReader::new(bits))
    }
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
        **self == VDDCMP_A::ENABLED
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == VDDCMP_A::DISABLED
    }
}
impl core::ops::Deref for VDDCMP_R {
    type Target = crate::FieldReader<bool, VDDCMP_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VDDCMP` writer - VDDCMP function select."]
pub struct VDDCMP_W<'a> {
    w: &'a mut W,
}
impl<'a> VDDCMP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: VDDCMP_A) -> &'a mut W {
        self.bit(variant.into())
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | ((value as u32 & 0x01) << 10);
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
#[doc = "Field `I2C0_SDA` reader - I2C0_SDA function select."]
pub struct I2C0_SDA_R(crate::FieldReader<bool, I2C0_SDA_A>);
impl I2C0_SDA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        I2C0_SDA_R(crate::FieldReader::new(bits))
    }
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
        **self == I2C0_SDA_A::ENABLED
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == I2C0_SDA_A::DISABLED
    }
}
impl core::ops::Deref for I2C0_SDA_R {
    type Target = crate::FieldReader<bool, I2C0_SDA_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `I2C0_SDA` writer - I2C0_SDA function select."]
pub struct I2C0_SDA_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C0_SDA_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: I2C0_SDA_A) -> &'a mut W {
        self.bit(variant.into())
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | ((value as u32 & 0x01) << 11);
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
#[doc = "Field `I2C0_SCL` reader - I2C0_SCL function select."]
pub struct I2C0_SCL_R(crate::FieldReader<bool, I2C0_SCL_A>);
impl I2C0_SCL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        I2C0_SCL_R(crate::FieldReader::new(bits))
    }
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
        **self == I2C0_SCL_A::ENABLED
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == I2C0_SCL_A::DISABLED
    }
}
impl core::ops::Deref for I2C0_SCL_R {
    type Target = crate::FieldReader<bool, I2C0_SCL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `I2C0_SCL` writer - I2C0_SCL function select."]
pub struct I2C0_SCL_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C0_SCL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: I2C0_SCL_A) -> &'a mut W {
        self.bit(variant.into())
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | ((value as u32 & 0x01) << 12);
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
#[doc = "Field `ADC_0` reader - ADC_0 function select."]
pub struct ADC_0_R(crate::FieldReader<bool, ADC_0_A>);
impl ADC_0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ADC_0_R(crate::FieldReader::new(bits))
    }
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
        **self == ADC_0_A::ENABLED
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == ADC_0_A::DISABLED
    }
}
impl core::ops::Deref for ADC_0_R {
    type Target = crate::FieldReader<bool, ADC_0_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADC_0` writer - ADC_0 function select."]
pub struct ADC_0_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC_0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADC_0_A) -> &'a mut W {
        self.bit(variant.into())
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | ((value as u32 & 0x01) << 13);
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
#[doc = "Field `ADC_1` reader - ADC_1 function select."]
pub struct ADC_1_R(crate::FieldReader<bool, ADC_1_A>);
impl ADC_1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ADC_1_R(crate::FieldReader::new(bits))
    }
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
        **self == ADC_1_A::ENABLED
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == ADC_1_A::DISABLED
    }
}
impl core::ops::Deref for ADC_1_R {
    type Target = crate::FieldReader<bool, ADC_1_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADC_1` writer - ADC_1 function select."]
pub struct ADC_1_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC_1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADC_1_A) -> &'a mut W {
        self.bit(variant.into())
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | ((value as u32 & 0x01) << 14);
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
#[doc = "Field `ADC_2` reader - ADC_2 function select."]
pub struct ADC_2_R(crate::FieldReader<bool, ADC_2_A>);
impl ADC_2_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ADC_2_R(crate::FieldReader::new(bits))
    }
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
        **self == ADC_2_A::ENABLED
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == ADC_2_A::DISABLED
    }
}
impl core::ops::Deref for ADC_2_R {
    type Target = crate::FieldReader<bool, ADC_2_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADC_2` writer - ADC_2 function select."]
pub struct ADC_2_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC_2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADC_2_A) -> &'a mut W {
        self.bit(variant.into())
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | ((value as u32 & 0x01) << 15);
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
#[doc = "Field `ADC_3` reader - ADC_3 function select."]
pub struct ADC_3_R(crate::FieldReader<bool, ADC_3_A>);
impl ADC_3_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ADC_3_R(crate::FieldReader::new(bits))
    }
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
        **self == ADC_3_A::ENABLED
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == ADC_3_A::DISABLED
    }
}
impl core::ops::Deref for ADC_3_R {
    type Target = crate::FieldReader<bool, ADC_3_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADC_3` writer - ADC_3 function select."]
pub struct ADC_3_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC_3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADC_3_A) -> &'a mut W {
        self.bit(variant.into())
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | ((value as u32 & 0x01) << 16);
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
#[doc = "Field `ADC_4` reader - ADC_4 function select."]
pub struct ADC_4_R(crate::FieldReader<bool, ADC_4_A>);
impl ADC_4_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ADC_4_R(crate::FieldReader::new(bits))
    }
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
        **self == ADC_4_A::ENABLED
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == ADC_4_A::DISABLED
    }
}
impl core::ops::Deref for ADC_4_R {
    type Target = crate::FieldReader<bool, ADC_4_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADC_4` writer - ADC_4 function select."]
pub struct ADC_4_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC_4_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADC_4_A) -> &'a mut W {
        self.bit(variant.into())
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | ((value as u32 & 0x01) << 17);
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
#[doc = "Field `ADC_5` reader - ADC_5 function select."]
pub struct ADC_5_R(crate::FieldReader<bool, ADC_5_A>);
impl ADC_5_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ADC_5_R(crate::FieldReader::new(bits))
    }
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
        **self == ADC_5_A::ENABLED
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == ADC_5_A::DISABLED
    }
}
impl core::ops::Deref for ADC_5_R {
    type Target = crate::FieldReader<bool, ADC_5_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADC_5` writer - ADC_5 function select."]
pub struct ADC_5_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC_5_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADC_5_A) -> &'a mut W {
        self.bit(variant.into())
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | ((value as u32 & 0x01) << 18);
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
#[doc = "Field `ADC_6` reader - ADC_6 function select."]
pub struct ADC_6_R(crate::FieldReader<bool, ADC_6_A>);
impl ADC_6_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ADC_6_R(crate::FieldReader::new(bits))
    }
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
        **self == ADC_6_A::ENABLED
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == ADC_6_A::DISABLED
    }
}
impl core::ops::Deref for ADC_6_R {
    type Target = crate::FieldReader<bool, ADC_6_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADC_6` writer - ADC_6 function select."]
pub struct ADC_6_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC_6_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADC_6_A) -> &'a mut W {
        self.bit(variant.into())
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | ((value as u32 & 0x01) << 19);
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
#[doc = "Field `ADC_7` reader - ADC_7 function select."]
pub struct ADC_7_R(crate::FieldReader<bool, ADC_7_A>);
impl ADC_7_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ADC_7_R(crate::FieldReader::new(bits))
    }
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
        **self == ADC_7_A::ENABLED
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == ADC_7_A::DISABLED
    }
}
impl core::ops::Deref for ADC_7_R {
    type Target = crate::FieldReader<bool, ADC_7_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADC_7` writer - ADC_7 function select."]
pub struct ADC_7_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC_7_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADC_7_A) -> &'a mut W {
        self.bit(variant.into())
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | ((value as u32 & 0x01) << 20);
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
#[doc = "Field `ADC_8` reader - ADC_8 function select."]
pub struct ADC_8_R(crate::FieldReader<bool, ADC_8_A>);
impl ADC_8_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ADC_8_R(crate::FieldReader::new(bits))
    }
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
        **self == ADC_8_A::ENABLED
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == ADC_8_A::DISABLED
    }
}
impl core::ops::Deref for ADC_8_R {
    type Target = crate::FieldReader<bool, ADC_8_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADC_8` writer - ADC_8 function select."]
pub struct ADC_8_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC_8_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADC_8_A) -> &'a mut W {
        self.bit(variant.into())
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
        self.w.bits = (self.w.bits & !(0x01 << 21)) | ((value as u32 & 0x01) << 21);
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
#[doc = "Field `ADC_9` reader - ADC_9 function select."]
pub struct ADC_9_R(crate::FieldReader<bool, ADC_9_A>);
impl ADC_9_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ADC_9_R(crate::FieldReader::new(bits))
    }
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
        **self == ADC_9_A::ENABLED
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == ADC_9_A::DISABLED
    }
}
impl core::ops::Deref for ADC_9_R {
    type Target = crate::FieldReader<bool, ADC_9_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADC_9` writer - ADC_9 function select."]
pub struct ADC_9_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC_9_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADC_9_A) -> &'a mut W {
        self.bit(variant.into())
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
        self.w.bits = (self.w.bits & !(0x01 << 22)) | ((value as u32 & 0x01) << 22);
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
#[doc = "Field `ADC_10` reader - ADC_10 function select."]
pub struct ADC_10_R(crate::FieldReader<bool, ADC_10_A>);
impl ADC_10_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ADC_10_R(crate::FieldReader::new(bits))
    }
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
        **self == ADC_10_A::ENABLED
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == ADC_10_A::DISABLED
    }
}
impl core::ops::Deref for ADC_10_R {
    type Target = crate::FieldReader<bool, ADC_10_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADC_10` writer - ADC_10 function select."]
pub struct ADC_10_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC_10_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADC_10_A) -> &'a mut W {
        self.bit(variant.into())
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
        self.w.bits = (self.w.bits & !(0x01 << 23)) | ((value as u32 & 0x01) << 23);
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
#[doc = "Field `ADC_11` reader - ADC_11 function select."]
pub struct ADC_11_R(crate::FieldReader<bool, ADC_11_A>);
impl ADC_11_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ADC_11_R(crate::FieldReader::new(bits))
    }
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
        **self == ADC_11_A::ENABLED
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == ADC_11_A::DISABLED
    }
}
impl core::ops::Deref for ADC_11_R {
    type Target = crate::FieldReader<bool, ADC_11_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADC_11` writer - ADC_11 function select."]
pub struct ADC_11_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC_11_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADC_11_A) -> &'a mut W {
        self.bit(variant.into())
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | ((value as u32 & 0x01) << 24);
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
    #[doc = "Bit 4 - SWCLK function select."]
    #[inline(always)]
    pub fn swclk(&self) -> SWCLK_R {
        SWCLK_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - SWDIO function select."]
    #[inline(always)]
    pub fn swdio(&self) -> SWDIO_R {
        SWDIO_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - XTALIN function select."]
    #[inline(always)]
    pub fn xtalin(&self) -> XTALIN_R {
        XTALIN_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - XTALOUT function select."]
    #[inline(always)]
    pub fn xtalout(&self) -> XTALOUT_R {
        XTALOUT_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - RESETN function select."]
    #[inline(always)]
    pub fn resetn(&self) -> RESETN_R {
        RESETN_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - CLKIN function select."]
    #[inline(always)]
    pub fn clkin(&self) -> CLKIN_R {
        CLKIN_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - VDDCMP function select."]
    #[inline(always)]
    pub fn vddcmp(&self) -> VDDCMP_R {
        VDDCMP_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - I2C0_SDA function select."]
    #[inline(always)]
    pub fn i2c0_sda(&self) -> I2C0_SDA_R {
        I2C0_SDA_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - I2C0_SCL function select."]
    #[inline(always)]
    pub fn i2c0_scl(&self) -> I2C0_SCL_R {
        I2C0_SCL_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - ADC_0 function select."]
    #[inline(always)]
    pub fn adc_0(&self) -> ADC_0_R {
        ADC_0_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - ADC_1 function select."]
    #[inline(always)]
    pub fn adc_1(&self) -> ADC_1_R {
        ADC_1_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - ADC_2 function select."]
    #[inline(always)]
    pub fn adc_2(&self) -> ADC_2_R {
        ADC_2_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - ADC_3 function select."]
    #[inline(always)]
    pub fn adc_3(&self) -> ADC_3_R {
        ADC_3_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - ADC_4 function select."]
    #[inline(always)]
    pub fn adc_4(&self) -> ADC_4_R {
        ADC_4_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - ADC_5 function select."]
    #[inline(always)]
    pub fn adc_5(&self) -> ADC_5_R {
        ADC_5_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - ADC_6 function select."]
    #[inline(always)]
    pub fn adc_6(&self) -> ADC_6_R {
        ADC_6_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - ADC_7 function select."]
    #[inline(always)]
    pub fn adc_7(&self) -> ADC_7_R {
        ADC_7_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - ADC_8 function select."]
    #[inline(always)]
    pub fn adc_8(&self) -> ADC_8_R {
        ADC_8_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - ADC_9 function select."]
    #[inline(always)]
    pub fn adc_9(&self) -> ADC_9_R {
        ADC_9_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - ADC_10 function select."]
    #[inline(always)]
    pub fn adc_10(&self) -> ADC_10_R {
        ADC_10_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - ADC_11 function select."]
    #[inline(always)]
    pub fn adc_11(&self) -> ADC_11_R {
        ADC_11_R::new(((self.bits >> 24) & 0x01) != 0)
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
    #[doc = "Bit 4 - SWCLK function select."]
    #[inline(always)]
    pub fn swclk(&mut self) -> SWCLK_W {
        SWCLK_W { w: self }
    }
    #[doc = "Bit 5 - SWDIO function select."]
    #[inline(always)]
    pub fn swdio(&mut self) -> SWDIO_W {
        SWDIO_W { w: self }
    }
    #[doc = "Bit 6 - XTALIN function select."]
    #[inline(always)]
    pub fn xtalin(&mut self) -> XTALIN_W {
        XTALIN_W { w: self }
    }
    #[doc = "Bit 7 - XTALOUT function select."]
    #[inline(always)]
    pub fn xtalout(&mut self) -> XTALOUT_W {
        XTALOUT_W { w: self }
    }
    #[doc = "Bit 8 - RESETN function select."]
    #[inline(always)]
    pub fn resetn(&mut self) -> RESETN_W {
        RESETN_W { w: self }
    }
    #[doc = "Bit 9 - CLKIN function select."]
    #[inline(always)]
    pub fn clkin(&mut self) -> CLKIN_W {
        CLKIN_W { w: self }
    }
    #[doc = "Bit 10 - VDDCMP function select."]
    #[inline(always)]
    pub fn vddcmp(&mut self) -> VDDCMP_W {
        VDDCMP_W { w: self }
    }
    #[doc = "Bit 11 - I2C0_SDA function select."]
    #[inline(always)]
    pub fn i2c0_sda(&mut self) -> I2C0_SDA_W {
        I2C0_SDA_W { w: self }
    }
    #[doc = "Bit 12 - I2C0_SCL function select."]
    #[inline(always)]
    pub fn i2c0_scl(&mut self) -> I2C0_SCL_W {
        I2C0_SCL_W { w: self }
    }
    #[doc = "Bit 13 - ADC_0 function select."]
    #[inline(always)]
    pub fn adc_0(&mut self) -> ADC_0_W {
        ADC_0_W { w: self }
    }
    #[doc = "Bit 14 - ADC_1 function select."]
    #[inline(always)]
    pub fn adc_1(&mut self) -> ADC_1_W {
        ADC_1_W { w: self }
    }
    #[doc = "Bit 15 - ADC_2 function select."]
    #[inline(always)]
    pub fn adc_2(&mut self) -> ADC_2_W {
        ADC_2_W { w: self }
    }
    #[doc = "Bit 16 - ADC_3 function select."]
    #[inline(always)]
    pub fn adc_3(&mut self) -> ADC_3_W {
        ADC_3_W { w: self }
    }
    #[doc = "Bit 17 - ADC_4 function select."]
    #[inline(always)]
    pub fn adc_4(&mut self) -> ADC_4_W {
        ADC_4_W { w: self }
    }
    #[doc = "Bit 18 - ADC_5 function select."]
    #[inline(always)]
    pub fn adc_5(&mut self) -> ADC_5_W {
        ADC_5_W { w: self }
    }
    #[doc = "Bit 19 - ADC_6 function select."]
    #[inline(always)]
    pub fn adc_6(&mut self) -> ADC_6_W {
        ADC_6_W { w: self }
    }
    #[doc = "Bit 20 - ADC_7 function select."]
    #[inline(always)]
    pub fn adc_7(&mut self) -> ADC_7_W {
        ADC_7_W { w: self }
    }
    #[doc = "Bit 21 - ADC_8 function select."]
    #[inline(always)]
    pub fn adc_8(&mut self) -> ADC_8_W {
        ADC_8_W { w: self }
    }
    #[doc = "Bit 22 - ADC_9 function select."]
    #[inline(always)]
    pub fn adc_9(&mut self) -> ADC_9_W {
        ADC_9_W { w: self }
    }
    #[doc = "Bit 23 - ADC_10 function select."]
    #[inline(always)]
    pub fn adc_10(&mut self) -> ADC_10_W {
        ADC_10_W { w: self }
    }
    #[doc = "Bit 24 - ADC_11 function select."]
    #[inline(always)]
    pub fn adc_11(&mut self) -> ADC_11_W {
        ADC_11_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Pin enable register 0. Enables fixed-pin functions ACMP_I0, ACMP_I1, SWCLK, SWDIO, XTALIN, XTALOUT, RESET, CLKIN, VDDCMP and so on.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pinenable0](index.html) module"]
pub struct PINENABLE0_SPEC;
impl crate::RegisterSpec for PINENABLE0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pinenable0::R](R) reader structure"]
impl crate::Readable for PINENABLE0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pinenable0::W](W) writer structure"]
impl crate::Writable for PINENABLE0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PINENABLE0 to value 0xffff_fecf"]
impl crate::Resettable for PINENABLE0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xffff_fecf
    }
}
