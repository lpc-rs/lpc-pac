#[doc = "Reader of register RXDATSTAT"]
pub type R = crate::R<u32, super::RXDATSTAT>;
#[doc = "Reader of field `RXDAT`"]
pub type RXDAT_R = crate::R<u16, u16>;
#[doc = "Reader of field `FRAMERR`"]
pub type FRAMERR_R = crate::R<bool, bool>;
#[doc = "Reader of field `PARITYERR`"]
pub type PARITYERR_R = crate::R<bool, bool>;
#[doc = "Reader of field `RXNOISE`"]
pub type RXNOISE_R = crate::R<bool, bool>;
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
