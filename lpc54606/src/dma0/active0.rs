#[doc = "Reader of register ACTIVE0"]
pub type R = crate::R<u32, super::ACTIVE0>;
#[doc = "Reader of field `ACT`"]
pub type ACT_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Active flag for DMA channel n. Bit n corresponds to DMA channel n. The number of bits = number of DMA channels in this device. Other bits are reserved. 0 = not active. 1 = active."]
    #[inline(always)]
    pub fn act(&self) -> ACT_R {
        ACT_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
