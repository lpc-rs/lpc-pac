///Register `FIFORDNOPOP` reader
pub struct R(crate::R<FIFORDNOPOP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FIFORDNOPOP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FIFORDNOPOP_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FIFORDNOPOP_SPEC>) -> Self {
        R(reader)
    }
}
///Field `RXDATA` reader - Received data from the FIFO.
pub struct RXDATA_R(crate::FieldReader<u32, u32>);
impl RXDATA_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        RXDATA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXDATA_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    ///Bits 0:31 - Received data from the FIFO.
    #[inline(always)]
    pub fn rxdata(&self) -> RXDATA_R {
        RXDATA_R::new(self.bits)
    }
}
///FIFO data read with no FIFO pop.
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [fifordnopop](index.html) module
pub struct FIFORDNOPOP_SPEC;
impl crate::RegisterSpec for FIFORDNOPOP_SPEC {
    type Ux = u32;
}
///`read()` method returns [fifordnopop::R](R) reader structure
impl crate::Readable for FIFORDNOPOP_SPEC {
    type Reader = R;
}
///`reset()` method sets FIFORDNOPOP to value 0
impl crate::Resettable for FIFORDNOPOP_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
