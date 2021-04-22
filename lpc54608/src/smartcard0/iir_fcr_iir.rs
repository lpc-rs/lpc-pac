#[doc = "Register `IIR` reader"]
pub struct R(crate::R<IIR_FCR_IIR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IIR_FCR_IIR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<IIR_FCR_IIR_SPEC>> for R {
    fn from(reader: crate::R<IIR_FCR_IIR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `INTSTATUS` reader - Interrupt status."]
pub struct INTSTATUS_R(crate::FieldReader<bool, bool>);
impl INTSTATUS_R {
    pub(crate) fn new(bits: bool) -> Self {
        INTSTATUS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INTSTATUS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INTID` reader - Interrupt identification."]
pub struct INTID_R(crate::FieldReader<u8, u8>);
impl INTID_R {
    pub(crate) fn new(bits: u8) -> Self {
        INTID_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INTID_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FIFOENABLE` reader - Copies of SCInFCR\\[0\\]."]
pub struct FIFOENABLE_R(crate::FieldReader<u8, u8>);
impl FIFOENABLE_R {
    pub(crate) fn new(bits: u8) -> Self {
        FIFOENABLE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FIFOENABLE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - Interrupt status."]
    #[inline(always)]
    pub fn intstatus(&self) -> INTSTATUS_R {
        INTSTATUS_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bits 1:3 - Interrupt identification."]
    #[inline(always)]
    pub fn intid(&self) -> INTID_R {
        INTID_R::new(((self.bits >> 1) & 0x07) as u8)
    }
    #[doc = "Bits 6:7 - Copies of SCInFCR\\[0\\]."]
    #[inline(always)]
    pub fn fifoenable(&self) -> FIFOENABLE_R {
        FIFOENABLE_R::new(((self.bits >> 6) & 0x03) as u8)
    }
}
#[doc = "Interrupt ID Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [iir_fcr_iir](index.html) module"]
pub struct IIR_FCR_IIR_SPEC;
impl crate::RegisterSpec for IIR_FCR_IIR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [iir_fcr_iir::R](R) reader structure"]
impl crate::Readable for IIR_FCR_IIR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets IIR to value 0x01"]
impl crate::Resettable for IIR_FCR_IIR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x01
    }
}
