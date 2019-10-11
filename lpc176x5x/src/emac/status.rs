#[doc = "Reader of register STATUS"]
pub type R = crate::R<u32, super::STATUS>;
#[doc = "Reader of field `RXSTATUS`"]
pub type RXSTATUS_R = crate::R<bool, bool>;
#[doc = "Reader of field `TXSTATUS`"]
pub type TXSTATUS_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - If 1, the receive channel is active. If 0, the receive channel is inactive."]
    #[inline(always)]
    pub fn rxstatus(&self) -> RXSTATUS_R {
        RXSTATUS_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - If 1, the transmit channel is active. If 0, the transmit channel is inactive."]
    #[inline(always)]
    pub fn txstatus(&self) -> TXSTATUS_R {
        TXSTATUS_R::new(((self.bits >> 1) & 0x01) != 0)
    }
}
