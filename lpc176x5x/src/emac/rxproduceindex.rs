#[doc = "Reader of register RXPRODUCEINDEX"]
pub type R = crate::R<u32, super::RXPRODUCEINDEX>;
#[doc = "Reader of field `RXPRODUCEIX`"]
pub type RXPRODUCEIX_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - Index of the descriptor that is going to be filled next by the receive datapath."]
    #[inline(always)]
    pub fn rxproduceix(&self) -> RXPRODUCEIX_R {
        RXPRODUCEIX_R::new((self.bits & 0xffff) as u16)
    }
}
