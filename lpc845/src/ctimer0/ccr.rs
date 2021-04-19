#[doc = "Register `CCR` reader"]
pub struct R(crate::R<CCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<CCR_SPEC>> for R {
    fn from(reader: crate::R<CCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CCR` writer"]
pub struct W(crate::W<CCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CCR_SPEC>;
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
impl core::convert::From<crate::W<CCR_SPEC>> for W {
    fn from(writer: crate::W<CCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CAP0RE` reader - Rising edge of capture channel 0: a sequence of 0 then 1 causes CR0 to be loaded with the contents of TC. 0 = disabled. 1 = enabled."]
pub struct CAP0RE_R(crate::FieldReader<bool, bool>);
impl CAP0RE_R {
    pub(crate) fn new(bits: bool) -> Self {
        CAP0RE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CAP0RE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CAP0RE` writer - Rising edge of capture channel 0: a sequence of 0 then 1 causes CR0 to be loaded with the contents of TC. 0 = disabled. 1 = enabled."]
pub struct CAP0RE_W<'a> {
    w: &'a mut W,
}
impl<'a> CAP0RE_W<'a> {
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
#[doc = "Field `CAP0FE` reader - Falling edge of capture channel 0: a sequence of 1 then 0 causes CR0 to be loaded with the contents of TC. 0 = disabled. 1 = enabled."]
pub struct CAP0FE_R(crate::FieldReader<bool, bool>);
impl CAP0FE_R {
    pub(crate) fn new(bits: bool) -> Self {
        CAP0FE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CAP0FE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CAP0FE` writer - Falling edge of capture channel 0: a sequence of 1 then 0 causes CR0 to be loaded with the contents of TC. 0 = disabled. 1 = enabled."]
pub struct CAP0FE_W<'a> {
    w: &'a mut W,
}
impl<'a> CAP0FE_W<'a> {
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
#[doc = "Field `CAP0I` reader - Generate interrupt on channel 0 capture event: a CR0 load generates an interrupt."]
pub struct CAP0I_R(crate::FieldReader<bool, bool>);
impl CAP0I_R {
    pub(crate) fn new(bits: bool) -> Self {
        CAP0I_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CAP0I_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CAP0I` writer - Generate interrupt on channel 0 capture event: a CR0 load generates an interrupt."]
pub struct CAP0I_W<'a> {
    w: &'a mut W,
}
impl<'a> CAP0I_W<'a> {
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
#[doc = "Field `CAP1RE` reader - Rising edge of capture channel 1: a sequence of 0 then 1 causes CR1 to be loaded with the contents of TC. 0 = disabled. 1 = enabled."]
pub struct CAP1RE_R(crate::FieldReader<bool, bool>);
impl CAP1RE_R {
    pub(crate) fn new(bits: bool) -> Self {
        CAP1RE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CAP1RE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CAP1RE` writer - Rising edge of capture channel 1: a sequence of 0 then 1 causes CR1 to be loaded with the contents of TC. 0 = disabled. 1 = enabled."]
pub struct CAP1RE_W<'a> {
    w: &'a mut W,
}
impl<'a> CAP1RE_W<'a> {
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
#[doc = "Field `CAP1FE` reader - Falling edge of capture channel 1: a sequence of 1 then 0 causes CR1 to be loaded with the contents of TC. 0 = disabled. 1 = enabled."]
pub struct CAP1FE_R(crate::FieldReader<bool, bool>);
impl CAP1FE_R {
    pub(crate) fn new(bits: bool) -> Self {
        CAP1FE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CAP1FE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CAP1FE` writer - Falling edge of capture channel 1: a sequence of 1 then 0 causes CR1 to be loaded with the contents of TC. 0 = disabled. 1 = enabled."]
pub struct CAP1FE_W<'a> {
    w: &'a mut W,
}
impl<'a> CAP1FE_W<'a> {
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
#[doc = "Field `CAP1I` reader - Generate interrupt on channel 1 capture event: a CR1 load generates an interrupt."]
pub struct CAP1I_R(crate::FieldReader<bool, bool>);
impl CAP1I_R {
    pub(crate) fn new(bits: bool) -> Self {
        CAP1I_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CAP1I_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CAP1I` writer - Generate interrupt on channel 1 capture event: a CR1 load generates an interrupt."]
pub struct CAP1I_W<'a> {
    w: &'a mut W,
}
impl<'a> CAP1I_W<'a> {
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
#[doc = "Field `CAP2RE` reader - Rising edge of capture channel 2: a sequence of 0 then 1 causes CR2 to be loaded with the contents of TC. 0 = disabled. 1 = enabled."]
pub struct CAP2RE_R(crate::FieldReader<bool, bool>);
impl CAP2RE_R {
    pub(crate) fn new(bits: bool) -> Self {
        CAP2RE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CAP2RE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CAP2RE` writer - Rising edge of capture channel 2: a sequence of 0 then 1 causes CR2 to be loaded with the contents of TC. 0 = disabled. 1 = enabled."]
pub struct CAP2RE_W<'a> {
    w: &'a mut W,
}
impl<'a> CAP2RE_W<'a> {
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
#[doc = "Field `CAP2FE` reader - Falling edge of capture channel 2: a sequence of 1 then 0 causes CR2 to be loaded with the contents of TC. 0 = disabled. 1 = enabled."]
pub struct CAP2FE_R(crate::FieldReader<bool, bool>);
impl CAP2FE_R {
    pub(crate) fn new(bits: bool) -> Self {
        CAP2FE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CAP2FE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CAP2FE` writer - Falling edge of capture channel 2: a sequence of 1 then 0 causes CR2 to be loaded with the contents of TC. 0 = disabled. 1 = enabled."]
pub struct CAP2FE_W<'a> {
    w: &'a mut W,
}
impl<'a> CAP2FE_W<'a> {
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
#[doc = "Field `CAP2I` reader - Generate interrupt on channel 2 capture event: a CR2 load generates an interrupt."]
pub struct CAP2I_R(crate::FieldReader<bool, bool>);
impl CAP2I_R {
    pub(crate) fn new(bits: bool) -> Self {
        CAP2I_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CAP2I_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CAP2I` writer - Generate interrupt on channel 2 capture event: a CR2 load generates an interrupt."]
pub struct CAP2I_W<'a> {
    w: &'a mut W,
}
impl<'a> CAP2I_W<'a> {
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
#[doc = "Field `CAP3RE` reader - Rising edge of capture channel 3: a sequence of 0 then 1 causes CR3 to be loaded with the contents of TC. 0 = disabled. 1 = enabled."]
pub struct CAP3RE_R(crate::FieldReader<bool, bool>);
impl CAP3RE_R {
    pub(crate) fn new(bits: bool) -> Self {
        CAP3RE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CAP3RE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CAP3RE` writer - Rising edge of capture channel 3: a sequence of 0 then 1 causes CR3 to be loaded with the contents of TC. 0 = disabled. 1 = enabled."]
pub struct CAP3RE_W<'a> {
    w: &'a mut W,
}
impl<'a> CAP3RE_W<'a> {
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
#[doc = "Field `CAP3FE` reader - Falling edge of capture channel 3: a sequence of 1 then 0 causes CR3 to be loaded with the contents of TC. 0 = disabled. 1 = enabled."]
pub struct CAP3FE_R(crate::FieldReader<bool, bool>);
impl CAP3FE_R {
    pub(crate) fn new(bits: bool) -> Self {
        CAP3FE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CAP3FE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CAP3FE` writer - Falling edge of capture channel 3: a sequence of 1 then 0 causes CR3 to be loaded with the contents of TC. 0 = disabled. 1 = enabled."]
pub struct CAP3FE_W<'a> {
    w: &'a mut W,
}
impl<'a> CAP3FE_W<'a> {
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
#[doc = "Field `CAP3I` reader - Generate interrupt on channel 3 capture event: a CR3 load generates an interrupt."]
pub struct CAP3I_R(crate::FieldReader<bool, bool>);
impl CAP3I_R {
    pub(crate) fn new(bits: bool) -> Self {
        CAP3I_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CAP3I_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CAP3I` writer - Generate interrupt on channel 3 capture event: a CR3 load generates an interrupt."]
pub struct CAP3I_W<'a> {
    w: &'a mut W,
}
impl<'a> CAP3I_W<'a> {
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
impl R {
    #[doc = "Bit 0 - Rising edge of capture channel 0: a sequence of 0 then 1 causes CR0 to be loaded with the contents of TC. 0 = disabled. 1 = enabled."]
    #[inline(always)]
    pub fn cap0re(&self) -> CAP0RE_R {
        CAP0RE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Falling edge of capture channel 0: a sequence of 1 then 0 causes CR0 to be loaded with the contents of TC. 0 = disabled. 1 = enabled."]
    #[inline(always)]
    pub fn cap0fe(&self) -> CAP0FE_R {
        CAP0FE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Generate interrupt on channel 0 capture event: a CR0 load generates an interrupt."]
    #[inline(always)]
    pub fn cap0i(&self) -> CAP0I_R {
        CAP0I_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Rising edge of capture channel 1: a sequence of 0 then 1 causes CR1 to be loaded with the contents of TC. 0 = disabled. 1 = enabled."]
    #[inline(always)]
    pub fn cap1re(&self) -> CAP1RE_R {
        CAP1RE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Falling edge of capture channel 1: a sequence of 1 then 0 causes CR1 to be loaded with the contents of TC. 0 = disabled. 1 = enabled."]
    #[inline(always)]
    pub fn cap1fe(&self) -> CAP1FE_R {
        CAP1FE_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Generate interrupt on channel 1 capture event: a CR1 load generates an interrupt."]
    #[inline(always)]
    pub fn cap1i(&self) -> CAP1I_R {
        CAP1I_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Rising edge of capture channel 2: a sequence of 0 then 1 causes CR2 to be loaded with the contents of TC. 0 = disabled. 1 = enabled."]
    #[inline(always)]
    pub fn cap2re(&self) -> CAP2RE_R {
        CAP2RE_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Falling edge of capture channel 2: a sequence of 1 then 0 causes CR2 to be loaded with the contents of TC. 0 = disabled. 1 = enabled."]
    #[inline(always)]
    pub fn cap2fe(&self) -> CAP2FE_R {
        CAP2FE_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Generate interrupt on channel 2 capture event: a CR2 load generates an interrupt."]
    #[inline(always)]
    pub fn cap2i(&self) -> CAP2I_R {
        CAP2I_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Rising edge of capture channel 3: a sequence of 0 then 1 causes CR3 to be loaded with the contents of TC. 0 = disabled. 1 = enabled."]
    #[inline(always)]
    pub fn cap3re(&self) -> CAP3RE_R {
        CAP3RE_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Falling edge of capture channel 3: a sequence of 1 then 0 causes CR3 to be loaded with the contents of TC. 0 = disabled. 1 = enabled."]
    #[inline(always)]
    pub fn cap3fe(&self) -> CAP3FE_R {
        CAP3FE_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Generate interrupt on channel 3 capture event: a CR3 load generates an interrupt."]
    #[inline(always)]
    pub fn cap3i(&self) -> CAP3I_R {
        CAP3I_R::new(((self.bits >> 11) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Rising edge of capture channel 0: a sequence of 0 then 1 causes CR0 to be loaded with the contents of TC. 0 = disabled. 1 = enabled."]
    #[inline(always)]
    pub fn cap0re(&mut self) -> CAP0RE_W {
        CAP0RE_W { w: self }
    }
    #[doc = "Bit 1 - Falling edge of capture channel 0: a sequence of 1 then 0 causes CR0 to be loaded with the contents of TC. 0 = disabled. 1 = enabled."]
    #[inline(always)]
    pub fn cap0fe(&mut self) -> CAP0FE_W {
        CAP0FE_W { w: self }
    }
    #[doc = "Bit 2 - Generate interrupt on channel 0 capture event: a CR0 load generates an interrupt."]
    #[inline(always)]
    pub fn cap0i(&mut self) -> CAP0I_W {
        CAP0I_W { w: self }
    }
    #[doc = "Bit 3 - Rising edge of capture channel 1: a sequence of 0 then 1 causes CR1 to be loaded with the contents of TC. 0 = disabled. 1 = enabled."]
    #[inline(always)]
    pub fn cap1re(&mut self) -> CAP1RE_W {
        CAP1RE_W { w: self }
    }
    #[doc = "Bit 4 - Falling edge of capture channel 1: a sequence of 1 then 0 causes CR1 to be loaded with the contents of TC. 0 = disabled. 1 = enabled."]
    #[inline(always)]
    pub fn cap1fe(&mut self) -> CAP1FE_W {
        CAP1FE_W { w: self }
    }
    #[doc = "Bit 5 - Generate interrupt on channel 1 capture event: a CR1 load generates an interrupt."]
    #[inline(always)]
    pub fn cap1i(&mut self) -> CAP1I_W {
        CAP1I_W { w: self }
    }
    #[doc = "Bit 6 - Rising edge of capture channel 2: a sequence of 0 then 1 causes CR2 to be loaded with the contents of TC. 0 = disabled. 1 = enabled."]
    #[inline(always)]
    pub fn cap2re(&mut self) -> CAP2RE_W {
        CAP2RE_W { w: self }
    }
    #[doc = "Bit 7 - Falling edge of capture channel 2: a sequence of 1 then 0 causes CR2 to be loaded with the contents of TC. 0 = disabled. 1 = enabled."]
    #[inline(always)]
    pub fn cap2fe(&mut self) -> CAP2FE_W {
        CAP2FE_W { w: self }
    }
    #[doc = "Bit 8 - Generate interrupt on channel 2 capture event: a CR2 load generates an interrupt."]
    #[inline(always)]
    pub fn cap2i(&mut self) -> CAP2I_W {
        CAP2I_W { w: self }
    }
    #[doc = "Bit 9 - Rising edge of capture channel 3: a sequence of 0 then 1 causes CR3 to be loaded with the contents of TC. 0 = disabled. 1 = enabled."]
    #[inline(always)]
    pub fn cap3re(&mut self) -> CAP3RE_W {
        CAP3RE_W { w: self }
    }
    #[doc = "Bit 10 - Falling edge of capture channel 3: a sequence of 1 then 0 causes CR3 to be loaded with the contents of TC. 0 = disabled. 1 = enabled."]
    #[inline(always)]
    pub fn cap3fe(&mut self) -> CAP3FE_W {
        CAP3FE_W { w: self }
    }
    #[doc = "Bit 11 - Generate interrupt on channel 3 capture event: a CR3 load generates an interrupt."]
    #[inline(always)]
    pub fn cap3i(&mut self) -> CAP3I_W {
        CAP3I_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Capture Control Register. The CCR controls which edges of the capture inputs are used to load the Capture Registers and whether or not an interrupt is generated when a capture takes place.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ccr](index.html) module"]
pub struct CCR_SPEC;
impl crate::RegisterSpec for CCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ccr::R](R) reader structure"]
impl crate::Readable for CCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ccr::W](W) writer structure"]
impl crate::Writable for CCR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CCR to value 0"]
impl crate::Resettable for CCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
