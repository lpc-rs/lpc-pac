#[doc = "Register `DLM` reader"]
pub struct R(crate::R<DLM_IER_DLM_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DLM_IER_DLM_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<DLM_IER_DLM_SPEC>> for R {
    fn from(reader: crate::R<DLM_IER_DLM_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DLM` writer"]
pub struct W(crate::W<DLM_IER_DLM_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DLM_IER_DLM_SPEC>;
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
impl core::convert::From<crate::W<DLM_IER_DLM_SPEC>> for W {
    fn from(writer: crate::W<DLM_IER_DLM_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DLMSB` reader - The SCIn Divisor Latch MSB Register, along with the DLL register, determines the baud rate of the SCIn."]
pub struct DLMSB_R(crate::FieldReader<u8, u8>);
impl DLMSB_R {
    pub(crate) fn new(bits: u8) -> Self {
        DLMSB_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DLMSB_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DLMSB` writer - The SCIn Divisor Latch MSB Register, along with the DLL register, determines the baud rate of the SCIn."]
pub struct DLMSB_W<'a> {
    w: &'a mut W,
}
impl<'a> DLMSB_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - The SCIn Divisor Latch MSB Register, along with the DLL register, determines the baud rate of the SCIn."]
    #[inline(always)]
    pub fn dlmsb(&self) -> DLMSB_R {
        DLMSB_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - The SCIn Divisor Latch MSB Register, along with the DLL register, determines the baud rate of the SCIn."]
    #[inline(always)]
    pub fn dlmsb(&mut self) -> DLMSB_W {
        DLMSB_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Divisor Latch MSB\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dlm_ier_dlm](index.html) module"]
pub struct DLM_IER_DLM_SPEC;
impl crate::RegisterSpec for DLM_IER_DLM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dlm_ier_dlm::R](R) reader structure"]
impl crate::Readable for DLM_IER_DLM_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dlm_ier_dlm::W](W) writer structure"]
impl crate::Writable for DLM_IER_DLM_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DLM to value 0"]
impl crate::Resettable for DLM_IER_DLM_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
