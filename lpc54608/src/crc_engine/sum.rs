#[doc = "Reader of register SUM"]
pub type R = crate::R<u32, super::SUM>;
#[doc = "Reader of field `CRC_SUM`"]
pub type CRC_SUM_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - The most recent CRC sum can be read through this register with selected bit order and 1's complement post-processes."]
    #[inline(always)]
    pub fn crc_sum(&self) -> CRC_SUM_R {
        CRC_SUM_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
