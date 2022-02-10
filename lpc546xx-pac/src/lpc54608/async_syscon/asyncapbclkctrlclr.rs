///Register `ASYNCAPBCLKCTRLCLR` writer
pub struct W(crate::W<ASYNCAPBCLKCTRLCLR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ASYNCAPBCLKCTRLCLR_SPEC>;
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
impl From<crate::W<ASYNCAPBCLKCTRLCLR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ASYNCAPBCLKCTRLCLR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `ACLK_CLR` writer - Writing ones to this register clears the corresponding bit or bits in the ASYNCAPBCLKCTRL register, if they are implemented. Bits that do not correspond to defined bits in ASYNCAPBCLKCTRL are reserved and only zeroes should be written to them.
pub struct ACLK_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> ACLK_CLR_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = value;
        self.w
    }
}
impl W {
    ///Bits 0:31 - Writing ones to this register clears the corresponding bit or bits in the ASYNCAPBCLKCTRL register, if they are implemented. Bits that do not correspond to defined bits in ASYNCAPBCLKCTRL are reserved and only zeroes should be written to them.
    #[inline(always)]
    pub fn aclk_clr(&mut self) -> ACLK_CLR_W {
        ACLK_CLR_W { w: self }
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///Clear bits in ASYNCAPBCLKCTRL
///
///This register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [asyncapbclkctrlclr](index.html) module
pub struct ASYNCAPBCLKCTRLCLR_SPEC;
impl crate::RegisterSpec for ASYNCAPBCLKCTRLCLR_SPEC {
    type Ux = u32;
}
///`write(|w| ..)` method takes [asyncapbclkctrlclr::W](W) writer structure
impl crate::Writable for ASYNCAPBCLKCTRLCLR_SPEC {
    type Writer = W;
}
///`reset()` method sets ASYNCAPBCLKCTRLCLR to value 0
impl crate::Resettable for ASYNCAPBCLKCTRLCLR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
