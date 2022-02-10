///Register `PIDR2` reader
pub struct R(crate::R<PIDR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PIDR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PIDR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PIDR2_SPEC>) -> Self {
        R(reader)
    }
}
///Field `JEP106_identity_code` reader - JEP106 identity code \[6:4\]
pub struct JEP106_IDENTITY_CODE_R(crate::FieldReader<u8, u8>);
impl JEP106_IDENTITY_CODE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        JEP106_IDENTITY_CODE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for JEP106_IDENTITY_CODE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `Revision` reader - Revision
pub struct REVISION_R(crate::FieldReader<u8, u8>);
impl REVISION_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        REVISION_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for REVISION_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    ///Bits 0:2 - JEP106 identity code \[6:4\]
    #[inline(always)]
    pub fn jep106_identity_code(&self) -> JEP106_IDENTITY_CODE_R {
        JEP106_IDENTITY_CODE_R::new((self.bits & 0x07) as u8)
    }
    ///Bits 4:7 - Revision
    #[inline(always)]
    pub fn revision(&self) -> REVISION_R {
        REVISION_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
///Peripheral Identification Register 2
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [pidr2](index.html) module
pub struct PIDR2_SPEC;
impl crate::RegisterSpec for PIDR2_SPEC {
    type Ux = u32;
}
///`read()` method returns [pidr2::R](R) reader structure
impl crate::Readable for PIDR2_SPEC {
    type Reader = R;
}
///`reset()` method sets PIDR2 to value 0x0b
impl crate::Resettable for PIDR2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0b
    }
}
