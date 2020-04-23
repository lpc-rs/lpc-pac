#[doc = "Reader of register BUSY0"]
pub type R = crate::R<u32, super::BUSY0>;
#[doc = "Reader of field `BSY`"]
pub type BSY_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Busy flag for DMA channel n. Bit n corresponds to DMA channel n. The number of bits = number of DMA channels in this device. Other bits are reserved. 0 = not busy. 1 = busy."]
    #[inline(always)]
    pub fn bsy(&self) -> BSY_R {
        BSY_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
