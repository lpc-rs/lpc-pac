#[doc = "Register `RXDAT` reader"]
pub struct R(crate::R<RXDAT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RXDAT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<RXDAT_SPEC>> for R {
    fn from(reader: crate::R<RXDAT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RXDAT` reader - The USART Receiver Data register contains the next received character. The number of bits that are relevant depends on the USART configuration settings."]
pub struct RXDAT_R(crate::FieldReader<u16, u16>);
impl RXDAT_R {
    pub(crate) fn new(bits: u16) -> Self {
        RXDAT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXDAT_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:8 - The USART Receiver Data register contains the next received character. The number of bits that are relevant depends on the USART configuration settings."]
    #[inline(always)]
    pub fn rxdat(&self) -> RXDAT_R {
        RXDAT_R::new((self.bits & 0x01ff) as u16)
    }
}
#[doc = "Receiver Data register. Contains the last character received.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rxdat](index.html) module"]
pub struct RXDAT_SPEC;
impl crate::RegisterSpec for RXDAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rxdat::R](R) reader structure"]
impl crate::Readable for RXDAT_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RXDAT to value 0"]
impl crate::Resettable for RXDAT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
