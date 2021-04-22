#[doc = "Register `ENABLECLR0` reader"]
pub struct R(crate::R<ENABLECLR0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ENABLECLR0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<ENABLECLR0_SPEC>> for R {
    fn from(reader: crate::R<ENABLECLR0_SPEC>) -> Self {
        R(reader)
    }
}
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
impl core::convert::From<crate::W<ENABLECLR0_SPEC>> for W {
    fn from(writer: crate::W<ENABLECLR0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CLR` reader - Writing ones to this register clears the corresponding bits in ENABLESET0. Bit n clears the channel enable bit n. The number of bits = number of DMA channels in this device. Other bits are reserved."]
pub struct CLR_R(crate::FieldReader<u32, u32>);
impl CLR_R {
    pub(crate) fn new(bits: u32) -> Self {
        CLR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CLR_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
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
        self.w.bits = (self.w.bits & !0x0003_ffff) | (value as u32 & 0x0003_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:17 - Writing ones to this register clears the corresponding bits in ENABLESET0. Bit n clears the channel enable bit n. The number of bits = number of DMA channels in this device. Other bits are reserved."]
    #[inline(always)]
    pub fn clr(&self) -> CLR_R {
        CLR_R::new((self.bits & 0x0003_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:17 - Writing ones to this register clears the corresponding bits in ENABLESET0. Bit n clears the channel enable bit n. The number of bits = number of DMA channels in this device. Other bits are reserved."]
    #[inline(always)]
    pub fn clr(&mut self) -> CLR_W {
        CLR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Channel Enable Clear for all DMA channels.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [enableclr0](index.html) module"]
pub struct ENABLECLR0_SPEC;
impl crate::RegisterSpec for ENABLECLR0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [enableclr0::R](R) reader structure"]
impl crate::Readable for ENABLECLR0_SPEC {
    type Reader = R;
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
