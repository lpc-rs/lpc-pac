#[doc = "Register `PIDR0` reader"]
pub struct R(crate::R<PIDR0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PIDR0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PIDR0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PIDR0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `PartNumber` reader - Part Number \\[7:0\\]"]
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
impl R {
    #[doc = "Bits 0:7 - Part Number \\[7:0\\]"]
    #[inline(always)]
    pub fn part_number(&self) -> PARTNUMBER_R {
        PARTNUMBER_R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "Peripheral Identification Register 0\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pidr0](index.html) module"]
pub struct PIDR0_SPEC;
impl crate::RegisterSpec for PIDR0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pidr0::R](R) reader structure"]
impl crate::Readable for PIDR0_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets PIDR0 to value 0x25"]
impl crate::Resettable for PIDR0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x25
    }
}
