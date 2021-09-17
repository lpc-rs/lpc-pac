#[doc = "Register `AUTOCGOR` reader"]
pub struct R(crate::R<AUTOCGOR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AUTOCGOR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AUTOCGOR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AUTOCGOR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `AUTOCGOR` writer"]
pub struct W(crate::W<AUTOCGOR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AUTOCGOR_SPEC>;
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
impl From<crate::W<AUTOCGOR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AUTOCGOR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RAM0X` reader - When 1, automatic clock gating for RAMX and RAM0 are turned off."]
pub struct RAM0X_R(crate::FieldReader<bool, bool>);
impl RAM0X_R {
    pub(crate) fn new(bits: bool) -> Self {
        RAM0X_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RAM0X_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RAM0X` writer - When 1, automatic clock gating for RAMX and RAM0 are turned off."]
pub struct RAM0X_W<'a> {
    w: &'a mut W,
}
impl<'a> RAM0X_W<'a> {
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
#[doc = "Field `RAM1` reader - When 1, automatic clock gating for RAM1 are turned off."]
pub struct RAM1_R(crate::FieldReader<bool, bool>);
impl RAM1_R {
    pub(crate) fn new(bits: bool) -> Self {
        RAM1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RAM1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RAM1` writer - When 1, automatic clock gating for RAM1 are turned off."]
pub struct RAM1_W<'a> {
    w: &'a mut W,
}
impl<'a> RAM1_W<'a> {
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
#[doc = "Field `RAM2` reader - When 1, automatic clock gating for RAM1 are turned off."]
pub struct RAM2_R(crate::FieldReader<bool, bool>);
impl RAM2_R {
    pub(crate) fn new(bits: bool) -> Self {
        RAM2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RAM2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RAM2` writer - When 1, automatic clock gating for RAM1 are turned off."]
pub struct RAM2_W<'a> {
    w: &'a mut W,
}
impl<'a> RAM2_W<'a> {
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
#[doc = "Field `RAM3` reader - When 1, automatic clock gating for RAM1 are turned off."]
pub struct RAM3_R(crate::FieldReader<bool, bool>);
impl RAM3_R {
    pub(crate) fn new(bits: bool) -> Self {
        RAM3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RAM3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RAM3` writer - When 1, automatic clock gating for RAM1 are turned off."]
pub struct RAM3_W<'a> {
    w: &'a mut W,
}
impl<'a> RAM3_W<'a> {
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
impl R {
    #[doc = "Bit 1 - When 1, automatic clock gating for RAMX and RAM0 are turned off."]
    #[inline(always)]
    pub fn ram0x(&self) -> RAM0X_R {
        RAM0X_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - When 1, automatic clock gating for RAM1 are turned off."]
    #[inline(always)]
    pub fn ram1(&self) -> RAM1_R {
        RAM1_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - When 1, automatic clock gating for RAM1 are turned off."]
    #[inline(always)]
    pub fn ram2(&self) -> RAM2_R {
        RAM2_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - When 1, automatic clock gating for RAM1 are turned off."]
    #[inline(always)]
    pub fn ram3(&self) -> RAM3_R {
        RAM3_R::new(((self.bits >> 4) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - When 1, automatic clock gating for RAMX and RAM0 are turned off."]
    #[inline(always)]
    pub fn ram0x(&mut self) -> RAM0X_W {
        RAM0X_W { w: self }
    }
    #[doc = "Bit 2 - When 1, automatic clock gating for RAM1 are turned off."]
    #[inline(always)]
    pub fn ram1(&mut self) -> RAM1_W {
        RAM1_W { w: self }
    }
    #[doc = "Bit 3 - When 1, automatic clock gating for RAM1 are turned off."]
    #[inline(always)]
    pub fn ram2(&mut self) -> RAM2_W {
        RAM2_W { w: self }
    }
    #[doc = "Bit 4 - When 1, automatic clock gating for RAM1 are turned off."]
    #[inline(always)]
    pub fn ram3(&mut self) -> RAM3_W {
        RAM3_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Auto Clock-Gate Override Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [autocgor](index.html) module"]
pub struct AUTOCGOR_SPEC;
impl crate::RegisterSpec for AUTOCGOR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [autocgor::R](R) reader structure"]
impl crate::Readable for AUTOCGOR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [autocgor::W](W) writer structure"]
impl crate::Writable for AUTOCGOR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets AUTOCGOR to value 0"]
impl crate::Resettable for AUTOCGOR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
