#[doc = "Register `BUSY0` reader"]
pub struct R(crate::R<BUSY0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BUSY0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<BUSY0_SPEC>> for R {
    fn from(reader: crate::R<BUSY0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `BSY` reader - Busy flag for DMA channel n. Bit n corresponds to DMA channel n. The number of bits = number of DMA channels in this device. Other bits are reserved. 0 = not busy. 1 = busy."]
pub struct BSY_R(crate::FieldReader<u32, u32>);
impl BSY_R {
    pub(crate) fn new(bits: u32) -> Self {
        BSY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BSY_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:31 - Busy flag for DMA channel n. Bit n corresponds to DMA channel n. The number of bits = number of DMA channels in this device. Other bits are reserved. 0 = not busy. 1 = busy."]
    #[inline(always)]
    pub fn bsy(&self) -> BSY_R {
        BSY_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
#[doc = "Channel Busy status for all DMA channels.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [busy0](index.html) module"]
pub struct BUSY0_SPEC;
impl crate::RegisterSpec for BUSY0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [busy0::R](R) reader structure"]
impl crate::Readable for BUSY0_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets BUSY0 to value 0"]
impl crate::Resettable for BUSY0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
