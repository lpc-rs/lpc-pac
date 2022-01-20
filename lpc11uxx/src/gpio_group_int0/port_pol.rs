#[doc = "Register `PORT_POL%s` reader"]
pub struct R(crate::R<PORT_POL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PORT_POL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PORT_POL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PORT_POL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PORT_POL%s` writer"]
pub struct W(crate::W<PORT_POL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PORT_POL_SPEC>;
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
impl From<crate::W<PORT_POL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PORT_POL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `POL_0` reader - Configure pin polarity of port pins for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1 . 0 = the pin is active LOW. If the level on this pin is LOW, the pin contributes to the group interrupt. 1 = the pin is active HIGH. If the level on this pin is HIGH, the pin contributes to the group interrupt."]
pub struct POL_0_R(crate::FieldReader<bool, bool>);
impl POL_0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        POL_0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for POL_0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `POL_0` writer - Configure pin polarity of port pins for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1 . 0 = the pin is active LOW. If the level on this pin is LOW, the pin contributes to the group interrupt. 1 = the pin is active HIGH. If the level on this pin is HIGH, the pin contributes to the group interrupt."]
pub struct POL_0_W<'a> {
    w: &'a mut W,
}
impl<'a> POL_0_W<'a> {
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
#[doc = "Field `POL_1` reader - Configure pin polarity of port 0/1 pins for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the pin is active LOW. If the level on this pin is LOW, the pin contributes to the group interrupt. 1 = the pin is active HIGH. If the level on this pin is HIGH, the pin contributes to the group interrupt."]
pub struct POL_1_R(crate::FieldReader<bool, bool>);
impl POL_1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        POL_1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for POL_1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `POL_1` writer - Configure pin polarity of port 0/1 pins for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the pin is active LOW. If the level on this pin is LOW, the pin contributes to the group interrupt. 1 = the pin is active HIGH. If the level on this pin is HIGH, the pin contributes to the group interrupt."]
pub struct POL_1_W<'a> {
    w: &'a mut W,
}
impl<'a> POL_1_W<'a> {
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
#[doc = "Field `POL_2` reader - Configure pin polarity of port 0/1 pins for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the pin is active LOW. If the level on this pin is LOW, the pin contributes to the group interrupt. 1 = the pin is active HIGH. If the level on this pin is HIGH, the pin contributes to the group interrupt."]
pub struct POL_2_R(crate::FieldReader<bool, bool>);
impl POL_2_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        POL_2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for POL_2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `POL_2` writer - Configure pin polarity of port 0/1 pins for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the pin is active LOW. If the level on this pin is LOW, the pin contributes to the group interrupt. 1 = the pin is active HIGH. If the level on this pin is HIGH, the pin contributes to the group interrupt."]
pub struct POL_2_W<'a> {
    w: &'a mut W,
}
impl<'a> POL_2_W<'a> {
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
#[doc = "Field `POL_3` reader - Configure pin polarity of port 0/1 pins for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the pin is active LOW. If the level on this pin is LOW, the pin contributes to the group interrupt. 1 = the pin is active HIGH. If the level on this pin is HIGH, the pin contributes to the group interrupt."]
pub struct POL_3_R(crate::FieldReader<bool, bool>);
impl POL_3_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        POL_3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for POL_3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `POL_3` writer - Configure pin polarity of port 0/1 pins for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the pin is active LOW. If the level on this pin is LOW, the pin contributes to the group interrupt. 1 = the pin is active HIGH. If the level on this pin is HIGH, the pin contributes to the group interrupt."]
pub struct POL_3_W<'a> {
    w: &'a mut W,
}
impl<'a> POL_3_W<'a> {
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
#[doc = "Field `POL_4` reader - Configure pin polarity of port 0/1 pins for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the pin is active LOW. If the level on this pin is LOW, the pin contributes to the group interrupt. 1 = the pin is active HIGH. If the level on this pin is HIGH, the pin contributes to the group interrupt."]
pub struct POL_4_R(crate::FieldReader<bool, bool>);
impl POL_4_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        POL_4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for POL_4_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `POL_4` writer - Configure pin polarity of port 0/1 pins for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the pin is active LOW. If the level on this pin is LOW, the pin contributes to the group interrupt. 1 = the pin is active HIGH. If the level on this pin is HIGH, the pin contributes to the group interrupt."]
pub struct POL_4_W<'a> {
    w: &'a mut W,
}
impl<'a> POL_4_W<'a> {
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
#[doc = "Field `POL_5` reader - Configure pin polarity of port 0/1 pins for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the pin is active LOW. If the level on this pin is LOW, the pin contributes to the group interrupt. 1 = the pin is active HIGH. If the level on this pin is HIGH, the pin contributes to the group interrupt."]
pub struct POL_5_R(crate::FieldReader<bool, bool>);
impl POL_5_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        POL_5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for POL_5_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `POL_5` writer - Configure pin polarity of port 0/1 pins for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the pin is active LOW. If the level on this pin is LOW, the pin contributes to the group interrupt. 1 = the pin is active HIGH. If the level on this pin is HIGH, the pin contributes to the group interrupt."]
pub struct POL_5_W<'a> {
    w: &'a mut W,
}
impl<'a> POL_5_W<'a> {
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
#[doc = "Field `POL_6` reader - Configure pin polarity of port 0/1 pins for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the pin is active LOW. If the level on this pin is LOW, the pin contributes to the group interrupt. 1 = the pin is active HIGH. If the level on this pin is HIGH, the pin contributes to the group interrupt."]
pub struct POL_6_R(crate::FieldReader<bool, bool>);
impl POL_6_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        POL_6_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for POL_6_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `POL_6` writer - Configure pin polarity of port 0/1 pins for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the pin is active LOW. If the level on this pin is LOW, the pin contributes to the group interrupt. 1 = the pin is active HIGH. If the level on this pin is HIGH, the pin contributes to the group interrupt."]
pub struct POL_6_W<'a> {
    w: &'a mut W,
}
impl<'a> POL_6_W<'a> {
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
#[doc = "Field `POL_7` reader - Configure pin polarity of port 0/1 pins for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the pin is active LOW. If the level on this pin is LOW, the pin contributes to the group interrupt. 1 = the pin is active HIGH. If the level on this pin is HIGH, the pin contributes to the group interrupt."]
pub struct POL_7_R(crate::FieldReader<bool, bool>);
impl POL_7_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        POL_7_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for POL_7_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `POL_7` writer - Configure pin polarity of port 0/1 pins for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the pin is active LOW. If the level on this pin is LOW, the pin contributes to the group interrupt. 1 = the pin is active HIGH. If the level on this pin is HIGH, the pin contributes to the group interrupt."]
pub struct POL_7_W<'a> {
    w: &'a mut W,
}
impl<'a> POL_7_W<'a> {
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
#[doc = "Field `POL_8` reader - Configure pin polarity of port 0/1 pins for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the pin is active LOW. If the level on this pin is LOW, the pin contributes to the group interrupt. 1 = the pin is active HIGH. If the level on this pin is HIGH, the pin contributes to the group interrupt."]
pub struct POL_8_R(crate::FieldReader<bool, bool>);
impl POL_8_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        POL_8_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for POL_8_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `POL_8` writer - Configure pin polarity of port 0/1 pins for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the pin is active LOW. If the level on this pin is LOW, the pin contributes to the group interrupt. 1 = the pin is active HIGH. If the level on this pin is HIGH, the pin contributes to the group interrupt."]
pub struct POL_8_W<'a> {
    w: &'a mut W,
}
impl<'a> POL_8_W<'a> {
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
#[doc = "Field `POL_9` reader - Configure pin polarity of port 0/1 pins for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the pin is active LOW. If the level on this pin is LOW, the pin contributes to the group interrupt. 1 = the pin is active HIGH. If the level on this pin is HIGH, the pin contributes to the group interrupt."]
pub struct POL_9_R(crate::FieldReader<bool, bool>);
impl POL_9_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        POL_9_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for POL_9_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `POL_9` writer - Configure pin polarity of port 0/1 pins for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the pin is active LOW. If the level on this pin is LOW, the pin contributes to the group interrupt. 1 = the pin is active HIGH. If the level on this pin is HIGH, the pin contributes to the group interrupt."]
pub struct POL_9_W<'a> {
    w: &'a mut W,
}
impl<'a> POL_9_W<'a> {
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
#[doc = "Field `POL_10` reader - Configure pin polarity of port 0/1 pins for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the pin is active LOW. If the level on this pin is LOW, the pin contributes to the group interrupt. 1 = the pin is active HIGH. If the level on this pin is HIGH, the pin contributes to the group interrupt."]
pub struct POL_10_R(crate::FieldReader<bool, bool>);
impl POL_10_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        POL_10_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for POL_10_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `POL_10` writer - Configure pin polarity of port 0/1 pins for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the pin is active LOW. If the level on this pin is LOW, the pin contributes to the group interrupt. 1 = the pin is active HIGH. If the level on this pin is HIGH, the pin contributes to the group interrupt."]
pub struct POL_10_W<'a> {
    w: &'a mut W,
}
impl<'a> POL_10_W<'a> {
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
#[doc = "Field `POL_11` reader - Configure pin polarity of port 0/1 pins for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the pin is active LOW. If the level on this pin is LOW, the pin contributes to the group interrupt. 1 = the pin is active HIGH. If the level on this pin is HIGH, the pin contributes to the group interrupt."]
pub struct POL_11_R(crate::FieldReader<bool, bool>);
impl POL_11_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        POL_11_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for POL_11_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `POL_11` writer - Configure pin polarity of port 0/1 pins for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the pin is active LOW. If the level on this pin is LOW, the pin contributes to the group interrupt. 1 = the pin is active HIGH. If the level on this pin is HIGH, the pin contributes to the group interrupt."]
pub struct POL_11_W<'a> {
    w: &'a mut W,
}
impl<'a> POL_11_W<'a> {
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
#[doc = "Field `POL_12` reader - Configure pin polarity of port 0/1 pins for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the pin is active LOW. If the level on this pin is LOW, the pin contributes to the group interrupt. 1 = the pin is active HIGH. If the level on this pin is HIGH, the pin contributes to the group interrupt."]
pub struct POL_12_R(crate::FieldReader<bool, bool>);
impl POL_12_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        POL_12_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for POL_12_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `POL_12` writer - Configure pin polarity of port 0/1 pins for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the pin is active LOW. If the level on this pin is LOW, the pin contributes to the group interrupt. 1 = the pin is active HIGH. If the level on this pin is HIGH, the pin contributes to the group interrupt."]
pub struct POL_12_W<'a> {
    w: &'a mut W,
}
impl<'a> POL_12_W<'a> {
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
#[doc = "Field `POL_13` reader - Configure pin polarity of port 0/1 pins for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the pin is active LOW. If the level on this pin is LOW, the pin contributes to the group interrupt. 1 = the pin is active HIGH. If the level on this pin is HIGH, the pin contributes to the group interrupt."]
pub struct POL_13_R(crate::FieldReader<bool, bool>);
impl POL_13_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        POL_13_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for POL_13_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `POL_13` writer - Configure pin polarity of port 0/1 pins for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the pin is active LOW. If the level on this pin is LOW, the pin contributes to the group interrupt. 1 = the pin is active HIGH. If the level on this pin is HIGH, the pin contributes to the group interrupt."]
pub struct POL_13_W<'a> {
    w: &'a mut W,
}
impl<'a> POL_13_W<'a> {
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
#[doc = "Field `POL_14` reader - Configure pin polarity of port 0/1 pins for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the pin is active LOW. If the level on this pin is LOW, the pin contributes to the group interrupt. 1 = the pin is active HIGH. If the level on this pin is HIGH, the pin contributes to the group interrupt."]
pub struct POL_14_R(crate::FieldReader<bool, bool>);
impl POL_14_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        POL_14_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for POL_14_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `POL_14` writer - Configure pin polarity of port 0/1 pins for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the pin is active LOW. If the level on this pin is LOW, the pin contributes to the group interrupt. 1 = the pin is active HIGH. If the level on this pin is HIGH, the pin contributes to the group interrupt."]
pub struct POL_14_W<'a> {
    w: &'a mut W,
}
impl<'a> POL_14_W<'a> {
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
#[doc = "Field `POL_15` reader - Configure pin polarity of port 0/1 pins for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the pin is active LOW. If the level on this pin is LOW, the pin contributes to the group interrupt. 1 = the pin is active HIGH. If the level on this pin is HIGH, the pin contributes to the group interrupt."]
pub struct POL_15_R(crate::FieldReader<bool, bool>);
impl POL_15_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        POL_15_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for POL_15_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `POL_15` writer - Configure pin polarity of port 0/1 pins for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the pin is active LOW. If the level on this pin is LOW, the pin contributes to the group interrupt. 1 = the pin is active HIGH. If the level on this pin is HIGH, the pin contributes to the group interrupt."]
pub struct POL_15_W<'a> {
    w: &'a mut W,
}
impl<'a> POL_15_W<'a> {
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
#[doc = "Field `POL_16` reader - Configure pin polarity of port 0/1 pins for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the pin is active LOW. If the level on this pin is LOW, the pin contributes to the group interrupt. 1 = the pin is active HIGH. If the level on this pin is HIGH, the pin contributes to the group interrupt."]
pub struct POL_16_R(crate::FieldReader<bool, bool>);
impl POL_16_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        POL_16_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for POL_16_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `POL_16` writer - Configure pin polarity of port 0/1 pins for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the pin is active LOW. If the level on this pin is LOW, the pin contributes to the group interrupt. 1 = the pin is active HIGH. If the level on this pin is HIGH, the pin contributes to the group interrupt."]
pub struct POL_16_W<'a> {
    w: &'a mut W,
}
impl<'a> POL_16_W<'a> {
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
#[doc = "Field `POL_17` reader - Configure pin polarity of port 0/1 pins for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the pin is active LOW. If the level on this pin is LOW, the pin contributes to the group interrupt. 1 = the pin is active HIGH. If the level on this pin is HIGH, the pin contributes to the group interrupt."]
pub struct POL_17_R(crate::FieldReader<bool, bool>);
impl POL_17_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        POL_17_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for POL_17_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `POL_17` writer - Configure pin polarity of port 0/1 pins for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the pin is active LOW. If the level on this pin is LOW, the pin contributes to the group interrupt. 1 = the pin is active HIGH. If the level on this pin is HIGH, the pin contributes to the group interrupt."]
pub struct POL_17_W<'a> {
    w: &'a mut W,
}
impl<'a> POL_17_W<'a> {
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
#[doc = "Field `POL_18` reader - Configure pin polarity of port 0/1 pins for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the pin is active LOW. If the level on this pin is LOW, the pin contributes to the group interrupt. 1 = the pin is active HIGH. If the level on this pin is HIGH, the pin contributes to the group interrupt."]
pub struct POL_18_R(crate::FieldReader<bool, bool>);
impl POL_18_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        POL_18_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for POL_18_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `POL_18` writer - Configure pin polarity of port 0/1 pins for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the pin is active LOW. If the level on this pin is LOW, the pin contributes to the group interrupt. 1 = the pin is active HIGH. If the level on this pin is HIGH, the pin contributes to the group interrupt."]
pub struct POL_18_W<'a> {
    w: &'a mut W,
}
impl<'a> POL_18_W<'a> {
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
#[doc = "Field `POL_19` reader - Configure pin polarity of port 0/1 pins for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the pin is active LOW. If the level on this pin is LOW, the pin contributes to the group interrupt. 1 = the pin is active HIGH. If the level on this pin is HIGH, the pin contributes to the group interrupt."]
pub struct POL_19_R(crate::FieldReader<bool, bool>);
impl POL_19_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        POL_19_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for POL_19_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `POL_19` writer - Configure pin polarity of port 0/1 pins for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the pin is active LOW. If the level on this pin is LOW, the pin contributes to the group interrupt. 1 = the pin is active HIGH. If the level on this pin is HIGH, the pin contributes to the group interrupt."]
pub struct POL_19_W<'a> {
    w: &'a mut W,
}
impl<'a> POL_19_W<'a> {
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
#[doc = "Field `POL_20` reader - Configure pin polarity of port 0/1 pins for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the pin is active LOW. If the level on this pin is LOW, the pin contributes to the group interrupt. 1 = the pin is active HIGH. If the level on this pin is HIGH, the pin contributes to the group interrupt."]
pub struct POL_20_R(crate::FieldReader<bool, bool>);
impl POL_20_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        POL_20_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for POL_20_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `POL_20` writer - Configure pin polarity of port 0/1 pins for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the pin is active LOW. If the level on this pin is LOW, the pin contributes to the group interrupt. 1 = the pin is active HIGH. If the level on this pin is HIGH, the pin contributes to the group interrupt."]
pub struct POL_20_W<'a> {
    w: &'a mut W,
}
impl<'a> POL_20_W<'a> {
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
#[doc = "Field `POL_21` reader - Configure pin polarity of port 0/1 pins for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the pin is active LOW. If the level on this pin is LOW, the pin contributes to the group interrupt. 1 = the pin is active HIGH. If the level on this pin is HIGH, the pin contributes to the group interrupt."]
pub struct POL_21_R(crate::FieldReader<bool, bool>);
impl POL_21_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        POL_21_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for POL_21_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `POL_21` writer - Configure pin polarity of port 0/1 pins for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the pin is active LOW. If the level on this pin is LOW, the pin contributes to the group interrupt. 1 = the pin is active HIGH. If the level on this pin is HIGH, the pin contributes to the group interrupt."]
pub struct POL_21_W<'a> {
    w: &'a mut W,
}
impl<'a> POL_21_W<'a> {
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
#[doc = "Field `POL_22` reader - Configure pin polarity of port 0/1 pins for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the pin is active LOW. If the level on this pin is LOW, the pin contributes to the group interrupt. 1 = the pin is active HIGH. If the level on this pin is HIGH, the pin contributes to the group interrupt."]
pub struct POL_22_R(crate::FieldReader<bool, bool>);
impl POL_22_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        POL_22_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for POL_22_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `POL_22` writer - Configure pin polarity of port 0/1 pins for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the pin is active LOW. If the level on this pin is LOW, the pin contributes to the group interrupt. 1 = the pin is active HIGH. If the level on this pin is HIGH, the pin contributes to the group interrupt."]
pub struct POL_22_W<'a> {
    w: &'a mut W,
}
impl<'a> POL_22_W<'a> {
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
#[doc = "Field `POL_23` reader - Configure pin polarity of port 0/1 pins for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the pin is active LOW. If the level on this pin is LOW, the pin contributes to the group interrupt. 1 = the pin is active HIGH. If the level on this pin is HIGH, the pin contributes to the group interrupt."]
pub struct POL_23_R(crate::FieldReader<bool, bool>);
impl POL_23_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        POL_23_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for POL_23_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `POL_23` writer - Configure pin polarity of port 0/1 pins for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the pin is active LOW. If the level on this pin is LOW, the pin contributes to the group interrupt. 1 = the pin is active HIGH. If the level on this pin is HIGH, the pin contributes to the group interrupt."]
pub struct POL_23_W<'a> {
    w: &'a mut W,
}
impl<'a> POL_23_W<'a> {
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
#[doc = "Field `POL_24` reader - Configure pin polarity of port 0/1 pins for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the pin is active LOW. If the level on this pin is LOW, the pin contributes to the group interrupt. 1 = the pin is active HIGH. If the level on this pin is HIGH, the pin contributes to the group interrupt."]
pub struct POL_24_R(crate::FieldReader<bool, bool>);
impl POL_24_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        POL_24_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for POL_24_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `POL_24` writer - Configure pin polarity of port 0/1 pins for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the pin is active LOW. If the level on this pin is LOW, the pin contributes to the group interrupt. 1 = the pin is active HIGH. If the level on this pin is HIGH, the pin contributes to the group interrupt."]
pub struct POL_24_W<'a> {
    w: &'a mut W,
}
impl<'a> POL_24_W<'a> {
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
#[doc = "Field `POL_25` reader - Configure pin polarity of port 0/1 pins for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the pin is active LOW. If the level on this pin is LOW, the pin contributes to the group interrupt. 1 = the pin is active HIGH. If the level on this pin is HIGH, the pin contributes to the group interrupt."]
pub struct POL_25_R(crate::FieldReader<bool, bool>);
impl POL_25_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        POL_25_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for POL_25_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `POL_25` writer - Configure pin polarity of port 0/1 pins for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the pin is active LOW. If the level on this pin is LOW, the pin contributes to the group interrupt. 1 = the pin is active HIGH. If the level on this pin is HIGH, the pin contributes to the group interrupt."]
pub struct POL_25_W<'a> {
    w: &'a mut W,
}
impl<'a> POL_25_W<'a> {
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
#[doc = "Field `POL_26` reader - Configure pin polarity of port 0/1 pins for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the pin is active LOW. If the level on this pin is LOW, the pin contributes to the group interrupt. 1 = the pin is active HIGH. If the level on this pin is HIGH, the pin contributes to the group interrupt."]
pub struct POL_26_R(crate::FieldReader<bool, bool>);
impl POL_26_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        POL_26_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for POL_26_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `POL_26` writer - Configure pin polarity of port 0/1 pins for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the pin is active LOW. If the level on this pin is LOW, the pin contributes to the group interrupt. 1 = the pin is active HIGH. If the level on this pin is HIGH, the pin contributes to the group interrupt."]
pub struct POL_26_W<'a> {
    w: &'a mut W,
}
impl<'a> POL_26_W<'a> {
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
#[doc = "Field `POL_27` reader - Configure pin polarity of port 0/1 pins for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the pin is active LOW. If the level on this pin is LOW, the pin contributes to the group interrupt. 1 = the pin is active HIGH. If the level on this pin is HIGH, the pin contributes to the group interrupt."]
pub struct POL_27_R(crate::FieldReader<bool, bool>);
impl POL_27_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        POL_27_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for POL_27_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `POL_27` writer - Configure pin polarity of port 0/1 pins for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the pin is active LOW. If the level on this pin is LOW, the pin contributes to the group interrupt. 1 = the pin is active HIGH. If the level on this pin is HIGH, the pin contributes to the group interrupt."]
pub struct POL_27_W<'a> {
    w: &'a mut W,
}
impl<'a> POL_27_W<'a> {
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
#[doc = "Field `POL_28` reader - Configure pin polarity of port 0/1 pins for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the pin is active LOW. If the level on this pin is LOW, the pin contributes to the group interrupt. 1 = the pin is active HIGH. If the level on this pin is HIGH, the pin contributes to the group interrupt."]
pub struct POL_28_R(crate::FieldReader<bool, bool>);
impl POL_28_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        POL_28_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for POL_28_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `POL_28` writer - Configure pin polarity of port 0/1 pins for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the pin is active LOW. If the level on this pin is LOW, the pin contributes to the group interrupt. 1 = the pin is active HIGH. If the level on this pin is HIGH, the pin contributes to the group interrupt."]
pub struct POL_28_W<'a> {
    w: &'a mut W,
}
impl<'a> POL_28_W<'a> {
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
#[doc = "Field `POL_29` reader - Configure pin polarity of port 0/1 pins for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the pin is active LOW. If the level on this pin is LOW, the pin contributes to the group interrupt. 1 = the pin is active HIGH. If the level on this pin is HIGH, the pin contributes to the group interrupt."]
pub struct POL_29_R(crate::FieldReader<bool, bool>);
impl POL_29_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        POL_29_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for POL_29_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `POL_29` writer - Configure pin polarity of port 0/1 pins for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the pin is active LOW. If the level on this pin is LOW, the pin contributes to the group interrupt. 1 = the pin is active HIGH. If the level on this pin is HIGH, the pin contributes to the group interrupt."]
pub struct POL_29_W<'a> {
    w: &'a mut W,
}
impl<'a> POL_29_W<'a> {
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
#[doc = "Field `POL_30` reader - Configure pin polarity of port 0/1 pins for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the pin is active LOW. If the level on this pin is LOW, the pin contributes to the group interrupt. 1 = the pin is active HIGH. If the level on this pin is HIGH, the pin contributes to the group interrupt."]
pub struct POL_30_R(crate::FieldReader<bool, bool>);
impl POL_30_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        POL_30_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for POL_30_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `POL_30` writer - Configure pin polarity of port 0/1 pins for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the pin is active LOW. If the level on this pin is LOW, the pin contributes to the group interrupt. 1 = the pin is active HIGH. If the level on this pin is HIGH, the pin contributes to the group interrupt."]
pub struct POL_30_W<'a> {
    w: &'a mut W,
}
impl<'a> POL_30_W<'a> {
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
#[doc = "Field `POL_31` reader - Configure pin polarity of port 0/1 pins for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the pin is active LOW. If the level on this pin is LOW, the pin contributes to the group interrupt. 1 = the pin is active HIGH. If the level on this pin is HIGH, the pin contributes to the group interrupt."]
pub struct POL_31_R(crate::FieldReader<bool, bool>);
impl POL_31_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        POL_31_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for POL_31_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `POL_31` writer - Configure pin polarity of port 0/1 pins for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the pin is active LOW. If the level on this pin is LOW, the pin contributes to the group interrupt. 1 = the pin is active HIGH. If the level on this pin is HIGH, the pin contributes to the group interrupt."]
pub struct POL_31_W<'a> {
    w: &'a mut W,
}
impl<'a> POL_31_W<'a> {
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
    #[doc = "Bit 0 - Configure pin polarity of port pins for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1 . 0 = the pin is active LOW. If the level on this pin is LOW, the pin contributes to the group interrupt. 1 = the pin is active HIGH. If the level on this pin is HIGH, the pin contributes to the group interrupt."]
    #[inline(always)]
    pub fn pol_0(&self) -> POL_0_R {
        POL_0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Configure pin polarity of port 0/1 pins for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the pin is active LOW. If the level on this pin is LOW, the pin contributes to the group interrupt. 1 = the pin is active HIGH. If the level on this pin is HIGH, the pin contributes to the group interrupt."]
    #[inline(always)]
    pub fn pol_1(&self) -> POL_1_R {
        POL_1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Configure pin polarity of port 0/1 pins for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the pin is active LOW. If the level on this pin is LOW, the pin contributes to the group interrupt. 1 = the pin is active HIGH. If the level on this pin is HIGH, the pin contributes to the group interrupt."]
    #[inline(always)]
    pub fn pol_2(&self) -> POL_2_R {
        POL_2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Configure pin polarity of port 0/1 pins for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the pin is active LOW. If the level on this pin is LOW, the pin contributes to the group interrupt. 1 = the pin is active HIGH. If the level on this pin is HIGH, the pin contributes to the group interrupt."]
    #[inline(always)]
    pub fn pol_3(&self) -> POL_3_R {
        POL_3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Configure pin polarity of port 0/1 pins for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the pin is active LOW. If the level on this pin is LOW, the pin contributes to the group interrupt. 1 = the pin is active HIGH. If the level on this pin is HIGH, the pin contributes to the group interrupt."]
    #[inline(always)]
    pub fn pol_4(&self) -> POL_4_R {
        POL_4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Configure pin polarity of port 0/1 pins for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the pin is active LOW. If the level on this pin is LOW, the pin contributes to the group interrupt. 1 = the pin is active HIGH. If the level on this pin is HIGH, the pin contributes to the group interrupt."]
    #[inline(always)]
    pub fn pol_5(&self) -> POL_5_R {
        POL_5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Configure pin polarity of port 0/1 pins for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the pin is active LOW. If the level on this pin is LOW, the pin contributes to the group interrupt. 1 = the pin is active HIGH. If the level on this pin is HIGH, the pin contributes to the group interrupt."]
    #[inline(always)]
    pub fn pol_6(&self) -> POL_6_R {
        POL_6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Configure pin polarity of port 0/1 pins for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the pin is active LOW. If the level on this pin is LOW, the pin contributes to the group interrupt. 1 = the pin is active HIGH. If the level on this pin is HIGH, the pin contributes to the group interrupt."]
    #[inline(always)]
    pub fn pol_7(&self) -> POL_7_R {
        POL_7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Configure pin polarity of port 0/1 pins for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the pin is active LOW. If the level on this pin is LOW, the pin contributes to the group interrupt. 1 = the pin is active HIGH. If the level on this pin is HIGH, the pin contributes to the group interrupt."]
    #[inline(always)]
    pub fn pol_8(&self) -> POL_8_R {
        POL_8_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Configure pin polarity of port 0/1 pins for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the pin is active LOW. If the level on this pin is LOW, the pin contributes to the group interrupt. 1 = the pin is active HIGH. If the level on this pin is HIGH, the pin contributes to the group interrupt."]
    #[inline(always)]
    pub fn pol_9(&self) -> POL_9_R {
        POL_9_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Configure pin polarity of port 0/1 pins for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the pin is active LOW. If the level on this pin is LOW, the pin contributes to the group interrupt. 1 = the pin is active HIGH. If the level on this pin is HIGH, the pin contributes to the group interrupt."]
    #[inline(always)]
    pub fn pol_10(&self) -> POL_10_R {
        POL_10_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Configure pin polarity of port 0/1 pins for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the pin is active LOW. If the level on this pin is LOW, the pin contributes to the group interrupt. 1 = the pin is active HIGH. If the level on this pin is HIGH, the pin contributes to the group interrupt."]
    #[inline(always)]
    pub fn pol_11(&self) -> POL_11_R {
        POL_11_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Configure pin polarity of port 0/1 pins for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the pin is active LOW. If the level on this pin is LOW, the pin contributes to the group interrupt. 1 = the pin is active HIGH. If the level on this pin is HIGH, the pin contributes to the group interrupt."]
    #[inline(always)]
    pub fn pol_12(&self) -> POL_12_R {
        POL_12_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Configure pin polarity of port 0/1 pins for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the pin is active LOW. If the level on this pin is LOW, the pin contributes to the group interrupt. 1 = the pin is active HIGH. If the level on this pin is HIGH, the pin contributes to the group interrupt."]
    #[inline(always)]
    pub fn pol_13(&self) -> POL_13_R {
        POL_13_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Configure pin polarity of port 0/1 pins for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the pin is active LOW. If the level on this pin is LOW, the pin contributes to the group interrupt. 1 = the pin is active HIGH. If the level on this pin is HIGH, the pin contributes to the group interrupt."]
    #[inline(always)]
    pub fn pol_14(&self) -> POL_14_R {
        POL_14_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Configure pin polarity of port 0/1 pins for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the pin is active LOW. If the level on this pin is LOW, the pin contributes to the group interrupt. 1 = the pin is active HIGH. If the level on this pin is HIGH, the pin contributes to the group interrupt."]
    #[inline(always)]
    pub fn pol_15(&self) -> POL_15_R {
        POL_15_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Configure pin polarity of port 0/1 pins for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the pin is active LOW. If the level on this pin is LOW, the pin contributes to the group interrupt. 1 = the pin is active HIGH. If the level on this pin is HIGH, the pin contributes to the group interrupt."]
    #[inline(always)]
    pub fn pol_16(&self) -> POL_16_R {
        POL_16_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Configure pin polarity of port 0/1 pins for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the pin is active LOW. If the level on this pin is LOW, the pin contributes to the group interrupt. 1 = the pin is active HIGH. If the level on this pin is HIGH, the pin contributes to the group interrupt."]
    #[inline(always)]
    pub fn pol_17(&self) -> POL_17_R {
        POL_17_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Configure pin polarity of port 0/1 pins for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the pin is active LOW. If the level on this pin is LOW, the pin contributes to the group interrupt. 1 = the pin is active HIGH. If the level on this pin is HIGH, the pin contributes to the group interrupt."]
    #[inline(always)]
    pub fn pol_18(&self) -> POL_18_R {
        POL_18_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Configure pin polarity of port 0/1 pins for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the pin is active LOW. If the level on this pin is LOW, the pin contributes to the group interrupt. 1 = the pin is active HIGH. If the level on this pin is HIGH, the pin contributes to the group interrupt."]
    #[inline(always)]
    pub fn pol_19(&self) -> POL_19_R {
        POL_19_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Configure pin polarity of port 0/1 pins for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the pin is active LOW. If the level on this pin is LOW, the pin contributes to the group interrupt. 1 = the pin is active HIGH. If the level on this pin is HIGH, the pin contributes to the group interrupt."]
    #[inline(always)]
    pub fn pol_20(&self) -> POL_20_R {
        POL_20_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Configure pin polarity of port 0/1 pins for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the pin is active LOW. If the level on this pin is LOW, the pin contributes to the group interrupt. 1 = the pin is active HIGH. If the level on this pin is HIGH, the pin contributes to the group interrupt."]
    #[inline(always)]
    pub fn pol_21(&self) -> POL_21_R {
        POL_21_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Configure pin polarity of port 0/1 pins for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the pin is active LOW. If the level on this pin is LOW, the pin contributes to the group interrupt. 1 = the pin is active HIGH. If the level on this pin is HIGH, the pin contributes to the group interrupt."]
    #[inline(always)]
    pub fn pol_22(&self) -> POL_22_R {
        POL_22_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - Configure pin polarity of port 0/1 pins for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the pin is active LOW. If the level on this pin is LOW, the pin contributes to the group interrupt. 1 = the pin is active HIGH. If the level on this pin is HIGH, the pin contributes to the group interrupt."]
    #[inline(always)]
    pub fn pol_23(&self) -> POL_23_R {
        POL_23_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Configure pin polarity of port 0/1 pins for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the pin is active LOW. If the level on this pin is LOW, the pin contributes to the group interrupt. 1 = the pin is active HIGH. If the level on this pin is HIGH, the pin contributes to the group interrupt."]
    #[inline(always)]
    pub fn pol_24(&self) -> POL_24_R {
        POL_24_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Configure pin polarity of port 0/1 pins for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the pin is active LOW. If the level on this pin is LOW, the pin contributes to the group interrupt. 1 = the pin is active HIGH. If the level on this pin is HIGH, the pin contributes to the group interrupt."]
    #[inline(always)]
    pub fn pol_25(&self) -> POL_25_R {
        POL_25_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - Configure pin polarity of port 0/1 pins for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the pin is active LOW. If the level on this pin is LOW, the pin contributes to the group interrupt. 1 = the pin is active HIGH. If the level on this pin is HIGH, the pin contributes to the group interrupt."]
    #[inline(always)]
    pub fn pol_26(&self) -> POL_26_R {
        POL_26_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - Configure pin polarity of port 0/1 pins for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the pin is active LOW. If the level on this pin is LOW, the pin contributes to the group interrupt. 1 = the pin is active HIGH. If the level on this pin is HIGH, the pin contributes to the group interrupt."]
    #[inline(always)]
    pub fn pol_27(&self) -> POL_27_R {
        POL_27_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - Configure pin polarity of port 0/1 pins for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the pin is active LOW. If the level on this pin is LOW, the pin contributes to the group interrupt. 1 = the pin is active HIGH. If the level on this pin is HIGH, the pin contributes to the group interrupt."]
    #[inline(always)]
    pub fn pol_28(&self) -> POL_28_R {
        POL_28_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - Configure pin polarity of port 0/1 pins for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the pin is active LOW. If the level on this pin is LOW, the pin contributes to the group interrupt. 1 = the pin is active HIGH. If the level on this pin is HIGH, the pin contributes to the group interrupt."]
    #[inline(always)]
    pub fn pol_29(&self) -> POL_29_R {
        POL_29_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - Configure pin polarity of port 0/1 pins for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the pin is active LOW. If the level on this pin is LOW, the pin contributes to the group interrupt. 1 = the pin is active HIGH. If the level on this pin is HIGH, the pin contributes to the group interrupt."]
    #[inline(always)]
    pub fn pol_30(&self) -> POL_30_R {
        POL_30_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Configure pin polarity of port 0/1 pins for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the pin is active LOW. If the level on this pin is LOW, the pin contributes to the group interrupt. 1 = the pin is active HIGH. If the level on this pin is HIGH, the pin contributes to the group interrupt."]
    #[inline(always)]
    pub fn pol_31(&self) -> POL_31_R {
        POL_31_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Configure pin polarity of port pins for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1 . 0 = the pin is active LOW. If the level on this pin is LOW, the pin contributes to the group interrupt. 1 = the pin is active HIGH. If the level on this pin is HIGH, the pin contributes to the group interrupt."]
    #[inline(always)]
    pub fn pol_0(&mut self) -> POL_0_W {
        POL_0_W { w: self }
    }
    #[doc = "Bit 1 - Configure pin polarity of port 0/1 pins for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the pin is active LOW. If the level on this pin is LOW, the pin contributes to the group interrupt. 1 = the pin is active HIGH. If the level on this pin is HIGH, the pin contributes to the group interrupt."]
    #[inline(always)]
    pub fn pol_1(&mut self) -> POL_1_W {
        POL_1_W { w: self }
    }
    #[doc = "Bit 2 - Configure pin polarity of port 0/1 pins for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the pin is active LOW. If the level on this pin is LOW, the pin contributes to the group interrupt. 1 = the pin is active HIGH. If the level on this pin is HIGH, the pin contributes to the group interrupt."]
    #[inline(always)]
    pub fn pol_2(&mut self) -> POL_2_W {
        POL_2_W { w: self }
    }
    #[doc = "Bit 3 - Configure pin polarity of port 0/1 pins for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the pin is active LOW. If the level on this pin is LOW, the pin contributes to the group interrupt. 1 = the pin is active HIGH. If the level on this pin is HIGH, the pin contributes to the group interrupt."]
    #[inline(always)]
    pub fn pol_3(&mut self) -> POL_3_W {
        POL_3_W { w: self }
    }
    #[doc = "Bit 4 - Configure pin polarity of port 0/1 pins for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the pin is active LOW. If the level on this pin is LOW, the pin contributes to the group interrupt. 1 = the pin is active HIGH. If the level on this pin is HIGH, the pin contributes to the group interrupt."]
    #[inline(always)]
    pub fn pol_4(&mut self) -> POL_4_W {
        POL_4_W { w: self }
    }
    #[doc = "Bit 5 - Configure pin polarity of port 0/1 pins for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the pin is active LOW. If the level on this pin is LOW, the pin contributes to the group interrupt. 1 = the pin is active HIGH. If the level on this pin is HIGH, the pin contributes to the group interrupt."]
    #[inline(always)]
    pub fn pol_5(&mut self) -> POL_5_W {
        POL_5_W { w: self }
    }
    #[doc = "Bit 6 - Configure pin polarity of port 0/1 pins for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the pin is active LOW. If the level on this pin is LOW, the pin contributes to the group interrupt. 1 = the pin is active HIGH. If the level on this pin is HIGH, the pin contributes to the group interrupt."]
    #[inline(always)]
    pub fn pol_6(&mut self) -> POL_6_W {
        POL_6_W { w: self }
    }
    #[doc = "Bit 7 - Configure pin polarity of port 0/1 pins for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the pin is active LOW. If the level on this pin is LOW, the pin contributes to the group interrupt. 1 = the pin is active HIGH. If the level on this pin is HIGH, the pin contributes to the group interrupt."]
    #[inline(always)]
    pub fn pol_7(&mut self) -> POL_7_W {
        POL_7_W { w: self }
    }
    #[doc = "Bit 8 - Configure pin polarity of port 0/1 pins for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the pin is active LOW. If the level on this pin is LOW, the pin contributes to the group interrupt. 1 = the pin is active HIGH. If the level on this pin is HIGH, the pin contributes to the group interrupt."]
    #[inline(always)]
    pub fn pol_8(&mut self) -> POL_8_W {
        POL_8_W { w: self }
    }
    #[doc = "Bit 9 - Configure pin polarity of port 0/1 pins for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the pin is active LOW. If the level on this pin is LOW, the pin contributes to the group interrupt. 1 = the pin is active HIGH. If the level on this pin is HIGH, the pin contributes to the group interrupt."]
    #[inline(always)]
    pub fn pol_9(&mut self) -> POL_9_W {
        POL_9_W { w: self }
    }
    #[doc = "Bit 10 - Configure pin polarity of port 0/1 pins for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the pin is active LOW. If the level on this pin is LOW, the pin contributes to the group interrupt. 1 = the pin is active HIGH. If the level on this pin is HIGH, the pin contributes to the group interrupt."]
    #[inline(always)]
    pub fn pol_10(&mut self) -> POL_10_W {
        POL_10_W { w: self }
    }
    #[doc = "Bit 11 - Configure pin polarity of port 0/1 pins for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the pin is active LOW. If the level on this pin is LOW, the pin contributes to the group interrupt. 1 = the pin is active HIGH. If the level on this pin is HIGH, the pin contributes to the group interrupt."]
    #[inline(always)]
    pub fn pol_11(&mut self) -> POL_11_W {
        POL_11_W { w: self }
    }
    #[doc = "Bit 12 - Configure pin polarity of port 0/1 pins for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the pin is active LOW. If the level on this pin is LOW, the pin contributes to the group interrupt. 1 = the pin is active HIGH. If the level on this pin is HIGH, the pin contributes to the group interrupt."]
    #[inline(always)]
    pub fn pol_12(&mut self) -> POL_12_W {
        POL_12_W { w: self }
    }
    #[doc = "Bit 13 - Configure pin polarity of port 0/1 pins for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the pin is active LOW. If the level on this pin is LOW, the pin contributes to the group interrupt. 1 = the pin is active HIGH. If the level on this pin is HIGH, the pin contributes to the group interrupt."]
    #[inline(always)]
    pub fn pol_13(&mut self) -> POL_13_W {
        POL_13_W { w: self }
    }
    #[doc = "Bit 14 - Configure pin polarity of port 0/1 pins for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the pin is active LOW. If the level on this pin is LOW, the pin contributes to the group interrupt. 1 = the pin is active HIGH. If the level on this pin is HIGH, the pin contributes to the group interrupt."]
    #[inline(always)]
    pub fn pol_14(&mut self) -> POL_14_W {
        POL_14_W { w: self }
    }
    #[doc = "Bit 15 - Configure pin polarity of port 0/1 pins for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the pin is active LOW. If the level on this pin is LOW, the pin contributes to the group interrupt. 1 = the pin is active HIGH. If the level on this pin is HIGH, the pin contributes to the group interrupt."]
    #[inline(always)]
    pub fn pol_15(&mut self) -> POL_15_W {
        POL_15_W { w: self }
    }
    #[doc = "Bit 16 - Configure pin polarity of port 0/1 pins for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the pin is active LOW. If the level on this pin is LOW, the pin contributes to the group interrupt. 1 = the pin is active HIGH. If the level on this pin is HIGH, the pin contributes to the group interrupt."]
    #[inline(always)]
    pub fn pol_16(&mut self) -> POL_16_W {
        POL_16_W { w: self }
    }
    #[doc = "Bit 17 - Configure pin polarity of port 0/1 pins for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the pin is active LOW. If the level on this pin is LOW, the pin contributes to the group interrupt. 1 = the pin is active HIGH. If the level on this pin is HIGH, the pin contributes to the group interrupt."]
    #[inline(always)]
    pub fn pol_17(&mut self) -> POL_17_W {
        POL_17_W { w: self }
    }
    #[doc = "Bit 18 - Configure pin polarity of port 0/1 pins for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the pin is active LOW. If the level on this pin is LOW, the pin contributes to the group interrupt. 1 = the pin is active HIGH. If the level on this pin is HIGH, the pin contributes to the group interrupt."]
    #[inline(always)]
    pub fn pol_18(&mut self) -> POL_18_W {
        POL_18_W { w: self }
    }
    #[doc = "Bit 19 - Configure pin polarity of port 0/1 pins for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the pin is active LOW. If the level on this pin is LOW, the pin contributes to the group interrupt. 1 = the pin is active HIGH. If the level on this pin is HIGH, the pin contributes to the group interrupt."]
    #[inline(always)]
    pub fn pol_19(&mut self) -> POL_19_W {
        POL_19_W { w: self }
    }
    #[doc = "Bit 20 - Configure pin polarity of port 0/1 pins for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the pin is active LOW. If the level on this pin is LOW, the pin contributes to the group interrupt. 1 = the pin is active HIGH. If the level on this pin is HIGH, the pin contributes to the group interrupt."]
    #[inline(always)]
    pub fn pol_20(&mut self) -> POL_20_W {
        POL_20_W { w: self }
    }
    #[doc = "Bit 21 - Configure pin polarity of port 0/1 pins for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the pin is active LOW. If the level on this pin is LOW, the pin contributes to the group interrupt. 1 = the pin is active HIGH. If the level on this pin is HIGH, the pin contributes to the group interrupt."]
    #[inline(always)]
    pub fn pol_21(&mut self) -> POL_21_W {
        POL_21_W { w: self }
    }
    #[doc = "Bit 22 - Configure pin polarity of port 0/1 pins for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the pin is active LOW. If the level on this pin is LOW, the pin contributes to the group interrupt. 1 = the pin is active HIGH. If the level on this pin is HIGH, the pin contributes to the group interrupt."]
    #[inline(always)]
    pub fn pol_22(&mut self) -> POL_22_W {
        POL_22_W { w: self }
    }
    #[doc = "Bit 23 - Configure pin polarity of port 0/1 pins for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the pin is active LOW. If the level on this pin is LOW, the pin contributes to the group interrupt. 1 = the pin is active HIGH. If the level on this pin is HIGH, the pin contributes to the group interrupt."]
    #[inline(always)]
    pub fn pol_23(&mut self) -> POL_23_W {
        POL_23_W { w: self }
    }
    #[doc = "Bit 24 - Configure pin polarity of port 0/1 pins for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the pin is active LOW. If the level on this pin is LOW, the pin contributes to the group interrupt. 1 = the pin is active HIGH. If the level on this pin is HIGH, the pin contributes to the group interrupt."]
    #[inline(always)]
    pub fn pol_24(&mut self) -> POL_24_W {
        POL_24_W { w: self }
    }
    #[doc = "Bit 25 - Configure pin polarity of port 0/1 pins for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the pin is active LOW. If the level on this pin is LOW, the pin contributes to the group interrupt. 1 = the pin is active HIGH. If the level on this pin is HIGH, the pin contributes to the group interrupt."]
    #[inline(always)]
    pub fn pol_25(&mut self) -> POL_25_W {
        POL_25_W { w: self }
    }
    #[doc = "Bit 26 - Configure pin polarity of port 0/1 pins for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the pin is active LOW. If the level on this pin is LOW, the pin contributes to the group interrupt. 1 = the pin is active HIGH. If the level on this pin is HIGH, the pin contributes to the group interrupt."]
    #[inline(always)]
    pub fn pol_26(&mut self) -> POL_26_W {
        POL_26_W { w: self }
    }
    #[doc = "Bit 27 - Configure pin polarity of port 0/1 pins for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the pin is active LOW. If the level on this pin is LOW, the pin contributes to the group interrupt. 1 = the pin is active HIGH. If the level on this pin is HIGH, the pin contributes to the group interrupt."]
    #[inline(always)]
    pub fn pol_27(&mut self) -> POL_27_W {
        POL_27_W { w: self }
    }
    #[doc = "Bit 28 - Configure pin polarity of port 0/1 pins for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the pin is active LOW. If the level on this pin is LOW, the pin contributes to the group interrupt. 1 = the pin is active HIGH. If the level on this pin is HIGH, the pin contributes to the group interrupt."]
    #[inline(always)]
    pub fn pol_28(&mut self) -> POL_28_W {
        POL_28_W { w: self }
    }
    #[doc = "Bit 29 - Configure pin polarity of port 0/1 pins for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the pin is active LOW. If the level on this pin is LOW, the pin contributes to the group interrupt. 1 = the pin is active HIGH. If the level on this pin is HIGH, the pin contributes to the group interrupt."]
    #[inline(always)]
    pub fn pol_29(&mut self) -> POL_29_W {
        POL_29_W { w: self }
    }
    #[doc = "Bit 30 - Configure pin polarity of port 0/1 pins for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the pin is active LOW. If the level on this pin is LOW, the pin contributes to the group interrupt. 1 = the pin is active HIGH. If the level on this pin is HIGH, the pin contributes to the group interrupt."]
    #[inline(always)]
    pub fn pol_30(&mut self) -> POL_30_W {
        POL_30_W { w: self }
    }
    #[doc = "Bit 31 - Configure pin polarity of port 0/1 pins for group interrupt. Bit n corresponds to pin P0/1_n of port 0/1. 0 = the pin is active LOW. If the level on this pin is LOW, the pin contributes to the group interrupt. 1 = the pin is active HIGH. If the level on this pin is HIGH, the pin contributes to the group interrupt."]
    #[inline(always)]
    pub fn pol_31(&mut self) -> POL_31_W {
        POL_31_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GPIO grouped interrupt port 0 polarity register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [port_pol](index.html) module"]
pub struct PORT_POL_SPEC;
impl crate::RegisterSpec for PORT_POL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [port_pol::R](R) reader structure"]
impl crate::Readable for PORT_POL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [port_pol::W](W) writer structure"]
impl crate::Writable for PORT_POL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PORT_POL%s to value 0xffff_ffff"]
impl crate::Resettable for PORT_POL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xffff_ffff
    }
}
