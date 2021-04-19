#[doc = "Register `ASYNCPRESETCTRLCLR` writer"]
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
impl core::convert::From<crate::W<ASYNCPRESETCTRLCLR_SPEC>> for W {
    fn from(writer: crate::W<ASYNCPRESETCTRLCLR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ARST_CLR` writer - Writing ones to this register clears the corresponding bit or bits in the ASYNCPRESETCTRL register, if they are implemented. Bits that do not correspond to defined bits in ASYNCPRESETCTRL are reserved and only zeroes should be written to them."]
pub struct ARST_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> ARST_CLR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | (value as u32 & 0xffff_ffff);
        self.w
    }
}
impl W {
    #[doc = "Bits 0:31 - Writing ones to this register clears the corresponding bit or bits in the ASYNCPRESETCTRL register, if they are implemented. Bits that do not correspond to defined bits in ASYNCPRESETCTRL are reserved and only zeroes should be written to them."]
    #[inline(always)]
    pub fn arst_clr(&mut self) -> ARST_CLR_W {
        ARST_CLR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Clear bits in ASYNCPRESETCTRL\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [asyncpresetctrlclr](index.html) module"]
pub struct ASYNCPRESETCTRLCLR_SPEC;
impl crate::RegisterSpec for ASYNCPRESETCTRLCLR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [asyncpresetctrlclr::W](W) writer structure"]
impl crate::Writable for ASYNCPRESETCTRLCLR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ASYNCPRESETCTRLCLR to value 0"]
impl crate::Resettable for ASYNCPRESETCTRLCLR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
