#[doc = "Register `INTSTAT` reader"]
pub struct R(crate::R<INTSTAT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTSTAT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<INTSTAT_SPEC>> for R {
    fn from(reader: crate::R<INTSTAT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `END_OF_PROG` reader - EEPROM program operation finished interrupt status bit."]
pub struct END_OF_PROG_R(crate::FieldReader<bool, bool>);
impl END_OF_PROG_R {
    pub(crate) fn new(bits: bool) -> Self {
        END_OF_PROG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for END_OF_PROG_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 2 - EEPROM program operation finished interrupt status bit."]
    #[inline(always)]
    pub fn end_of_prog(&self) -> END_OF_PROG_R {
        END_OF_PROG_R::new(((self.bits >> 2) & 0x01) != 0)
    }
}
#[doc = "EEPROM interrupt status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intstat](index.html) module"]
pub struct INTSTAT_SPEC;
impl crate::RegisterSpec for INTSTAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [intstat::R](R) reader structure"]
impl crate::Readable for INTSTAT_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets INTSTAT to value 0"]
impl crate::Resettable for INTSTAT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
