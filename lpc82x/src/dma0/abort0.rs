#[doc = "Register `ABORT0` reader"]
pub struct R(crate::R<ABORT0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ABORT0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<ABORT0_SPEC>> for R {
    fn from(reader: crate::R<ABORT0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ABORT0` writer"]
pub struct W(crate::W<ABORT0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ABORT0_SPEC>;
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
impl core::convert::From<crate::W<ABORT0_SPEC>> for W {
    fn from(writer: crate::W<ABORT0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ABORTCTRL` reader - Abort control for DMA channel 0. Bit n corresponds to DMA channel n. 0 = no effect. 1 = aborts DMA operations on channel n."]
pub struct ABORTCTRL_R(crate::FieldReader<u32, u32>);
impl ABORTCTRL_R {
    pub(crate) fn new(bits: u32) -> Self {
        ABORTCTRL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ABORTCTRL_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ABORTCTRL` writer - Abort control for DMA channel 0. Bit n corresponds to DMA channel n. 0 = no effect. 1 = aborts DMA operations on channel n."]
pub struct ABORTCTRL_W<'a> {
    w: &'a mut W,
}
impl<'a> ABORTCTRL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0003_ffff) | (value as u32 & 0x0003_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:17 - Abort control for DMA channel 0. Bit n corresponds to DMA channel n. 0 = no effect. 1 = aborts DMA operations on channel n."]
    #[inline(always)]
    pub fn abortctrl(&self) -> ABORTCTRL_R {
        ABORTCTRL_R::new((self.bits & 0x0003_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:17 - Abort control for DMA channel 0. Bit n corresponds to DMA channel n. 0 = no effect. 1 = aborts DMA operations on channel n."]
    #[inline(always)]
    pub fn abortctrl(&mut self) -> ABORTCTRL_W {
        ABORTCTRL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Channel Abort control for all DMA channels.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [abort0](index.html) module"]
pub struct ABORT0_SPEC;
impl crate::RegisterSpec for ABORT0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [abort0::R](R) reader structure"]
impl crate::Readable for ABORT0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [abort0::W](W) writer structure"]
impl crate::Writable for ABORT0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ABORT0 to value 0"]
impl crate::Resettable for ABORT0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
