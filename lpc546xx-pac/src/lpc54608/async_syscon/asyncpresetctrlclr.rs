///Register `ASYNCPRESETCTRLCLR` writer
pub struct W(crate::W<ASYNCPRESETCTRLCLR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ASYNCPRESETCTRLCLR_SPEC>;
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
impl From<crate::W<ASYNCPRESETCTRLCLR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ASYNCPRESETCTRLCLR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `ARST_CLR` writer - Writing ones to this register clears the corresponding bit or bits in the ASYNCPRESETCTRL register, if they are implemented. Bits that do not correspond to defined bits in ASYNCPRESETCTRL are reserved and only zeroes should be written to them.
pub struct ARST_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> ARST_CLR_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = value;
        self.w
    }
}
impl W {
    ///Bits 0:31 - Writing ones to this register clears the corresponding bit or bits in the ASYNCPRESETCTRL register, if they are implemented. Bits that do not correspond to defined bits in ASYNCPRESETCTRL are reserved and only zeroes should be written to them.
    #[inline(always)]
    pub fn arst_clr(&mut self) -> ARST_CLR_W {
        ARST_CLR_W { w: self }
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///Clear bits in ASYNCPRESETCTRL
///
///This register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [asyncpresetctrlclr](index.html) module
pub struct ASYNCPRESETCTRLCLR_SPEC;
impl crate::RegisterSpec for ASYNCPRESETCTRLCLR_SPEC {
    type Ux = u32;
}
///`write(|w| ..)` method takes [asyncpresetctrlclr::W](W) writer structure
impl crate::Writable for ASYNCPRESETCTRLCLR_SPEC {
    type Writer = W;
}
///`reset()` method sets ASYNCPRESETCTRLCLR to value 0
impl crate::Resettable for ASYNCPRESETCTRLCLR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
