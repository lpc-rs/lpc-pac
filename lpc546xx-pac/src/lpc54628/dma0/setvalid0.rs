///Register `SETVALID0` writer
pub struct W(crate::W<SETVALID0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SETVALID0_SPEC>;
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
impl From<crate::W<SETVALID0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SETVALID0_SPEC>) -> Self {
        W(writer)
    }
}
///Field `SV` writer - SETVALID control for DMA channel n. Bit n corresponds to DMA channel n. The number of bits = number of DMA channels in this device. Other bits are reserved. 0 = no effect. 1 = sets the VALIDPENDING control bit for DMA channel n
pub struct SV_W<'a> {
    w: &'a mut W,
}
impl<'a> SV_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = value;
        self.w
    }
}
impl W {
    ///Bits 0:31 - SETVALID control for DMA channel n. Bit n corresponds to DMA channel n. The number of bits = number of DMA channels in this device. Other bits are reserved. 0 = no effect. 1 = sets the VALIDPENDING control bit for DMA channel n
    #[inline(always)]
    pub fn sv(&mut self) -> SV_W {
        SV_W { w: self }
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///Set ValidPending control bits for all DMA channels.
///
///This register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [setvalid0](index.html) module
pub struct SETVALID0_SPEC;
impl crate::RegisterSpec for SETVALID0_SPEC {
    type Ux = u32;
}
///`write(|w| ..)` method takes [setvalid0::W](W) writer structure
impl crate::Writable for SETVALID0_SPEC {
    type Writer = W;
}
///`reset()` method sets SETVALID0 to value 0
impl crate::Resettable for SETVALID0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
