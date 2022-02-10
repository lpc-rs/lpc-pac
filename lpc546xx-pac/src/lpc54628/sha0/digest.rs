///Register `DIGEST[%s]` reader
pub struct R(crate::R<DIGEST_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DIGEST_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DIGEST_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DIGEST_SPEC>) -> Self {
        R(reader)
    }
}
///Field `DIGEST` reader - This field contains one word of the Digest.
pub struct DIGEST_R(crate::FieldReader<u32, u32>);
impl DIGEST_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        DIGEST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DIGEST_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    ///Bits 0:31 - This field contains one word of the Digest.
    #[inline(always)]
    pub fn digest(&self) -> DIGEST_R {
        DIGEST_R::new(self.bits)
    }
}
///Digest register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [digest](index.html) module
pub struct DIGEST_SPEC;
impl crate::RegisterSpec for DIGEST_SPEC {
    type Ux = u32;
}
///`read()` method returns [digest::R](R) reader structure
impl crate::Readable for DIGEST_SPEC {
    type Reader = R;
}
///`reset()` method sets DIGEST[%s]
///to value 0
impl crate::Resettable for DIGEST_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
