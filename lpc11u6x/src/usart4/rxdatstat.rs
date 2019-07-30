#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::RXDATSTAT {
    #[doc = r"Reads the contents of the register"]
    #[inline(always)]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = r"Reader of the field"]
pub type RXDAT_R = crate::FR<u16, u16>;
#[doc = r"Reader of the field"]
pub type FRAMERR_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type PARITYERR_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type RXNOISE_R = crate::FR<bool, bool>;
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:8 - The USART Receiver Data register contains the next received character. The number of bits that are relevant depends on the USART configuration settings."]
    #[inline(always)]
    pub fn rxdat(&self) -> RXDAT_R {
        RXDAT_R::new((self.bits() & 0x01ff) as u16)
    }
    #[doc = "Bit 13 - Framing Error status flag. This bit is valid when there is a character to be read in the RXDAT register and reflects the status of that character. This bit will set when the character in RXDAT was received with a missing stop bit at the expected location. This could be an indication of a baud rate or configuration mismatch with the transmitting source."]
    #[inline(always)]
    pub fn framerr(&self) -> FRAMERR_R {
        FRAMERR_R::new(((self.bits() >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Parity Error status flag. This bit is valid when there is a character to be read in the RXDAT register and reflects the status of that character. This bit will be set when a parity error is detected in a received character."]
    #[inline(always)]
    pub fn parityerr(&self) -> PARITYERR_R {
        PARITYERR_R::new(((self.bits() >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Received Noise flag. See description of the RXNOISEINT bit in Table 133."]
    #[inline(always)]
    pub fn rxnoise(&self) -> RXNOISE_R {
        RXNOISE_R::new(((self.bits() >> 15) & 0x01) != 0)
    }
}
