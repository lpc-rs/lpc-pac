#[doc = "Register `TIMH` reader"]
pub struct R(crate::R<TIMH_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TIMH_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<TIMH_SPEC>> for R {
    fn from(reader: crate::R<TIMH_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TIMH` writer"]
pub struct W(crate::W<TIMH_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TIMH_SPEC>;
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
impl core::convert::From<crate::W<TIMH_SPEC>> for W {
    fn from(writer: crate::W<TIMH_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PPL` reader - Pixels-per-line."]
pub struct PPL_R(crate::FieldReader<u8, u8>);
impl PPL_R {
    pub(crate) fn new(bits: u8) -> Self {
        PPL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PPL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PPL` writer - Pixels-per-line."]
pub struct PPL_W<'a> {
    w: &'a mut W,
}
impl<'a> PPL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 2)) | ((value as u32 & 0x3f) << 2);
        self.w
    }
}
#[doc = "Field `HSW` reader - Horizontal synchronization pulse width."]
pub struct HSW_R(crate::FieldReader<u8, u8>);
impl HSW_R {
    pub(crate) fn new(bits: u8) -> Self {
        HSW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HSW_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HSW` writer - Horizontal synchronization pulse width."]
pub struct HSW_W<'a> {
    w: &'a mut W,
}
impl<'a> HSW_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | ((value as u32 & 0xff) << 8);
        self.w
    }
}
#[doc = "Field `HFP` reader - Horizontal front porch."]
pub struct HFP_R(crate::FieldReader<u8, u8>);
impl HFP_R {
    pub(crate) fn new(bits: u8) -> Self {
        HFP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HFP_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HFP` writer - Horizontal front porch."]
pub struct HFP_W<'a> {
    w: &'a mut W,
}
impl<'a> HFP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | ((value as u32 & 0xff) << 16);
        self.w
    }
}
#[doc = "Field `HBP` reader - Horizontal back porch."]
pub struct HBP_R(crate::FieldReader<u8, u8>);
impl HBP_R {
    pub(crate) fn new(bits: u8) -> Self {
        HBP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HBP_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HBP` writer - Horizontal back porch."]
pub struct HBP_W<'a> {
    w: &'a mut W,
}
impl<'a> HBP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 24)) | ((value as u32 & 0xff) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bits 2:7 - Pixels-per-line."]
    #[inline(always)]
    pub fn ppl(&self) -> PPL_R {
        PPL_R::new(((self.bits >> 2) & 0x3f) as u8)
    }
    #[doc = "Bits 8:15 - Horizontal synchronization pulse width."]
    #[inline(always)]
    pub fn hsw(&self) -> HSW_R {
        HSW_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Horizontal front porch."]
    #[inline(always)]
    pub fn hfp(&self) -> HFP_R {
        HFP_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Horizontal back porch."]
    #[inline(always)]
    pub fn hbp(&self) -> HBP_R {
        HBP_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 2:7 - Pixels-per-line."]
    #[inline(always)]
    pub fn ppl(&mut self) -> PPL_W {
        PPL_W { w: self }
    }
    #[doc = "Bits 8:15 - Horizontal synchronization pulse width."]
    #[inline(always)]
    pub fn hsw(&mut self) -> HSW_W {
        HSW_W { w: self }
    }
    #[doc = "Bits 16:23 - Horizontal front porch."]
    #[inline(always)]
    pub fn hfp(&mut self) -> HFP_W {
        HFP_W { w: self }
    }
    #[doc = "Bits 24:31 - Horizontal back porch."]
    #[inline(always)]
    pub fn hbp(&mut self) -> HBP_W {
        HBP_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Horizontal Timing Control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [timh](index.html) module"]
pub struct TIMH_SPEC;
impl crate::RegisterSpec for TIMH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [timh::R](R) reader structure"]
impl crate::Readable for TIMH_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [timh::W](W) writer structure"]
impl crate::Writable for TIMH_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TIMH to value 0"]
impl crate::Resettable for TIMH_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
