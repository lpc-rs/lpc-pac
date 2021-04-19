#[doc = "Register `ASYNCAPBCLKCTRLSET` writer"]
pub struct W(crate::W<ASYNCAPBCLKCTRLSET_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ASYNCAPBCLKCTRLSET_SPEC>;
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
impl core::convert::From<crate::W<ASYNCAPBCLKCTRLSET_SPEC>> for W {
    fn from(writer: crate::W<ASYNCAPBCLKCTRLSET_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ACLK_SET` writer - Writing ones to this register sets the corresponding bit or bits in the ASYNCAPBCLKCTRL register, if they are implemented. Bits that do not correspond to defined bits in ASYNCPRESETCTRL are reserved and only zeroes should be written to them."]
pub struct ACLK_SET_W<'a> {
    w: &'a mut W,
}
impl<'a> ACLK_SET_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | (value as u32 & 0xffff_ffff);
        self.w
    }
}
impl W {
    #[doc = "Bits 0:31 - Writing ones to this register sets the corresponding bit or bits in the ASYNCAPBCLKCTRL register, if they are implemented. Bits that do not correspond to defined bits in ASYNCPRESETCTRL are reserved and only zeroes should be written to them."]
    #[inline(always)]
    pub fn aclk_set(&mut self) -> ACLK_SET_W {
        ACLK_SET_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Set bits in ASYNCAPBCLKCTRL\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [asyncapbclkctrlset](index.html) module"]
pub struct ASYNCAPBCLKCTRLSET_SPEC;
impl crate::RegisterSpec for ASYNCAPBCLKCTRLSET_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [asyncapbclkctrlset::W](W) writer structure"]
impl crate::Writable for ASYNCAPBCLKCTRLSET_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ASYNCAPBCLKCTRLSET to value 0"]
impl crate::Resettable for ASYNCAPBCLKCTRLSET_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
