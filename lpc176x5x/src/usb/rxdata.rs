#[doc = "Reader of register RXDATA"]
pub type R = crate::R<u32, super::RXDATA>;
#[doc = "Reader of field `RX_DATA`"]
pub type RX_DATA_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Data received."]
    #[inline(always)]
    pub fn rx_data(&self) -> RX_DATA_R {
        RX_DATA_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
