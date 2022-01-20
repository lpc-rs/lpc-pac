#[doc = "Register `PIOPORCAP1` reader"]
pub struct R(crate::R<PIOPORCAP1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PIOPORCAP1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PIOPORCAP1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PIOPORCAP1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `PIOSTAT` reader - State of PIO1_31 through PIO1_0 at power-on reset"]
pub struct PIOSTAT_R(crate::FieldReader<u32, u32>);
impl PIOSTAT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        PIOSTAT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PIOSTAT_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:31 - State of PIO1_31 through PIO1_0 at power-on reset"]
    #[inline(always)]
    pub fn piostat(&self) -> PIOSTAT_R {
        PIOSTAT_R::new(self.bits)
    }
}
#[doc = "POR captured PIO status 1\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pioporcap1](index.html) module"]
pub struct PIOPORCAP1_SPEC;
impl crate::RegisterSpec for PIOPORCAP1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pioporcap1::R](R) reader structure"]
impl crate::Readable for PIOPORCAP1_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets PIOPORCAP1 to value 0"]
impl crate::Resettable for PIOPORCAP1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
