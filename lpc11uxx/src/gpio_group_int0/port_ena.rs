#[doc = "Register `PORT_ENA%s` reader"]
pub struct R(crate::R<PORT_ENA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PORT_ENA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PORT_ENA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PORT_ENA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PORT_ENA%s` writer"]
pub struct W(crate::W<PORT_ENA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PORT_ENA_SPEC>;
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
impl From<crate::W<PORT_ENA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PORT_ENA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ENA_0` reader - Enable port 0/1 pin for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the port 0/1 pin is disabled and does not contribute to the grouped interrupt. 1 = the port 0/1 pin is enabled and contributes to the grouped interrupt."]
pub struct ENA_0_R(crate::FieldReader<bool, bool>);
impl ENA_0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ENA_0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ENA_0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ENA_0` writer - Enable port 0/1 pin for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the port 0/1 pin is disabled and does not contribute to the grouped interrupt. 1 = the port 0/1 pin is enabled and contributes to the grouped interrupt."]
pub struct ENA_0_W<'a> {
    w: &'a mut W,
}
impl<'a> ENA_0_W<'a> {
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
#[doc = "Field `ENA_1` reader - Enable port 0/1 pin for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the port 0/1 pin is disabled and does not contribute to the grouped interrupt. 1 = the port 0/1 pin is enabled and contributes to the grouped interrupt."]
pub struct ENA_1_R(crate::FieldReader<bool, bool>);
impl ENA_1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ENA_1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ENA_1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ENA_1` writer - Enable port 0/1 pin for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the port 0/1 pin is disabled and does not contribute to the grouped interrupt. 1 = the port 0/1 pin is enabled and contributes to the grouped interrupt."]
pub struct ENA_1_W<'a> {
    w: &'a mut W,
}
impl<'a> ENA_1_W<'a> {
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
#[doc = "Field `ENA_2` reader - Enable port 0/1 pin for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the port 0/1 pin is disabled and does not contribute to the grouped interrupt. 1 = the port 0/1 pin is enabled and contributes to the grouped interrupt."]
pub struct ENA_2_R(crate::FieldReader<bool, bool>);
impl ENA_2_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ENA_2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ENA_2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ENA_2` writer - Enable port 0/1 pin for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the port 0/1 pin is disabled and does not contribute to the grouped interrupt. 1 = the port 0/1 pin is enabled and contributes to the grouped interrupt."]
pub struct ENA_2_W<'a> {
    w: &'a mut W,
}
impl<'a> ENA_2_W<'a> {
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
#[doc = "Field `ENA_3` reader - Enable port 0/1 pin for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the port 0/1 pin is disabled and does not contribute to the grouped interrupt. 1 = the port 0/1 pin is enabled and contributes to the grouped interrupt."]
pub struct ENA_3_R(crate::FieldReader<bool, bool>);
impl ENA_3_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ENA_3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ENA_3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ENA_3` writer - Enable port 0/1 pin for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the port 0/1 pin is disabled and does not contribute to the grouped interrupt. 1 = the port 0/1 pin is enabled and contributes to the grouped interrupt."]
pub struct ENA_3_W<'a> {
    w: &'a mut W,
}
impl<'a> ENA_3_W<'a> {
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
#[doc = "Field `ENA_4` reader - Enable port 0/1 pin for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the port 0/1 pin is disabled and does not contribute to the grouped interrupt. 1 = the port 0/1 pin is enabled and contributes to the grouped interrupt."]
pub struct ENA_4_R(crate::FieldReader<bool, bool>);
impl ENA_4_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ENA_4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ENA_4_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ENA_4` writer - Enable port 0/1 pin for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the port 0/1 pin is disabled and does not contribute to the grouped interrupt. 1 = the port 0/1 pin is enabled and contributes to the grouped interrupt."]
pub struct ENA_4_W<'a> {
    w: &'a mut W,
}
impl<'a> ENA_4_W<'a> {
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
#[doc = "Field `ENA_5` reader - Enable port 0/1 pin for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the port 0/1 pin is disabled and does not contribute to the grouped interrupt. 1 = the port 0/1 pin is enabled and contributes to the grouped interrupt."]
pub struct ENA_5_R(crate::FieldReader<bool, bool>);
impl ENA_5_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ENA_5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ENA_5_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ENA_5` writer - Enable port 0/1 pin for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the port 0/1 pin is disabled and does not contribute to the grouped interrupt. 1 = the port 0/1 pin is enabled and contributes to the grouped interrupt."]
pub struct ENA_5_W<'a> {
    w: &'a mut W,
}
impl<'a> ENA_5_W<'a> {
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
#[doc = "Field `ENA_6` reader - Enable port 0/1 pin for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the port 0/1 pin is disabled and does not contribute to the grouped interrupt. 1 = the port 0/1 pin is enabled and contributes to the grouped interrupt."]
pub struct ENA_6_R(crate::FieldReader<bool, bool>);
impl ENA_6_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ENA_6_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ENA_6_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ENA_6` writer - Enable port 0/1 pin for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the port 0/1 pin is disabled and does not contribute to the grouped interrupt. 1 = the port 0/1 pin is enabled and contributes to the grouped interrupt."]
pub struct ENA_6_W<'a> {
    w: &'a mut W,
}
impl<'a> ENA_6_W<'a> {
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
#[doc = "Field `ENA_7` reader - Enable port 0/1 pin for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the port 0/1 pin is disabled and does not contribute to the grouped interrupt. 1 = the port 0/1 pin is enabled and contributes to the grouped interrupt."]
pub struct ENA_7_R(crate::FieldReader<bool, bool>);
impl ENA_7_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ENA_7_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ENA_7_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ENA_7` writer - Enable port 0/1 pin for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the port 0/1 pin is disabled and does not contribute to the grouped interrupt. 1 = the port 0/1 pin is enabled and contributes to the grouped interrupt."]
pub struct ENA_7_W<'a> {
    w: &'a mut W,
}
impl<'a> ENA_7_W<'a> {
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
#[doc = "Field `ENA_8` reader - Enable port 0/1 pin for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the port 0/1 pin is disabled and does not contribute to the grouped interrupt. 1 = the port 0/1 pin is enabled and contributes to the grouped interrupt."]
pub struct ENA_8_R(crate::FieldReader<bool, bool>);
impl ENA_8_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ENA_8_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ENA_8_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ENA_8` writer - Enable port 0/1 pin for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the port 0/1 pin is disabled and does not contribute to the grouped interrupt. 1 = the port 0/1 pin is enabled and contributes to the grouped interrupt."]
pub struct ENA_8_W<'a> {
    w: &'a mut W,
}
impl<'a> ENA_8_W<'a> {
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
#[doc = "Field `ENA_9` reader - Enable port 0/1 pin for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the port 0/1 pin is disabled and does not contribute to the grouped interrupt. 1 = the port 0/1 pin is enabled and contributes to the grouped interrupt."]
pub struct ENA_9_R(crate::FieldReader<bool, bool>);
impl ENA_9_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ENA_9_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ENA_9_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ENA_9` writer - Enable port 0/1 pin for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the port 0/1 pin is disabled and does not contribute to the grouped interrupt. 1 = the port 0/1 pin is enabled and contributes to the grouped interrupt."]
pub struct ENA_9_W<'a> {
    w: &'a mut W,
}
impl<'a> ENA_9_W<'a> {
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
#[doc = "Field `ENA_10` reader - Enable port 0/1 pin for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the port 0/1 pin is disabled and does not contribute to the grouped interrupt. 1 = the port 0/1 pin is enabled and contributes to the grouped interrupt."]
pub struct ENA_10_R(crate::FieldReader<bool, bool>);
impl ENA_10_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ENA_10_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ENA_10_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ENA_10` writer - Enable port 0/1 pin for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the port 0/1 pin is disabled and does not contribute to the grouped interrupt. 1 = the port 0/1 pin is enabled and contributes to the grouped interrupt."]
pub struct ENA_10_W<'a> {
    w: &'a mut W,
}
impl<'a> ENA_10_W<'a> {
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
#[doc = "Field `ENA_11` reader - Enable port 0/1 pin for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the port 0/1 pin is disabled and does not contribute to the grouped interrupt. 1 = the port 0/1 pin is enabled and contributes to the grouped interrupt."]
pub struct ENA_11_R(crate::FieldReader<bool, bool>);
impl ENA_11_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ENA_11_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ENA_11_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ENA_11` writer - Enable port 0/1 pin for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the port 0/1 pin is disabled and does not contribute to the grouped interrupt. 1 = the port 0/1 pin is enabled and contributes to the grouped interrupt."]
pub struct ENA_11_W<'a> {
    w: &'a mut W,
}
impl<'a> ENA_11_W<'a> {
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
#[doc = "Field `ENA_12` reader - Enable port 0/1 pin for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the port 0/1 pin is disabled and does not contribute to the grouped interrupt. 1 = the port 0/1 pin is enabled and contributes to the grouped interrupt."]
pub struct ENA_12_R(crate::FieldReader<bool, bool>);
impl ENA_12_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ENA_12_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ENA_12_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ENA_12` writer - Enable port 0/1 pin for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the port 0/1 pin is disabled and does not contribute to the grouped interrupt. 1 = the port 0/1 pin is enabled and contributes to the grouped interrupt."]
pub struct ENA_12_W<'a> {
    w: &'a mut W,
}
impl<'a> ENA_12_W<'a> {
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
#[doc = "Field `ENA_13` reader - Enable port 0/1 pin for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the port 0/1 pin is disabled and does not contribute to the grouped interrupt. 1 = the port 0/1 pin is enabled and contributes to the grouped interrupt."]
pub struct ENA_13_R(crate::FieldReader<bool, bool>);
impl ENA_13_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ENA_13_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ENA_13_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ENA_13` writer - Enable port 0/1 pin for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the port 0/1 pin is disabled and does not contribute to the grouped interrupt. 1 = the port 0/1 pin is enabled and contributes to the grouped interrupt."]
pub struct ENA_13_W<'a> {
    w: &'a mut W,
}
impl<'a> ENA_13_W<'a> {
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
#[doc = "Field `ENA_14` reader - Enable port 0/1 pin for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the port 0/1 pin is disabled and does not contribute to the grouped interrupt. 1 = the port 0/1 pin is enabled and contributes to the grouped interrupt."]
pub struct ENA_14_R(crate::FieldReader<bool, bool>);
impl ENA_14_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ENA_14_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ENA_14_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ENA_14` writer - Enable port 0/1 pin for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the port 0/1 pin is disabled and does not contribute to the grouped interrupt. 1 = the port 0/1 pin is enabled and contributes to the grouped interrupt."]
pub struct ENA_14_W<'a> {
    w: &'a mut W,
}
impl<'a> ENA_14_W<'a> {
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
#[doc = "Field `ENA_15` reader - Enable port 0/1 pin for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the port 0/1 pin is disabled and does not contribute to the grouped interrupt. 1 = the port 0/1 pin is enabled and contributes to the grouped interrupt."]
pub struct ENA_15_R(crate::FieldReader<bool, bool>);
impl ENA_15_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ENA_15_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ENA_15_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ENA_15` writer - Enable port 0/1 pin for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the port 0/1 pin is disabled and does not contribute to the grouped interrupt. 1 = the port 0/1 pin is enabled and contributes to the grouped interrupt."]
pub struct ENA_15_W<'a> {
    w: &'a mut W,
}
impl<'a> ENA_15_W<'a> {
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
#[doc = "Field `ENA_16` reader - Enable port 0/1 pin for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the port 0/1 pin is disabled and does not contribute to the grouped interrupt. 1 = the port 0/1 pin is enabled and contributes to the grouped interrupt."]
pub struct ENA_16_R(crate::FieldReader<bool, bool>);
impl ENA_16_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ENA_16_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ENA_16_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ENA_16` writer - Enable port 0/1 pin for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the port 0/1 pin is disabled and does not contribute to the grouped interrupt. 1 = the port 0/1 pin is enabled and contributes to the grouped interrupt."]
pub struct ENA_16_W<'a> {
    w: &'a mut W,
}
impl<'a> ENA_16_W<'a> {
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
#[doc = "Field `ENA_17` reader - Enable port 0/1 pin for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the port 0/1 pin is disabled and does not contribute to the grouped interrupt. 1 = the port 0/1 pin is enabled and contributes to the grouped interrupt."]
pub struct ENA_17_R(crate::FieldReader<bool, bool>);
impl ENA_17_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ENA_17_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ENA_17_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ENA_17` writer - Enable port 0/1 pin for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the port 0/1 pin is disabled and does not contribute to the grouped interrupt. 1 = the port 0/1 pin is enabled and contributes to the grouped interrupt."]
pub struct ENA_17_W<'a> {
    w: &'a mut W,
}
impl<'a> ENA_17_W<'a> {
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
#[doc = "Field `ENA_18` reader - Enable port 0/1 pin for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the port 0/1 pin is disabled and does not contribute to the grouped interrupt. 1 = the port 0/1 pin is enabled and contributes to the grouped interrupt."]
pub struct ENA_18_R(crate::FieldReader<bool, bool>);
impl ENA_18_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ENA_18_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ENA_18_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ENA_18` writer - Enable port 0/1 pin for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the port 0/1 pin is disabled and does not contribute to the grouped interrupt. 1 = the port 0/1 pin is enabled and contributes to the grouped interrupt."]
pub struct ENA_18_W<'a> {
    w: &'a mut W,
}
impl<'a> ENA_18_W<'a> {
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
#[doc = "Field `ENA_19` reader - Enable port 0/1 pin for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the port 0/1 pin is disabled and does not contribute to the grouped interrupt. 1 = the port 0/1 pin is enabled and contributes to the grouped interrupt."]
pub struct ENA_19_R(crate::FieldReader<bool, bool>);
impl ENA_19_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ENA_19_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ENA_19_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ENA_19` writer - Enable port 0/1 pin for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the port 0/1 pin is disabled and does not contribute to the grouped interrupt. 1 = the port 0/1 pin is enabled and contributes to the grouped interrupt."]
pub struct ENA_19_W<'a> {
    w: &'a mut W,
}
impl<'a> ENA_19_W<'a> {
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
#[doc = "Field `ENA_20` reader - Enable port 0/1 pin for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the port 0/1 pin is disabled and does not contribute to the grouped interrupt. 1 = the port 0/1 pin is enabled and contributes to the grouped interrupt."]
pub struct ENA_20_R(crate::FieldReader<bool, bool>);
impl ENA_20_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ENA_20_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ENA_20_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ENA_20` writer - Enable port 0/1 pin for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the port 0/1 pin is disabled and does not contribute to the grouped interrupt. 1 = the port 0/1 pin is enabled and contributes to the grouped interrupt."]
pub struct ENA_20_W<'a> {
    w: &'a mut W,
}
impl<'a> ENA_20_W<'a> {
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
#[doc = "Field `ENA_21` reader - Enable port 0/1 pin for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the port 0/1 pin is disabled and does not contribute to the grouped interrupt. 1 = the port 0/1 pin is enabled and contributes to the grouped interrupt."]
pub struct ENA_21_R(crate::FieldReader<bool, bool>);
impl ENA_21_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ENA_21_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ENA_21_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ENA_21` writer - Enable port 0/1 pin for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the port 0/1 pin is disabled and does not contribute to the grouped interrupt. 1 = the port 0/1 pin is enabled and contributes to the grouped interrupt."]
pub struct ENA_21_W<'a> {
    w: &'a mut W,
}
impl<'a> ENA_21_W<'a> {
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
#[doc = "Field `ENA_22` reader - Enable port 0/1 pin for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the port 0/1 pin is disabled and does not contribute to the grouped interrupt. 1 = the port 0/1 pin is enabled and contributes to the grouped interrupt."]
pub struct ENA_22_R(crate::FieldReader<bool, bool>);
impl ENA_22_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ENA_22_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ENA_22_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ENA_22` writer - Enable port 0/1 pin for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the port 0/1 pin is disabled and does not contribute to the grouped interrupt. 1 = the port 0/1 pin is enabled and contributes to the grouped interrupt."]
pub struct ENA_22_W<'a> {
    w: &'a mut W,
}
impl<'a> ENA_22_W<'a> {
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
#[doc = "Field `ENA_23` reader - Enable port 0/1 pin for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the port 0/1 pin is disabled and does not contribute to the grouped interrupt. 1 = the port 0/1 pin is enabled and contributes to the grouped interrupt."]
pub struct ENA_23_R(crate::FieldReader<bool, bool>);
impl ENA_23_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ENA_23_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ENA_23_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ENA_23` writer - Enable port 0/1 pin for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the port 0/1 pin is disabled and does not contribute to the grouped interrupt. 1 = the port 0/1 pin is enabled and contributes to the grouped interrupt."]
pub struct ENA_23_W<'a> {
    w: &'a mut W,
}
impl<'a> ENA_23_W<'a> {
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
#[doc = "Field `ENA_24` reader - Enable port 0/1 pin for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the port 0/1 pin is disabled and does not contribute to the grouped interrupt. 1 = the port 0/1 pin is enabled and contributes to the grouped interrupt."]
pub struct ENA_24_R(crate::FieldReader<bool, bool>);
impl ENA_24_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ENA_24_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ENA_24_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ENA_24` writer - Enable port 0/1 pin for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the port 0/1 pin is disabled and does not contribute to the grouped interrupt. 1 = the port 0/1 pin is enabled and contributes to the grouped interrupt."]
pub struct ENA_24_W<'a> {
    w: &'a mut W,
}
impl<'a> ENA_24_W<'a> {
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
#[doc = "Field `ENA_25` reader - Enable port 0/1 pin for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the port 0/1 pin is disabled and does not contribute to the grouped interrupt. 1 = the port 0/1 pin is enabled and contributes to the grouped interrupt."]
pub struct ENA_25_R(crate::FieldReader<bool, bool>);
impl ENA_25_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ENA_25_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ENA_25_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ENA_25` writer - Enable port 0/1 pin for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the port 0/1 pin is disabled and does not contribute to the grouped interrupt. 1 = the port 0/1 pin is enabled and contributes to the grouped interrupt."]
pub struct ENA_25_W<'a> {
    w: &'a mut W,
}
impl<'a> ENA_25_W<'a> {
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
#[doc = "Field `ENA_26` reader - Enable port 0/1 pin for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the port 0/1 pin is disabled and does not contribute to the grouped interrupt. 1 = the port 0/1 pin is enabled and contributes to the grouped interrupt."]
pub struct ENA_26_R(crate::FieldReader<bool, bool>);
impl ENA_26_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ENA_26_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ENA_26_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ENA_26` writer - Enable port 0/1 pin for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the port 0/1 pin is disabled and does not contribute to the grouped interrupt. 1 = the port 0/1 pin is enabled and contributes to the grouped interrupt."]
pub struct ENA_26_W<'a> {
    w: &'a mut W,
}
impl<'a> ENA_26_W<'a> {
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
#[doc = "Field `ENA_27` reader - Enable port 0/1 pin for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the port 0/1 pin is disabled and does not contribute to the grouped interrupt. 1 = the port 0/1 pin is enabled and contributes to the grouped interrupt."]
pub struct ENA_27_R(crate::FieldReader<bool, bool>);
impl ENA_27_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ENA_27_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ENA_27_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ENA_27` writer - Enable port 0/1 pin for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the port 0/1 pin is disabled and does not contribute to the grouped interrupt. 1 = the port 0/1 pin is enabled and contributes to the grouped interrupt."]
pub struct ENA_27_W<'a> {
    w: &'a mut W,
}
impl<'a> ENA_27_W<'a> {
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
#[doc = "Field `ENA_28` reader - Enable port 0/1 pin for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the port 0/1 pin is disabled and does not contribute to the grouped interrupt. 1 = the port 0/1 pin is enabled and contributes to the grouped interrupt."]
pub struct ENA_28_R(crate::FieldReader<bool, bool>);
impl ENA_28_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ENA_28_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ENA_28_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ENA_28` writer - Enable port 0/1 pin for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the port 0/1 pin is disabled and does not contribute to the grouped interrupt. 1 = the port 0/1 pin is enabled and contributes to the grouped interrupt."]
pub struct ENA_28_W<'a> {
    w: &'a mut W,
}
impl<'a> ENA_28_W<'a> {
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
#[doc = "Field `ENA_29` reader - Enable port 0/1 pin for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the port 0/1 pin is disabled and does not contribute to the grouped interrupt. 1 = the port 0/1 pin is enabled and contributes to the grouped interrupt."]
pub struct ENA_29_R(crate::FieldReader<bool, bool>);
impl ENA_29_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ENA_29_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ENA_29_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ENA_29` writer - Enable port 0/1 pin for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the port 0/1 pin is disabled and does not contribute to the grouped interrupt. 1 = the port 0/1 pin is enabled and contributes to the grouped interrupt."]
pub struct ENA_29_W<'a> {
    w: &'a mut W,
}
impl<'a> ENA_29_W<'a> {
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
#[doc = "Field `ENA_30` reader - Enable port 0/1 pin for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the port 0/1 pin is disabled and does not contribute to the grouped interrupt. 1 = the port 0/1 pin is enabled and contributes to the grouped interrupt."]
pub struct ENA_30_R(crate::FieldReader<bool, bool>);
impl ENA_30_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ENA_30_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ENA_30_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ENA_30` writer - Enable port 0/1 pin for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the port 0/1 pin is disabled and does not contribute to the grouped interrupt. 1 = the port 0/1 pin is enabled and contributes to the grouped interrupt."]
pub struct ENA_30_W<'a> {
    w: &'a mut W,
}
impl<'a> ENA_30_W<'a> {
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
#[doc = "Field `ENA_31` reader - Enable port 0/1 pin for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the port 0/1 pin is disabled and does not contribute to the grouped interrupt. 1 = the port 0/1 pin is enabled and contributes to the grouped interrupt."]
pub struct ENA_31_R(crate::FieldReader<bool, bool>);
impl ENA_31_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ENA_31_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ENA_31_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ENA_31` writer - Enable port 0/1 pin for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the port 0/1 pin is disabled and does not contribute to the grouped interrupt. 1 = the port 0/1 pin is enabled and contributes to the grouped interrupt."]
pub struct ENA_31_W<'a> {
    w: &'a mut W,
}
impl<'a> ENA_31_W<'a> {
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
    #[doc = "Bit 0 - Enable port 0/1 pin for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the port 0/1 pin is disabled and does not contribute to the grouped interrupt. 1 = the port 0/1 pin is enabled and contributes to the grouped interrupt."]
    #[inline(always)]
    pub fn ena_0(&self) -> ENA_0_R {
        ENA_0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Enable port 0/1 pin for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the port 0/1 pin is disabled and does not contribute to the grouped interrupt. 1 = the port 0/1 pin is enabled and contributes to the grouped interrupt."]
    #[inline(always)]
    pub fn ena_1(&self) -> ENA_1_R {
        ENA_1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Enable port 0/1 pin for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the port 0/1 pin is disabled and does not contribute to the grouped interrupt. 1 = the port 0/1 pin is enabled and contributes to the grouped interrupt."]
    #[inline(always)]
    pub fn ena_2(&self) -> ENA_2_R {
        ENA_2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Enable port 0/1 pin for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the port 0/1 pin is disabled and does not contribute to the grouped interrupt. 1 = the port 0/1 pin is enabled and contributes to the grouped interrupt."]
    #[inline(always)]
    pub fn ena_3(&self) -> ENA_3_R {
        ENA_3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Enable port 0/1 pin for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the port 0/1 pin is disabled and does not contribute to the grouped interrupt. 1 = the port 0/1 pin is enabled and contributes to the grouped interrupt."]
    #[inline(always)]
    pub fn ena_4(&self) -> ENA_4_R {
        ENA_4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Enable port 0/1 pin for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the port 0/1 pin is disabled and does not contribute to the grouped interrupt. 1 = the port 0/1 pin is enabled and contributes to the grouped interrupt."]
    #[inline(always)]
    pub fn ena_5(&self) -> ENA_5_R {
        ENA_5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Enable port 0/1 pin for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the port 0/1 pin is disabled and does not contribute to the grouped interrupt. 1 = the port 0/1 pin is enabled and contributes to the grouped interrupt."]
    #[inline(always)]
    pub fn ena_6(&self) -> ENA_6_R {
        ENA_6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Enable port 0/1 pin for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the port 0/1 pin is disabled and does not contribute to the grouped interrupt. 1 = the port 0/1 pin is enabled and contributes to the grouped interrupt."]
    #[inline(always)]
    pub fn ena_7(&self) -> ENA_7_R {
        ENA_7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Enable port 0/1 pin for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the port 0/1 pin is disabled and does not contribute to the grouped interrupt. 1 = the port 0/1 pin is enabled and contributes to the grouped interrupt."]
    #[inline(always)]
    pub fn ena_8(&self) -> ENA_8_R {
        ENA_8_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Enable port 0/1 pin for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the port 0/1 pin is disabled and does not contribute to the grouped interrupt. 1 = the port 0/1 pin is enabled and contributes to the grouped interrupt."]
    #[inline(always)]
    pub fn ena_9(&self) -> ENA_9_R {
        ENA_9_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Enable port 0/1 pin for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the port 0/1 pin is disabled and does not contribute to the grouped interrupt. 1 = the port 0/1 pin is enabled and contributes to the grouped interrupt."]
    #[inline(always)]
    pub fn ena_10(&self) -> ENA_10_R {
        ENA_10_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Enable port 0/1 pin for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the port 0/1 pin is disabled and does not contribute to the grouped interrupt. 1 = the port 0/1 pin is enabled and contributes to the grouped interrupt."]
    #[inline(always)]
    pub fn ena_11(&self) -> ENA_11_R {
        ENA_11_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Enable port 0/1 pin for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the port 0/1 pin is disabled and does not contribute to the grouped interrupt. 1 = the port 0/1 pin is enabled and contributes to the grouped interrupt."]
    #[inline(always)]
    pub fn ena_12(&self) -> ENA_12_R {
        ENA_12_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Enable port 0/1 pin for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the port 0/1 pin is disabled and does not contribute to the grouped interrupt. 1 = the port 0/1 pin is enabled and contributes to the grouped interrupt."]
    #[inline(always)]
    pub fn ena_13(&self) -> ENA_13_R {
        ENA_13_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Enable port 0/1 pin for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the port 0/1 pin is disabled and does not contribute to the grouped interrupt. 1 = the port 0/1 pin is enabled and contributes to the grouped interrupt."]
    #[inline(always)]
    pub fn ena_14(&self) -> ENA_14_R {
        ENA_14_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Enable port 0/1 pin for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the port 0/1 pin is disabled and does not contribute to the grouped interrupt. 1 = the port 0/1 pin is enabled and contributes to the grouped interrupt."]
    #[inline(always)]
    pub fn ena_15(&self) -> ENA_15_R {
        ENA_15_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Enable port 0/1 pin for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the port 0/1 pin is disabled and does not contribute to the grouped interrupt. 1 = the port 0/1 pin is enabled and contributes to the grouped interrupt."]
    #[inline(always)]
    pub fn ena_16(&self) -> ENA_16_R {
        ENA_16_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Enable port 0/1 pin for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the port 0/1 pin is disabled and does not contribute to the grouped interrupt. 1 = the port 0/1 pin is enabled and contributes to the grouped interrupt."]
    #[inline(always)]
    pub fn ena_17(&self) -> ENA_17_R {
        ENA_17_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Enable port 0/1 pin for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the port 0/1 pin is disabled and does not contribute to the grouped interrupt. 1 = the port 0/1 pin is enabled and contributes to the grouped interrupt."]
    #[inline(always)]
    pub fn ena_18(&self) -> ENA_18_R {
        ENA_18_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Enable port 0/1 pin for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the port 0/1 pin is disabled and does not contribute to the grouped interrupt. 1 = the port 0/1 pin is enabled and contributes to the grouped interrupt."]
    #[inline(always)]
    pub fn ena_19(&self) -> ENA_19_R {
        ENA_19_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Enable port 0/1 pin for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the port 0/1 pin is disabled and does not contribute to the grouped interrupt. 1 = the port 0/1 pin is enabled and contributes to the grouped interrupt."]
    #[inline(always)]
    pub fn ena_20(&self) -> ENA_20_R {
        ENA_20_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Enable port 0/1 pin for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the port 0/1 pin is disabled and does not contribute to the grouped interrupt. 1 = the port 0/1 pin is enabled and contributes to the grouped interrupt."]
    #[inline(always)]
    pub fn ena_21(&self) -> ENA_21_R {
        ENA_21_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Enable port 0/1 pin for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the port 0/1 pin is disabled and does not contribute to the grouped interrupt. 1 = the port 0/1 pin is enabled and contributes to the grouped interrupt."]
    #[inline(always)]
    pub fn ena_22(&self) -> ENA_22_R {
        ENA_22_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - Enable port 0/1 pin for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the port 0/1 pin is disabled and does not contribute to the grouped interrupt. 1 = the port 0/1 pin is enabled and contributes to the grouped interrupt."]
    #[inline(always)]
    pub fn ena_23(&self) -> ENA_23_R {
        ENA_23_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Enable port 0/1 pin for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the port 0/1 pin is disabled and does not contribute to the grouped interrupt. 1 = the port 0/1 pin is enabled and contributes to the grouped interrupt."]
    #[inline(always)]
    pub fn ena_24(&self) -> ENA_24_R {
        ENA_24_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Enable port 0/1 pin for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the port 0/1 pin is disabled and does not contribute to the grouped interrupt. 1 = the port 0/1 pin is enabled and contributes to the grouped interrupt."]
    #[inline(always)]
    pub fn ena_25(&self) -> ENA_25_R {
        ENA_25_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - Enable port 0/1 pin for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the port 0/1 pin is disabled and does not contribute to the grouped interrupt. 1 = the port 0/1 pin is enabled and contributes to the grouped interrupt."]
    #[inline(always)]
    pub fn ena_26(&self) -> ENA_26_R {
        ENA_26_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - Enable port 0/1 pin for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the port 0/1 pin is disabled and does not contribute to the grouped interrupt. 1 = the port 0/1 pin is enabled and contributes to the grouped interrupt."]
    #[inline(always)]
    pub fn ena_27(&self) -> ENA_27_R {
        ENA_27_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - Enable port 0/1 pin for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the port 0/1 pin is disabled and does not contribute to the grouped interrupt. 1 = the port 0/1 pin is enabled and contributes to the grouped interrupt."]
    #[inline(always)]
    pub fn ena_28(&self) -> ENA_28_R {
        ENA_28_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - Enable port 0/1 pin for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the port 0/1 pin is disabled and does not contribute to the grouped interrupt. 1 = the port 0/1 pin is enabled and contributes to the grouped interrupt."]
    #[inline(always)]
    pub fn ena_29(&self) -> ENA_29_R {
        ENA_29_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - Enable port 0/1 pin for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the port 0/1 pin is disabled and does not contribute to the grouped interrupt. 1 = the port 0/1 pin is enabled and contributes to the grouped interrupt."]
    #[inline(always)]
    pub fn ena_30(&self) -> ENA_30_R {
        ENA_30_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Enable port 0/1 pin for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the port 0/1 pin is disabled and does not contribute to the grouped interrupt. 1 = the port 0/1 pin is enabled and contributes to the grouped interrupt."]
    #[inline(always)]
    pub fn ena_31(&self) -> ENA_31_R {
        ENA_31_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable port 0/1 pin for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the port 0/1 pin is disabled and does not contribute to the grouped interrupt. 1 = the port 0/1 pin is enabled and contributes to the grouped interrupt."]
    #[inline(always)]
    pub fn ena_0(&mut self) -> ENA_0_W {
        ENA_0_W { w: self }
    }
    #[doc = "Bit 1 - Enable port 0/1 pin for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the port 0/1 pin is disabled and does not contribute to the grouped interrupt. 1 = the port 0/1 pin is enabled and contributes to the grouped interrupt."]
    #[inline(always)]
    pub fn ena_1(&mut self) -> ENA_1_W {
        ENA_1_W { w: self }
    }
    #[doc = "Bit 2 - Enable port 0/1 pin for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the port 0/1 pin is disabled and does not contribute to the grouped interrupt. 1 = the port 0/1 pin is enabled and contributes to the grouped interrupt."]
    #[inline(always)]
    pub fn ena_2(&mut self) -> ENA_2_W {
        ENA_2_W { w: self }
    }
    #[doc = "Bit 3 - Enable port 0/1 pin for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the port 0/1 pin is disabled and does not contribute to the grouped interrupt. 1 = the port 0/1 pin is enabled and contributes to the grouped interrupt."]
    #[inline(always)]
    pub fn ena_3(&mut self) -> ENA_3_W {
        ENA_3_W { w: self }
    }
    #[doc = "Bit 4 - Enable port 0/1 pin for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the port 0/1 pin is disabled and does not contribute to the grouped interrupt. 1 = the port 0/1 pin is enabled and contributes to the grouped interrupt."]
    #[inline(always)]
    pub fn ena_4(&mut self) -> ENA_4_W {
        ENA_4_W { w: self }
    }
    #[doc = "Bit 5 - Enable port 0/1 pin for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the port 0/1 pin is disabled and does not contribute to the grouped interrupt. 1 = the port 0/1 pin is enabled and contributes to the grouped interrupt."]
    #[inline(always)]
    pub fn ena_5(&mut self) -> ENA_5_W {
        ENA_5_W { w: self }
    }
    #[doc = "Bit 6 - Enable port 0/1 pin for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the port 0/1 pin is disabled and does not contribute to the grouped interrupt. 1 = the port 0/1 pin is enabled and contributes to the grouped interrupt."]
    #[inline(always)]
    pub fn ena_6(&mut self) -> ENA_6_W {
        ENA_6_W { w: self }
    }
    #[doc = "Bit 7 - Enable port 0/1 pin for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the port 0/1 pin is disabled and does not contribute to the grouped interrupt. 1 = the port 0/1 pin is enabled and contributes to the grouped interrupt."]
    #[inline(always)]
    pub fn ena_7(&mut self) -> ENA_7_W {
        ENA_7_W { w: self }
    }
    #[doc = "Bit 8 - Enable port 0/1 pin for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the port 0/1 pin is disabled and does not contribute to the grouped interrupt. 1 = the port 0/1 pin is enabled and contributes to the grouped interrupt."]
    #[inline(always)]
    pub fn ena_8(&mut self) -> ENA_8_W {
        ENA_8_W { w: self }
    }
    #[doc = "Bit 9 - Enable port 0/1 pin for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the port 0/1 pin is disabled and does not contribute to the grouped interrupt. 1 = the port 0/1 pin is enabled and contributes to the grouped interrupt."]
    #[inline(always)]
    pub fn ena_9(&mut self) -> ENA_9_W {
        ENA_9_W { w: self }
    }
    #[doc = "Bit 10 - Enable port 0/1 pin for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the port 0/1 pin is disabled and does not contribute to the grouped interrupt. 1 = the port 0/1 pin is enabled and contributes to the grouped interrupt."]
    #[inline(always)]
    pub fn ena_10(&mut self) -> ENA_10_W {
        ENA_10_W { w: self }
    }
    #[doc = "Bit 11 - Enable port 0/1 pin for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the port 0/1 pin is disabled and does not contribute to the grouped interrupt. 1 = the port 0/1 pin is enabled and contributes to the grouped interrupt."]
    #[inline(always)]
    pub fn ena_11(&mut self) -> ENA_11_W {
        ENA_11_W { w: self }
    }
    #[doc = "Bit 12 - Enable port 0/1 pin for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the port 0/1 pin is disabled and does not contribute to the grouped interrupt. 1 = the port 0/1 pin is enabled and contributes to the grouped interrupt."]
    #[inline(always)]
    pub fn ena_12(&mut self) -> ENA_12_W {
        ENA_12_W { w: self }
    }
    #[doc = "Bit 13 - Enable port 0/1 pin for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the port 0/1 pin is disabled and does not contribute to the grouped interrupt. 1 = the port 0/1 pin is enabled and contributes to the grouped interrupt."]
    #[inline(always)]
    pub fn ena_13(&mut self) -> ENA_13_W {
        ENA_13_W { w: self }
    }
    #[doc = "Bit 14 - Enable port 0/1 pin for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the port 0/1 pin is disabled and does not contribute to the grouped interrupt. 1 = the port 0/1 pin is enabled and contributes to the grouped interrupt."]
    #[inline(always)]
    pub fn ena_14(&mut self) -> ENA_14_W {
        ENA_14_W { w: self }
    }
    #[doc = "Bit 15 - Enable port 0/1 pin for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the port 0/1 pin is disabled and does not contribute to the grouped interrupt. 1 = the port 0/1 pin is enabled and contributes to the grouped interrupt."]
    #[inline(always)]
    pub fn ena_15(&mut self) -> ENA_15_W {
        ENA_15_W { w: self }
    }
    #[doc = "Bit 16 - Enable port 0/1 pin for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the port 0/1 pin is disabled and does not contribute to the grouped interrupt. 1 = the port 0/1 pin is enabled and contributes to the grouped interrupt."]
    #[inline(always)]
    pub fn ena_16(&mut self) -> ENA_16_W {
        ENA_16_W { w: self }
    }
    #[doc = "Bit 17 - Enable port 0/1 pin for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the port 0/1 pin is disabled and does not contribute to the grouped interrupt. 1 = the port 0/1 pin is enabled and contributes to the grouped interrupt."]
    #[inline(always)]
    pub fn ena_17(&mut self) -> ENA_17_W {
        ENA_17_W { w: self }
    }
    #[doc = "Bit 18 - Enable port 0/1 pin for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the port 0/1 pin is disabled and does not contribute to the grouped interrupt. 1 = the port 0/1 pin is enabled and contributes to the grouped interrupt."]
    #[inline(always)]
    pub fn ena_18(&mut self) -> ENA_18_W {
        ENA_18_W { w: self }
    }
    #[doc = "Bit 19 - Enable port 0/1 pin for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the port 0/1 pin is disabled and does not contribute to the grouped interrupt. 1 = the port 0/1 pin is enabled and contributes to the grouped interrupt."]
    #[inline(always)]
    pub fn ena_19(&mut self) -> ENA_19_W {
        ENA_19_W { w: self }
    }
    #[doc = "Bit 20 - Enable port 0/1 pin for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the port 0/1 pin is disabled and does not contribute to the grouped interrupt. 1 = the port 0/1 pin is enabled and contributes to the grouped interrupt."]
    #[inline(always)]
    pub fn ena_20(&mut self) -> ENA_20_W {
        ENA_20_W { w: self }
    }
    #[doc = "Bit 21 - Enable port 0/1 pin for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the port 0/1 pin is disabled and does not contribute to the grouped interrupt. 1 = the port 0/1 pin is enabled and contributes to the grouped interrupt."]
    #[inline(always)]
    pub fn ena_21(&mut self) -> ENA_21_W {
        ENA_21_W { w: self }
    }
    #[doc = "Bit 22 - Enable port 0/1 pin for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the port 0/1 pin is disabled and does not contribute to the grouped interrupt. 1 = the port 0/1 pin is enabled and contributes to the grouped interrupt."]
    #[inline(always)]
    pub fn ena_22(&mut self) -> ENA_22_W {
        ENA_22_W { w: self }
    }
    #[doc = "Bit 23 - Enable port 0/1 pin for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the port 0/1 pin is disabled and does not contribute to the grouped interrupt. 1 = the port 0/1 pin is enabled and contributes to the grouped interrupt."]
    #[inline(always)]
    pub fn ena_23(&mut self) -> ENA_23_W {
        ENA_23_W { w: self }
    }
    #[doc = "Bit 24 - Enable port 0/1 pin for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the port 0/1 pin is disabled and does not contribute to the grouped interrupt. 1 = the port 0/1 pin is enabled and contributes to the grouped interrupt."]
    #[inline(always)]
    pub fn ena_24(&mut self) -> ENA_24_W {
        ENA_24_W { w: self }
    }
    #[doc = "Bit 25 - Enable port 0/1 pin for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the port 0/1 pin is disabled and does not contribute to the grouped interrupt. 1 = the port 0/1 pin is enabled and contributes to the grouped interrupt."]
    #[inline(always)]
    pub fn ena_25(&mut self) -> ENA_25_W {
        ENA_25_W { w: self }
    }
    #[doc = "Bit 26 - Enable port 0/1 pin for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the port 0/1 pin is disabled and does not contribute to the grouped interrupt. 1 = the port 0/1 pin is enabled and contributes to the grouped interrupt."]
    #[inline(always)]
    pub fn ena_26(&mut self) -> ENA_26_W {
        ENA_26_W { w: self }
    }
    #[doc = "Bit 27 - Enable port 0/1 pin for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the port 0/1 pin is disabled and does not contribute to the grouped interrupt. 1 = the port 0/1 pin is enabled and contributes to the grouped interrupt."]
    #[inline(always)]
    pub fn ena_27(&mut self) -> ENA_27_W {
        ENA_27_W { w: self }
    }
    #[doc = "Bit 28 - Enable port 0/1 pin for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the port 0/1 pin is disabled and does not contribute to the grouped interrupt. 1 = the port 0/1 pin is enabled and contributes to the grouped interrupt."]
    #[inline(always)]
    pub fn ena_28(&mut self) -> ENA_28_W {
        ENA_28_W { w: self }
    }
    #[doc = "Bit 29 - Enable port 0/1 pin for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the port 0/1 pin is disabled and does not contribute to the grouped interrupt. 1 = the port 0/1 pin is enabled and contributes to the grouped interrupt."]
    #[inline(always)]
    pub fn ena_29(&mut self) -> ENA_29_W {
        ENA_29_W { w: self }
    }
    #[doc = "Bit 30 - Enable port 0/1 pin for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the port 0/1 pin is disabled and does not contribute to the grouped interrupt. 1 = the port 0/1 pin is enabled and contributes to the grouped interrupt."]
    #[inline(always)]
    pub fn ena_30(&mut self) -> ENA_30_W {
        ENA_30_W { w: self }
    }
    #[doc = "Bit 31 - Enable port 0/1 pin for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the port 0/1 pin is disabled and does not contribute to the grouped interrupt. 1 = the port 0/1 pin is enabled and contributes to the grouped interrupt."]
    #[inline(always)]
    pub fn ena_31(&mut self) -> ENA_31_W {
        ENA_31_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GPIO grouped interrupt port 0/1 enable register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [port_ena](index.html) module"]
pub struct PORT_ENA_SPEC;
impl crate::RegisterSpec for PORT_ENA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [port_ena::R](R) reader structure"]
impl crate::Readable for PORT_ENA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [port_ena::W](W) writer structure"]
impl crate::Writable for PORT_ENA_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PORT_ENA%s to value 0"]
impl crate::Resettable for PORT_ENA_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
