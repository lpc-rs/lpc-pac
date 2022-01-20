#[doc = "Register `CIENR` writer"]
pub struct W(crate::W<CIENR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CIENR_SPEC>;
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
impl From<crate::W<CIENR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CIENR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CENRL` writer - Ones written to this address clear bits in the IENR, thus disabling the interrupts. Bit n clears bit n in the IENR register. 0 = No operation. 1 = Disable rising edge or level interrupt."]
pub struct CENRL_W<'a> {
    w: &'a mut W,
}
impl<'a> CENRL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
impl W {
    #[doc = "Bits 0:7 - Ones written to this address clear bits in the IENR, thus disabling the interrupts. Bit n clears bit n in the IENR register. 0 = No operation. 1 = Disable rising edge or level interrupt."]
    #[inline(always)]
    pub fn cenrl(&mut self) -> CENRL_W {
        CENRL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Pin interrupt level (rising edge interrupt) clear register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cienr](index.html) module"]
pub struct CIENR_SPEC;
impl crate::RegisterSpec for CIENR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [cienr::W](W) writer structure"]
impl crate::Writable for CIENR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CIENR to value 0"]
impl crate::Resettable for CIENR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
