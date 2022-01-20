#[doc = "Register `ENABLESET0` reader"]
pub struct R(crate::R<ENABLESET0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ENABLESET0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ENABLESET0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ENABLESET0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ENABLESET0` writer"]
pub struct W(crate::W<ENABLESET0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ENABLESET0_SPEC>;
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
impl From<crate::W<ENABLESET0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ENABLESET0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ENA` reader - Enable for DMA channels. Bit n enables or disables DMA channel n. The number of bits = number of DMA channels in this device. Other bits are reserved. 0 = disabled. 1 = enabled."]
pub struct ENA_R(crate::FieldReader<u32, u32>);
impl ENA_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        ENA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ENA_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ENA` writer - Enable for DMA channels. Bit n enables or disables DMA channel n. The number of bits = number of DMA channels in this device. Other bits are reserved. 0 = disabled. 1 = enabled."]
pub struct ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> ENA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0003_ffff) | (value as u32 & 0x0003_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:17 - Enable for DMA channels. Bit n enables or disables DMA channel n. The number of bits = number of DMA channels in this device. Other bits are reserved. 0 = disabled. 1 = enabled."]
    #[inline(always)]
    pub fn ena(&self) -> ENA_R {
        ENA_R::new((self.bits & 0x0003_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:17 - Enable for DMA channels. Bit n enables or disables DMA channel n. The number of bits = number of DMA channels in this device. Other bits are reserved. 0 = disabled. 1 = enabled."]
    #[inline(always)]
    pub fn ena(&mut self) -> ENA_W {
        ENA_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Channel Enable read and Set for all DMA channels.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [enableset0](index.html) module"]
pub struct ENABLESET0_SPEC;
impl crate::RegisterSpec for ENABLESET0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [enableset0::R](R) reader structure"]
impl crate::Readable for ENABLESET0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [enableset0::W](W) writer structure"]
impl crate::Writable for ENABLESET0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ENABLESET0 to value 0"]
impl crate::Resettable for ENABLESET0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
