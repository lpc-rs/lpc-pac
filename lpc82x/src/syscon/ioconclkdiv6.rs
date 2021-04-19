#[doc = "Register `IOCONCLKDIV6` reader"]
pub struct R(crate::R<IOCONCLKDIV6_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IOCONCLKDIV6_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<IOCONCLKDIV6_SPEC>> for R {
    fn from(reader: crate::R<IOCONCLKDIV6_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IOCONCLKDIV6` writer"]
pub struct W(crate::W<IOCONCLKDIV6_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IOCONCLKDIV6_SPEC>;
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
impl core::convert::From<crate::W<IOCONCLKDIV6_SPEC>> for W {
    fn from(writer: crate::W<IOCONCLKDIV6_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DIV` reader - IOCON glitch filter clock divider values 0: Disable IOCONFILTR_PCLK. 1: Divide by 1. to 255: Divide by 255."]
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
#[doc = "Field `DIV` writer - IOCON glitch filter clock divider values 0: Disable IOCONFILTR_PCLK. 1: Divide by 1. to 255: Divide by 255."]
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
    #[doc = "Bits 0:7 - IOCON glitch filter clock divider values 0: Disable IOCONFILTR_PCLK. 1: Divide by 1. to 255: Divide by 255."]
    #[inline(always)]
    pub fn div(&self) -> DIV_R {
        DIV_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - IOCON glitch filter clock divider values 0: Disable IOCONFILTR_PCLK. 1: Divide by 1. to 255: Divide by 255."]
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
#[doc = "Peripheral clock 6 to the IOCON block for programmable glitch filter\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ioconclkdiv6](index.html) module"]
pub struct IOCONCLKDIV6_SPEC;
impl crate::RegisterSpec for IOCONCLKDIV6_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ioconclkdiv6::R](R) reader structure"]
impl crate::Readable for IOCONCLKDIV6_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ioconclkdiv6::W](W) writer structure"]
impl crate::Writable for IOCONCLKDIV6_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets IOCONCLKDIV6 to value 0"]
impl crate::Resettable for IOCONCLKDIV6_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
