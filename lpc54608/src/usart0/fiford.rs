#[doc = "Reader of register FIFORD"]
pub type R = crate::R<u32, super::FIFORD>;
#[doc = "Reader of field `RXDATA`"]
pub type RXDATA_R = crate::R<u16, u16>;
#[doc = "Reader of field `FRAMERR`"]
pub type FRAMERR_R = crate::R<bool, bool>;
#[doc = "Reader of field `PARITYERR`"]
pub type PARITYERR_R = crate::R<bool, bool>;
#[doc = "Reader of field `RXNOISE`"]
pub type RXNOISE_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bits 0:8 - Received data from the FIFO. The number of bits used depends on the DATALEN and PARITYSEL settings."]
    #[inline(always)]
    pub fn rxdata(&self) -> RXDATA_R {
        RXDATA_R::new((self.bits & 0x01ff) as u16)
    }
    #[doc = "Bit 13 - Framing Error status flag. This bit reflects the status for the data it is read along with from the FIFO, and indicates that the character was received with a missing stop bit at the expected location. This could be an indication of a baud rate or configuration mismatch with the transmitting source."]
    #[inline(always)]
    pub fn framerr(&self) -> FRAMERR_R {
        FRAMERR_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Parity Error status flag. This bit reflects the status for the data it is read along with from the FIFO. This bit will be set when a parity error is detected in a received character."]
    #[inline(always)]
    pub fn parityerr(&self) -> PARITYERR_R {
        PARITYERR_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Received Noise flag. See description of the RxNoiseInt bit in Table 354."]
    #[inline(always)]
    pub fn rxnoise(&self) -> RXNOISE_R {
        RXNOISE_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
