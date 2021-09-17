#[doc = "Register `STAT` reader"]
pub struct R(crate::R<STAT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STAT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STAT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STAT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `Status` reader - These bits give the actual status information about the I2C interface."]
pub struct STATUS_R(crate::FieldReader<u8, u8>);
impl STATUS_R {
    pub(crate) fn new(bits: u8) -> Self {
        STATUS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for STATUS_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 3:7 - These bits give the actual status information about the I2C interface."]
    #[inline(always)]
    pub fn status(&self) -> STATUS_R {
        STATUS_R::new(((self.bits >> 3) & 0x1f) as u8)
    }
}
#[doc = "I2C Status Register. During I2C operation, this register provides detailed status codes that allow software to determine the next action needed.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [stat](index.html) module"]
pub struct STAT_SPEC;
impl crate::RegisterSpec for STAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [stat::R](R) reader structure"]
impl crate::Readable for STAT_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets STAT to value 0xf8"]
impl crate::Resettable for STAT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xf8
    }
}
