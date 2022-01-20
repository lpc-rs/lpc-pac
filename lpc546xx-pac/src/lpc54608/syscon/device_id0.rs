#[doc = "Register `DEVICE_ID0` reader"]
pub struct R(crate::R<DEVICE_ID0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DEVICE_ID0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DEVICE_ID0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DEVICE_ID0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `PARTID` reader - Part ID"]
pub struct PARTID_R(crate::FieldReader<u32, u32>);
impl PARTID_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        PARTID_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PARTID_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:31 - Part ID"]
    #[inline(always)]
    pub fn partid(&self) -> PARTID_R {
        PARTID_R::new(self.bits)
    }
}
#[doc = "Part ID register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [device_id0](index.html) module"]
pub struct DEVICE_ID0_SPEC;
impl crate::RegisterSpec for DEVICE_ID0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [device_id0::R](R) reader structure"]
impl crate::Readable for DEVICE_ID0_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets DEVICE_ID0 to value 0"]
impl crate::Resettable for DEVICE_ID0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
