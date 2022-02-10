///Register `CIDR3` reader
pub struct R(crate::R<CIDR3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CIDR3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CIDR3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CIDR3_SPEC>) -> Self {
        R(reader)
    }
}
///Field `Preamble` reader - Preamble
pub struct PREAMBLE_R(crate::FieldReader<u8, u8>);
impl PREAMBLE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PREAMBLE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PREAMBLE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    ///Bits 0:7 - Preamble
    #[inline(always)]
    pub fn preamble(&self) -> PREAMBLE_R {
        PREAMBLE_R::new((self.bits & 0xff) as u8)
    }
}
///Component Identification Register 3
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [cidr3](index.html) module
pub struct CIDR3_SPEC;
impl crate::RegisterSpec for CIDR3_SPEC {
    type Ux = u32;
}
///`read()` method returns [cidr3::R](R) reader structure
impl crate::Readable for CIDR3_SPEC {
    type Reader = R;
}
///`reset()` method sets CIDR3 to value 0xb1
impl crate::Resettable for CIDR3_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xb1
    }
}
