#[doc = "Reader of register MAC_Tx_TIMESTAMP_STATUS_NANOSECONDS"]
pub type R = crate::R<u32, super::MAC_TX_TIMESTAMP_STATUS_NANOSECONDS>;
#[doc = "Reader of field `TXTSSTSLO`"]
pub type TXTSSTSLO_R = crate::R<u32, u32>;
#[doc = "Reader of field `TXTSSTSMIS`"]
pub type TXTSSTSMIS_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bits 0:30 - Transmit timestamp status low."]
    #[inline(always)]
    pub fn txtsstslo(&self) -> TXTSSTSLO_R {
        TXTSSTSLO_R::new((self.bits & 0x7fff_ffff) as u32)
    }
    #[doc = "Bit 31 - Transmit timestamp status missed."]
    #[inline(always)]
    pub fn txtsstsmis(&self) -> TXTSSTSMIS_R {
        TXTSSTSMIS_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
