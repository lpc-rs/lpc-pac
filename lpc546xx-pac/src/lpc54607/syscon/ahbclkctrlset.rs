///Register `AHBCLKCTRLSET[%s]` writer
pub struct W(crate::W<AHBCLKCTRLSET_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AHBCLKCTRLSET_SPEC>;
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
impl From<crate::W<AHBCLKCTRLSET_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AHBCLKCTRLSET_SPEC>) -> Self {
        W(writer)
    }
}
///Field `CLK_SET` writer - Writing ones to this register sets the corresponding bit or bits in the AHBCLKCTRLn register, if they are implemented. Bits that do not correspond to defined bits in AHBCLKCTRLn are reserved and only zeroes should be written to them.
pub struct CLK_SET_W<'a> {
    w: &'a mut W,
}
impl<'a> CLK_SET_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = value;
        self.w
    }
}
impl W {
    ///Bits 0:31 - Writing ones to this register sets the corresponding bit or bits in the AHBCLKCTRLn register, if they are implemented. Bits that do not correspond to defined bits in AHBCLKCTRLn are reserved and only zeroes should be written to them.
    #[inline(always)]
    pub fn clk_set(&mut self) -> CLK_SET_W {
        CLK_SET_W { w: self }
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///Set bits in AHBCLKCTRLn
///
///This register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [ahbclkctrlset](index.html) module
pub struct AHBCLKCTRLSET_SPEC;
impl crate::RegisterSpec for AHBCLKCTRLSET_SPEC {
    type Ux = u32;
}
///`write(|w| ..)` method takes [ahbclkctrlset::W](W) writer structure
impl crate::Writable for AHBCLKCTRLSET_SPEC {
    type Writer = W;
}
///`reset()` method sets AHBCLKCTRLSET[%s]
///to value 0
impl crate::Resettable for AHBCLKCTRLSET_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
