#[doc = "Register `ENABLECLR0` writer"]
pub struct W(crate::W<ENABLECLR0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ENABLECLR0_SPEC>;
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
impl From<crate::W<ENABLECLR0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ENABLECLR0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CLR` writer - Writing ones to this register clears the corresponding bits in ENABLESET0. Bit n clears the channel enable bit n. The number of bits = number of DMA channels in this device. Other bits are reserved."]
pub struct CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> CLR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01ff_ffff) | (value as u32 & 0x01ff_ffff);
        self.w
    }
}
impl W {
    #[doc = "Bits 0:24 - Writing ones to this register clears the corresponding bits in ENABLESET0. Bit n clears the channel enable bit n. The number of bits = number of DMA channels in this device. Other bits are reserved."]
    #[inline(always)]
    pub fn clr(&mut self) -> CLR_W {
        CLR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Channel Enable Clear for all DMA channels.\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [enableclr0](index.html) module"]
pub struct ENABLECLR0_SPEC;
impl crate::RegisterSpec for ENABLECLR0_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [enableclr0::W](W) writer structure"]
impl crate::Writable for ENABLECLR0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ENABLECLR0 to value 0"]
impl crate::Resettable for ENABLECLR0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
