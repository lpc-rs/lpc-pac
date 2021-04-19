#[doc = "Register `ACTIVE0` reader"]
pub struct R(crate::R<ACTIVE0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ACTIVE0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<ACTIVE0_SPEC>> for R {
    fn from(reader: crate::R<ACTIVE0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `ACT` reader - Active flag for DMA channel n. Bit n corresponds to DMA channel n. The number of bits = number of DMA channels in this device. Other bits are reserved. 0 = not active. 1 = active."]
pub struct ACT_R(crate::FieldReader<u32, u32>);
impl ACT_R {
    pub(crate) fn new(bits: u32) -> Self {
        ACT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ACT_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:31 - Active flag for DMA channel n. Bit n corresponds to DMA channel n. The number of bits = number of DMA channels in this device. Other bits are reserved. 0 = not active. 1 = active."]
    #[inline(always)]
    pub fn act(&self) -> ACT_R {
        ACT_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
#[doc = "Channel Active status for all DMA channels.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [active0](index.html) module"]
pub struct ACTIVE0_SPEC;
impl crate::RegisterSpec for ACTIVE0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [active0::R](R) reader structure"]
impl crate::Readable for ACTIVE0_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets ACTIVE0 to value 0"]
impl crate::Resettable for ACTIVE0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
