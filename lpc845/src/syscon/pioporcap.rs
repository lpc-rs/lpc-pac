#[doc = "Register `PIOPORCAP[%s]` reader"]
pub struct R(crate::R<PIOPORCAP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PIOPORCAP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PIOPORCAP_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PIOPORCAP_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `PIOSTAT` reader - State of PION_31 through PION_0 at power-on reset"]
pub struct PIOSTAT_R(crate::FieldReader<u32, u32>);
impl PIOSTAT_R {
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
    #[doc = "Bits 0:31 - State of PION_31 through PION_0 at power-on reset"]
    #[inline(always)]
    pub fn piostat(&self) -> PIOSTAT_R {
        PIOSTAT_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
#[doc = "POR captured PIO N status register(PIO0 has 32 PIOs, PIO1 has 22 PIOs)\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pioporcap](index.html) module"]
pub struct PIOPORCAP_SPEC;
impl crate::RegisterSpec for PIOPORCAP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pioporcap::R](R) reader structure"]
impl crate::Readable for PIOPORCAP_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets PIOPORCAP[%s]
to value 0"]
impl crate::Resettable for PIOPORCAP_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
