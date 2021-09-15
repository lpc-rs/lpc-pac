#[doc = "Register `HCFMNUMBER` reader"]
pub struct R(crate::R<HCFMNUMBER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HCFMNUMBER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HCFMNUMBER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HCFMNUMBER_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HCFMNUMBER` writer"]
pub struct W(crate::W<HCFMNUMBER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HCFMNUMBER_SPEC>;
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
impl From<crate::W<HCFMNUMBER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HCFMNUMBER_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FN` reader - FrameNumber This is incremented when HcFmRemaining is re-loaded."]
pub struct FN_R(crate::FieldReader<u16, u16>);
impl FN_R {
    pub(crate) fn new(bits: u16) -> Self {
        FN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FN_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FN` writer - FrameNumber This is incremented when HcFmRemaining is re-loaded."]
pub struct FN_W<'a> {
    w: &'a mut W,
}
impl<'a> FN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u32 & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - FrameNumber This is incremented when HcFmRemaining is re-loaded."]
    #[inline(always)]
    pub fn fn_(&self) -> FN_R {
        FN_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - FrameNumber This is incremented when HcFmRemaining is re-loaded."]
    #[inline(always)]
    pub fn fn_(&mut self) -> FN_W {
        FN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Contains a 16-bit counter and provides the timing reference among events happening in the HC and the HCD\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hcfmnumber](index.html) module"]
pub struct HCFMNUMBER_SPEC;
impl crate::RegisterSpec for HCFMNUMBER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hcfmnumber::R](R) reader structure"]
impl crate::Readable for HCFMNUMBER_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hcfmnumber::W](W) writer structure"]
impl crate::Writable for HCFMNUMBER_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets HCFMNUMBER to value 0"]
impl crate::Resettable for HCFMNUMBER_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
