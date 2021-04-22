#[doc = "Register `RESP[%s]` reader"]
pub struct R(crate::R<RESP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RESP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<RESP_SPEC>> for R {
    fn from(reader: crate::R<RESP_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RESP[%s]` writer"]
pub struct W(crate::W<RESP_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RESP_SPEC>;
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
impl core::convert::From<crate::W<RESP_SPEC>> for W {
    fn from(writer: crate::W<RESP_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RESPONSE` reader - Bits of response."]
pub struct RESPONSE_R(crate::FieldReader<u32, u32>);
impl RESPONSE_R {
    pub(crate) fn new(bits: u32) -> Self {
        RESPONSE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RESPONSE_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RESPONSE` writer - Bits of response."]
pub struct RESPONSE_W<'a> {
    w: &'a mut W,
}
impl<'a> RESPONSE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | (value as u32 & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Bits of response."]
    #[inline(always)]
    pub fn response(&self) -> RESPONSE_R {
        RESPONSE_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Bits of response."]
    #[inline(always)]
    pub fn response(&mut self) -> RESPONSE_W {
        RESPONSE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Response register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [resp](index.html) module"]
pub struct RESP_SPEC;
impl crate::RegisterSpec for RESP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [resp::R](R) reader structure"]
impl crate::Readable for RESP_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [resp::W](W) writer structure"]
impl crate::Writable for RESP_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RESP[%s]
to value 0"]
impl crate::Resettable for RESP_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
