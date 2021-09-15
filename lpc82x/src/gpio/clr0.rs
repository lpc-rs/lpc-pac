#[doc = "Register `CLR0` reader"]
pub struct R(crate::R<CLR0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CLR0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CLR0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CLR0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CLR0` writer"]
pub struct W(crate::W<CLR0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CLR0_SPEC>;
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
impl From<crate::W<CLR0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CLR0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CLRP` writer - Clear output bits (bit 0 = PIOn_0, bit 1 = PIOn_1, etc.). Supported pins depends on the specific device and package. 0 = No operation. 1 = Clear output bit."]
pub struct CLRP_W<'a> {
    w: &'a mut W,
}
impl<'a> CLRP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1fff_ffff) | (value as u32 & 0x1fff_ffff);
        self.w
    }
}
impl W {
    #[doc = "Bits 0:28 - Clear output bits (bit 0 = PIOn_0, bit 1 = PIOn_1, etc.). Supported pins depends on the specific device and package. 0 = No operation. 1 = Clear output bit."]
    #[inline(always)]
    pub fn clrp(&mut self) -> CLRP_W {
        CLRP_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Clear port\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clr0](index.html) module"]
pub struct CLR0_SPEC;
impl crate::RegisterSpec for CLR0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [clr0::R](R) reader structure"]
impl crate::Readable for CLR0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [clr0::W](W) writer structure"]
impl crate::Writable for CLR0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CLR0 to value 0"]
impl crate::Resettable for CLR0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
