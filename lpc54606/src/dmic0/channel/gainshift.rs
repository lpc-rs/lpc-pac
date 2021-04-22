#[doc = "Register `GAINSHIFT` reader"]
pub struct R(crate::R<GAINSHIFT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GAINSHIFT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<GAINSHIFT_SPEC>> for R {
    fn from(reader: crate::R<GAINSHIFT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GAINSHIFT` writer"]
pub struct W(crate::W<GAINSHIFT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GAINSHIFT_SPEC>;
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
impl core::convert::From<crate::W<GAINSHIFT_SPEC>> for W {
    fn from(writer: crate::W<GAINSHIFT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `GAIN` reader - Gain control, as a positive or negative (two's complement) number of bits to shift."]
pub struct GAIN_R(crate::FieldReader<u8, u8>);
impl GAIN_R {
    pub(crate) fn new(bits: u8) -> Self {
        GAIN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GAIN_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GAIN` writer - Gain control, as a positive or negative (two's complement) number of bits to shift."]
pub struct GAIN_W<'a> {
    w: &'a mut W,
}
impl<'a> GAIN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | (value as u32 & 0x3f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:5 - Gain control, as a positive or negative (two's complement) number of bits to shift."]
    #[inline(always)]
    pub fn gain(&self) -> GAIN_R {
        GAIN_R::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - Gain control, as a positive or negative (two's complement) number of bits to shift."]
    #[inline(always)]
    pub fn gain(&mut self) -> GAIN_W {
        GAIN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Decimator Gain Shift register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gainshift](index.html) module"]
pub struct GAINSHIFT_SPEC;
impl crate::RegisterSpec for GAINSHIFT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gainshift::R](R) reader structure"]
impl crate::Readable for GAINSHIFT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gainshift::W](W) writer structure"]
impl crate::Writable for GAINSHIFT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets GAINSHIFT to value 0"]
impl crate::Resettable for GAINSHIFT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
