#[doc = "Register `ADCCLKDIV` reader"]
pub struct R(crate::R<ADCCLKDIV_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADCCLKDIV_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADCCLKDIV_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADCCLKDIV_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADCCLKDIV` writer"]
pub struct W(crate::W<ADCCLKDIV_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADCCLKDIV_SPEC>;
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
impl From<crate::W<ADCCLKDIV_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADCCLKDIV_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DIV` reader - ADC clock divider values 0: ADC clock disabled. 1: Divide by 1. to 255: Divide by 255."]
pub struct DIV_R(crate::FieldReader<u8, u8>);
impl DIV_R {
    pub(crate) fn new(bits: u8) -> Self {
        DIV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DIV_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DIV` writer - ADC clock divider values 0: ADC clock disabled. 1: Divide by 1. to 255: Divide by 255."]
pub struct DIV_W<'a> {
    w: &'a mut W,
}
impl<'a> DIV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - ADC clock divider values 0: ADC clock disabled. 1: Divide by 1. to 255: Divide by 255."]
    #[inline(always)]
    pub fn div(&self) -> DIV_R {
        DIV_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - ADC clock divider values 0: ADC clock disabled. 1: Divide by 1. to 255: Divide by 255."]
    #[inline(always)]
    pub fn div(&mut self) -> DIV_W {
        DIV_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ADC clock divider register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adcclkdiv](index.html) module"]
pub struct ADCCLKDIV_SPEC;
impl crate::RegisterSpec for ADCCLKDIV_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [adcclkdiv::R](R) reader structure"]
impl crate::Readable for ADCCLKDIV_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adcclkdiv::W](W) writer structure"]
impl crate::Writable for ADCCLKDIV_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ADCCLKDIV to value 0"]
impl crate::Resettable for ADCCLKDIV_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
