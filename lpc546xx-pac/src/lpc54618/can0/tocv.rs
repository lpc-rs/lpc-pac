#[doc = "Register `TOCV` reader"]
pub struct R(crate::R<TOCV_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TOCV_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TOCV_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TOCV_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `TOC` reader - Timeout counter."]
pub struct TOC_R(crate::FieldReader<u16, u16>);
impl TOC_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        TOC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TOC_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:15 - Timeout counter."]
    #[inline(always)]
    pub fn toc(&self) -> TOC_R {
        TOC_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "Timeout Counter Value\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tocv](index.html) module"]
pub struct TOCV_SPEC;
impl crate::RegisterSpec for TOCV_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tocv::R](R) reader structure"]
impl crate::Readable for TOCV_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets TOCV to value 0xffff"]
impl crate::Resettable for TOCV_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xffff
    }
}
