#[doc = "Register `SYSPLLSTAT` reader"]
pub struct R(crate::R<SYSPLLSTAT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SYSPLLSTAT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SYSPLLSTAT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SYSPLLSTAT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `LOCK` reader - PLL0 lock indicator"]
pub struct LOCK_R(crate::FieldReader<bool, bool>);
impl LOCK_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        LOCK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LOCK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - PLL0 lock indicator"]
    #[inline(always)]
    pub fn lock(&self) -> LOCK_R {
        LOCK_R::new((self.bits & 0x01) != 0)
    }
}
#[doc = "PLL status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [syspllstat](index.html) module"]
pub struct SYSPLLSTAT_SPEC;
impl crate::RegisterSpec for SYSPLLSTAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [syspllstat::R](R) reader structure"]
impl crate::Readable for SYSPLLSTAT_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SYSPLLSTAT to value 0"]
impl crate::Resettable for SYSPLLSTAT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
