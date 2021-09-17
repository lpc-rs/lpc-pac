#[doc = "Register `FMSTAT` reader"]
pub struct R(crate::R<FMSTAT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FMSTAT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FMSTAT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FMSTAT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `SIG_DONE` reader - When 1, a previously started signature generation has completed. See FMSTATCLR register description for clearing this flag."]
pub struct SIG_DONE_R(crate::FieldReader<bool, bool>);
impl SIG_DONE_R {
    pub(crate) fn new(bits: bool) -> Self {
        SIG_DONE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SIG_DONE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 2 - When 1, a previously started signature generation has completed. See FMSTATCLR register description for clearing this flag."]
    #[inline(always)]
    pub fn sig_done(&self) -> SIG_DONE_R {
        SIG_DONE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
}
#[doc = "Signature generation status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fmstat](index.html) module"]
pub struct FMSTAT_SPEC;
impl crate::RegisterSpec for FMSTAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fmstat::R](R) reader structure"]
impl crate::Readable for FMSTAT_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets FMSTAT to value 0"]
impl crate::Resettable for FMSTAT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
