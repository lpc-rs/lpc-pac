#[doc = "Reader of register MAC_DBG"]
pub type R = crate::R<u32, super::MAC_DBG>;
#[doc = "Reader of field `REPESTS`"]
pub type REPESTS_R = crate::R<bool, bool>;
#[doc = "Reader of field `RFCFCSTS`"]
pub type RFCFCSTS_R = crate::R<u8, u8>;
#[doc = "Reader of field `TPESTS`"]
pub type TPESTS_R = crate::R<bool, bool>;
#[doc = "Reader of field `TFCSTS`"]
pub type TFCSTS_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bit 0 - MAC MII Receive Protocol Engine Status When this bit is set, it indicates that the MAC MII receive protocol engine is actively receiving data, and it is not in the Idle state."]
    #[inline(always)]
    pub fn repests(&self) -> REPESTS_R {
        REPESTS_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bits 1:2 - MAC Receive Packet Controller FIFO Status When this bit is set, this field indicates the active state of the small FIFO Read and Write controllers of the MAC Receive Packet Controller module."]
    #[inline(always)]
    pub fn rfcfcsts(&self) -> RFCFCSTS_R {
        RFCFCSTS_R::new(((self.bits >> 1) & 0x03) as u8)
    }
    #[doc = "Bit 16 - MAC MII Transmit Protocol Engine Status When this bit is set, it indicates that the MAC or MII transmit protocol engine is actively transmitting data, and it is not in the Idle state."]
    #[inline(always)]
    pub fn tpests(&self) -> TPESTS_R {
        TPESTS_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bits 17:18 - MAC Transmit Packet Controller Status This field indicates the state of the MAC Transmit Packet Controller module."]
    #[inline(always)]
    pub fn tfcsts(&self) -> TFCSTS_R {
        TFCSTS_R::new(((self.bits >> 17) & 0x03) as u8)
    }
}
