#[doc = "Register `SIENF` writer"]
pub struct W(crate::W<SIENF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SIENF_SPEC>;
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
impl From<crate::W<SIENF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SIENF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SETENAF` writer - Ones written to this address set bits in the IENF, thus enabling interrupts. Bit n sets bit n in the IENF register. 0 = No operation. 1 = Select HIGH-active interrupt or enable falling edge interrupt."]
pub struct SETENAF_W<'a> {
    w: &'a mut W,
}
impl<'a> SETENAF_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
impl W {
    #[doc = "Bits 0:7 - Ones written to this address set bits in the IENF, thus enabling interrupts. Bit n sets bit n in the IENF register. 0 = No operation. 1 = Select HIGH-active interrupt or enable falling edge interrupt."]
    #[inline(always)]
    pub fn setenaf(&mut self) -> SETENAF_W {
        SETENAF_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Pin interrupt active level or falling edge interrupt set register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sienf](index.html) module"]
pub struct SIENF_SPEC;
impl crate::RegisterSpec for SIENF_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [sienf::W](W) writer structure"]
impl crate::Writable for SIENF_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SIENF to value 0"]
impl crate::Resettable for SIENF_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
