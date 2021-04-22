#[doc = "Register `ISEL` reader"]
pub struct R(crate::R<ISEL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ISEL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<ISEL_SPEC>> for R {
    fn from(reader: crate::R<ISEL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ISEL` writer"]
pub struct W(crate::W<ISEL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ISEL_SPEC>;
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
impl core::convert::From<crate::W<ISEL_SPEC>> for W {
    fn from(writer: crate::W<ISEL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PMODE` reader - Selects the interrupt mode for each pin interrupt. Bit n configures the pin interrupt selected in PINTSELn. 0 = Edge sensitive 1 = Level sensitive"]
pub struct PMODE_R(crate::FieldReader<u8, u8>);
impl PMODE_R {
    pub(crate) fn new(bits: u8) -> Self {
        PMODE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PMODE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PMODE` writer - Selects the interrupt mode for each pin interrupt. Bit n configures the pin interrupt selected in PINTSELn. 0 = Edge sensitive 1 = Level sensitive"]
pub struct PMODE_W<'a> {
    w: &'a mut W,
}
impl<'a> PMODE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Selects the interrupt mode for each pin interrupt. Bit n configures the pin interrupt selected in PINTSELn. 0 = Edge sensitive 1 = Level sensitive"]
    #[inline(always)]
    pub fn pmode(&self) -> PMODE_R {
        PMODE_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Selects the interrupt mode for each pin interrupt. Bit n configures the pin interrupt selected in PINTSELn. 0 = Edge sensitive 1 = Level sensitive"]
    #[inline(always)]
    pub fn pmode(&mut self) -> PMODE_W {
        PMODE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Pin Interrupt Mode register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [isel](index.html) module"]
pub struct ISEL_SPEC;
impl crate::RegisterSpec for ISEL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [isel::R](R) reader structure"]
impl crate::Readable for ISEL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [isel::W](W) writer structure"]
impl crate::Writable for ISEL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ISEL to value 0"]
impl crate::Resettable for ISEL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
