#[doc = "Register `ETSCV` reader"]
pub struct R(crate::R<ETSCV_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ETSCV_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<ETSCV_SPEC>> for R {
    fn from(reader: crate::R<ETSCV_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ETSCV` writer"]
pub struct W(crate::W<ETSCV_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ETSCV_SPEC>;
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
impl core::convert::From<crate::W<ETSCV_SPEC>> for W {
    fn from(writer: crate::W<ETSCV_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ETSC` reader - External timestamp counter."]
pub struct ETSC_R(crate::FieldReader<u16, u16>);
impl ETSC_R {
    pub(crate) fn new(bits: u16) -> Self {
        ETSC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ETSC_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ETSC` writer - External timestamp counter."]
pub struct ETSC_W<'a> {
    w: &'a mut W,
}
impl<'a> ETSC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u32 & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - External timestamp counter."]
    #[inline(always)]
    pub fn etsc(&self) -> ETSC_R {
        ETSC_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - External timestamp counter."]
    #[inline(always)]
    pub fn etsc(&mut self) -> ETSC_W {
        ETSC_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "External Timestamp Counter Value\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [etscv](index.html) module"]
pub struct ETSCV_SPEC;
impl crate::RegisterSpec for ETSCV_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [etscv::R](R) reader structure"]
impl crate::Readable for ETSCV_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [etscv::W](W) writer structure"]
impl crate::Writable for ETSCV_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ETSCV to value 0"]
impl crate::Resettable for ETSCV_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
