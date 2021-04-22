#[doc = "Register `SSP1CLKDIV` reader"]
pub struct R(crate::R<SSP1CLKDIV_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SSP1CLKDIV_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<SSP1CLKDIV_SPEC>> for R {
    fn from(reader: crate::R<SSP1CLKDIV_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SSP1CLKDIV` writer"]
pub struct W(crate::W<SSP1CLKDIV_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SSP1CLKDIV_SPEC>;
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
impl core::convert::From<crate::W<SSP1CLKDIV_SPEC>> for W {
    fn from(writer: crate::W<SSP1CLKDIV_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DIV` reader - SSP1_PCLK clock divider values 0: Disable SSP1_PCLK. 1: Divide by 1. to 255: Divide by 255."]
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
#[doc = "Field `DIV` writer - SSP1_PCLK clock divider values 0: Disable SSP1_PCLK. 1: Divide by 1. to 255: Divide by 255."]
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
    #[doc = "Bits 0:7 - SSP1_PCLK clock divider values 0: Disable SSP1_PCLK. 1: Divide by 1. to 255: Divide by 255."]
    #[inline(always)]
    pub fn div(&self) -> DIV_R {
        DIV_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - SSP1_PCLK clock divider values 0: Disable SSP1_PCLK. 1: Divide by 1. to 255: Divide by 255."]
    #[inline(always)]
    pub fn div(&mut self) -> DIV_W {
        DIV_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SSP1 clock divider\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ssp1clkdiv](index.html) module"]
pub struct SSP1CLKDIV_SPEC;
impl crate::RegisterSpec for SSP1CLKDIV_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ssp1clkdiv::R](R) reader structure"]
impl crate::Readable for SSP1CLKDIV_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ssp1clkdiv::W](W) writer structure"]
impl crate::Writable for SSP1CLKDIV_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SSP1CLKDIV to value 0"]
impl crate::Resettable for SSP1CLKDIV_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
