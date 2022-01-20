#[doc = "Register `TIMV` reader"]
pub struct R(crate::R<TIMV_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TIMV_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TIMV_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TIMV_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TIMV` writer"]
pub struct W(crate::W<TIMV_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TIMV_SPEC>;
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
impl From<crate::W<TIMV_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TIMV_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LPP` reader - Lines per panel."]
pub struct LPP_R(crate::FieldReader<u16, u16>);
impl LPP_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        LPP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LPP_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LPP` writer - Lines per panel."]
pub struct LPP_W<'a> {
    w: &'a mut W,
}
impl<'a> LPP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03ff) | (value as u32 & 0x03ff);
        self.w
    }
}
#[doc = "Field `VSW` reader - Vertical synchronization pulse width."]
pub struct VSW_R(crate::FieldReader<u8, u8>);
impl VSW_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        VSW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VSW_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VSW` writer - Vertical synchronization pulse width."]
pub struct VSW_W<'a> {
    w: &'a mut W,
}
impl<'a> VSW_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 10)) | ((value as u32 & 0x3f) << 10);
        self.w
    }
}
#[doc = "Field `VFP` reader - Vertical front porch."]
pub struct VFP_R(crate::FieldReader<u8, u8>);
impl VFP_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        VFP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VFP_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VFP` writer - Vertical front porch."]
pub struct VFP_W<'a> {
    w: &'a mut W,
}
impl<'a> VFP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | ((value as u32 & 0xff) << 16);
        self.w
    }
}
#[doc = "Field `VBP` reader - Vertical back porch."]
pub struct VBP_R(crate::FieldReader<u8, u8>);
impl VBP_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        VBP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VBP_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VBP` writer - Vertical back porch."]
pub struct VBP_W<'a> {
    w: &'a mut W,
}
impl<'a> VBP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 24)) | ((value as u32 & 0xff) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:9 - Lines per panel."]
    #[inline(always)]
    pub fn lpp(&self) -> LPP_R {
        LPP_R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 10:15 - Vertical synchronization pulse width."]
    #[inline(always)]
    pub fn vsw(&self) -> VSW_R {
        VSW_R::new(((self.bits >> 10) & 0x3f) as u8)
    }
    #[doc = "Bits 16:23 - Vertical front porch."]
    #[inline(always)]
    pub fn vfp(&self) -> VFP_R {
        VFP_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Vertical back porch."]
    #[inline(always)]
    pub fn vbp(&self) -> VBP_R {
        VBP_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:9 - Lines per panel."]
    #[inline(always)]
    pub fn lpp(&mut self) -> LPP_W {
        LPP_W { w: self }
    }
    #[doc = "Bits 10:15 - Vertical synchronization pulse width."]
    #[inline(always)]
    pub fn vsw(&mut self) -> VSW_W {
        VSW_W { w: self }
    }
    #[doc = "Bits 16:23 - Vertical front porch."]
    #[inline(always)]
    pub fn vfp(&mut self) -> VFP_W {
        VFP_W { w: self }
    }
    #[doc = "Bits 24:31 - Vertical back porch."]
    #[inline(always)]
    pub fn vbp(&mut self) -> VBP_W {
        VBP_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Vertical Timing Control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [timv](index.html) module"]
pub struct TIMV_SPEC;
impl crate::RegisterSpec for TIMV_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [timv::R](R) reader structure"]
impl crate::Readable for TIMV_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [timv::W](W) writer structure"]
impl crate::Writable for TIMV_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TIMV to value 0"]
impl crate::Resettable for TIMV_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
