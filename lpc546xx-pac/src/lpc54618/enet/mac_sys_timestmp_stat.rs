///Register `MAC_SYS_TIMESTMP_STAT` reader
pub struct R(crate::R<MAC_SYS_TIMESTMP_STAT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MAC_SYS_TIMESTMP_STAT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MAC_SYS_TIMESTMP_STAT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MAC_SYS_TIMESTMP_STAT_SPEC>) -> Self {
        R(reader)
    }
}
///Field `TSSOVF` reader - Time stamp seconds overflow When set, indicates that the seconds value of the Time stamp has overflowed beyond 0xFFFF_FFFF.
pub struct TSSOVF_R(crate::FieldReader<bool, bool>);
impl TSSOVF_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TSSOVF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TSSOVF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    ///Bit 0 - Time stamp seconds overflow When set, indicates that the seconds value of the Time stamp has overflowed beyond 0xFFFF_FFFF.
    #[inline(always)]
    pub fn tssovf(&self) -> TSSOVF_R {
        TSSOVF_R::new((self.bits & 0x01) != 0)
    }
}
///Time stamp status register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [mac_sys_timestmp_stat](index.html) module
pub struct MAC_SYS_TIMESTMP_STAT_SPEC;
impl crate::RegisterSpec for MAC_SYS_TIMESTMP_STAT_SPEC {
    type Ux = u32;
}
///`read()` method returns [mac_sys_timestmp_stat::R](R) reader structure
impl crate::Readable for MAC_SYS_TIMESTMP_STAT_SPEC {
    type Reader = R;
}
///`reset()` method sets MAC_SYS_TIMESTMP_STAT to value 0
impl crate::Resettable for MAC_SYS_TIMESTMP_STAT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
