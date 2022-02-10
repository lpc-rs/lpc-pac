///Register `THR` writer
pub struct W(crate::W<RBR_THR_DLL_THR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RBR_THR_DLL_THR_SPEC>;
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
impl From<crate::W<RBR_THR_DLL_THR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RBR_THR_DLL_THR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `THR` writer - Writing to the SCIn Transmit Holding Register causes the data to be stored in the SCIn transmit FIFO.
pub struct THR_W<'a> {
    w: &'a mut W,
}
impl<'a> THR_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
impl W {
    ///Bits 0:7 - Writing to the SCIn Transmit Holding Register causes the data to be stored in the SCIn transmit FIFO.
    #[inline(always)]
    pub fn thr(&mut self) -> THR_W {
        THR_W { w: self }
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///Transmit Holding Register
///
///This register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [rbr_thr_dll_thr](index.html) module
pub struct RBR_THR_DLL_THR_SPEC;
impl crate::RegisterSpec for RBR_THR_DLL_THR_SPEC {
    type Ux = u32;
}
///`write(|w| ..)` method takes [rbr_thr_dll_thr::W](W) writer structure
impl crate::Writable for RBR_THR_DLL_THR_SPEC {
    type Writer = W;
}
///`reset()` method sets THR to value 0
impl crate::Resettable for RBR_THR_DLL_THR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
