#[doc = "Register `SETTRIG0` reader"]
pub struct R(crate::R<SETTRIG0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SETTRIG0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<SETTRIG0_SPEC>> for R {
    fn from(reader: crate::R<SETTRIG0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SETTRIG0` writer"]
pub struct W(crate::W<SETTRIG0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SETTRIG0_SPEC>;
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
impl core::convert::From<crate::W<SETTRIG0_SPEC>> for W {
    fn from(writer: crate::W<SETTRIG0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TRIG` reader - Set Trigger control bit for DMA channel 0. Bit n corresponds to DMA channel n. The number of bits = number of DMA channels in this device. Other bits are reserved. 0 = no effect. 1 = sets the TRIG bit for DMA channel n."]
pub struct TRIG_R(crate::FieldReader<u32, u32>);
impl TRIG_R {
    pub(crate) fn new(bits: u32) -> Self {
        TRIG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TRIG_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TRIG` writer - Set Trigger control bit for DMA channel 0. Bit n corresponds to DMA channel n. The number of bits = number of DMA channels in this device. Other bits are reserved. 0 = no effect. 1 = sets the TRIG bit for DMA channel n."]
pub struct TRIG_W<'a> {
    w: &'a mut W,
}
impl<'a> TRIG_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0003_ffff) | (value as u32 & 0x0003_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:17 - Set Trigger control bit for DMA channel 0. Bit n corresponds to DMA channel n. The number of bits = number of DMA channels in this device. Other bits are reserved. 0 = no effect. 1 = sets the TRIG bit for DMA channel n."]
    #[inline(always)]
    pub fn trig(&self) -> TRIG_R {
        TRIG_R::new((self.bits & 0x0003_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:17 - Set Trigger control bit for DMA channel 0. Bit n corresponds to DMA channel n. The number of bits = number of DMA channels in this device. Other bits are reserved. 0 = no effect. 1 = sets the TRIG bit for DMA channel n."]
    #[inline(always)]
    pub fn trig(&mut self) -> TRIG_W {
        TRIG_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Set Trigger control bits for all DMA channels.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [settrig0](index.html) module"]
pub struct SETTRIG0_SPEC;
impl crate::RegisterSpec for SETTRIG0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [settrig0::R](R) reader structure"]
impl crate::Readable for SETTRIG0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [settrig0::W](W) writer structure"]
impl crate::Writable for SETTRIG0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SETTRIG0 to value 0"]
impl crate::Resettable for SETTRIG0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
