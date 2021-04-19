#[doc = "Register `FIFOWR48H` writer"]
pub struct W(crate::W<FIFOWR48H_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FIFOWR48H_SPEC>;
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
impl core::convert::From<crate::W<FIFOWR48H_SPEC>> for W {
    fn from(writer: crate::W<FIFOWR48H_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TXDATA` writer - Transmit data to the FIFO. Whether this register is used and the number of bits used depends on configuration details."]
pub struct TXDATA_W<'a> {
    w: &'a mut W,
}
impl<'a> TXDATA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x00ff_ffff) | (value as u32 & 0x00ff_ffff);
        self.w
    }
}
impl W {
    #[doc = "Bits 0:23 - Transmit data to the FIFO. Whether this register is used and the number of bits used depends on configuration details."]
    #[inline(always)]
    pub fn txdata(&mut self) -> TXDATA_W {
        TXDATA_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "FIFO write data for upper data bits. May only be used if the I2S is configured for 2x 24-bit data and not using DMA.\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fifowr48h](index.html) module"]
pub struct FIFOWR48H_SPEC;
impl crate::RegisterSpec for FIFOWR48H_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [fifowr48h::W](W) writer structure"]
impl crate::Writable for FIFOWR48H_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FIFOWR48H to value 0"]
impl crate::Resettable for FIFOWR48H_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
