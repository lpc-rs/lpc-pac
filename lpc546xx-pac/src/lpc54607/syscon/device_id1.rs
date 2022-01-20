#[doc = "Register `DEVICE_ID1` reader"]
pub struct R(crate::R<DEVICE_ID1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DEVICE_ID1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DEVICE_ID1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DEVICE_ID1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `REVID` reader - Revision."]
pub struct REVID_R(crate::FieldReader<u32, u32>);
impl REVID_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        REVID_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for REVID_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:31 - Revision."]
    #[inline(always)]
    pub fn revid(&self) -> REVID_R {
        REVID_R::new(self.bits)
    }
}
#[doc = "Boot ROM and die revision register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [device_id1](index.html) module"]
pub struct DEVICE_ID1_SPEC;
impl crate::RegisterSpec for DEVICE_ID1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [device_id1::R](R) reader structure"]
impl crate::Readable for DEVICE_ID1_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets DEVICE_ID1 to value 0"]
impl crate::Resettable for DEVICE_ID1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
