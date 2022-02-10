///Register `MAC_SYS_TIME_NSCND` reader
pub struct R(crate::R<MAC_SYS_TIME_NSCND_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MAC_SYS_TIME_NSCND_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MAC_SYS_TIME_NSCND_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MAC_SYS_TIME_NSCND_SPEC>) -> Self {
        R(reader)
    }
}
///Field `TSSS` reader - Time stamp sub seconds The value in this field has the sub second representation of time, with an accuracy of 0.
pub struct TSSS_R(crate::FieldReader<u32, u32>);
impl TSSS_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        TSSS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TSSS_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    ///Bits 0:30 - Time stamp sub seconds The value in this field has the sub second representation of time, with an accuracy of 0.
    #[inline(always)]
    pub fn tsss(&self) -> TSSS_R {
        TSSS_R::new((self.bits & 0x7fff_ffff) as u32)
    }
}
///System time nanoseconds register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [mac_sys_time_nscnd](index.html) module
pub struct MAC_SYS_TIME_NSCND_SPEC;
impl crate::RegisterSpec for MAC_SYS_TIME_NSCND_SPEC {
    type Ux = u32;
}
///`read()` method returns [mac_sys_time_nscnd::R](R) reader structure
impl crate::Readable for MAC_SYS_TIME_NSCND_SPEC {
    type Reader = R;
}
///`reset()` method sets MAC_SYS_TIME_NSCND to value 0
impl crate::Resettable for MAC_SYS_TIME_NSCND_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
