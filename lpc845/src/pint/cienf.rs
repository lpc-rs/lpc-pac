#[doc = "Register `CIENF` writer"]
pub struct W(crate::W<CIENF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CIENF_SPEC>;
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
impl core::convert::From<crate::W<CIENF_SPEC>> for W {
    fn from(writer: crate::W<CIENF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CENAF` writer - Ones written to this address clears bits in the IENF, thus disabling interrupts. Bit n clears bit n in the IENF register. 0 = No operation. 1 = LOW-active interrupt selected or falling edge interrupt disabled."]
pub struct CENAF_W<'a> {
    w: &'a mut W,
}
impl<'a> CENAF_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
impl W {
    #[doc = "Bits 0:7 - Ones written to this address clears bits in the IENF, thus disabling interrupts. Bit n clears bit n in the IENF register. 0 = No operation. 1 = LOW-active interrupt selected or falling edge interrupt disabled."]
    #[inline(always)]
    pub fn cenaf(&mut self) -> CENAF_W {
        CENAF_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Pin interrupt active level or falling edge interrupt clear register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cienf](index.html) module"]
pub struct CIENF_SPEC;
impl crate::RegisterSpec for CIENF_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [cienf::W](W) writer structure"]
impl crate::Writable for CIENF_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CIENF to value 0"]
impl crate::Resettable for CIENF_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
