#[doc = "Register `CFG` reader"]
pub struct R(crate::R<CFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<CFG_SPEC>> for R {
    fn from(reader: crate::R<CFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CFG` writer"]
pub struct W(crate::W<CFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CFG_SPEC>;
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
impl core::convert::From<crate::W<CFG_SPEC>> for W {
    fn from(writer: crate::W<CFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CAPEN0` reader - Enable Capture 0. 1 = Enabled, 0 = Disabled."]
pub struct CAPEN0_R(crate::FieldReader<bool, bool>);
impl CAPEN0_R {
    pub(crate) fn new(bits: bool) -> Self {
        CAPEN0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CAPEN0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CAPEN0` writer - Enable Capture 0. 1 = Enabled, 0 = Disabled."]
pub struct CAPEN0_W<'a> {
    w: &'a mut W,
}
impl<'a> CAPEN0_W<'a> {
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
#[doc = "Field `CAPEN1` reader - Enable Capture 1. 1 = Enabled, 0 = Disabled."]
pub struct CAPEN1_R(crate::FieldReader<bool, bool>);
impl CAPEN1_R {
    pub(crate) fn new(bits: bool) -> Self {
        CAPEN1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CAPEN1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CAPEN1` writer - Enable Capture 1. 1 = Enabled, 0 = Disabled."]
pub struct CAPEN1_W<'a> {
    w: &'a mut W,
}
impl<'a> CAPEN1_W<'a> {
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
#[doc = "Field `CAPEN2` reader - Enable Capture 2. 1 = Enabled, 0 = Disabled."]
pub struct CAPEN2_R(crate::FieldReader<bool, bool>);
impl CAPEN2_R {
    pub(crate) fn new(bits: bool) -> Self {
        CAPEN2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CAPEN2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CAPEN2` writer - Enable Capture 2. 1 = Enabled, 0 = Disabled."]
pub struct CAPEN2_W<'a> {
    w: &'a mut W,
}
impl<'a> CAPEN2_W<'a> {
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
#[doc = "Field `CAPEN3` reader - Enable Capture 3. 1 = Enabled, 0 = Disabled."]
pub struct CAPEN3_R(crate::FieldReader<bool, bool>);
impl CAPEN3_R {
    pub(crate) fn new(bits: bool) -> Self {
        CAPEN3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CAPEN3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CAPEN3` writer - Enable Capture 3. 1 = Enabled, 0 = Disabled."]
pub struct CAPEN3_W<'a> {
    w: &'a mut W,
}
impl<'a> CAPEN3_W<'a> {
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
#[doc = "Field `CAPPOL0` reader - Capture Polarity 0. 0 = Positive edge capture, 1 = Negative edge capture."]
pub struct CAPPOL0_R(crate::FieldReader<bool, bool>);
impl CAPPOL0_R {
    pub(crate) fn new(bits: bool) -> Self {
        CAPPOL0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CAPPOL0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CAPPOL0` writer - Capture Polarity 0. 0 = Positive edge capture, 1 = Negative edge capture."]
pub struct CAPPOL0_W<'a> {
    w: &'a mut W,
}
impl<'a> CAPPOL0_W<'a> {
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
#[doc = "Field `CAPPOL1` reader - Capture Polarity 1. 0 = Positive edge capture, 1 = Negative edge capture."]
pub struct CAPPOL1_R(crate::FieldReader<bool, bool>);
impl CAPPOL1_R {
    pub(crate) fn new(bits: bool) -> Self {
        CAPPOL1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CAPPOL1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CAPPOL1` writer - Capture Polarity 1. 0 = Positive edge capture, 1 = Negative edge capture."]
pub struct CAPPOL1_W<'a> {
    w: &'a mut W,
}
impl<'a> CAPPOL1_W<'a> {
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
#[doc = "Field `CAPPOL2` reader - Capture Polarity 2. 0 = Positive edge capture, 1 = Negative edge capture."]
pub struct CAPPOL2_R(crate::FieldReader<bool, bool>);
impl CAPPOL2_R {
    pub(crate) fn new(bits: bool) -> Self {
        CAPPOL2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CAPPOL2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CAPPOL2` writer - Capture Polarity 2. 0 = Positive edge capture, 1 = Negative edge capture."]
pub struct CAPPOL2_W<'a> {
    w: &'a mut W,
}
impl<'a> CAPPOL2_W<'a> {
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
#[doc = "Field `CAPPOL3` reader - Capture Polarity 3. 0 = Positive edge capture, 1 = Negative edge capture."]
pub struct CAPPOL3_R(crate::FieldReader<bool, bool>);
impl CAPPOL3_R {
    pub(crate) fn new(bits: bool) -> Self {
        CAPPOL3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CAPPOL3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CAPPOL3` writer - Capture Polarity 3. 0 = Positive edge capture, 1 = Negative edge capture."]
pub struct CAPPOL3_W<'a> {
    w: &'a mut W,
}
impl<'a> CAPPOL3_W<'a> {
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
    #[doc = "Bit 0 - Enable Capture 0. 1 = Enabled, 0 = Disabled."]
    #[inline(always)]
    pub fn capen0(&self) -> CAPEN0_R {
        CAPEN0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Enable Capture 1. 1 = Enabled, 0 = Disabled."]
    #[inline(always)]
    pub fn capen1(&self) -> CAPEN1_R {
        CAPEN1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Enable Capture 2. 1 = Enabled, 0 = Disabled."]
    #[inline(always)]
    pub fn capen2(&self) -> CAPEN2_R {
        CAPEN2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Enable Capture 3. 1 = Enabled, 0 = Disabled."]
    #[inline(always)]
    pub fn capen3(&self) -> CAPEN3_R {
        CAPEN3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Capture Polarity 0. 0 = Positive edge capture, 1 = Negative edge capture."]
    #[inline(always)]
    pub fn cappol0(&self) -> CAPPOL0_R {
        CAPPOL0_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Capture Polarity 1. 0 = Positive edge capture, 1 = Negative edge capture."]
    #[inline(always)]
    pub fn cappol1(&self) -> CAPPOL1_R {
        CAPPOL1_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Capture Polarity 2. 0 = Positive edge capture, 1 = Negative edge capture."]
    #[inline(always)]
    pub fn cappol2(&self) -> CAPPOL2_R {
        CAPPOL2_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Capture Polarity 3. 0 = Positive edge capture, 1 = Negative edge capture."]
    #[inline(always)]
    pub fn cappol3(&self) -> CAPPOL3_R {
        CAPPOL3_R::new(((self.bits >> 11) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable Capture 0. 1 = Enabled, 0 = Disabled."]
    #[inline(always)]
    pub fn capen0(&mut self) -> CAPEN0_W {
        CAPEN0_W { w: self }
    }
    #[doc = "Bit 1 - Enable Capture 1. 1 = Enabled, 0 = Disabled."]
    #[inline(always)]
    pub fn capen1(&mut self) -> CAPEN1_W {
        CAPEN1_W { w: self }
    }
    #[doc = "Bit 2 - Enable Capture 2. 1 = Enabled, 0 = Disabled."]
    #[inline(always)]
    pub fn capen2(&mut self) -> CAPEN2_W {
        CAPEN2_W { w: self }
    }
    #[doc = "Bit 3 - Enable Capture 3. 1 = Enabled, 0 = Disabled."]
    #[inline(always)]
    pub fn capen3(&mut self) -> CAPEN3_W {
        CAPEN3_W { w: self }
    }
    #[doc = "Bit 8 - Capture Polarity 0. 0 = Positive edge capture, 1 = Negative edge capture."]
    #[inline(always)]
    pub fn cappol0(&mut self) -> CAPPOL0_W {
        CAPPOL0_W { w: self }
    }
    #[doc = "Bit 9 - Capture Polarity 1. 0 = Positive edge capture, 1 = Negative edge capture."]
    #[inline(always)]
    pub fn cappol1(&mut self) -> CAPPOL1_W {
        CAPPOL1_W { w: self }
    }
    #[doc = "Bit 10 - Capture Polarity 2. 0 = Positive edge capture, 1 = Negative edge capture."]
    #[inline(always)]
    pub fn cappol2(&mut self) -> CAPPOL2_W {
        CAPPOL2_W { w: self }
    }
    #[doc = "Bit 11 - Capture Polarity 3. 0 = Positive edge capture, 1 = Negative edge capture."]
    #[inline(always)]
    pub fn cappol3(&mut self) -> CAPPOL3_W {
        CAPPOL3_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Capture configuration register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CFG_SPEC;
impl crate::RegisterSpec for CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cfg::R](R) reader structure"]
impl crate::Readable for CFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cfg::W](W) writer structure"]
impl crate::Writable for CFG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CFG to value 0"]
impl crate::Resettable for CFG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
