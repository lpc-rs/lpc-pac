#[doc = "Register `FIFORD48H` reader"]
pub struct R(crate::R<FIFORD48H_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FIFORD48H_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FIFORD48H_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FIFORD48H_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RXDATA` reader - Received data from the FIFO. Whether this register is used and the number of bits used depends on configuration details."]
pub struct RXDATA_R(crate::FieldReader<u32, u32>);
impl RXDATA_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        RXDATA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXDATA_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:23 - Received data from the FIFO. Whether this register is used and the number of bits used depends on configuration details."]
    #[inline(always)]
    pub fn rxdata(&self) -> RXDATA_R {
        RXDATA_R::new((self.bits & 0x00ff_ffff) as u32)
    }
}
#[doc = "FIFO read data for upper data bits. May only be used if the I2S is configured for 2x 24-bit data and not using DMA.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fiford48h](index.html) module"]
pub struct FIFORD48H_SPEC;
impl crate::RegisterSpec for FIFORD48H_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fiford48h::R](R) reader structure"]
impl crate::Readable for FIFORD48H_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets FIFORD48H to value 0"]
impl crate::Resettable for FIFORD48H_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
