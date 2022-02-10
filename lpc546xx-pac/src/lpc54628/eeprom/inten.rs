///Register `INTEN` reader
pub struct R(crate::R<INTEN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTEN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INTEN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INTEN_SPEC>) -> Self {
        R(reader)
    }
}
///Field `EE_PROG_DONE` reader - EEPROM program operation finished interrupt enable bit.
pub struct EE_PROG_DONE_R(crate::FieldReader<bool, bool>);
impl EE_PROG_DONE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EE_PROG_DONE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EE_PROG_DONE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    ///Bit 2 - EEPROM program operation finished interrupt enable bit.
    #[inline(always)]
    pub fn ee_prog_done(&self) -> EE_PROG_DONE_R {
        EE_PROG_DONE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
}
///EEPROM interrupt enable
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [inten](index.html) module
pub struct INTEN_SPEC;
impl crate::RegisterSpec for INTEN_SPEC {
    type Ux = u32;
}
///`read()` method returns [inten::R](R) reader structure
impl crate::Readable for INTEN_SPEC {
    type Reader = R;
}
///`reset()` method sets INTEN to value 0
impl crate::Resettable for INTEN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
