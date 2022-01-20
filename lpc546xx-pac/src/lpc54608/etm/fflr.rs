#[doc = "Register `FFLR` reader"]
pub struct R(crate::R<FFLR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FFLR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FFLR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FFLR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FFLR` writer"]
pub struct W(crate::W<FFLR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FFLR_SPEC>;
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
impl From<crate::W<FFLR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FFLR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FIFOFullLevel` reader - FIFO full level. The number of bytes left in FIFO, below which the FIFOFULL or SupressData signal is asserted. For example, setting this value to 15 causes data trace suppression or processor stalling, if enabled, when there are less than 15 free bytes in the FIFO."]
pub struct FIFOFULLLEVEL_R(crate::FieldReader<u8, u8>);
impl FIFOFULLLEVEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        FIFOFULLLEVEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FIFOFULLLEVEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FIFOFullLevel` writer - FIFO full level. The number of bytes left in FIFO, below which the FIFOFULL or SupressData signal is asserted. For example, setting this value to 15 causes data trace suppression or processor stalling, if enabled, when there are less than 15 free bytes in the FIFO."]
pub struct FIFOFULLLEVEL_W<'a> {
    w: &'a mut W,
}
impl<'a> FIFOFULLLEVEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - FIFO full level. The number of bytes left in FIFO, below which the FIFOFULL or SupressData signal is asserted. For example, setting this value to 15 causes data trace suppression or processor stalling, if enabled, when there are less than 15 free bytes in the FIFO."]
    #[inline(always)]
    pub fn fifofull_level(&self) -> FIFOFULLLEVEL_R {
        FIFOFULLLEVEL_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - FIFO full level. The number of bytes left in FIFO, below which the FIFOFULL or SupressData signal is asserted. For example, setting this value to 15 causes data trace suppression or processor stalling, if enabled, when there are less than 15 free bytes in the FIFO."]
    #[inline(always)]
    pub fn fifofull_level(&mut self) -> FIFOFULLLEVEL_W {
        FIFOFULLLEVEL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "FIFOFULL Level Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fflr](index.html) module"]
pub struct FFLR_SPEC;
impl crate::RegisterSpec for FFLR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fflr::R](R) reader structure"]
impl crate::Readable for FFLR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fflr::W](W) writer structure"]
impl crate::Writable for FFLR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FFLR to value 0"]
impl crate::Resettable for FFLR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
