#[doc = "Register `DEVICE_ID` reader"]
pub struct R(crate::R<DEVICE_ID_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DEVICE_ID_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<DEVICE_ID_SPEC>> for R {
    fn from(reader: crate::R<DEVICE_ID_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `DEVICEID` reader - Part ID"]
pub struct DEVICEID_R(crate::FieldReader<u32, u32>);
impl DEVICEID_R {
    pub(crate) fn new(bits: u32) -> Self {
        DEVICEID_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DEVICEID_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:31 - Part ID"]
    #[inline(always)]
    pub fn deviceid(&self) -> DEVICEID_R {
        DEVICEID_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
#[doc = "Part ID register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [device_id](index.html) module"]
pub struct DEVICE_ID_SPEC;
impl crate::RegisterSpec for DEVICE_ID_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [device_id::R](R) reader structure"]
impl crate::Readable for DEVICE_ID_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets DEVICE_ID to value 0"]
impl crate::Resettable for DEVICE_ID_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
