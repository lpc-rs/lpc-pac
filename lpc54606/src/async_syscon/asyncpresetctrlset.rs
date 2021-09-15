#[doc = "Register `ASYNCPRESETCTRLSET` writer"]
pub struct W(crate::W<ASYNCPRESETCTRLSET_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ASYNCPRESETCTRLSET_SPEC>;
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
impl From<crate::W<ASYNCPRESETCTRLSET_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ASYNCPRESETCTRLSET_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ARST_SET` writer - Writing ones to this register sets the corresponding bit or bits in the ASYNCPRESETCTRL register, if they are implemented. Bits that do not correspond to defined bits in ASYNCPRESETCTRL are reserved and only zeroes should be written to them."]
pub struct ARST_SET_W<'a> {
    w: &'a mut W,
}
impl<'a> ARST_SET_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | (value as u32 & 0xffff_ffff);
        self.w
    }
}
impl W {
    #[doc = "Bits 0:31 - Writing ones to this register sets the corresponding bit or bits in the ASYNCPRESETCTRL register, if they are implemented. Bits that do not correspond to defined bits in ASYNCPRESETCTRL are reserved and only zeroes should be written to them."]
    #[inline(always)]
    pub fn arst_set(&mut self) -> ARST_SET_W {
        ARST_SET_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Set bits in ASYNCPRESETCTRL\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [asyncpresetctrlset](index.html) module"]
pub struct ASYNCPRESETCTRLSET_SPEC;
impl crate::RegisterSpec for ASYNCPRESETCTRLSET_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [asyncpresetctrlset::W](W) writer structure"]
impl crate::Writable for ASYNCPRESETCTRLSET_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ASYNCPRESETCTRLSET to value 0"]
impl crate::Resettable for ASYNCPRESETCTRLSET_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
