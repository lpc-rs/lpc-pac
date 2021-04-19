#[doc = "Register `PIDR4` reader"]
pub struct R(crate::R<PIDR4_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PIDR4_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<PIDR4_SPEC>> for R {
    fn from(reader: crate::R<PIDR4_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `JEP106` reader - JEP106 continuation code."]
pub struct JEP106_R(crate::FieldReader<u8, u8>);
impl JEP106_R {
    pub(crate) fn new(bits: u8) -> Self {
        JEP106_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for JEP106_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `c4KB` reader - 4KB Count"]
pub struct C4KB_R(crate::FieldReader<u8, u8>);
impl C4KB_R {
    pub(crate) fn new(bits: u8) -> Self {
        C4KB_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for C4KB_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:3 - JEP106 continuation code."]
    #[inline(always)]
    pub fn jep106(&self) -> JEP106_R {
        JEP106_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - 4KB Count"]
    #[inline(always)]
    pub fn c4kb(&self) -> C4KB_R {
        C4KB_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
#[doc = "Peripheral Identification Register 4\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pidr4](index.html) module"]
pub struct PIDR4_SPEC;
impl crate::RegisterSpec for PIDR4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pidr4::R](R) reader structure"]
impl crate::Readable for PIDR4_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets PIDR4 to value 0x04"]
impl crate::Resettable for PIDR4_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x04
    }
}
