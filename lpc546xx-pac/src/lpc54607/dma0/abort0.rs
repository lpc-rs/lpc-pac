///Register `ABORT0` writer
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
impl From<crate::W<ABORT0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ABORT0_SPEC>) -> Self {
        W(writer)
    }
}
///Field `ABORTCTRL` writer - Abort control for DMA channel 0. Bit n corresponds to DMA channel n. 0 = no effect. 1 = aborts DMA operations on channel n.
pub struct ABORTCTRL_W<'a> {
    w: &'a mut W,
}
impl<'a> ABORTCTRL_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = value;
        self.w
    }
}
impl W {
    ///Bits 0:31 - Abort control for DMA channel 0. Bit n corresponds to DMA channel n. 0 = no effect. 1 = aborts DMA operations on channel n.
    #[inline(always)]
    pub fn abortctrl(&mut self) -> ABORTCTRL_W {
        ABORTCTRL_W { w: self }
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///Channel Abort control for all DMA channels.
///
///This register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [abort0](index.html) module
pub struct ABORT0_SPEC;
impl crate::RegisterSpec for ABORT0_SPEC {
    type Ux = u32;
}
///`write(|w| ..)` method takes [abort0::W](W) writer structure
impl crate::Writable for ABORT0_SPEC {
    type Writer = W;
}
///`reset()` method sets ABORT0 to value 0
impl crate::Resettable for ABORT0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
