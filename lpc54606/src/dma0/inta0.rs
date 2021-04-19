#[doc = "Register `INTA0` reader"]
pub struct R(crate::R<INTA0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTA0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<INTA0_SPEC>> for R {
    fn from(reader: crate::R<INTA0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INTA0` writer"]
pub struct W(crate::W<INTA0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INTA0_SPEC>;
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
impl core::convert::From<crate::W<INTA0_SPEC>> for W {
    fn from(writer: crate::W<INTA0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `IA` reader - Interrupt A status for DMA channel n. Bit n corresponds to DMA channel n. The number of bits = number of DMA channels in this device. Other bits are reserved. 0 = the DMA channel interrupt A is not active. 1 = the DMA channel interrupt A is active."]
pub struct IA_R(crate::FieldReader<u32, u32>);
impl IA_R {
    pub(crate) fn new(bits: u32) -> Self {
        IA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IA_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IA` writer - Interrupt A status for DMA channel n. Bit n corresponds to DMA channel n. The number of bits = number of DMA channels in this device. Other bits are reserved. 0 = the DMA channel interrupt A is not active. 1 = the DMA channel interrupt A is active."]
pub struct IA_W<'a> {
    w: &'a mut W,
}
impl<'a> IA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | (value as u32 & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Interrupt A status for DMA channel n. Bit n corresponds to DMA channel n. The number of bits = number of DMA channels in this device. Other bits are reserved. 0 = the DMA channel interrupt A is not active. 1 = the DMA channel interrupt A is active."]
    #[inline(always)]
    pub fn ia(&self) -> IA_R {
        IA_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Interrupt A status for DMA channel n. Bit n corresponds to DMA channel n. The number of bits = number of DMA channels in this device. Other bits are reserved. 0 = the DMA channel interrupt A is not active. 1 = the DMA channel interrupt A is active."]
    #[inline(always)]
    pub fn ia(&mut self) -> IA_W {
        IA_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt A status for all DMA channels.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [inta0](index.html) module"]
pub struct INTA0_SPEC;
impl crate::RegisterSpec for INTA0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [inta0::R](R) reader structure"]
impl crate::Readable for INTA0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [inta0::W](W) writer structure"]
impl crate::Writable for INTA0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets INTA0 to value 0"]
impl crate::Resettable for INTA0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
