#[doc = "Register `CAP[%s]` reader"]
pub struct R(crate::R<CAP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CAP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<CAP_SPEC>> for R {
    fn from(reader: crate::R<CAP_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CAP_VALUE` reader - Capture value for the related capture event (UTICK_CAPn. Note: the value is 1 lower than the actual value of the Micro-tick Timer at the moment of the capture event."]
pub struct CAP_VALUE_R(crate::FieldReader<u32, u32>);
impl CAP_VALUE_R {
    pub(crate) fn new(bits: u32) -> Self {
        CAP_VALUE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CAP_VALUE_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VALID` reader - Capture Valid. When 1, a value has been captured based on a transition of the related UTICK_CAPn pin. Cleared by writing to the related bit in the CAPCLR register."]
pub struct VALID_R(crate::FieldReader<bool, bool>);
impl VALID_R {
    pub(crate) fn new(bits: bool) -> Self {
        VALID_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VALID_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:30 - Capture value for the related capture event (UTICK_CAPn. Note: the value is 1 lower than the actual value of the Micro-tick Timer at the moment of the capture event."]
    #[inline(always)]
    pub fn cap_value(&self) -> CAP_VALUE_R {
        CAP_VALUE_R::new((self.bits & 0x7fff_ffff) as u32)
    }
    #[doc = "Bit 31 - Capture Valid. When 1, a value has been captured based on a transition of the related UTICK_CAPn pin. Cleared by writing to the related bit in the CAPCLR register."]
    #[inline(always)]
    pub fn valid(&self) -> VALID_R {
        VALID_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
#[doc = "Capture register .\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cap](index.html) module"]
pub struct CAP_SPEC;
impl crate::RegisterSpec for CAP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cap::R](R) reader structure"]
impl crate::Readable for CAP_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CAP[%s]
to value 0"]
impl crate::Resettable for CAP_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
