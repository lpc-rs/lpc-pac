#[doc = "Register `STARTERCLR[%s]` writer"]
pub struct W(crate::W<STARTERCLR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<STARTERCLR_SPEC>;
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
impl From<crate::W<STARTERCLR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<STARTERCLR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `START_CLR` writer - Writing ones to this register clears the corresponding bit or bits in the STARTER0 register, if they are implemented."]
pub struct START_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> START_CLR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = value;
        self.w
    }
}
impl W {
    #[doc = "Bits 0:31 - Writing ones to this register clears the corresponding bit or bits in the STARTER0 register, if they are implemented."]
    #[inline(always)]
    pub fn start_clr(&mut self) -> START_CLR_W {
        START_CLR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Clear bits in STARTER0\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [starterclr](index.html) module"]
pub struct STARTERCLR_SPEC;
impl crate::RegisterSpec for STARTERCLR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [starterclr::W](W) writer structure"]
impl crate::Writable for STARTERCLR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets STARTERCLR[%s]
to value 0"]
impl crate::Resettable for STARTERCLR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
