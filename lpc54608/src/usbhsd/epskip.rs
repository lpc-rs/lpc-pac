#[doc = "Register `EPSKIP` reader"]
pub struct R(crate::R<EPSKIP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EPSKIP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<EPSKIP_SPEC>> for R {
    fn from(reader: crate::R<EPSKIP_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EPSKIP` writer"]
pub struct W(crate::W<EPSKIP_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EPSKIP_SPEC>;
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
impl core::convert::From<crate::W<EPSKIP_SPEC>> for W {
    fn from(writer: crate::W<EPSKIP_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SKIP` reader - Endpoint skip: Writing 1 to one of these bits, will indicate to HW that it must deactivate the buffer assigned to this endpoint and return control back to software."]
pub struct SKIP_R(crate::FieldReader<u16, u16>);
impl SKIP_R {
    pub(crate) fn new(bits: u16) -> Self {
        SKIP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SKIP_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SKIP` writer - Endpoint skip: Writing 1 to one of these bits, will indicate to HW that it must deactivate the buffer assigned to this endpoint and return control back to software."]
pub struct SKIP_W<'a> {
    w: &'a mut W,
}
impl<'a> SKIP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0fff) | (value as u32 & 0x0fff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:11 - Endpoint skip: Writing 1 to one of these bits, will indicate to HW that it must deactivate the buffer assigned to this endpoint and return control back to software."]
    #[inline(always)]
    pub fn skip(&self) -> SKIP_R {
        SKIP_R::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - Endpoint skip: Writing 1 to one of these bits, will indicate to HW that it must deactivate the buffer assigned to this endpoint and return control back to software."]
    #[inline(always)]
    pub fn skip(&mut self) -> SKIP_W {
        SKIP_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USB Endpoint skip\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [epskip](index.html) module"]
pub struct EPSKIP_SPEC;
impl crate::RegisterSpec for EPSKIP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [epskip::R](R) reader structure"]
impl crate::Readable for EPSKIP_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [epskip::W](W) writer structure"]
impl crate::Writable for EPSKIP_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets EPSKIP to value 0"]
impl crate::Resettable for EPSKIP_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
