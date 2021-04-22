#[doc = "Register `PIDR1` reader"]
pub struct R(crate::R<PIDR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PIDR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<PIDR1_SPEC>> for R {
    fn from(reader: crate::R<PIDR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `PartNumber` reader - Part Number \\[11:8\\]"]
pub struct PARTNUMBER_R(crate::FieldReader<u8, u8>);
impl PARTNUMBER_R {
    pub(crate) fn new(bits: u8) -> Self {
        PARTNUMBER_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PARTNUMBER_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `JEP106_identity_code` reader - JEP106 identity code \\[3:0\\]"]
pub struct JEP106_IDENTITY_CODE_R(crate::FieldReader<u8, u8>);
impl JEP106_IDENTITY_CODE_R {
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
impl R {
    #[doc = "Bits 0:3 - Part Number \\[11:8\\]"]
    #[inline(always)]
    pub fn part_number(&self) -> PARTNUMBER_R {
        PARTNUMBER_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - JEP106 identity code \\[3:0\\]"]
    #[inline(always)]
    pub fn jep106_identity_code(&self) -> JEP106_IDENTITY_CODE_R {
        JEP106_IDENTITY_CODE_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
#[doc = "Peripheral Identification Register 1\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pidr1](index.html) module"]
pub struct PIDR1_SPEC;
impl crate::RegisterSpec for PIDR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pidr1::R](R) reader structure"]
impl crate::Readable for PIDR1_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets PIDR1 to value 0xb9"]
impl crate::Resettable for PIDR1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xb9
    }
}
