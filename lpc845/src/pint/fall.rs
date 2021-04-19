#[doc = "Register `FALL` reader"]
pub struct R(crate::R<FALL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FALL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<FALL_SPEC>> for R {
    fn from(reader: crate::R<FALL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FALL` writer"]
pub struct W(crate::W<FALL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FALL_SPEC>;
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
impl core::convert::From<crate::W<FALL_SPEC>> for W {
    fn from(writer: crate::W<FALL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FDET` reader - Falling edge detect. Bit n detects the falling edge of the pin selected in PINTSELn. Read 0: No falling edge has been detected on this pin since Reset or the last time a one was written to this bit. Write 0: no operation. Read 1: a falling edge has been detected since Reset or the last time a one was written to this bit. Write 1: clear falling edge detection for this pin."]
pub struct FDET_R(crate::FieldReader<u8, u8>);
impl FDET_R {
    pub(crate) fn new(bits: u8) -> Self {
        FDET_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FDET_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FDET` writer - Falling edge detect. Bit n detects the falling edge of the pin selected in PINTSELn. Read 0: No falling edge has been detected on this pin since Reset or the last time a one was written to this bit. Write 0: no operation. Read 1: a falling edge has been detected since Reset or the last time a one was written to this bit. Write 1: clear falling edge detection for this pin."]
pub struct FDET_W<'a> {
    w: &'a mut W,
}
impl<'a> FDET_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Falling edge detect. Bit n detects the falling edge of the pin selected in PINTSELn. Read 0: No falling edge has been detected on this pin since Reset or the last time a one was written to this bit. Write 0: no operation. Read 1: a falling edge has been detected since Reset or the last time a one was written to this bit. Write 1: clear falling edge detection for this pin."]
    #[inline(always)]
    pub fn fdet(&self) -> FDET_R {
        FDET_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Falling edge detect. Bit n detects the falling edge of the pin selected in PINTSELn. Read 0: No falling edge has been detected on this pin since Reset or the last time a one was written to this bit. Write 0: no operation. Read 1: a falling edge has been detected since Reset or the last time a one was written to this bit. Write 1: clear falling edge detection for this pin."]
    #[inline(always)]
    pub fn fdet(&mut self) -> FDET_W {
        FDET_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Pin interrupt falling edge register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fall](index.html) module"]
pub struct FALL_SPEC;
impl crate::RegisterSpec for FALL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fall::R](R) reader structure"]
impl crate::Readable for FALL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fall::W](W) writer structure"]
impl crate::Writable for FALL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FALL to value 0"]
impl crate::Resettable for FALL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
