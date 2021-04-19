#[doc = "Register `INTENSET0` reader"]
pub struct R(crate::R<INTENSET0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTENSET0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<INTENSET0_SPEC>> for R {
    fn from(reader: crate::R<INTENSET0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INTENSET0` writer"]
pub struct W(crate::W<INTENSET0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INTENSET0_SPEC>;
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
impl core::convert::From<crate::W<INTENSET0_SPEC>> for W {
    fn from(writer: crate::W<INTENSET0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `INTEN` reader - Interrupt Enable read and set for DMA channel n. Bit n corresponds to DMA channel n. The number of bits = number of DMA channels in this device. Other bits are reserved. 0 = interrupt for DMA channel is disabled. 1 = interrupt for DMA channel is enabled."]
pub struct INTEN_R(crate::FieldReader<u32, u32>);
impl INTEN_R {
    pub(crate) fn new(bits: u32) -> Self {
        INTEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INTEN_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INTEN` writer - Interrupt Enable read and set for DMA channel n. Bit n corresponds to DMA channel n. The number of bits = number of DMA channels in this device. Other bits are reserved. 0 = interrupt for DMA channel is disabled. 1 = interrupt for DMA channel is enabled."]
pub struct INTEN_W<'a> {
    w: &'a mut W,
}
impl<'a> INTEN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | (value as u32 & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Interrupt Enable read and set for DMA channel n. Bit n corresponds to DMA channel n. The number of bits = number of DMA channels in this device. Other bits are reserved. 0 = interrupt for DMA channel is disabled. 1 = interrupt for DMA channel is enabled."]
    #[inline(always)]
    pub fn inten(&self) -> INTEN_R {
        INTEN_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Interrupt Enable read and set for DMA channel n. Bit n corresponds to DMA channel n. The number of bits = number of DMA channels in this device. Other bits are reserved. 0 = interrupt for DMA channel is disabled. 1 = interrupt for DMA channel is enabled."]
    #[inline(always)]
    pub fn inten(&mut self) -> INTEN_W {
        INTEN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Enable read and Set for all DMA channels.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intenset0](index.html) module"]
pub struct INTENSET0_SPEC;
impl crate::RegisterSpec for INTENSET0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [intenset0::R](R) reader structure"]
impl crate::Readable for INTENSET0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [intenset0::W](W) writer structure"]
impl crate::Writable for INTENSET0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets INTENSET0 to value 0"]
impl crate::Resettable for INTENSET0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
