#[doc = "Register `RISE` reader"]
pub struct R(crate::R<RISE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RISE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<RISE_SPEC>> for R {
    fn from(reader: crate::R<RISE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RISE` writer"]
pub struct W(crate::W<RISE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RISE_SPEC>;
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
impl core::convert::From<crate::W<RISE_SPEC>> for W {
    fn from(writer: crate::W<RISE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RDET` reader - Rising edge detect. Bit n detects the rising edge of the pin selected in PINTSELn. Read 0: No rising edge has been detected on this pin since Reset or the last time a one was written to this bit. Write 0: no operation. Read 1: a rising edge has been detected since Reset or the last time a one was written to this bit. Write 1: clear rising edge detection for this pin."]
pub struct RDET_R(crate::FieldReader<u8, u8>);
impl RDET_R {
    pub(crate) fn new(bits: u8) -> Self {
        RDET_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RDET_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RDET` writer - Rising edge detect. Bit n detects the rising edge of the pin selected in PINTSELn. Read 0: No rising edge has been detected on this pin since Reset or the last time a one was written to this bit. Write 0: no operation. Read 1: a rising edge has been detected since Reset or the last time a one was written to this bit. Write 1: clear rising edge detection for this pin."]
pub struct RDET_W<'a> {
    w: &'a mut W,
}
impl<'a> RDET_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Rising edge detect. Bit n detects the rising edge of the pin selected in PINTSELn. Read 0: No rising edge has been detected on this pin since Reset or the last time a one was written to this bit. Write 0: no operation. Read 1: a rising edge has been detected since Reset or the last time a one was written to this bit. Write 1: clear rising edge detection for this pin."]
    #[inline(always)]
    pub fn rdet(&self) -> RDET_R {
        RDET_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Rising edge detect. Bit n detects the rising edge of the pin selected in PINTSELn. Read 0: No rising edge has been detected on this pin since Reset or the last time a one was written to this bit. Write 0: no operation. Read 1: a rising edge has been detected since Reset or the last time a one was written to this bit. Write 1: clear rising edge detection for this pin."]
    #[inline(always)]
    pub fn rdet(&mut self) -> RDET_W {
        RDET_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Pin interrupt rising edge register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rise](index.html) module"]
pub struct RISE_SPEC;
impl crate::RegisterSpec for RISE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rise::R](R) reader structure"]
impl crate::Readable for RISE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rise::W](W) writer structure"]
impl crate::Writable for RISE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RISE to value 0"]
impl crate::Resettable for RISE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
