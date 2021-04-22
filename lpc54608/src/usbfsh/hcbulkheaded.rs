#[doc = "Register `HCBULKHEADED` reader"]
pub struct R(crate::R<HCBULKHEADED_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HCBULKHEADED_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<HCBULKHEADED_SPEC>> for R {
    fn from(reader: crate::R<HCBULKHEADED_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HCBULKHEADED` writer"]
pub struct W(crate::W<HCBULKHEADED_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HCBULKHEADED_SPEC>;
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
impl core::convert::From<crate::W<HCBULKHEADED_SPEC>> for W {
    fn from(writer: crate::W<HCBULKHEADED_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BHED` reader - BulkHeadED HC traverses the bulk list starting with the HcBulkHeadED pointer."]
pub struct BHED_R(crate::FieldReader<u32, u32>);
impl BHED_R {
    pub(crate) fn new(bits: u32) -> Self {
        BHED_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BHED_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BHED` writer - BulkHeadED HC traverses the bulk list starting with the HcBulkHeadED pointer."]
pub struct BHED_W<'a> {
    w: &'a mut W,
}
impl<'a> BHED_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0fff_ffff << 4)) | ((value as u32 & 0x0fff_ffff) << 4);
        self.w
    }
}
impl R {
    #[doc = "Bits 4:31 - BulkHeadED HC traverses the bulk list starting with the HcBulkHeadED pointer."]
    #[inline(always)]
    pub fn bhed(&self) -> BHED_R {
        BHED_R::new(((self.bits >> 4) & 0x0fff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 4:31 - BulkHeadED HC traverses the bulk list starting with the HcBulkHeadED pointer."]
    #[inline(always)]
    pub fn bhed(&mut self) -> BHED_W {
        BHED_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Contains the physical address of the first endpoint descriptor of the bulk list\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hcbulkheaded](index.html) module"]
pub struct HCBULKHEADED_SPEC;
impl crate::RegisterSpec for HCBULKHEADED_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hcbulkheaded::R](R) reader structure"]
impl crate::Readable for HCBULKHEADED_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hcbulkheaded::W](W) writer structure"]
impl crate::Writable for HCBULKHEADED_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets HCBULKHEADED to value 0"]
impl crate::Resettable for HCBULKHEADED_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
