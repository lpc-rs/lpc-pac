#[doc = "Reader of register TXCONSUMEINDEX"]
pub type R = crate::R<u32, super::TXCONSUMEINDEX>;
#[doc = "Reader of field `TXCI`"]
pub type TXCI_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - TxConsumeIndex. Index of the descriptor that is going to be transmitted next by the transmit datapath."]
    #[inline(always)]
    pub fn txci(&self) -> TXCI_R {
        TXCI_R::new((self.bits & 0xffff) as u16)
    }
}
