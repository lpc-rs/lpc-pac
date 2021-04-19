#[doc = "Register `MAC_SYS_TIME_SCND` reader"]
pub struct R(crate::R<MAC_SYS_TIME_SCND_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MAC_SYS_TIME_SCND_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<MAC_SYS_TIME_SCND_SPEC>> for R {
    fn from(reader: crate::R<MAC_SYS_TIME_SCND_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `TSS` reader - Time stamp second The value in this field indicates the current value in seconds of the System Time maintained by the MAC."]
pub struct TSS_R(crate::FieldReader<u32, u32>);
impl TSS_R {
    pub(crate) fn new(bits: u32) -> Self {
        TSS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TSS_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:31 - Time stamp second The value in this field indicates the current value in seconds of the System Time maintained by the MAC."]
    #[inline(always)]
    pub fn tss(&self) -> TSS_R {
        TSS_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
#[doc = "System time seconds register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mac_sys_time_scnd](index.html) module"]
pub struct MAC_SYS_TIME_SCND_SPEC;
impl crate::RegisterSpec for MAC_SYS_TIME_SCND_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mac_sys_time_scnd::R](R) reader structure"]
impl crate::Readable for MAC_SYS_TIME_SCND_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets MAC_SYS_TIME_SCND to value 0"]
impl crate::Resettable for MAC_SYS_TIME_SCND_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
