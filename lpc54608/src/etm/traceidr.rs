#[doc = "Register `TRACEIDR` reader"]
pub struct R(crate::R<TRACEIDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TRACEIDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TRACEIDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TRACEIDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TRACEIDR` writer"]
pub struct W(crate::W<TRACEIDR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TRACEIDR_SPEC>;
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
impl From<crate::W<TRACEIDR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TRACEIDR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TraceID` reader - Trace ID to output onto the trace bus. On an ETM reset this field is cleared to 0x00."]
pub struct TRACEID_R(crate::FieldReader<u8, u8>);
impl TRACEID_R {
    pub(crate) fn new(bits: u8) -> Self {
        TRACEID_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TRACEID_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TraceID` writer - Trace ID to output onto the trace bus. On an ETM reset this field is cleared to 0x00."]
pub struct TRACEID_W<'a> {
    w: &'a mut W,
}
impl<'a> TRACEID_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x7f) | (value as u32 & 0x7f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:6 - Trace ID to output onto the trace bus. On an ETM reset this field is cleared to 0x00."]
    #[inline(always)]
    pub fn trace_id(&self) -> TRACEID_R {
        TRACEID_R::new((self.bits & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - Trace ID to output onto the trace bus. On an ETM reset this field is cleared to 0x00."]
    #[inline(always)]
    pub fn trace_id(&mut self) -> TRACEID_W {
        TRACEID_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CoreSight Trace ID Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [traceidr](index.html) module"]
pub struct TRACEIDR_SPEC;
impl crate::RegisterSpec for TRACEIDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [traceidr::R](R) reader structure"]
impl crate::Readable for TRACEIDR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [traceidr::W](W) writer structure"]
impl crate::Writable for TRACEIDR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TRACEIDR to value 0"]
impl crate::Resettable for TRACEIDR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
