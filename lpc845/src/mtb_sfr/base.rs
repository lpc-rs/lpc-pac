#[doc = "Register `BASE` reader"]
pub struct R(crate::R<BASE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BASE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BASE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BASE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `BASE` reader - The value provided is the value of the SRAMBASEADDR\\[31:0\\]
signal."]
pub struct BASE_R(crate::FieldReader<u32, u32>);
impl BASE_R {
    pub(crate) fn new(bits: u32) -> Self {
        BASE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BASE_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:31 - The value provided is the value of the SRAMBASEADDR\\[31:0\\]
signal."]
    #[inline(always)]
    pub fn base(&self) -> BASE_R {
        BASE_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
#[doc = "Indicates where the SRAM is located in the processor memory map. This register is provided to enable auto discovery of the MTB SRAM location, by a debug agent.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [base](index.html) module"]
pub struct BASE_SPEC;
impl crate::RegisterSpec for BASE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [base::R](R) reader structure"]
impl crate::Readable for BASE_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets BASE to value 0"]
impl crate::Resettable for BASE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
