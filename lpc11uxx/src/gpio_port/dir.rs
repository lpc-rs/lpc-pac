#[doc = "Register `DIR%s` reader"]
pub struct R(crate::R<DIR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DIR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DIR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DIR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DIR%s` writer"]
pub struct W(crate::W<DIR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DIR_SPEC>;
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
impl From<crate::W<DIR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DIR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DIRP0` reader - Selects pin direction for pin P0/1_n (bit 0 = P0/1_0, bit 1 = P0_1, ..., bit 31 = P0/1_31). 0 = input. 1 = output."]
pub struct DIRP0_R(crate::FieldReader<bool, bool>);
impl DIRP0_R {
    pub(crate) fn new(bits: bool) -> Self {
        DIRP0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DIRP0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DIRP0` writer - Selects pin direction for pin P0/1_n (bit 0 = P0/1_0, bit 1 = P0_1, ..., bit 31 = P0/1_31). 0 = input. 1 = output."]
pub struct DIRP0_W<'a> {
    w: &'a mut W,
}
impl<'a> DIRP0_W<'a> {
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
#[doc = "Field `DIRP1` reader - Selects pin direction for pin P0/1_n (bit 0 = P0/1_0, bit 1 = P0_1, ..., bit 31 = P0/1_31). 0 = input. 1 = output."]
pub struct DIRP1_R(crate::FieldReader<bool, bool>);
impl DIRP1_R {
    pub(crate) fn new(bits: bool) -> Self {
        DIRP1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DIRP1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DIRP1` writer - Selects pin direction for pin P0/1_n (bit 0 = P0/1_0, bit 1 = P0_1, ..., bit 31 = P0/1_31). 0 = input. 1 = output."]
pub struct DIRP1_W<'a> {
    w: &'a mut W,
}
impl<'a> DIRP1_W<'a> {
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
#[doc = "Field `DIRP2` reader - Selects pin direction for pin P0/1_n (bit 0 = P0/1_0, bit 1 = P0_1, ..., bit 31 = P0/1_31). 0 = input. 1 = output."]
pub struct DIRP2_R(crate::FieldReader<bool, bool>);
impl DIRP2_R {
    pub(crate) fn new(bits: bool) -> Self {
        DIRP2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DIRP2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DIRP2` writer - Selects pin direction for pin P0/1_n (bit 0 = P0/1_0, bit 1 = P0_1, ..., bit 31 = P0/1_31). 0 = input. 1 = output."]
pub struct DIRP2_W<'a> {
    w: &'a mut W,
}
impl<'a> DIRP2_W<'a> {
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
#[doc = "Field `DIRP3` reader - Selects pin direction for pin P0/1_n (bit 0 = P0/1_0, bit 1 = P0_1, ..., bit 31 = P0/1_31). 0 = input. 1 = output."]
pub struct DIRP3_R(crate::FieldReader<bool, bool>);
impl DIRP3_R {
    pub(crate) fn new(bits: bool) -> Self {
        DIRP3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DIRP3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DIRP3` writer - Selects pin direction for pin P0/1_n (bit 0 = P0/1_0, bit 1 = P0_1, ..., bit 31 = P0/1_31). 0 = input. 1 = output."]
pub struct DIRP3_W<'a> {
    w: &'a mut W,
}
impl<'a> DIRP3_W<'a> {
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
#[doc = "Field `DIRP4` reader - Selects pin direction for pin P0/1_n (bit 0 = P0/1_0, bit 1 = P0_1, ..., bit 31 = P0/1_31). 0 = input. 1 = output."]
pub struct DIRP4_R(crate::FieldReader<bool, bool>);
impl DIRP4_R {
    pub(crate) fn new(bits: bool) -> Self {
        DIRP4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DIRP4_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DIRP4` writer - Selects pin direction for pin P0/1_n (bit 0 = P0/1_0, bit 1 = P0_1, ..., bit 31 = P0/1_31). 0 = input. 1 = output."]
pub struct DIRP4_W<'a> {
    w: &'a mut W,
}
impl<'a> DIRP4_W<'a> {
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
#[doc = "Field `DIRP5` reader - Selects pin direction for pin P0/1_n (bit 0 = P0/1_0, bit 1 = P0_1, ..., bit 31 = P0/1_31). 0 = input. 1 = output."]
pub struct DIRP5_R(crate::FieldReader<bool, bool>);
impl DIRP5_R {
    pub(crate) fn new(bits: bool) -> Self {
        DIRP5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DIRP5_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DIRP5` writer - Selects pin direction for pin P0/1_n (bit 0 = P0/1_0, bit 1 = P0_1, ..., bit 31 = P0/1_31). 0 = input. 1 = output."]
pub struct DIRP5_W<'a> {
    w: &'a mut W,
}
impl<'a> DIRP5_W<'a> {
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
#[doc = "Field `DIRP6` reader - Selects pin direction for pin P0/1_n (bit 0 = P0/1_0, bit 1 = P0_1, ..., bit 31 = P0/1_31). 0 = input. 1 = output."]
pub struct DIRP6_R(crate::FieldReader<bool, bool>);
impl DIRP6_R {
    pub(crate) fn new(bits: bool) -> Self {
        DIRP6_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DIRP6_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DIRP6` writer - Selects pin direction for pin P0/1_n (bit 0 = P0/1_0, bit 1 = P0_1, ..., bit 31 = P0/1_31). 0 = input. 1 = output."]
pub struct DIRP6_W<'a> {
    w: &'a mut W,
}
impl<'a> DIRP6_W<'a> {
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
#[doc = "Field `DIRP7` reader - Selects pin direction for pin P0/1_n (bit 0 = P0/1_0, bit 1 = P0_1, ..., bit 31 = P0/1_31). 0 = input. 1 = output."]
pub struct DIRP7_R(crate::FieldReader<bool, bool>);
impl DIRP7_R {
    pub(crate) fn new(bits: bool) -> Self {
        DIRP7_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DIRP7_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DIRP7` writer - Selects pin direction for pin P0/1_n (bit 0 = P0/1_0, bit 1 = P0_1, ..., bit 31 = P0/1_31). 0 = input. 1 = output."]
pub struct DIRP7_W<'a> {
    w: &'a mut W,
}
impl<'a> DIRP7_W<'a> {
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
#[doc = "Field `DIRP8` reader - Selects pin direction for pin P0/1_n (bit 0 = P0/1_0, bit 1 = P0_1, ..., bit 31 = P0/1_31). 0 = input. 1 = output."]
pub struct DIRP8_R(crate::FieldReader<bool, bool>);
impl DIRP8_R {
    pub(crate) fn new(bits: bool) -> Self {
        DIRP8_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DIRP8_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DIRP8` writer - Selects pin direction for pin P0/1_n (bit 0 = P0/1_0, bit 1 = P0_1, ..., bit 31 = P0/1_31). 0 = input. 1 = output."]
pub struct DIRP8_W<'a> {
    w: &'a mut W,
}
impl<'a> DIRP8_W<'a> {
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
#[doc = "Field `DIRP9` reader - Selects pin direction for pin P0/1_n (bit 0 = P0/1_0, bit 1 = P0_1, ..., bit 31 = P0/1_31). 0 = input. 1 = output."]
pub struct DIRP9_R(crate::FieldReader<bool, bool>);
impl DIRP9_R {
    pub(crate) fn new(bits: bool) -> Self {
        DIRP9_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DIRP9_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DIRP9` writer - Selects pin direction for pin P0/1_n (bit 0 = P0/1_0, bit 1 = P0_1, ..., bit 31 = P0/1_31). 0 = input. 1 = output."]
pub struct DIRP9_W<'a> {
    w: &'a mut W,
}
impl<'a> DIRP9_W<'a> {
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
#[doc = "Field `DIRP10` reader - Selects pin direction for pin P0/1_n (bit 0 = P0/1_0, bit 1 = P0_1, ..., bit 31 = P0/1_31). 0 = input. 1 = output."]
pub struct DIRP10_R(crate::FieldReader<bool, bool>);
impl DIRP10_R {
    pub(crate) fn new(bits: bool) -> Self {
        DIRP10_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DIRP10_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DIRP10` writer - Selects pin direction for pin P0/1_n (bit 0 = P0/1_0, bit 1 = P0_1, ..., bit 31 = P0/1_31). 0 = input. 1 = output."]
pub struct DIRP10_W<'a> {
    w: &'a mut W,
}
impl<'a> DIRP10_W<'a> {
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
#[doc = "Field `DIRP11` reader - Selects pin direction for pin P0/1_n (bit 0 = P0/1_0, bit 1 = P0_1, ..., bit 31 = P0/1_31). 0 = input. 1 = output."]
pub struct DIRP11_R(crate::FieldReader<bool, bool>);
impl DIRP11_R {
    pub(crate) fn new(bits: bool) -> Self {
        DIRP11_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DIRP11_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DIRP11` writer - Selects pin direction for pin P0/1_n (bit 0 = P0/1_0, bit 1 = P0_1, ..., bit 31 = P0/1_31). 0 = input. 1 = output."]
pub struct DIRP11_W<'a> {
    w: &'a mut W,
}
impl<'a> DIRP11_W<'a> {
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
#[doc = "Field `DIRP12` reader - Selects pin direction for pin P0/1_n (bit 0 = P0/1_0, bit 1 = P0_1, ..., bit 31 = P0/1_31). 0 = input. 1 = output."]
pub struct DIRP12_R(crate::FieldReader<bool, bool>);
impl DIRP12_R {
    pub(crate) fn new(bits: bool) -> Self {
        DIRP12_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DIRP12_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DIRP12` writer - Selects pin direction for pin P0/1_n (bit 0 = P0/1_0, bit 1 = P0_1, ..., bit 31 = P0/1_31). 0 = input. 1 = output."]
pub struct DIRP12_W<'a> {
    w: &'a mut W,
}
impl<'a> DIRP12_W<'a> {
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
#[doc = "Field `DIRP13` reader - Selects pin direction for pin P0/1_n (bit 0 = P0/1_0, bit 1 = P0_1, ..., bit 31 = P0/1_31). 0 = input. 1 = output."]
pub struct DIRP13_R(crate::FieldReader<bool, bool>);
impl DIRP13_R {
    pub(crate) fn new(bits: bool) -> Self {
        DIRP13_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DIRP13_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DIRP13` writer - Selects pin direction for pin P0/1_n (bit 0 = P0/1_0, bit 1 = P0_1, ..., bit 31 = P0/1_31). 0 = input. 1 = output."]
pub struct DIRP13_W<'a> {
    w: &'a mut W,
}
impl<'a> DIRP13_W<'a> {
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
#[doc = "Field `DIRP14` reader - Selects pin direction for pin P0/1_n (bit 0 = P0/1_0, bit 1 = P0_1, ..., bit 31 = P0/1_31). 0 = input. 1 = output."]
pub struct DIRP14_R(crate::FieldReader<bool, bool>);
impl DIRP14_R {
    pub(crate) fn new(bits: bool) -> Self {
        DIRP14_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DIRP14_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DIRP14` writer - Selects pin direction for pin P0/1_n (bit 0 = P0/1_0, bit 1 = P0_1, ..., bit 31 = P0/1_31). 0 = input. 1 = output."]
pub struct DIRP14_W<'a> {
    w: &'a mut W,
}
impl<'a> DIRP14_W<'a> {
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
#[doc = "Field `DIRP15` reader - Selects pin direction for pin P0/1_n (bit 0 = P0/1_0, bit 1 = P0_1, ..., bit 31 = P0/1_31). 0 = input. 1 = output."]
pub struct DIRP15_R(crate::FieldReader<bool, bool>);
impl DIRP15_R {
    pub(crate) fn new(bits: bool) -> Self {
        DIRP15_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DIRP15_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DIRP15` writer - Selects pin direction for pin P0/1_n (bit 0 = P0/1_0, bit 1 = P0_1, ..., bit 31 = P0/1_31). 0 = input. 1 = output."]
pub struct DIRP15_W<'a> {
    w: &'a mut W,
}
impl<'a> DIRP15_W<'a> {
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
#[doc = "Field `DIRP16` reader - Selects pin direction for pin P0/1_n (bit 0 = P0/1_0, bit 1 = P0_1, ..., bit 31 = P0/1_31). 0 = input. 1 = output."]
pub struct DIRP16_R(crate::FieldReader<bool, bool>);
impl DIRP16_R {
    pub(crate) fn new(bits: bool) -> Self {
        DIRP16_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DIRP16_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DIRP16` writer - Selects pin direction for pin P0/1_n (bit 0 = P0/1_0, bit 1 = P0_1, ..., bit 31 = P0/1_31). 0 = input. 1 = output."]
pub struct DIRP16_W<'a> {
    w: &'a mut W,
}
impl<'a> DIRP16_W<'a> {
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
#[doc = "Field `DIRP17` reader - Selects pin direction for pin P0/1_n (bit 0 = P0/1_0, bit 1 = P0_1, ..., bit 31 = P0/1_31). 0 = input. 1 = output."]
pub struct DIRP17_R(crate::FieldReader<bool, bool>);
impl DIRP17_R {
    pub(crate) fn new(bits: bool) -> Self {
        DIRP17_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DIRP17_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DIRP17` writer - Selects pin direction for pin P0/1_n (bit 0 = P0/1_0, bit 1 = P0_1, ..., bit 31 = P0/1_31). 0 = input. 1 = output."]
pub struct DIRP17_W<'a> {
    w: &'a mut W,
}
impl<'a> DIRP17_W<'a> {
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
#[doc = "Field `DIRP18` reader - Selects pin direction for pin P0/1_n (bit 0 = P0/1_0, bit 1 = P0_1, ..., bit 31 = P0/1_31). 0 = input. 1 = output."]
pub struct DIRP18_R(crate::FieldReader<bool, bool>);
impl DIRP18_R {
    pub(crate) fn new(bits: bool) -> Self {
        DIRP18_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DIRP18_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DIRP18` writer - Selects pin direction for pin P0/1_n (bit 0 = P0/1_0, bit 1 = P0_1, ..., bit 31 = P0/1_31). 0 = input. 1 = output."]
pub struct DIRP18_W<'a> {
    w: &'a mut W,
}
impl<'a> DIRP18_W<'a> {
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
#[doc = "Field `DIRP19` reader - Selects pin direction for pin P0/1_n (bit 0 = P0/1_0, bit 1 = P0_1, ..., bit 31 = P0/1_31). 0 = input. 1 = output."]
pub struct DIRP19_R(crate::FieldReader<bool, bool>);
impl DIRP19_R {
    pub(crate) fn new(bits: bool) -> Self {
        DIRP19_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DIRP19_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DIRP19` writer - Selects pin direction for pin P0/1_n (bit 0 = P0/1_0, bit 1 = P0_1, ..., bit 31 = P0/1_31). 0 = input. 1 = output."]
pub struct DIRP19_W<'a> {
    w: &'a mut W,
}
impl<'a> DIRP19_W<'a> {
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
#[doc = "Field `DIRP20` reader - Selects pin direction for pin P0/1_n (bit 0 = P0/1_0, bit 1 = P0_1, ..., bit 31 = P0/1_31). 0 = input. 1 = output."]
pub struct DIRP20_R(crate::FieldReader<bool, bool>);
impl DIRP20_R {
    pub(crate) fn new(bits: bool) -> Self {
        DIRP20_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DIRP20_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DIRP20` writer - Selects pin direction for pin P0/1_n (bit 0 = P0/1_0, bit 1 = P0_1, ..., bit 31 = P0/1_31). 0 = input. 1 = output."]
pub struct DIRP20_W<'a> {
    w: &'a mut W,
}
impl<'a> DIRP20_W<'a> {
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
#[doc = "Field `DIRP21` reader - Selects pin direction for pin P0/1_n (bit 0 = P0/1_0, bit 1 = P0_1, ..., bit 31 = P0/1_31). 0 = input. 1 = output."]
pub struct DIRP21_R(crate::FieldReader<bool, bool>);
impl DIRP21_R {
    pub(crate) fn new(bits: bool) -> Self {
        DIRP21_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DIRP21_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DIRP21` writer - Selects pin direction for pin P0/1_n (bit 0 = P0/1_0, bit 1 = P0_1, ..., bit 31 = P0/1_31). 0 = input. 1 = output."]
pub struct DIRP21_W<'a> {
    w: &'a mut W,
}
impl<'a> DIRP21_W<'a> {
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
#[doc = "Field `DIRP22` reader - Selects pin direction for pin P0/1_n (bit 0 = P0/1_0, bit 1 = P0_1, ..., bit 31 = P0/1_31). 0 = input. 1 = output."]
pub struct DIRP22_R(crate::FieldReader<bool, bool>);
impl DIRP22_R {
    pub(crate) fn new(bits: bool) -> Self {
        DIRP22_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DIRP22_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DIRP22` writer - Selects pin direction for pin P0/1_n (bit 0 = P0/1_0, bit 1 = P0_1, ..., bit 31 = P0/1_31). 0 = input. 1 = output."]
pub struct DIRP22_W<'a> {
    w: &'a mut W,
}
impl<'a> DIRP22_W<'a> {
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
#[doc = "Field `DIRP23` reader - Selects pin direction for pin P0/1_n (bit 0 = P0/1_0, bit 1 = P0_1, ..., bit 31 = P0/1_31). 0 = input. 1 = output."]
pub struct DIRP23_R(crate::FieldReader<bool, bool>);
impl DIRP23_R {
    pub(crate) fn new(bits: bool) -> Self {
        DIRP23_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DIRP23_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DIRP23` writer - Selects pin direction for pin P0/1_n (bit 0 = P0/1_0, bit 1 = P0_1, ..., bit 31 = P0/1_31). 0 = input. 1 = output."]
pub struct DIRP23_W<'a> {
    w: &'a mut W,
}
impl<'a> DIRP23_W<'a> {
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
#[doc = "Field `DIRP24` reader - Selects pin direction for pin P0/1_n (bit 0 = P0/1_0, bit 1 = P0_1, ..., bit 31 = P0/1_31). 0 = input. 1 = output."]
pub struct DIRP24_R(crate::FieldReader<bool, bool>);
impl DIRP24_R {
    pub(crate) fn new(bits: bool) -> Self {
        DIRP24_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DIRP24_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DIRP24` writer - Selects pin direction for pin P0/1_n (bit 0 = P0/1_0, bit 1 = P0_1, ..., bit 31 = P0/1_31). 0 = input. 1 = output."]
pub struct DIRP24_W<'a> {
    w: &'a mut W,
}
impl<'a> DIRP24_W<'a> {
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
#[doc = "Field `DIRP25` reader - Selects pin direction for pin P0/1_n (bit 0 = P0/1_0, bit 1 = P0_1, ..., bit 31 = P0/1_31). 0 = input. 1 = output."]
pub struct DIRP25_R(crate::FieldReader<bool, bool>);
impl DIRP25_R {
    pub(crate) fn new(bits: bool) -> Self {
        DIRP25_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DIRP25_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DIRP25` writer - Selects pin direction for pin P0/1_n (bit 0 = P0/1_0, bit 1 = P0_1, ..., bit 31 = P0/1_31). 0 = input. 1 = output."]
pub struct DIRP25_W<'a> {
    w: &'a mut W,
}
impl<'a> DIRP25_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 25)) | ((value as u32 & 0x01) << 25);
        self.w
    }
}
#[doc = "Field `DIRP26` reader - Selects pin direction for pin P0/1_n (bit 0 = P0/1_0, bit 1 = P0_1, ..., bit 31 = P0/1_31). 0 = input. 1 = output."]
pub struct DIRP26_R(crate::FieldReader<bool, bool>);
impl DIRP26_R {
    pub(crate) fn new(bits: bool) -> Self {
        DIRP26_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DIRP26_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DIRP26` writer - Selects pin direction for pin P0/1_n (bit 0 = P0/1_0, bit 1 = P0_1, ..., bit 31 = P0/1_31). 0 = input. 1 = output."]
pub struct DIRP26_W<'a> {
    w: &'a mut W,
}
impl<'a> DIRP26_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 26)) | ((value as u32 & 0x01) << 26);
        self.w
    }
}
#[doc = "Field `DIRP27` reader - Selects pin direction for pin P0/1_n (bit 0 = P0/1_0, bit 1 = P0_1, ..., bit 31 = P0/1_31). 0 = input. 1 = output."]
pub struct DIRP27_R(crate::FieldReader<bool, bool>);
impl DIRP27_R {
    pub(crate) fn new(bits: bool) -> Self {
        DIRP27_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DIRP27_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DIRP27` writer - Selects pin direction for pin P0/1_n (bit 0 = P0/1_0, bit 1 = P0_1, ..., bit 31 = P0/1_31). 0 = input. 1 = output."]
pub struct DIRP27_W<'a> {
    w: &'a mut W,
}
impl<'a> DIRP27_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 27)) | ((value as u32 & 0x01) << 27);
        self.w
    }
}
#[doc = "Field `DIRP28` reader - Selects pin direction for pin P0/1_n (bit 0 = P0/1_0, bit 1 = P0_1, ..., bit 31 = P0/1_31). 0 = input. 1 = output."]
pub struct DIRP28_R(crate::FieldReader<bool, bool>);
impl DIRP28_R {
    pub(crate) fn new(bits: bool) -> Self {
        DIRP28_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DIRP28_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DIRP28` writer - Selects pin direction for pin P0/1_n (bit 0 = P0/1_0, bit 1 = P0_1, ..., bit 31 = P0/1_31). 0 = input. 1 = output."]
pub struct DIRP28_W<'a> {
    w: &'a mut W,
}
impl<'a> DIRP28_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 28)) | ((value as u32 & 0x01) << 28);
        self.w
    }
}
#[doc = "Field `DIRP29` reader - Selects pin direction for pin P0/1_n (bit 0 = P0/1_0, bit 1 = P0_1, ..., bit 31 = P0/1_31). 0 = input. 1 = output."]
pub struct DIRP29_R(crate::FieldReader<bool, bool>);
impl DIRP29_R {
    pub(crate) fn new(bits: bool) -> Self {
        DIRP29_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DIRP29_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DIRP29` writer - Selects pin direction for pin P0/1_n (bit 0 = P0/1_0, bit 1 = P0_1, ..., bit 31 = P0/1_31). 0 = input. 1 = output."]
pub struct DIRP29_W<'a> {
    w: &'a mut W,
}
impl<'a> DIRP29_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 29)) | ((value as u32 & 0x01) << 29);
        self.w
    }
}
#[doc = "Field `DIRP30` reader - Selects pin direction for pin P0/1_n (bit 0 = P0/1_0, bit 1 = P0_1, ..., bit 31 = P0/1_31). 0 = input. 1 = output."]
pub struct DIRP30_R(crate::FieldReader<bool, bool>);
impl DIRP30_R {
    pub(crate) fn new(bits: bool) -> Self {
        DIRP30_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DIRP30_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DIRP30` writer - Selects pin direction for pin P0/1_n (bit 0 = P0/1_0, bit 1 = P0_1, ..., bit 31 = P0/1_31). 0 = input. 1 = output."]
pub struct DIRP30_W<'a> {
    w: &'a mut W,
}
impl<'a> DIRP30_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 30)) | ((value as u32 & 0x01) << 30);
        self.w
    }
}
#[doc = "Field `DIRP31` reader - Selects pin direction for pin P0/1_n (bit 0 = P0/1_0, bit 1 = P0_1, ..., bit 31 = P0/1_31). 0 = input. 1 = output."]
pub struct DIRP31_R(crate::FieldReader<bool, bool>);
impl DIRP31_R {
    pub(crate) fn new(bits: bool) -> Self {
        DIRP31_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DIRP31_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DIRP31` writer - Selects pin direction for pin P0/1_n (bit 0 = P0/1_0, bit 1 = P0_1, ..., bit 31 = P0/1_31). 0 = input. 1 = output."]
pub struct DIRP31_W<'a> {
    w: &'a mut W,
}
impl<'a> DIRP31_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | ((value as u32 & 0x01) << 31);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Selects pin direction for pin P0/1_n (bit 0 = P0/1_0, bit 1 = P0_1, ..., bit 31 = P0/1_31). 0 = input. 1 = output."]
    #[inline(always)]
    pub fn dirp0(&self) -> DIRP0_R {
        DIRP0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Selects pin direction for pin P0/1_n (bit 0 = P0/1_0, bit 1 = P0_1, ..., bit 31 = P0/1_31). 0 = input. 1 = output."]
    #[inline(always)]
    pub fn dirp1(&self) -> DIRP1_R {
        DIRP1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Selects pin direction for pin P0/1_n (bit 0 = P0/1_0, bit 1 = P0_1, ..., bit 31 = P0/1_31). 0 = input. 1 = output."]
    #[inline(always)]
    pub fn dirp2(&self) -> DIRP2_R {
        DIRP2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Selects pin direction for pin P0/1_n (bit 0 = P0/1_0, bit 1 = P0_1, ..., bit 31 = P0/1_31). 0 = input. 1 = output."]
    #[inline(always)]
    pub fn dirp3(&self) -> DIRP3_R {
        DIRP3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Selects pin direction for pin P0/1_n (bit 0 = P0/1_0, bit 1 = P0_1, ..., bit 31 = P0/1_31). 0 = input. 1 = output."]
    #[inline(always)]
    pub fn dirp4(&self) -> DIRP4_R {
        DIRP4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Selects pin direction for pin P0/1_n (bit 0 = P0/1_0, bit 1 = P0_1, ..., bit 31 = P0/1_31). 0 = input. 1 = output."]
    #[inline(always)]
    pub fn dirp5(&self) -> DIRP5_R {
        DIRP5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Selects pin direction for pin P0/1_n (bit 0 = P0/1_0, bit 1 = P0_1, ..., bit 31 = P0/1_31). 0 = input. 1 = output."]
    #[inline(always)]
    pub fn dirp6(&self) -> DIRP6_R {
        DIRP6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Selects pin direction for pin P0/1_n (bit 0 = P0/1_0, bit 1 = P0_1, ..., bit 31 = P0/1_31). 0 = input. 1 = output."]
    #[inline(always)]
    pub fn dirp7(&self) -> DIRP7_R {
        DIRP7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Selects pin direction for pin P0/1_n (bit 0 = P0/1_0, bit 1 = P0_1, ..., bit 31 = P0/1_31). 0 = input. 1 = output."]
    #[inline(always)]
    pub fn dirp8(&self) -> DIRP8_R {
        DIRP8_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Selects pin direction for pin P0/1_n (bit 0 = P0/1_0, bit 1 = P0_1, ..., bit 31 = P0/1_31). 0 = input. 1 = output."]
    #[inline(always)]
    pub fn dirp9(&self) -> DIRP9_R {
        DIRP9_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Selects pin direction for pin P0/1_n (bit 0 = P0/1_0, bit 1 = P0_1, ..., bit 31 = P0/1_31). 0 = input. 1 = output."]
    #[inline(always)]
    pub fn dirp10(&self) -> DIRP10_R {
        DIRP10_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Selects pin direction for pin P0/1_n (bit 0 = P0/1_0, bit 1 = P0_1, ..., bit 31 = P0/1_31). 0 = input. 1 = output."]
    #[inline(always)]
    pub fn dirp11(&self) -> DIRP11_R {
        DIRP11_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Selects pin direction for pin P0/1_n (bit 0 = P0/1_0, bit 1 = P0_1, ..., bit 31 = P0/1_31). 0 = input. 1 = output."]
    #[inline(always)]
    pub fn dirp12(&self) -> DIRP12_R {
        DIRP12_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Selects pin direction for pin P0/1_n (bit 0 = P0/1_0, bit 1 = P0_1, ..., bit 31 = P0/1_31). 0 = input. 1 = output."]
    #[inline(always)]
    pub fn dirp13(&self) -> DIRP13_R {
        DIRP13_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Selects pin direction for pin P0/1_n (bit 0 = P0/1_0, bit 1 = P0_1, ..., bit 31 = P0/1_31). 0 = input. 1 = output."]
    #[inline(always)]
    pub fn dirp14(&self) -> DIRP14_R {
        DIRP14_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Selects pin direction for pin P0/1_n (bit 0 = P0/1_0, bit 1 = P0_1, ..., bit 31 = P0/1_31). 0 = input. 1 = output."]
    #[inline(always)]
    pub fn dirp15(&self) -> DIRP15_R {
        DIRP15_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Selects pin direction for pin P0/1_n (bit 0 = P0/1_0, bit 1 = P0_1, ..., bit 31 = P0/1_31). 0 = input. 1 = output."]
    #[inline(always)]
    pub fn dirp16(&self) -> DIRP16_R {
        DIRP16_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Selects pin direction for pin P0/1_n (bit 0 = P0/1_0, bit 1 = P0_1, ..., bit 31 = P0/1_31). 0 = input. 1 = output."]
    #[inline(always)]
    pub fn dirp17(&self) -> DIRP17_R {
        DIRP17_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Selects pin direction for pin P0/1_n (bit 0 = P0/1_0, bit 1 = P0_1, ..., bit 31 = P0/1_31). 0 = input. 1 = output."]
    #[inline(always)]
    pub fn dirp18(&self) -> DIRP18_R {
        DIRP18_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Selects pin direction for pin P0/1_n (bit 0 = P0/1_0, bit 1 = P0_1, ..., bit 31 = P0/1_31). 0 = input. 1 = output."]
    #[inline(always)]
    pub fn dirp19(&self) -> DIRP19_R {
        DIRP19_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Selects pin direction for pin P0/1_n (bit 0 = P0/1_0, bit 1 = P0_1, ..., bit 31 = P0/1_31). 0 = input. 1 = output."]
    #[inline(always)]
    pub fn dirp20(&self) -> DIRP20_R {
        DIRP20_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Selects pin direction for pin P0/1_n (bit 0 = P0/1_0, bit 1 = P0_1, ..., bit 31 = P0/1_31). 0 = input. 1 = output."]
    #[inline(always)]
    pub fn dirp21(&self) -> DIRP21_R {
        DIRP21_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Selects pin direction for pin P0/1_n (bit 0 = P0/1_0, bit 1 = P0_1, ..., bit 31 = P0/1_31). 0 = input. 1 = output."]
    #[inline(always)]
    pub fn dirp22(&self) -> DIRP22_R {
        DIRP22_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - Selects pin direction for pin P0/1_n (bit 0 = P0/1_0, bit 1 = P0_1, ..., bit 31 = P0/1_31). 0 = input. 1 = output."]
    #[inline(always)]
    pub fn dirp23(&self) -> DIRP23_R {
        DIRP23_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Selects pin direction for pin P0/1_n (bit 0 = P0/1_0, bit 1 = P0_1, ..., bit 31 = P0/1_31). 0 = input. 1 = output."]
    #[inline(always)]
    pub fn dirp24(&self) -> DIRP24_R {
        DIRP24_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Selects pin direction for pin P0/1_n (bit 0 = P0/1_0, bit 1 = P0_1, ..., bit 31 = P0/1_31). 0 = input. 1 = output."]
    #[inline(always)]
    pub fn dirp25(&self) -> DIRP25_R {
        DIRP25_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - Selects pin direction for pin P0/1_n (bit 0 = P0/1_0, bit 1 = P0_1, ..., bit 31 = P0/1_31). 0 = input. 1 = output."]
    #[inline(always)]
    pub fn dirp26(&self) -> DIRP26_R {
        DIRP26_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - Selects pin direction for pin P0/1_n (bit 0 = P0/1_0, bit 1 = P0_1, ..., bit 31 = P0/1_31). 0 = input. 1 = output."]
    #[inline(always)]
    pub fn dirp27(&self) -> DIRP27_R {
        DIRP27_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - Selects pin direction for pin P0/1_n (bit 0 = P0/1_0, bit 1 = P0_1, ..., bit 31 = P0/1_31). 0 = input. 1 = output."]
    #[inline(always)]
    pub fn dirp28(&self) -> DIRP28_R {
        DIRP28_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - Selects pin direction for pin P0/1_n (bit 0 = P0/1_0, bit 1 = P0_1, ..., bit 31 = P0/1_31). 0 = input. 1 = output."]
    #[inline(always)]
    pub fn dirp29(&self) -> DIRP29_R {
        DIRP29_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - Selects pin direction for pin P0/1_n (bit 0 = P0/1_0, bit 1 = P0_1, ..., bit 31 = P0/1_31). 0 = input. 1 = output."]
    #[inline(always)]
    pub fn dirp30(&self) -> DIRP30_R {
        DIRP30_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Selects pin direction for pin P0/1_n (bit 0 = P0/1_0, bit 1 = P0_1, ..., bit 31 = P0/1_31). 0 = input. 1 = output."]
    #[inline(always)]
    pub fn dirp31(&self) -> DIRP31_R {
        DIRP31_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Selects pin direction for pin P0/1_n (bit 0 = P0/1_0, bit 1 = P0_1, ..., bit 31 = P0/1_31). 0 = input. 1 = output."]
    #[inline(always)]
    pub fn dirp0(&mut self) -> DIRP0_W {
        DIRP0_W { w: self }
    }
    #[doc = "Bit 1 - Selects pin direction for pin P0/1_n (bit 0 = P0/1_0, bit 1 = P0_1, ..., bit 31 = P0/1_31). 0 = input. 1 = output."]
    #[inline(always)]
    pub fn dirp1(&mut self) -> DIRP1_W {
        DIRP1_W { w: self }
    }
    #[doc = "Bit 2 - Selects pin direction for pin P0/1_n (bit 0 = P0/1_0, bit 1 = P0_1, ..., bit 31 = P0/1_31). 0 = input. 1 = output."]
    #[inline(always)]
    pub fn dirp2(&mut self) -> DIRP2_W {
        DIRP2_W { w: self }
    }
    #[doc = "Bit 3 - Selects pin direction for pin P0/1_n (bit 0 = P0/1_0, bit 1 = P0_1, ..., bit 31 = P0/1_31). 0 = input. 1 = output."]
    #[inline(always)]
    pub fn dirp3(&mut self) -> DIRP3_W {
        DIRP3_W { w: self }
    }
    #[doc = "Bit 4 - Selects pin direction for pin P0/1_n (bit 0 = P0/1_0, bit 1 = P0_1, ..., bit 31 = P0/1_31). 0 = input. 1 = output."]
    #[inline(always)]
    pub fn dirp4(&mut self) -> DIRP4_W {
        DIRP4_W { w: self }
    }
    #[doc = "Bit 5 - Selects pin direction for pin P0/1_n (bit 0 = P0/1_0, bit 1 = P0_1, ..., bit 31 = P0/1_31). 0 = input. 1 = output."]
    #[inline(always)]
    pub fn dirp5(&mut self) -> DIRP5_W {
        DIRP5_W { w: self }
    }
    #[doc = "Bit 6 - Selects pin direction for pin P0/1_n (bit 0 = P0/1_0, bit 1 = P0_1, ..., bit 31 = P0/1_31). 0 = input. 1 = output."]
    #[inline(always)]
    pub fn dirp6(&mut self) -> DIRP6_W {
        DIRP6_W { w: self }
    }
    #[doc = "Bit 7 - Selects pin direction for pin P0/1_n (bit 0 = P0/1_0, bit 1 = P0_1, ..., bit 31 = P0/1_31). 0 = input. 1 = output."]
    #[inline(always)]
    pub fn dirp7(&mut self) -> DIRP7_W {
        DIRP7_W { w: self }
    }
    #[doc = "Bit 8 - Selects pin direction for pin P0/1_n (bit 0 = P0/1_0, bit 1 = P0_1, ..., bit 31 = P0/1_31). 0 = input. 1 = output."]
    #[inline(always)]
    pub fn dirp8(&mut self) -> DIRP8_W {
        DIRP8_W { w: self }
    }
    #[doc = "Bit 9 - Selects pin direction for pin P0/1_n (bit 0 = P0/1_0, bit 1 = P0_1, ..., bit 31 = P0/1_31). 0 = input. 1 = output."]
    #[inline(always)]
    pub fn dirp9(&mut self) -> DIRP9_W {
        DIRP9_W { w: self }
    }
    #[doc = "Bit 10 - Selects pin direction for pin P0/1_n (bit 0 = P0/1_0, bit 1 = P0_1, ..., bit 31 = P0/1_31). 0 = input. 1 = output."]
    #[inline(always)]
    pub fn dirp10(&mut self) -> DIRP10_W {
        DIRP10_W { w: self }
    }
    #[doc = "Bit 11 - Selects pin direction for pin P0/1_n (bit 0 = P0/1_0, bit 1 = P0_1, ..., bit 31 = P0/1_31). 0 = input. 1 = output."]
    #[inline(always)]
    pub fn dirp11(&mut self) -> DIRP11_W {
        DIRP11_W { w: self }
    }
    #[doc = "Bit 12 - Selects pin direction for pin P0/1_n (bit 0 = P0/1_0, bit 1 = P0_1, ..., bit 31 = P0/1_31). 0 = input. 1 = output."]
    #[inline(always)]
    pub fn dirp12(&mut self) -> DIRP12_W {
        DIRP12_W { w: self }
    }
    #[doc = "Bit 13 - Selects pin direction for pin P0/1_n (bit 0 = P0/1_0, bit 1 = P0_1, ..., bit 31 = P0/1_31). 0 = input. 1 = output."]
    #[inline(always)]
    pub fn dirp13(&mut self) -> DIRP13_W {
        DIRP13_W { w: self }
    }
    #[doc = "Bit 14 - Selects pin direction for pin P0/1_n (bit 0 = P0/1_0, bit 1 = P0_1, ..., bit 31 = P0/1_31). 0 = input. 1 = output."]
    #[inline(always)]
    pub fn dirp14(&mut self) -> DIRP14_W {
        DIRP14_W { w: self }
    }
    #[doc = "Bit 15 - Selects pin direction for pin P0/1_n (bit 0 = P0/1_0, bit 1 = P0_1, ..., bit 31 = P0/1_31). 0 = input. 1 = output."]
    #[inline(always)]
    pub fn dirp15(&mut self) -> DIRP15_W {
        DIRP15_W { w: self }
    }
    #[doc = "Bit 16 - Selects pin direction for pin P0/1_n (bit 0 = P0/1_0, bit 1 = P0_1, ..., bit 31 = P0/1_31). 0 = input. 1 = output."]
    #[inline(always)]
    pub fn dirp16(&mut self) -> DIRP16_W {
        DIRP16_W { w: self }
    }
    #[doc = "Bit 17 - Selects pin direction for pin P0/1_n (bit 0 = P0/1_0, bit 1 = P0_1, ..., bit 31 = P0/1_31). 0 = input. 1 = output."]
    #[inline(always)]
    pub fn dirp17(&mut self) -> DIRP17_W {
        DIRP17_W { w: self }
    }
    #[doc = "Bit 18 - Selects pin direction for pin P0/1_n (bit 0 = P0/1_0, bit 1 = P0_1, ..., bit 31 = P0/1_31). 0 = input. 1 = output."]
    #[inline(always)]
    pub fn dirp18(&mut self) -> DIRP18_W {
        DIRP18_W { w: self }
    }
    #[doc = "Bit 19 - Selects pin direction for pin P0/1_n (bit 0 = P0/1_0, bit 1 = P0_1, ..., bit 31 = P0/1_31). 0 = input. 1 = output."]
    #[inline(always)]
    pub fn dirp19(&mut self) -> DIRP19_W {
        DIRP19_W { w: self }
    }
    #[doc = "Bit 20 - Selects pin direction for pin P0/1_n (bit 0 = P0/1_0, bit 1 = P0_1, ..., bit 31 = P0/1_31). 0 = input. 1 = output."]
    #[inline(always)]
    pub fn dirp20(&mut self) -> DIRP20_W {
        DIRP20_W { w: self }
    }
    #[doc = "Bit 21 - Selects pin direction for pin P0/1_n (bit 0 = P0/1_0, bit 1 = P0_1, ..., bit 31 = P0/1_31). 0 = input. 1 = output."]
    #[inline(always)]
    pub fn dirp21(&mut self) -> DIRP21_W {
        DIRP21_W { w: self }
    }
    #[doc = "Bit 22 - Selects pin direction for pin P0/1_n (bit 0 = P0/1_0, bit 1 = P0_1, ..., bit 31 = P0/1_31). 0 = input. 1 = output."]
    #[inline(always)]
    pub fn dirp22(&mut self) -> DIRP22_W {
        DIRP22_W { w: self }
    }
    #[doc = "Bit 23 - Selects pin direction for pin P0/1_n (bit 0 = P0/1_0, bit 1 = P0_1, ..., bit 31 = P0/1_31). 0 = input. 1 = output."]
    #[inline(always)]
    pub fn dirp23(&mut self) -> DIRP23_W {
        DIRP23_W { w: self }
    }
    #[doc = "Bit 24 - Selects pin direction for pin P0/1_n (bit 0 = P0/1_0, bit 1 = P0_1, ..., bit 31 = P0/1_31). 0 = input. 1 = output."]
    #[inline(always)]
    pub fn dirp24(&mut self) -> DIRP24_W {
        DIRP24_W { w: self }
    }
    #[doc = "Bit 25 - Selects pin direction for pin P0/1_n (bit 0 = P0/1_0, bit 1 = P0_1, ..., bit 31 = P0/1_31). 0 = input. 1 = output."]
    #[inline(always)]
    pub fn dirp25(&mut self) -> DIRP25_W {
        DIRP25_W { w: self }
    }
    #[doc = "Bit 26 - Selects pin direction for pin P0/1_n (bit 0 = P0/1_0, bit 1 = P0_1, ..., bit 31 = P0/1_31). 0 = input. 1 = output."]
    #[inline(always)]
    pub fn dirp26(&mut self) -> DIRP26_W {
        DIRP26_W { w: self }
    }
    #[doc = "Bit 27 - Selects pin direction for pin P0/1_n (bit 0 = P0/1_0, bit 1 = P0_1, ..., bit 31 = P0/1_31). 0 = input. 1 = output."]
    #[inline(always)]
    pub fn dirp27(&mut self) -> DIRP27_W {
        DIRP27_W { w: self }
    }
    #[doc = "Bit 28 - Selects pin direction for pin P0/1_n (bit 0 = P0/1_0, bit 1 = P0_1, ..., bit 31 = P0/1_31). 0 = input. 1 = output."]
    #[inline(always)]
    pub fn dirp28(&mut self) -> DIRP28_W {
        DIRP28_W { w: self }
    }
    #[doc = "Bit 29 - Selects pin direction for pin P0/1_n (bit 0 = P0/1_0, bit 1 = P0_1, ..., bit 31 = P0/1_31). 0 = input. 1 = output."]
    #[inline(always)]
    pub fn dirp29(&mut self) -> DIRP29_W {
        DIRP29_W { w: self }
    }
    #[doc = "Bit 30 - Selects pin direction for pin P0/1_n (bit 0 = P0/1_0, bit 1 = P0_1, ..., bit 31 = P0/1_31). 0 = input. 1 = output."]
    #[inline(always)]
    pub fn dirp30(&mut self) -> DIRP30_W {
        DIRP30_W { w: self }
    }
    #[doc = "Bit 31 - Selects pin direction for pin P0/1_n (bit 0 = P0/1_0, bit 1 = P0_1, ..., bit 31 = P0/1_31). 0 = input. 1 = output."]
    #[inline(always)]
    pub fn dirp31(&mut self) -> DIRP31_W {
        DIRP31_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Direction registers port 0/1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dir](index.html) module"]
pub struct DIR_SPEC;
impl crate::RegisterSpec for DIR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dir::R](R) reader structure"]
impl crate::Readable for DIR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dir::W](W) writer structure"]
impl crate::Writable for DIR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DIR%s to value 0"]
impl crate::Resettable for DIR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
