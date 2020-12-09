#[doc = "Reader of register MAC_Tx_TIMESTAMP_STATUS_SECONDS"]
pub type R = crate::R<u32, super::MAC_TX_TIMESTAMP_STATUS_SECONDS>;
#[doc = "Reader of field `TXTSSTSHI`"]
pub type TXTSSTSHI_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Transmit timestamp status high."]
    #[inline(always)]
    pub fn txtsstshi(&self) -> TXTSSTSHI_R {
        TXTSSTSHI_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
