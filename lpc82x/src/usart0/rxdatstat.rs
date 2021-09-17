#[doc = "Register `RXDATSTAT` reader"]
pub struct R(crate::R<RXDATSTAT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RXDATSTAT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RXDATSTAT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RXDATSTAT_SPEC>) -> Self {
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
#[doc = "Field `FRAMERR` reader - Framing Error status flag. This bit is valid when there is a character to be read in the RXDAT register and reflects the status of that character. This bit will set when the character in RXDAT was received with a missing stop bit at the expected location. This could be an indication of a baud rate or configuration mismatch with the transmitting source."]
pub struct FRAMERR_R(crate::FieldReader<bool, bool>);
impl FRAMERR_R {
    pub(crate) fn new(bits: bool) -> Self {
        FRAMERR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FRAMERR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PARITYERR` reader - Parity Error status flag. This bit is valid when there is a character to be read in the RXDAT register and reflects the status of that character. This bit will be set when a parity error is detected in a received character."]
pub struct PARITYERR_R(crate::FieldReader<bool, bool>);
impl PARITYERR_R {
    pub(crate) fn new(bits: bool) -> Self {
        PARITYERR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PARITYERR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXNOISE` reader - Received Noise flag."]
pub struct RXNOISE_R(crate::FieldReader<bool, bool>);
impl RXNOISE_R {
    pub(crate) fn new(bits: bool) -> Self {
        RXNOISE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXNOISE_R {
    type Target = crate::FieldReader<bool, bool>;
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
    #[doc = "Bit 13 - Framing Error status flag. This bit is valid when there is a character to be read in the RXDAT register and reflects the status of that character. This bit will set when the character in RXDAT was received with a missing stop bit at the expected location. This could be an indication of a baud rate or configuration mismatch with the transmitting source."]
    #[inline(always)]
    pub fn framerr(&self) -> FRAMERR_R {
        FRAMERR_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Parity Error status flag. This bit is valid when there is a character to be read in the RXDAT register and reflects the status of that character. This bit will be set when a parity error is detected in a received character."]
    #[inline(always)]
    pub fn parityerr(&self) -> PARITYERR_R {
        PARITYERR_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Received Noise flag."]
    #[inline(always)]
    pub fn rxnoise(&self) -> RXNOISE_R {
        RXNOISE_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
#[doc = "Receiver Data with Status register. Combines the last character received with the current USART receive status. Allows DMA or software to recover incoming data and status together.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rxdatstat](index.html) module"]
pub struct RXDATSTAT_SPEC;
impl crate::RegisterSpec for RXDATSTAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rxdatstat::R](R) reader structure"]
impl crate::Readable for RXDATSTAT_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RXDATSTAT to value 0"]
impl crate::Resettable for RXDATSTAT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
