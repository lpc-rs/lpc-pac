///Register `PIORESCAP[%s]` reader
pub struct R(crate::R<PIORESCAP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PIORESCAP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PIORESCAP_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PIORESCAP_SPEC>) -> Self {
        R(reader)
    }
}
///Field `PIORESCAP` reader - State of PIOn_31 through PIOn_0 for resets other than POR.
pub struct PIORESCAP_R(crate::FieldReader<u32, u32>);
impl PIORESCAP_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        PIORESCAP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PIORESCAP_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    ///Bits 0:31 - State of PIOn_31 through PIOn_0 for resets other than POR.
    #[inline(always)]
    pub fn piorescap(&self) -> PIORESCAP_R {
        PIORESCAP_R::new(self.bits)
    }
}
///Reset captured value of port n
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [piorescap](index.html) module
pub struct PIORESCAP_SPEC;
impl crate::RegisterSpec for PIORESCAP_SPEC {
    type Ux = u32;
}
///`read()` method returns [piorescap::R](R) reader structure
impl crate::Readable for PIORESCAP_SPEC {
    type Reader = R;
}
///`reset()` method sets PIORESCAP[%s]
///to value 0
impl crate::Resettable for PIORESCAP_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
