#[doc = "Register `IENR` reader"]
pub struct R(crate::R<IENR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IENR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IENR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IENR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IENR` writer"]
pub struct W(crate::W<IENR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IENR_SPEC>;
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
impl From<crate::W<IENR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IENR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ENRL` reader - Enables the rising edge or level interrupt for each pin interrupt. Bit n configures the pin interrupt selected in PINTSELn. 0 = Disable rising edge or level interrupt. 1 = Enable rising edge or level interrupt."]
pub struct ENRL_R(crate::FieldReader<u8, u8>);
impl ENRL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        ENRL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ENRL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ENRL` writer - Enables the rising edge or level interrupt for each pin interrupt. Bit n configures the pin interrupt selected in PINTSELn. 0 = Disable rising edge or level interrupt. 1 = Enable rising edge or level interrupt."]
pub struct ENRL_W<'a> {
    w: &'a mut W,
}
impl<'a> ENRL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Enables the rising edge or level interrupt for each pin interrupt. Bit n configures the pin interrupt selected in PINTSELn. 0 = Disable rising edge or level interrupt. 1 = Enable rising edge or level interrupt."]
    #[inline(always)]
    pub fn enrl(&self) -> ENRL_R {
        ENRL_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Enables the rising edge or level interrupt for each pin interrupt. Bit n configures the pin interrupt selected in PINTSELn. 0 = Disable rising edge or level interrupt. 1 = Enable rising edge or level interrupt."]
    #[inline(always)]
    pub fn enrl(&mut self) -> ENRL_W {
        ENRL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Pin interrupt level or rising edge interrupt enable register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ienr](index.html) module"]
pub struct IENR_SPEC;
impl crate::RegisterSpec for IENR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ienr::R](R) reader structure"]
impl crate::Readable for IENR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ienr::W](W) writer structure"]
impl crate::Writable for IENR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets IENR to value 0"]
impl crate::Resettable for IENR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
