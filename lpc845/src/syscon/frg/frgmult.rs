#[doc = "Register `FRGMULT` reader"]
pub struct R(crate::R<FRGMULT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FRGMULT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<FRGMULT_SPEC>> for R {
    fn from(reader: crate::R<FRGMULT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FRGMULT` writer"]
pub struct W(crate::W<FRGMULT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FRGMULT_SPEC>;
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
impl core::convert::From<crate::W<FRGMULT_SPEC>> for W {
    fn from(writer: crate::W<FRGMULT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MULT` reader - Numerator of the fractional divider. MULT is equal to the programmed value."]
pub struct MULT_R(crate::FieldReader<u8, u8>);
impl MULT_R {
    pub(crate) fn new(bits: u8) -> Self {
        MULT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MULT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MULT` writer - Numerator of the fractional divider. MULT is equal to the programmed value."]
pub struct MULT_W<'a> {
    w: &'a mut W,
}
impl<'a> MULT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Numerator of the fractional divider. MULT is equal to the programmed value."]
    #[inline(always)]
    pub fn mult(&self) -> MULT_R {
        MULT_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Numerator of the fractional divider. MULT is equal to the programmed value."]
    #[inline(always)]
    pub fn mult(&mut self) -> MULT_W {
        MULT_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "fractional generator N multiplier value register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [frgmult](index.html) module"]
pub struct FRGMULT_SPEC;
impl crate::RegisterSpec for FRGMULT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [frgmult::R](R) reader structure"]
impl crate::Readable for FRGMULT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [frgmult::W](W) writer structure"]
impl crate::Writable for FRGMULT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FRGMULT to value 0"]
impl crate::Resettable for FRGMULT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
