#[doc = "Register `SHCSR` reader"]
pub struct R(crate::R<SHCSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SHCSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SHCSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SHCSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SHCSR` writer"]
pub struct W(crate::W<SHCSR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SHCSR_SPEC>;
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
impl From<crate::W<SHCSR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SHCSR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MEMFAULTACT_A {
    #[doc = "0: exception is not active"]
    MEMFAULTACT_0 = 0,
    #[doc = "1: exception is active"]
    MEMFAULTACT_1 = 1,
}
impl From<MEMFAULTACT_A> for bool {
    #[inline(always)]
    fn from(variant: MEMFAULTACT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MEMFAULTACT` reader - no description available"]
pub struct MEMFAULTACT_R(crate::FieldReader<bool, MEMFAULTACT_A>);
impl MEMFAULTACT_R {
    pub(crate) fn new(bits: bool) -> Self {
        MEMFAULTACT_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MEMFAULTACT_A {
        match self.bits {
            false => MEMFAULTACT_A::MEMFAULTACT_0,
            true => MEMFAULTACT_A::MEMFAULTACT_1,
        }
    }
    #[doc = "Checks if the value of the field is `MEMFAULTACT_0`"]
    #[inline(always)]
    pub fn is_memfaultact_0(&self) -> bool {
        **self == MEMFAULTACT_A::MEMFAULTACT_0
    }
    #[doc = "Checks if the value of the field is `MEMFAULTACT_1`"]
    #[inline(always)]
    pub fn is_memfaultact_1(&self) -> bool {
        **self == MEMFAULTACT_A::MEMFAULTACT_1
    }
}
impl core::ops::Deref for MEMFAULTACT_R {
    type Target = crate::FieldReader<bool, MEMFAULTACT_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MEMFAULTACT` writer - no description available"]
pub struct MEMFAULTACT_W<'a> {
    w: &'a mut W,
}
impl<'a> MEMFAULTACT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MEMFAULTACT_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "exception is not active"]
    #[inline(always)]
    pub fn memfaultact_0(self) -> &'a mut W {
        self.variant(MEMFAULTACT_A::MEMFAULTACT_0)
    }
    #[doc = "exception is active"]
    #[inline(always)]
    pub fn memfaultact_1(self) -> &'a mut W {
        self.variant(MEMFAULTACT_A::MEMFAULTACT_1)
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
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BUSFAULTACT_A {
    #[doc = "0: exception is not active"]
    BUSFAULTACT_0 = 0,
    #[doc = "1: exception is active"]
    BUSFAULTACT_1 = 1,
}
impl From<BUSFAULTACT_A> for bool {
    #[inline(always)]
    fn from(variant: BUSFAULTACT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BUSFAULTACT` reader - no description available"]
pub struct BUSFAULTACT_R(crate::FieldReader<bool, BUSFAULTACT_A>);
impl BUSFAULTACT_R {
    pub(crate) fn new(bits: bool) -> Self {
        BUSFAULTACT_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BUSFAULTACT_A {
        match self.bits {
            false => BUSFAULTACT_A::BUSFAULTACT_0,
            true => BUSFAULTACT_A::BUSFAULTACT_1,
        }
    }
    #[doc = "Checks if the value of the field is `BUSFAULTACT_0`"]
    #[inline(always)]
    pub fn is_busfaultact_0(&self) -> bool {
        **self == BUSFAULTACT_A::BUSFAULTACT_0
    }
    #[doc = "Checks if the value of the field is `BUSFAULTACT_1`"]
    #[inline(always)]
    pub fn is_busfaultact_1(&self) -> bool {
        **self == BUSFAULTACT_A::BUSFAULTACT_1
    }
}
impl core::ops::Deref for BUSFAULTACT_R {
    type Target = crate::FieldReader<bool, BUSFAULTACT_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BUSFAULTACT` writer - no description available"]
pub struct BUSFAULTACT_W<'a> {
    w: &'a mut W,
}
impl<'a> BUSFAULTACT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BUSFAULTACT_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "exception is not active"]
    #[inline(always)]
    pub fn busfaultact_0(self) -> &'a mut W {
        self.variant(BUSFAULTACT_A::BUSFAULTACT_0)
    }
    #[doc = "exception is active"]
    #[inline(always)]
    pub fn busfaultact_1(self) -> &'a mut W {
        self.variant(BUSFAULTACT_A::BUSFAULTACT_1)
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
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum USGFAULTACT_A {
    #[doc = "0: exception is not active"]
    USGFAULTACT_0 = 0,
    #[doc = "1: exception is active"]
    USGFAULTACT_1 = 1,
}
impl From<USGFAULTACT_A> for bool {
    #[inline(always)]
    fn from(variant: USGFAULTACT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `USGFAULTACT` reader - no description available"]
pub struct USGFAULTACT_R(crate::FieldReader<bool, USGFAULTACT_A>);
impl USGFAULTACT_R {
    pub(crate) fn new(bits: bool) -> Self {
        USGFAULTACT_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> USGFAULTACT_A {
        match self.bits {
            false => USGFAULTACT_A::USGFAULTACT_0,
            true => USGFAULTACT_A::USGFAULTACT_1,
        }
    }
    #[doc = "Checks if the value of the field is `USGFAULTACT_0`"]
    #[inline(always)]
    pub fn is_usgfaultact_0(&self) -> bool {
        **self == USGFAULTACT_A::USGFAULTACT_0
    }
    #[doc = "Checks if the value of the field is `USGFAULTACT_1`"]
    #[inline(always)]
    pub fn is_usgfaultact_1(&self) -> bool {
        **self == USGFAULTACT_A::USGFAULTACT_1
    }
}
impl core::ops::Deref for USGFAULTACT_R {
    type Target = crate::FieldReader<bool, USGFAULTACT_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `USGFAULTACT` writer - no description available"]
pub struct USGFAULTACT_W<'a> {
    w: &'a mut W,
}
impl<'a> USGFAULTACT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: USGFAULTACT_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "exception is not active"]
    #[inline(always)]
    pub fn usgfaultact_0(self) -> &'a mut W {
        self.variant(USGFAULTACT_A::USGFAULTACT_0)
    }
    #[doc = "exception is active"]
    #[inline(always)]
    pub fn usgfaultact_1(self) -> &'a mut W {
        self.variant(USGFAULTACT_A::USGFAULTACT_1)
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
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SVCALLACT_A {
    #[doc = "0: exception is not active"]
    SVCALLACT_0 = 0,
    #[doc = "1: exception is active"]
    SVCALLACT_1 = 1,
}
impl From<SVCALLACT_A> for bool {
    #[inline(always)]
    fn from(variant: SVCALLACT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SVCALLACT` reader - no description available"]
pub struct SVCALLACT_R(crate::FieldReader<bool, SVCALLACT_A>);
impl SVCALLACT_R {
    pub(crate) fn new(bits: bool) -> Self {
        SVCALLACT_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SVCALLACT_A {
        match self.bits {
            false => SVCALLACT_A::SVCALLACT_0,
            true => SVCALLACT_A::SVCALLACT_1,
        }
    }
    #[doc = "Checks if the value of the field is `SVCALLACT_0`"]
    #[inline(always)]
    pub fn is_svcallact_0(&self) -> bool {
        **self == SVCALLACT_A::SVCALLACT_0
    }
    #[doc = "Checks if the value of the field is `SVCALLACT_1`"]
    #[inline(always)]
    pub fn is_svcallact_1(&self) -> bool {
        **self == SVCALLACT_A::SVCALLACT_1
    }
}
impl core::ops::Deref for SVCALLACT_R {
    type Target = crate::FieldReader<bool, SVCALLACT_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SVCALLACT` writer - no description available"]
pub struct SVCALLACT_W<'a> {
    w: &'a mut W,
}
impl<'a> SVCALLACT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SVCALLACT_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "exception is not active"]
    #[inline(always)]
    pub fn svcallact_0(self) -> &'a mut W {
        self.variant(SVCALLACT_A::SVCALLACT_0)
    }
    #[doc = "exception is active"]
    #[inline(always)]
    pub fn svcallact_1(self) -> &'a mut W {
        self.variant(SVCALLACT_A::SVCALLACT_1)
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
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MONITORACT_A {
    #[doc = "0: exception is not active"]
    MONITORACT_0 = 0,
    #[doc = "1: exception is active"]
    MONITORACT_1 = 1,
}
impl From<MONITORACT_A> for bool {
    #[inline(always)]
    fn from(variant: MONITORACT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MONITORACT` reader - no description available"]
pub struct MONITORACT_R(crate::FieldReader<bool, MONITORACT_A>);
impl MONITORACT_R {
    pub(crate) fn new(bits: bool) -> Self {
        MONITORACT_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MONITORACT_A {
        match self.bits {
            false => MONITORACT_A::MONITORACT_0,
            true => MONITORACT_A::MONITORACT_1,
        }
    }
    #[doc = "Checks if the value of the field is `MONITORACT_0`"]
    #[inline(always)]
    pub fn is_monitoract_0(&self) -> bool {
        **self == MONITORACT_A::MONITORACT_0
    }
    #[doc = "Checks if the value of the field is `MONITORACT_1`"]
    #[inline(always)]
    pub fn is_monitoract_1(&self) -> bool {
        **self == MONITORACT_A::MONITORACT_1
    }
}
impl core::ops::Deref for MONITORACT_R {
    type Target = crate::FieldReader<bool, MONITORACT_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MONITORACT` writer - no description available"]
pub struct MONITORACT_W<'a> {
    w: &'a mut W,
}
impl<'a> MONITORACT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MONITORACT_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "exception is not active"]
    #[inline(always)]
    pub fn monitoract_0(self) -> &'a mut W {
        self.variant(MONITORACT_A::MONITORACT_0)
    }
    #[doc = "exception is active"]
    #[inline(always)]
    pub fn monitoract_1(self) -> &'a mut W {
        self.variant(MONITORACT_A::MONITORACT_1)
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
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PENDSVACT_A {
    #[doc = "0: exception is not active"]
    PENDSVACT_0 = 0,
    #[doc = "1: exception is active"]
    PENDSVACT_1 = 1,
}
impl From<PENDSVACT_A> for bool {
    #[inline(always)]
    fn from(variant: PENDSVACT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PENDSVACT` reader - no description available"]
pub struct PENDSVACT_R(crate::FieldReader<bool, PENDSVACT_A>);
impl PENDSVACT_R {
    pub(crate) fn new(bits: bool) -> Self {
        PENDSVACT_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PENDSVACT_A {
        match self.bits {
            false => PENDSVACT_A::PENDSVACT_0,
            true => PENDSVACT_A::PENDSVACT_1,
        }
    }
    #[doc = "Checks if the value of the field is `PENDSVACT_0`"]
    #[inline(always)]
    pub fn is_pendsvact_0(&self) -> bool {
        **self == PENDSVACT_A::PENDSVACT_0
    }
    #[doc = "Checks if the value of the field is `PENDSVACT_1`"]
    #[inline(always)]
    pub fn is_pendsvact_1(&self) -> bool {
        **self == PENDSVACT_A::PENDSVACT_1
    }
}
impl core::ops::Deref for PENDSVACT_R {
    type Target = crate::FieldReader<bool, PENDSVACT_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PENDSVACT` writer - no description available"]
pub struct PENDSVACT_W<'a> {
    w: &'a mut W,
}
impl<'a> PENDSVACT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PENDSVACT_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "exception is not active"]
    #[inline(always)]
    pub fn pendsvact_0(self) -> &'a mut W {
        self.variant(PENDSVACT_A::PENDSVACT_0)
    }
    #[doc = "exception is active"]
    #[inline(always)]
    pub fn pendsvact_1(self) -> &'a mut W {
        self.variant(PENDSVACT_A::PENDSVACT_1)
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
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SYSTICKACT_A {
    #[doc = "0: exception is not active"]
    SYSTICKACT_0 = 0,
    #[doc = "1: exception is active"]
    SYSTICKACT_1 = 1,
}
impl From<SYSTICKACT_A> for bool {
    #[inline(always)]
    fn from(variant: SYSTICKACT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SYSTICKACT` reader - no description available"]
pub struct SYSTICKACT_R(crate::FieldReader<bool, SYSTICKACT_A>);
impl SYSTICKACT_R {
    pub(crate) fn new(bits: bool) -> Self {
        SYSTICKACT_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SYSTICKACT_A {
        match self.bits {
            false => SYSTICKACT_A::SYSTICKACT_0,
            true => SYSTICKACT_A::SYSTICKACT_1,
        }
    }
    #[doc = "Checks if the value of the field is `SYSTICKACT_0`"]
    #[inline(always)]
    pub fn is_systickact_0(&self) -> bool {
        **self == SYSTICKACT_A::SYSTICKACT_0
    }
    #[doc = "Checks if the value of the field is `SYSTICKACT_1`"]
    #[inline(always)]
    pub fn is_systickact_1(&self) -> bool {
        **self == SYSTICKACT_A::SYSTICKACT_1
    }
}
impl core::ops::Deref for SYSTICKACT_R {
    type Target = crate::FieldReader<bool, SYSTICKACT_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SYSTICKACT` writer - no description available"]
pub struct SYSTICKACT_W<'a> {
    w: &'a mut W,
}
impl<'a> SYSTICKACT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SYSTICKACT_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "exception is not active"]
    #[inline(always)]
    pub fn systickact_0(self) -> &'a mut W {
        self.variant(SYSTICKACT_A::SYSTICKACT_0)
    }
    #[doc = "exception is active"]
    #[inline(always)]
    pub fn systickact_1(self) -> &'a mut W {
        self.variant(SYSTICKACT_A::SYSTICKACT_1)
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
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum USGFAULTPENDED_A {
    #[doc = "0: exception is not pending"]
    USGFAULTPENDED_0 = 0,
    #[doc = "1: exception is pending"]
    USGFAULTPENDED_1 = 1,
}
impl From<USGFAULTPENDED_A> for bool {
    #[inline(always)]
    fn from(variant: USGFAULTPENDED_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `USGFAULTPENDED` reader - no description available"]
pub struct USGFAULTPENDED_R(crate::FieldReader<bool, USGFAULTPENDED_A>);
impl USGFAULTPENDED_R {
    pub(crate) fn new(bits: bool) -> Self {
        USGFAULTPENDED_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> USGFAULTPENDED_A {
        match self.bits {
            false => USGFAULTPENDED_A::USGFAULTPENDED_0,
            true => USGFAULTPENDED_A::USGFAULTPENDED_1,
        }
    }
    #[doc = "Checks if the value of the field is `USGFAULTPENDED_0`"]
    #[inline(always)]
    pub fn is_usgfaultpended_0(&self) -> bool {
        **self == USGFAULTPENDED_A::USGFAULTPENDED_0
    }
    #[doc = "Checks if the value of the field is `USGFAULTPENDED_1`"]
    #[inline(always)]
    pub fn is_usgfaultpended_1(&self) -> bool {
        **self == USGFAULTPENDED_A::USGFAULTPENDED_1
    }
}
impl core::ops::Deref for USGFAULTPENDED_R {
    type Target = crate::FieldReader<bool, USGFAULTPENDED_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `USGFAULTPENDED` writer - no description available"]
pub struct USGFAULTPENDED_W<'a> {
    w: &'a mut W,
}
impl<'a> USGFAULTPENDED_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: USGFAULTPENDED_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "exception is not pending"]
    #[inline(always)]
    pub fn usgfaultpended_0(self) -> &'a mut W {
        self.variant(USGFAULTPENDED_A::USGFAULTPENDED_0)
    }
    #[doc = "exception is pending"]
    #[inline(always)]
    pub fn usgfaultpended_1(self) -> &'a mut W {
        self.variant(USGFAULTPENDED_A::USGFAULTPENDED_1)
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
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MEMFAULTPENDED_A {
    #[doc = "0: exception is not pending"]
    MEMFAULTPENDED_0 = 0,
    #[doc = "1: exception is pending"]
    MEMFAULTPENDED_1 = 1,
}
impl From<MEMFAULTPENDED_A> for bool {
    #[inline(always)]
    fn from(variant: MEMFAULTPENDED_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MEMFAULTPENDED` reader - no description available"]
pub struct MEMFAULTPENDED_R(crate::FieldReader<bool, MEMFAULTPENDED_A>);
impl MEMFAULTPENDED_R {
    pub(crate) fn new(bits: bool) -> Self {
        MEMFAULTPENDED_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MEMFAULTPENDED_A {
        match self.bits {
            false => MEMFAULTPENDED_A::MEMFAULTPENDED_0,
            true => MEMFAULTPENDED_A::MEMFAULTPENDED_1,
        }
    }
    #[doc = "Checks if the value of the field is `MEMFAULTPENDED_0`"]
    #[inline(always)]
    pub fn is_memfaultpended_0(&self) -> bool {
        **self == MEMFAULTPENDED_A::MEMFAULTPENDED_0
    }
    #[doc = "Checks if the value of the field is `MEMFAULTPENDED_1`"]
    #[inline(always)]
    pub fn is_memfaultpended_1(&self) -> bool {
        **self == MEMFAULTPENDED_A::MEMFAULTPENDED_1
    }
}
impl core::ops::Deref for MEMFAULTPENDED_R {
    type Target = crate::FieldReader<bool, MEMFAULTPENDED_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MEMFAULTPENDED` writer - no description available"]
pub struct MEMFAULTPENDED_W<'a> {
    w: &'a mut W,
}
impl<'a> MEMFAULTPENDED_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MEMFAULTPENDED_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "exception is not pending"]
    #[inline(always)]
    pub fn memfaultpended_0(self) -> &'a mut W {
        self.variant(MEMFAULTPENDED_A::MEMFAULTPENDED_0)
    }
    #[doc = "exception is pending"]
    #[inline(always)]
    pub fn memfaultpended_1(self) -> &'a mut W {
        self.variant(MEMFAULTPENDED_A::MEMFAULTPENDED_1)
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
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BUSFAULTPENDED_A {
    #[doc = "0: exception is not pending"]
    BUSFAULTPENDED_0 = 0,
    #[doc = "1: exception is pending"]
    BUSFAULTPENDED_1 = 1,
}
impl From<BUSFAULTPENDED_A> for bool {
    #[inline(always)]
    fn from(variant: BUSFAULTPENDED_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BUSFAULTPENDED` reader - no description available"]
pub struct BUSFAULTPENDED_R(crate::FieldReader<bool, BUSFAULTPENDED_A>);
impl BUSFAULTPENDED_R {
    pub(crate) fn new(bits: bool) -> Self {
        BUSFAULTPENDED_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BUSFAULTPENDED_A {
        match self.bits {
            false => BUSFAULTPENDED_A::BUSFAULTPENDED_0,
            true => BUSFAULTPENDED_A::BUSFAULTPENDED_1,
        }
    }
    #[doc = "Checks if the value of the field is `BUSFAULTPENDED_0`"]
    #[inline(always)]
    pub fn is_busfaultpended_0(&self) -> bool {
        **self == BUSFAULTPENDED_A::BUSFAULTPENDED_0
    }
    #[doc = "Checks if the value of the field is `BUSFAULTPENDED_1`"]
    #[inline(always)]
    pub fn is_busfaultpended_1(&self) -> bool {
        **self == BUSFAULTPENDED_A::BUSFAULTPENDED_1
    }
}
impl core::ops::Deref for BUSFAULTPENDED_R {
    type Target = crate::FieldReader<bool, BUSFAULTPENDED_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BUSFAULTPENDED` writer - no description available"]
pub struct BUSFAULTPENDED_W<'a> {
    w: &'a mut W,
}
impl<'a> BUSFAULTPENDED_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BUSFAULTPENDED_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "exception is not pending"]
    #[inline(always)]
    pub fn busfaultpended_0(self) -> &'a mut W {
        self.variant(BUSFAULTPENDED_A::BUSFAULTPENDED_0)
    }
    #[doc = "exception is pending"]
    #[inline(always)]
    pub fn busfaultpended_1(self) -> &'a mut W {
        self.variant(BUSFAULTPENDED_A::BUSFAULTPENDED_1)
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
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SVCALLPENDED_A {
    #[doc = "0: exception is not pending"]
    SVCALLPENDED_0 = 0,
    #[doc = "1: exception is pending"]
    SVCALLPENDED_1 = 1,
}
impl From<SVCALLPENDED_A> for bool {
    #[inline(always)]
    fn from(variant: SVCALLPENDED_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SVCALLPENDED` reader - no description available"]
pub struct SVCALLPENDED_R(crate::FieldReader<bool, SVCALLPENDED_A>);
impl SVCALLPENDED_R {
    pub(crate) fn new(bits: bool) -> Self {
        SVCALLPENDED_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SVCALLPENDED_A {
        match self.bits {
            false => SVCALLPENDED_A::SVCALLPENDED_0,
            true => SVCALLPENDED_A::SVCALLPENDED_1,
        }
    }
    #[doc = "Checks if the value of the field is `SVCALLPENDED_0`"]
    #[inline(always)]
    pub fn is_svcallpended_0(&self) -> bool {
        **self == SVCALLPENDED_A::SVCALLPENDED_0
    }
    #[doc = "Checks if the value of the field is `SVCALLPENDED_1`"]
    #[inline(always)]
    pub fn is_svcallpended_1(&self) -> bool {
        **self == SVCALLPENDED_A::SVCALLPENDED_1
    }
}
impl core::ops::Deref for SVCALLPENDED_R {
    type Target = crate::FieldReader<bool, SVCALLPENDED_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SVCALLPENDED` writer - no description available"]
pub struct SVCALLPENDED_W<'a> {
    w: &'a mut W,
}
impl<'a> SVCALLPENDED_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SVCALLPENDED_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "exception is not pending"]
    #[inline(always)]
    pub fn svcallpended_0(self) -> &'a mut W {
        self.variant(SVCALLPENDED_A::SVCALLPENDED_0)
    }
    #[doc = "exception is pending"]
    #[inline(always)]
    pub fn svcallpended_1(self) -> &'a mut W {
        self.variant(SVCALLPENDED_A::SVCALLPENDED_1)
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
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MEMFAULTENA_A {
    #[doc = "0: disable the exception"]
    MEMFAULTENA_0 = 0,
    #[doc = "1: enable the exception"]
    MEMFAULTENA_1 = 1,
}
impl From<MEMFAULTENA_A> for bool {
    #[inline(always)]
    fn from(variant: MEMFAULTENA_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MEMFAULTENA` reader - no description available"]
pub struct MEMFAULTENA_R(crate::FieldReader<bool, MEMFAULTENA_A>);
impl MEMFAULTENA_R {
    pub(crate) fn new(bits: bool) -> Self {
        MEMFAULTENA_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MEMFAULTENA_A {
        match self.bits {
            false => MEMFAULTENA_A::MEMFAULTENA_0,
            true => MEMFAULTENA_A::MEMFAULTENA_1,
        }
    }
    #[doc = "Checks if the value of the field is `MEMFAULTENA_0`"]
    #[inline(always)]
    pub fn is_memfaultena_0(&self) -> bool {
        **self == MEMFAULTENA_A::MEMFAULTENA_0
    }
    #[doc = "Checks if the value of the field is `MEMFAULTENA_1`"]
    #[inline(always)]
    pub fn is_memfaultena_1(&self) -> bool {
        **self == MEMFAULTENA_A::MEMFAULTENA_1
    }
}
impl core::ops::Deref for MEMFAULTENA_R {
    type Target = crate::FieldReader<bool, MEMFAULTENA_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MEMFAULTENA` writer - no description available"]
pub struct MEMFAULTENA_W<'a> {
    w: &'a mut W,
}
impl<'a> MEMFAULTENA_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MEMFAULTENA_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "disable the exception"]
    #[inline(always)]
    pub fn memfaultena_0(self) -> &'a mut W {
        self.variant(MEMFAULTENA_A::MEMFAULTENA_0)
    }
    #[doc = "enable the exception"]
    #[inline(always)]
    pub fn memfaultena_1(self) -> &'a mut W {
        self.variant(MEMFAULTENA_A::MEMFAULTENA_1)
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
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BUSFAULTENA_A {
    #[doc = "0: disable the exception"]
    BUSFAULTENA_0 = 0,
    #[doc = "1: enable the exception"]
    BUSFAULTENA_1 = 1,
}
impl From<BUSFAULTENA_A> for bool {
    #[inline(always)]
    fn from(variant: BUSFAULTENA_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BUSFAULTENA` reader - no description available"]
pub struct BUSFAULTENA_R(crate::FieldReader<bool, BUSFAULTENA_A>);
impl BUSFAULTENA_R {
    pub(crate) fn new(bits: bool) -> Self {
        BUSFAULTENA_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BUSFAULTENA_A {
        match self.bits {
            false => BUSFAULTENA_A::BUSFAULTENA_0,
            true => BUSFAULTENA_A::BUSFAULTENA_1,
        }
    }
    #[doc = "Checks if the value of the field is `BUSFAULTENA_0`"]
    #[inline(always)]
    pub fn is_busfaultena_0(&self) -> bool {
        **self == BUSFAULTENA_A::BUSFAULTENA_0
    }
    #[doc = "Checks if the value of the field is `BUSFAULTENA_1`"]
    #[inline(always)]
    pub fn is_busfaultena_1(&self) -> bool {
        **self == BUSFAULTENA_A::BUSFAULTENA_1
    }
}
impl core::ops::Deref for BUSFAULTENA_R {
    type Target = crate::FieldReader<bool, BUSFAULTENA_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BUSFAULTENA` writer - no description available"]
pub struct BUSFAULTENA_W<'a> {
    w: &'a mut W,
}
impl<'a> BUSFAULTENA_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BUSFAULTENA_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "disable the exception"]
    #[inline(always)]
    pub fn busfaultena_0(self) -> &'a mut W {
        self.variant(BUSFAULTENA_A::BUSFAULTENA_0)
    }
    #[doc = "enable the exception"]
    #[inline(always)]
    pub fn busfaultena_1(self) -> &'a mut W {
        self.variant(BUSFAULTENA_A::BUSFAULTENA_1)
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
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum USGFAULTENA_A {
    #[doc = "0: disable the exception"]
    USGFAULTENA_0 = 0,
    #[doc = "1: enable the exception"]
    USGFAULTENA_1 = 1,
}
impl From<USGFAULTENA_A> for bool {
    #[inline(always)]
    fn from(variant: USGFAULTENA_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `USGFAULTENA` reader - no description available"]
pub struct USGFAULTENA_R(crate::FieldReader<bool, USGFAULTENA_A>);
impl USGFAULTENA_R {
    pub(crate) fn new(bits: bool) -> Self {
        USGFAULTENA_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> USGFAULTENA_A {
        match self.bits {
            false => USGFAULTENA_A::USGFAULTENA_0,
            true => USGFAULTENA_A::USGFAULTENA_1,
        }
    }
    #[doc = "Checks if the value of the field is `USGFAULTENA_0`"]
    #[inline(always)]
    pub fn is_usgfaultena_0(&self) -> bool {
        **self == USGFAULTENA_A::USGFAULTENA_0
    }
    #[doc = "Checks if the value of the field is `USGFAULTENA_1`"]
    #[inline(always)]
    pub fn is_usgfaultena_1(&self) -> bool {
        **self == USGFAULTENA_A::USGFAULTENA_1
    }
}
impl core::ops::Deref for USGFAULTENA_R {
    type Target = crate::FieldReader<bool, USGFAULTENA_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `USGFAULTENA` writer - no description available"]
pub struct USGFAULTENA_W<'a> {
    w: &'a mut W,
}
impl<'a> USGFAULTENA_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: USGFAULTENA_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "disable the exception"]
    #[inline(always)]
    pub fn usgfaultena_0(self) -> &'a mut W {
        self.variant(USGFAULTENA_A::USGFAULTENA_0)
    }
    #[doc = "enable the exception"]
    #[inline(always)]
    pub fn usgfaultena_1(self) -> &'a mut W {
        self.variant(USGFAULTENA_A::USGFAULTENA_1)
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
impl R {
    #[doc = "Bit 0 - no description available"]
    #[inline(always)]
    pub fn memfaultact(&self) -> MEMFAULTACT_R {
        MEMFAULTACT_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - no description available"]
    #[inline(always)]
    pub fn busfaultact(&self) -> BUSFAULTACT_R {
        BUSFAULTACT_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 3 - no description available"]
    #[inline(always)]
    pub fn usgfaultact(&self) -> USGFAULTACT_R {
        USGFAULTACT_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 7 - no description available"]
    #[inline(always)]
    pub fn svcallact(&self) -> SVCALLACT_R {
        SVCALLACT_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - no description available"]
    #[inline(always)]
    pub fn monitoract(&self) -> MONITORACT_R {
        MONITORACT_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 10 - no description available"]
    #[inline(always)]
    pub fn pendsvact(&self) -> PENDSVACT_R {
        PENDSVACT_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - no description available"]
    #[inline(always)]
    pub fn systickact(&self) -> SYSTICKACT_R {
        SYSTICKACT_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - no description available"]
    #[inline(always)]
    pub fn usgfaultpended(&self) -> USGFAULTPENDED_R {
        USGFAULTPENDED_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - no description available"]
    #[inline(always)]
    pub fn memfaultpended(&self) -> MEMFAULTPENDED_R {
        MEMFAULTPENDED_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - no description available"]
    #[inline(always)]
    pub fn busfaultpended(&self) -> BUSFAULTPENDED_R {
        BUSFAULTPENDED_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - no description available"]
    #[inline(always)]
    pub fn svcallpended(&self) -> SVCALLPENDED_R {
        SVCALLPENDED_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - no description available"]
    #[inline(always)]
    pub fn memfaultena(&self) -> MEMFAULTENA_R {
        MEMFAULTENA_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - no description available"]
    #[inline(always)]
    pub fn busfaultena(&self) -> BUSFAULTENA_R {
        BUSFAULTENA_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - no description available"]
    #[inline(always)]
    pub fn usgfaultena(&self) -> USGFAULTENA_R {
        USGFAULTENA_R::new(((self.bits >> 18) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - no description available"]
    #[inline(always)]
    pub fn memfaultact(&mut self) -> MEMFAULTACT_W {
        MEMFAULTACT_W { w: self }
    }
    #[doc = "Bit 1 - no description available"]
    #[inline(always)]
    pub fn busfaultact(&mut self) -> BUSFAULTACT_W {
        BUSFAULTACT_W { w: self }
    }
    #[doc = "Bit 3 - no description available"]
    #[inline(always)]
    pub fn usgfaultact(&mut self) -> USGFAULTACT_W {
        USGFAULTACT_W { w: self }
    }
    #[doc = "Bit 7 - no description available"]
    #[inline(always)]
    pub fn svcallact(&mut self) -> SVCALLACT_W {
        SVCALLACT_W { w: self }
    }
    #[doc = "Bit 8 - no description available"]
    #[inline(always)]
    pub fn monitoract(&mut self) -> MONITORACT_W {
        MONITORACT_W { w: self }
    }
    #[doc = "Bit 10 - no description available"]
    #[inline(always)]
    pub fn pendsvact(&mut self) -> PENDSVACT_W {
        PENDSVACT_W { w: self }
    }
    #[doc = "Bit 11 - no description available"]
    #[inline(always)]
    pub fn systickact(&mut self) -> SYSTICKACT_W {
        SYSTICKACT_W { w: self }
    }
    #[doc = "Bit 12 - no description available"]
    #[inline(always)]
    pub fn usgfaultpended(&mut self) -> USGFAULTPENDED_W {
        USGFAULTPENDED_W { w: self }
    }
    #[doc = "Bit 13 - no description available"]
    #[inline(always)]
    pub fn memfaultpended(&mut self) -> MEMFAULTPENDED_W {
        MEMFAULTPENDED_W { w: self }
    }
    #[doc = "Bit 14 - no description available"]
    #[inline(always)]
    pub fn busfaultpended(&mut self) -> BUSFAULTPENDED_W {
        BUSFAULTPENDED_W { w: self }
    }
    #[doc = "Bit 15 - no description available"]
    #[inline(always)]
    pub fn svcallpended(&mut self) -> SVCALLPENDED_W {
        SVCALLPENDED_W { w: self }
    }
    #[doc = "Bit 16 - no description available"]
    #[inline(always)]
    pub fn memfaultena(&mut self) -> MEMFAULTENA_W {
        MEMFAULTENA_W { w: self }
    }
    #[doc = "Bit 17 - no description available"]
    #[inline(always)]
    pub fn busfaultena(&mut self) -> BUSFAULTENA_W {
        BUSFAULTENA_W { w: self }
    }
    #[doc = "Bit 18 - no description available"]
    #[inline(always)]
    pub fn usgfaultena(&mut self) -> USGFAULTENA_W {
        USGFAULTENA_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "System Handler Control and State Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [shcsr](index.html) module"]
pub struct SHCSR_SPEC;
impl crate::RegisterSpec for SHCSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [shcsr::R](R) reader structure"]
impl crate::Readable for SHCSR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [shcsr::W](W) writer structure"]
impl crate::Writable for SHCSR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SHCSR to value 0"]
impl crate::Resettable for SHCSR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
