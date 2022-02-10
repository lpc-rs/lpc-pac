///Register `SETTRIG0` writer
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
impl From<crate::W<SETTRIG0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SETTRIG0_SPEC>) -> Self {
        W(writer)
    }
}
///Field `TRIG` writer - Set Trigger control bit for DMA channel 0. Bit n corresponds to DMA channel n. The number of bits = number of DMA channels in this device. Other bits are reserved. 0 = no effect. 1 = sets the TRIG bit for DMA channel n.
pub struct TRIG_W<'a> {
    w: &'a mut W,
}
impl<'a> TRIG_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = value;
        self.w
    }
}
impl W {
    ///Bits 0:31 - Set Trigger control bit for DMA channel 0. Bit n corresponds to DMA channel n. The number of bits = number of DMA channels in this device. Other bits are reserved. 0 = no effect. 1 = sets the TRIG bit for DMA channel n.
    #[inline(always)]
    pub fn trig(&mut self) -> TRIG_W {
        TRIG_W { w: self }
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///Set Trigger control bits for all DMA channels.
///
///This register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [settrig0](index.html) module
pub struct SETTRIG0_SPEC;
impl crate::RegisterSpec for SETTRIG0_SPEC {
    type Ux = u32;
}
///`write(|w| ..)` method takes [settrig0::W](W) writer structure
impl crate::Writable for SETTRIG0_SPEC {
    type Writer = W;
}
///`reset()` method sets SETTRIG0 to value 0
impl crate::Resettable for SETTRIG0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
