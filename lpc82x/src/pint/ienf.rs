#[doc = "Register `IENF` reader"]
pub struct R(crate::R<IENF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IENF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<IENF_SPEC>> for R {
    fn from(reader: crate::R<IENF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IENF` writer"]
pub struct W(crate::W<IENF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IENF_SPEC>;
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
impl core::convert::From<crate::W<IENF_SPEC>> for W {
    fn from(writer: crate::W<IENF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ENAF` reader - Enables the falling edge or configures the active level interrupt for each pin interrupt. Bit n configures the pin interrupt selected in PINTSELn. 0 = Disable falling edge interrupt or set active interrupt level LOW. 1 = Enable falling edge interrupt enabled or set active interrupt level HIGH."]
pub struct ENAF_R(crate::FieldReader<u8, u8>);
impl ENAF_R {
    pub(crate) fn new(bits: u8) -> Self {
        ENAF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ENAF_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ENAF` writer - Enables the falling edge or configures the active level interrupt for each pin interrupt. Bit n configures the pin interrupt selected in PINTSELn. 0 = Disable falling edge interrupt or set active interrupt level LOW. 1 = Enable falling edge interrupt enabled or set active interrupt level HIGH."]
pub struct ENAF_W<'a> {
    w: &'a mut W,
}
impl<'a> ENAF_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Enables the falling edge or configures the active level interrupt for each pin interrupt. Bit n configures the pin interrupt selected in PINTSELn. 0 = Disable falling edge interrupt or set active interrupt level LOW. 1 = Enable falling edge interrupt enabled or set active interrupt level HIGH."]
    #[inline(always)]
    pub fn enaf(&self) -> ENAF_R {
        ENAF_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Enables the falling edge or configures the active level interrupt for each pin interrupt. Bit n configures the pin interrupt selected in PINTSELn. 0 = Disable falling edge interrupt or set active interrupt level LOW. 1 = Enable falling edge interrupt enabled or set active interrupt level HIGH."]
    #[inline(always)]
    pub fn enaf(&mut self) -> ENAF_W {
        ENAF_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Pin interrupt active level or falling edge interrupt enable register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ienf](index.html) module"]
pub struct IENF_SPEC;
impl crate::RegisterSpec for IENF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ienf::R](R) reader structure"]
impl crate::Readable for IENF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ienf::W](W) writer structure"]
impl crate::Writable for IENF_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets IENF to value 0"]
impl crate::Resettable for IENF_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
