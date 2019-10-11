#[doc = "Reader of register RXDAT"]
pub type R = crate::R<u32, super::RXDAT>;
#[doc = "Reader of field `RXDAT`"]
pub type RXDAT_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 0:8 - The USART Receiver Data register contains the next received character. The number of bits that are relevant depends on the USART configuration settings."]
    #[inline(always)]
    pub fn rxdat(&self) -> RXDAT_R {
        RXDAT_R::new((self.bits & 0x01ff) as u16)
    }
}
