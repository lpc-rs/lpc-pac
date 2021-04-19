#[doc = "Register `SET%s` reader"]
pub struct R(crate::R<SET_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SET_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<SET_SPEC>> for R {
    fn from(reader: crate::R<SET_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SET%s` writer"]
pub struct W(crate::W<SET_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SET_SPEC>;
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
impl core::convert::From<crate::W<SET_SPEC>> for W {
    fn from(writer: crate::W<SET_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SETP0` reader - Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: output bit; write: set output bit."]
pub struct SETP0_R(crate::FieldReader<bool, bool>);
impl SETP0_R {
    pub(crate) fn new(bits: bool) -> Self {
        SETP0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SETP0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SETP0` writer - Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: output bit; write: set output bit."]
pub struct SETP0_W<'a> {
    w: &'a mut W,
}
impl<'a> SETP0_W<'a> {
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
#[doc = "Field `SETP1` reader - Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: output bit; write: set output bit."]
pub struct SETP1_R(crate::FieldReader<bool, bool>);
impl SETP1_R {
    pub(crate) fn new(bits: bool) -> Self {
        SETP1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SETP1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SETP1` writer - Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: output bit; write: set output bit."]
pub struct SETP1_W<'a> {
    w: &'a mut W,
}
impl<'a> SETP1_W<'a> {
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
#[doc = "Field `SETP2` reader - Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: output bit; write: set output bit."]
pub struct SETP2_R(crate::FieldReader<bool, bool>);
impl SETP2_R {
    pub(crate) fn new(bits: bool) -> Self {
        SETP2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SETP2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SETP2` writer - Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: output bit; write: set output bit."]
pub struct SETP2_W<'a> {
    w: &'a mut W,
}
impl<'a> SETP2_W<'a> {
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
#[doc = "Field `SETP3` reader - Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: output bit; write: set output bit."]
pub struct SETP3_R(crate::FieldReader<bool, bool>);
impl SETP3_R {
    pub(crate) fn new(bits: bool) -> Self {
        SETP3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SETP3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SETP3` writer - Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: output bit; write: set output bit."]
pub struct SETP3_W<'a> {
    w: &'a mut W,
}
impl<'a> SETP3_W<'a> {
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
#[doc = "Field `SETP4` reader - Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: output bit; write: set output bit."]
pub struct SETP4_R(crate::FieldReader<bool, bool>);
impl SETP4_R {
    pub(crate) fn new(bits: bool) -> Self {
        SETP4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SETP4_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SETP4` writer - Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: output bit; write: set output bit."]
pub struct SETP4_W<'a> {
    w: &'a mut W,
}
impl<'a> SETP4_W<'a> {
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
#[doc = "Field `SETP5` reader - Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: output bit; write: set output bit."]
pub struct SETP5_R(crate::FieldReader<bool, bool>);
impl SETP5_R {
    pub(crate) fn new(bits: bool) -> Self {
        SETP5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SETP5_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SETP5` writer - Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: output bit; write: set output bit."]
pub struct SETP5_W<'a> {
    w: &'a mut W,
}
impl<'a> SETP5_W<'a> {
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
#[doc = "Field `SETP6` reader - Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: output bit; write: set output bit."]
pub struct SETP6_R(crate::FieldReader<bool, bool>);
impl SETP6_R {
    pub(crate) fn new(bits: bool) -> Self {
        SETP6_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SETP6_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SETP6` writer - Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: output bit; write: set output bit."]
pub struct SETP6_W<'a> {
    w: &'a mut W,
}
impl<'a> SETP6_W<'a> {
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
#[doc = "Field `SETP7` reader - Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: output bit; write: set output bit."]
pub struct SETP7_R(crate::FieldReader<bool, bool>);
impl SETP7_R {
    pub(crate) fn new(bits: bool) -> Self {
        SETP7_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SETP7_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SETP7` writer - Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: output bit; write: set output bit."]
pub struct SETP7_W<'a> {
    w: &'a mut W,
}
impl<'a> SETP7_W<'a> {
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
#[doc = "Field `SETP8` reader - Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: output bit; write: set output bit."]
pub struct SETP8_R(crate::FieldReader<bool, bool>);
impl SETP8_R {
    pub(crate) fn new(bits: bool) -> Self {
        SETP8_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SETP8_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SETP8` writer - Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: output bit; write: set output bit."]
pub struct SETP8_W<'a> {
    w: &'a mut W,
}
impl<'a> SETP8_W<'a> {
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
#[doc = "Field `SETP9` reader - Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: output bit; write: set output bit."]
pub struct SETP9_R(crate::FieldReader<bool, bool>);
impl SETP9_R {
    pub(crate) fn new(bits: bool) -> Self {
        SETP9_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SETP9_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SETP9` writer - Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: output bit; write: set output bit."]
pub struct SETP9_W<'a> {
    w: &'a mut W,
}
impl<'a> SETP9_W<'a> {
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
#[doc = "Field `SETP10` reader - Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: output bit; write: set output bit."]
pub struct SETP10_R(crate::FieldReader<bool, bool>);
impl SETP10_R {
    pub(crate) fn new(bits: bool) -> Self {
        SETP10_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SETP10_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SETP10` writer - Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: output bit; write: set output bit."]
pub struct SETP10_W<'a> {
    w: &'a mut W,
}
impl<'a> SETP10_W<'a> {
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
#[doc = "Field `SETP11` reader - Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: output bit; write: set output bit."]
pub struct SETP11_R(crate::FieldReader<bool, bool>);
impl SETP11_R {
    pub(crate) fn new(bits: bool) -> Self {
        SETP11_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SETP11_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SETP11` writer - Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: output bit; write: set output bit."]
pub struct SETP11_W<'a> {
    w: &'a mut W,
}
impl<'a> SETP11_W<'a> {
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
#[doc = "Field `SETP12` reader - Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: output bit; write: set output bit."]
pub struct SETP12_R(crate::FieldReader<bool, bool>);
impl SETP12_R {
    pub(crate) fn new(bits: bool) -> Self {
        SETP12_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SETP12_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SETP12` writer - Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: output bit; write: set output bit."]
pub struct SETP12_W<'a> {
    w: &'a mut W,
}
impl<'a> SETP12_W<'a> {
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
#[doc = "Field `SETP13` reader - Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: output bit; write: set output bit."]
pub struct SETP13_R(crate::FieldReader<bool, bool>);
impl SETP13_R {
    pub(crate) fn new(bits: bool) -> Self {
        SETP13_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SETP13_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SETP13` writer - Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: output bit; write: set output bit."]
pub struct SETP13_W<'a> {
    w: &'a mut W,
}
impl<'a> SETP13_W<'a> {
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
#[doc = "Field `SETP14` reader - Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: output bit; write: set output bit."]
pub struct SETP14_R(crate::FieldReader<bool, bool>);
impl SETP14_R {
    pub(crate) fn new(bits: bool) -> Self {
        SETP14_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SETP14_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SETP14` writer - Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: output bit; write: set output bit."]
pub struct SETP14_W<'a> {
    w: &'a mut W,
}
impl<'a> SETP14_W<'a> {
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
#[doc = "Field `SETP15` reader - Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: output bit; write: set output bit."]
pub struct SETP15_R(crate::FieldReader<bool, bool>);
impl SETP15_R {
    pub(crate) fn new(bits: bool) -> Self {
        SETP15_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SETP15_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SETP15` writer - Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: output bit; write: set output bit."]
pub struct SETP15_W<'a> {
    w: &'a mut W,
}
impl<'a> SETP15_W<'a> {
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
#[doc = "Field `SETP16` reader - Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: output bit; write: set output bit."]
pub struct SETP16_R(crate::FieldReader<bool, bool>);
impl SETP16_R {
    pub(crate) fn new(bits: bool) -> Self {
        SETP16_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SETP16_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SETP16` writer - Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: output bit; write: set output bit."]
pub struct SETP16_W<'a> {
    w: &'a mut W,
}
impl<'a> SETP16_W<'a> {
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
#[doc = "Field `SETP17` reader - Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: output bit; write: set output bit."]
pub struct SETP17_R(crate::FieldReader<bool, bool>);
impl SETP17_R {
    pub(crate) fn new(bits: bool) -> Self {
        SETP17_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SETP17_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SETP17` writer - Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: output bit; write: set output bit."]
pub struct SETP17_W<'a> {
    w: &'a mut W,
}
impl<'a> SETP17_W<'a> {
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
#[doc = "Field `SETP18` reader - Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: output bit; write: set output bit."]
pub struct SETP18_R(crate::FieldReader<bool, bool>);
impl SETP18_R {
    pub(crate) fn new(bits: bool) -> Self {
        SETP18_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SETP18_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SETP18` writer - Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: output bit; write: set output bit."]
pub struct SETP18_W<'a> {
    w: &'a mut W,
}
impl<'a> SETP18_W<'a> {
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
#[doc = "Field `SETP19` reader - Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: output bit; write: set output bit."]
pub struct SETP19_R(crate::FieldReader<bool, bool>);
impl SETP19_R {
    pub(crate) fn new(bits: bool) -> Self {
        SETP19_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SETP19_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SETP19` writer - Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: output bit; write: set output bit."]
pub struct SETP19_W<'a> {
    w: &'a mut W,
}
impl<'a> SETP19_W<'a> {
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
#[doc = "Field `SETP20` reader - Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: output bit; write: set output bit."]
pub struct SETP20_R(crate::FieldReader<bool, bool>);
impl SETP20_R {
    pub(crate) fn new(bits: bool) -> Self {
        SETP20_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SETP20_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SETP20` writer - Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: output bit; write: set output bit."]
pub struct SETP20_W<'a> {
    w: &'a mut W,
}
impl<'a> SETP20_W<'a> {
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
#[doc = "Field `SETP21` reader - Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: output bit; write: set output bit."]
pub struct SETP21_R(crate::FieldReader<bool, bool>);
impl SETP21_R {
    pub(crate) fn new(bits: bool) -> Self {
        SETP21_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SETP21_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SETP21` writer - Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: output bit; write: set output bit."]
pub struct SETP21_W<'a> {
    w: &'a mut W,
}
impl<'a> SETP21_W<'a> {
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
#[doc = "Field `SETP22` reader - Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: output bit; write: set output bit."]
pub struct SETP22_R(crate::FieldReader<bool, bool>);
impl SETP22_R {
    pub(crate) fn new(bits: bool) -> Self {
        SETP22_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SETP22_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SETP22` writer - Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: output bit; write: set output bit."]
pub struct SETP22_W<'a> {
    w: &'a mut W,
}
impl<'a> SETP22_W<'a> {
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
#[doc = "Field `SETP23` reader - Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: output bit; write: set output bit."]
pub struct SETP23_R(crate::FieldReader<bool, bool>);
impl SETP23_R {
    pub(crate) fn new(bits: bool) -> Self {
        SETP23_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SETP23_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SETP23` writer - Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: output bit; write: set output bit."]
pub struct SETP23_W<'a> {
    w: &'a mut W,
}
impl<'a> SETP23_W<'a> {
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
#[doc = "Field `SETP24` reader - Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: output bit; write: set output bit."]
pub struct SETP24_R(crate::FieldReader<bool, bool>);
impl SETP24_R {
    pub(crate) fn new(bits: bool) -> Self {
        SETP24_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SETP24_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SETP24` writer - Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: output bit; write: set output bit."]
pub struct SETP24_W<'a> {
    w: &'a mut W,
}
impl<'a> SETP24_W<'a> {
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
#[doc = "Field `SETP25` reader - Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: output bit; write: set output bit."]
pub struct SETP25_R(crate::FieldReader<bool, bool>);
impl SETP25_R {
    pub(crate) fn new(bits: bool) -> Self {
        SETP25_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SETP25_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SETP25` writer - Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: output bit; write: set output bit."]
pub struct SETP25_W<'a> {
    w: &'a mut W,
}
impl<'a> SETP25_W<'a> {
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
#[doc = "Field `SETP26` reader - Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: output bit; write: set output bit."]
pub struct SETP26_R(crate::FieldReader<bool, bool>);
impl SETP26_R {
    pub(crate) fn new(bits: bool) -> Self {
        SETP26_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SETP26_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SETP26` writer - Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: output bit; write: set output bit."]
pub struct SETP26_W<'a> {
    w: &'a mut W,
}
impl<'a> SETP26_W<'a> {
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
#[doc = "Field `SETP27` reader - Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: output bit; write: set output bit."]
pub struct SETP27_R(crate::FieldReader<bool, bool>);
impl SETP27_R {
    pub(crate) fn new(bits: bool) -> Self {
        SETP27_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SETP27_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SETP27` writer - Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: output bit; write: set output bit."]
pub struct SETP27_W<'a> {
    w: &'a mut W,
}
impl<'a> SETP27_W<'a> {
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
#[doc = "Field `SETP28` reader - Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: output bit; write: set output bit."]
pub struct SETP28_R(crate::FieldReader<bool, bool>);
impl SETP28_R {
    pub(crate) fn new(bits: bool) -> Self {
        SETP28_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SETP28_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SETP28` writer - Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: output bit; write: set output bit."]
pub struct SETP28_W<'a> {
    w: &'a mut W,
}
impl<'a> SETP28_W<'a> {
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
#[doc = "Field `SETP29` reader - Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: output bit; write: set output bit."]
pub struct SETP29_R(crate::FieldReader<bool, bool>);
impl SETP29_R {
    pub(crate) fn new(bits: bool) -> Self {
        SETP29_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SETP29_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SETP29` writer - Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: output bit; write: set output bit."]
pub struct SETP29_W<'a> {
    w: &'a mut W,
}
impl<'a> SETP29_W<'a> {
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
#[doc = "Field `SETP30` reader - Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: output bit; write: set output bit."]
pub struct SETP30_R(crate::FieldReader<bool, bool>);
impl SETP30_R {
    pub(crate) fn new(bits: bool) -> Self {
        SETP30_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SETP30_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SETP30` writer - Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: output bit; write: set output bit."]
pub struct SETP30_W<'a> {
    w: &'a mut W,
}
impl<'a> SETP30_W<'a> {
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
#[doc = "Field `SETP31` reader - Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: output bit; write: set output bit."]
pub struct SETP31_R(crate::FieldReader<bool, bool>);
impl SETP31_R {
    pub(crate) fn new(bits: bool) -> Self {
        SETP31_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SETP31_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SETP31` writer - Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: output bit; write: set output bit."]
pub struct SETP31_W<'a> {
    w: &'a mut W,
}
impl<'a> SETP31_W<'a> {
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
    #[doc = "Bit 0 - Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: output bit; write: set output bit."]
    #[inline(always)]
    pub fn setp0(&self) -> SETP0_R {
        SETP0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: output bit; write: set output bit."]
    #[inline(always)]
    pub fn setp1(&self) -> SETP1_R {
        SETP1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: output bit; write: set output bit."]
    #[inline(always)]
    pub fn setp2(&self) -> SETP2_R {
        SETP2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: output bit; write: set output bit."]
    #[inline(always)]
    pub fn setp3(&self) -> SETP3_R {
        SETP3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: output bit; write: set output bit."]
    #[inline(always)]
    pub fn setp4(&self) -> SETP4_R {
        SETP4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: output bit; write: set output bit."]
    #[inline(always)]
    pub fn setp5(&self) -> SETP5_R {
        SETP5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: output bit; write: set output bit."]
    #[inline(always)]
    pub fn setp6(&self) -> SETP6_R {
        SETP6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: output bit; write: set output bit."]
    #[inline(always)]
    pub fn setp7(&self) -> SETP7_R {
        SETP7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: output bit; write: set output bit."]
    #[inline(always)]
    pub fn setp8(&self) -> SETP8_R {
        SETP8_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: output bit; write: set output bit."]
    #[inline(always)]
    pub fn setp9(&self) -> SETP9_R {
        SETP9_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: output bit; write: set output bit."]
    #[inline(always)]
    pub fn setp10(&self) -> SETP10_R {
        SETP10_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: output bit; write: set output bit."]
    #[inline(always)]
    pub fn setp11(&self) -> SETP11_R {
        SETP11_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: output bit; write: set output bit."]
    #[inline(always)]
    pub fn setp12(&self) -> SETP12_R {
        SETP12_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: output bit; write: set output bit."]
    #[inline(always)]
    pub fn setp13(&self) -> SETP13_R {
        SETP13_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: output bit; write: set output bit."]
    #[inline(always)]
    pub fn setp14(&self) -> SETP14_R {
        SETP14_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: output bit; write: set output bit."]
    #[inline(always)]
    pub fn setp15(&self) -> SETP15_R {
        SETP15_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: output bit; write: set output bit."]
    #[inline(always)]
    pub fn setp16(&self) -> SETP16_R {
        SETP16_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: output bit; write: set output bit."]
    #[inline(always)]
    pub fn setp17(&self) -> SETP17_R {
        SETP17_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: output bit; write: set output bit."]
    #[inline(always)]
    pub fn setp18(&self) -> SETP18_R {
        SETP18_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: output bit; write: set output bit."]
    #[inline(always)]
    pub fn setp19(&self) -> SETP19_R {
        SETP19_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: output bit; write: set output bit."]
    #[inline(always)]
    pub fn setp20(&self) -> SETP20_R {
        SETP20_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: output bit; write: set output bit."]
    #[inline(always)]
    pub fn setp21(&self) -> SETP21_R {
        SETP21_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: output bit; write: set output bit."]
    #[inline(always)]
    pub fn setp22(&self) -> SETP22_R {
        SETP22_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: output bit; write: set output bit."]
    #[inline(always)]
    pub fn setp23(&self) -> SETP23_R {
        SETP23_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: output bit; write: set output bit."]
    #[inline(always)]
    pub fn setp24(&self) -> SETP24_R {
        SETP24_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: output bit; write: set output bit."]
    #[inline(always)]
    pub fn setp25(&self) -> SETP25_R {
        SETP25_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: output bit; write: set output bit."]
    #[inline(always)]
    pub fn setp26(&self) -> SETP26_R {
        SETP26_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: output bit; write: set output bit."]
    #[inline(always)]
    pub fn setp27(&self) -> SETP27_R {
        SETP27_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: output bit; write: set output bit."]
    #[inline(always)]
    pub fn setp28(&self) -> SETP28_R {
        SETP28_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: output bit; write: set output bit."]
    #[inline(always)]
    pub fn setp29(&self) -> SETP29_R {
        SETP29_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: output bit; write: set output bit."]
    #[inline(always)]
    pub fn setp30(&self) -> SETP30_R {
        SETP30_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: output bit; write: set output bit."]
    #[inline(always)]
    pub fn setp31(&self) -> SETP31_R {
        SETP31_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: output bit; write: set output bit."]
    #[inline(always)]
    pub fn setp0(&mut self) -> SETP0_W {
        SETP0_W { w: self }
    }
    #[doc = "Bit 1 - Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: output bit; write: set output bit."]
    #[inline(always)]
    pub fn setp1(&mut self) -> SETP1_W {
        SETP1_W { w: self }
    }
    #[doc = "Bit 2 - Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: output bit; write: set output bit."]
    #[inline(always)]
    pub fn setp2(&mut self) -> SETP2_W {
        SETP2_W { w: self }
    }
    #[doc = "Bit 3 - Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: output bit; write: set output bit."]
    #[inline(always)]
    pub fn setp3(&mut self) -> SETP3_W {
        SETP3_W { w: self }
    }
    #[doc = "Bit 4 - Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: output bit; write: set output bit."]
    #[inline(always)]
    pub fn setp4(&mut self) -> SETP4_W {
        SETP4_W { w: self }
    }
    #[doc = "Bit 5 - Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: output bit; write: set output bit."]
    #[inline(always)]
    pub fn setp5(&mut self) -> SETP5_W {
        SETP5_W { w: self }
    }
    #[doc = "Bit 6 - Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: output bit; write: set output bit."]
    #[inline(always)]
    pub fn setp6(&mut self) -> SETP6_W {
        SETP6_W { w: self }
    }
    #[doc = "Bit 7 - Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: output bit; write: set output bit."]
    #[inline(always)]
    pub fn setp7(&mut self) -> SETP7_W {
        SETP7_W { w: self }
    }
    #[doc = "Bit 8 - Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: output bit; write: set output bit."]
    #[inline(always)]
    pub fn setp8(&mut self) -> SETP8_W {
        SETP8_W { w: self }
    }
    #[doc = "Bit 9 - Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: output bit; write: set output bit."]
    #[inline(always)]
    pub fn setp9(&mut self) -> SETP9_W {
        SETP9_W { w: self }
    }
    #[doc = "Bit 10 - Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: output bit; write: set output bit."]
    #[inline(always)]
    pub fn setp10(&mut self) -> SETP10_W {
        SETP10_W { w: self }
    }
    #[doc = "Bit 11 - Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: output bit; write: set output bit."]
    #[inline(always)]
    pub fn setp11(&mut self) -> SETP11_W {
        SETP11_W { w: self }
    }
    #[doc = "Bit 12 - Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: output bit; write: set output bit."]
    #[inline(always)]
    pub fn setp12(&mut self) -> SETP12_W {
        SETP12_W { w: self }
    }
    #[doc = "Bit 13 - Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: output bit; write: set output bit."]
    #[inline(always)]
    pub fn setp13(&mut self) -> SETP13_W {
        SETP13_W { w: self }
    }
    #[doc = "Bit 14 - Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: output bit; write: set output bit."]
    #[inline(always)]
    pub fn setp14(&mut self) -> SETP14_W {
        SETP14_W { w: self }
    }
    #[doc = "Bit 15 - Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: output bit; write: set output bit."]
    #[inline(always)]
    pub fn setp15(&mut self) -> SETP15_W {
        SETP15_W { w: self }
    }
    #[doc = "Bit 16 - Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: output bit; write: set output bit."]
    #[inline(always)]
    pub fn setp16(&mut self) -> SETP16_W {
        SETP16_W { w: self }
    }
    #[doc = "Bit 17 - Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: output bit; write: set output bit."]
    #[inline(always)]
    pub fn setp17(&mut self) -> SETP17_W {
        SETP17_W { w: self }
    }
    #[doc = "Bit 18 - Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: output bit; write: set output bit."]
    #[inline(always)]
    pub fn setp18(&mut self) -> SETP18_W {
        SETP18_W { w: self }
    }
    #[doc = "Bit 19 - Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: output bit; write: set output bit."]
    #[inline(always)]
    pub fn setp19(&mut self) -> SETP19_W {
        SETP19_W { w: self }
    }
    #[doc = "Bit 20 - Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: output bit; write: set output bit."]
    #[inline(always)]
    pub fn setp20(&mut self) -> SETP20_W {
        SETP20_W { w: self }
    }
    #[doc = "Bit 21 - Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: output bit; write: set output bit."]
    #[inline(always)]
    pub fn setp21(&mut self) -> SETP21_W {
        SETP21_W { w: self }
    }
    #[doc = "Bit 22 - Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: output bit; write: set output bit."]
    #[inline(always)]
    pub fn setp22(&mut self) -> SETP22_W {
        SETP22_W { w: self }
    }
    #[doc = "Bit 23 - Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: output bit; write: set output bit."]
    #[inline(always)]
    pub fn setp23(&mut self) -> SETP23_W {
        SETP23_W { w: self }
    }
    #[doc = "Bit 24 - Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: output bit; write: set output bit."]
    #[inline(always)]
    pub fn setp24(&mut self) -> SETP24_W {
        SETP24_W { w: self }
    }
    #[doc = "Bit 25 - Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: output bit; write: set output bit."]
    #[inline(always)]
    pub fn setp25(&mut self) -> SETP25_W {
        SETP25_W { w: self }
    }
    #[doc = "Bit 26 - Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: output bit; write: set output bit."]
    #[inline(always)]
    pub fn setp26(&mut self) -> SETP26_W {
        SETP26_W { w: self }
    }
    #[doc = "Bit 27 - Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: output bit; write: set output bit."]
    #[inline(always)]
    pub fn setp27(&mut self) -> SETP27_W {
        SETP27_W { w: self }
    }
    #[doc = "Bit 28 - Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: output bit; write: set output bit."]
    #[inline(always)]
    pub fn setp28(&mut self) -> SETP28_W {
        SETP28_W { w: self }
    }
    #[doc = "Bit 29 - Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: output bit; write: set output bit."]
    #[inline(always)]
    pub fn setp29(&mut self) -> SETP29_W {
        SETP29_W { w: self }
    }
    #[doc = "Bit 30 - Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: output bit; write: set output bit."]
    #[inline(always)]
    pub fn setp30(&mut self) -> SETP30_W {
        SETP30_W { w: self }
    }
    #[doc = "Bit 31 - Read or set output bits. 0 = Read: output bit: write: no operation. 1 = Read: output bit; write: set output bit."]
    #[inline(always)]
    pub fn setp31(&mut self) -> SETP31_W {
        SETP31_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Write: Set register for port 0/1 Read: output bits for port 0/1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [set](index.html) module"]
pub struct SET_SPEC;
impl crate::RegisterSpec for SET_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [set::R](R) reader structure"]
impl crate::Readable for SET_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [set::W](W) writer structure"]
impl crate::Writable for SET_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SET%s to value 0"]
impl crate::Resettable for SET_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
