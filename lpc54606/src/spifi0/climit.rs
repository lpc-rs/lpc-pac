#[doc = "Register `CLIMIT` reader"]
pub struct R(crate::R<CLIMIT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CLIMIT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CLIMIT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CLIMIT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CLIMIT` writer"]
pub struct W(crate::W<CLIMIT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CLIMIT_SPEC>;
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
impl From<crate::W<CLIMIT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CLIMIT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CLIMIT` reader - Zero-based upper limit of cacheable memory"]
pub struct CLIMIT_R(crate::FieldReader<u32, u32>);
impl CLIMIT_R {
    pub(crate) fn new(bits: u32) -> Self {
        CLIMIT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CLIMIT_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CLIMIT` writer - Zero-based upper limit of cacheable memory"]
pub struct CLIMIT_W<'a> {
    w: &'a mut W,
}
impl<'a> CLIMIT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | (value as u32 & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Zero-based upper limit of cacheable memory"]
    #[inline(always)]
    pub fn climit(&self) -> CLIMIT_R {
        CLIMIT_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Zero-based upper limit of cacheable memory"]
    #[inline(always)]
    pub fn climit(&mut self) -> CLIMIT_W {
        CLIMIT_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SPIFI limit register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [climit](index.html) module"]
pub struct CLIMIT_SPEC;
impl crate::RegisterSpec for CLIMIT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [climit::R](R) reader structure"]
impl crate::Readable for CLIMIT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [climit::W](W) writer structure"]
impl crate::Writable for CLIMIT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CLIMIT to value 0x0800_0000"]
impl crate::Resettable for CLIMIT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0800_0000
    }
}
