#[doc = "Reader of register FIFORD48H"]
pub type R = crate::R<u32, super::FIFORD48H>;
#[doc = "Reader of field `RXDATA`"]
pub type RXDATA_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:23 - Received data from the FIFO. Whether this register is used and the number of bits used depends on configuration details."]
    #[inline(always)]
    pub fn rxdata(&self) -> RXDATA_R {
        RXDATA_R::new((self.bits & 0x00ff_ffff) as u32)
    }
}
