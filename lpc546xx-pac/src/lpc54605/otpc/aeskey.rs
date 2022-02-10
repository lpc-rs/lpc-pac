///Register `AESKEY[%s]` reader
pub struct R(crate::R<AESKEY_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AESKEY_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AESKEY_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AESKEY_SPEC>) -> Self {
        R(reader)
    }
}
///Field `KEY` reader - AES key.
pub struct KEY_R(crate::FieldReader<u32, u32>);
impl KEY_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        KEY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for KEY_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    ///Bits 0:31 - AES key.
    #[inline(always)]
    pub fn key(&self) -> KEY_R {
        KEY_R::new(self.bits)
    }
}
///Register for reading the AES key.
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [aeskey](index.html) module
pub struct AESKEY_SPEC;
impl crate::RegisterSpec for AESKEY_SPEC {
    type Ux = u32;
}
///`read()` method returns [aeskey::R](R) reader structure
impl crate::Readable for AESKEY_SPEC {
    type Reader = R;
}
///`reset()` method sets AESKEY[%s]
///to value 0
impl crate::Resettable for AESKEY_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
