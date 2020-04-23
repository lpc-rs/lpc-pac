#[doc = "Reader of register MTL_TXQx_DBG"]
pub type R = crate::R<u32, super::MTL_TXQX_DBG>;
#[doc = "Reader of field `TXQPAUSED`"]
pub type TXQPAUSED_R = crate::R<bool, bool>;
#[doc = "Reader of field `TRCSTS`"]
pub type TRCSTS_R = crate::R<u8, u8>;
#[doc = "Reader of field `TWCSTS`"]
pub type TWCSTS_R = crate::R<bool, bool>;
#[doc = "Reader of field `TXQSTS`"]
pub type TXQSTS_R = crate::R<bool, bool>;
#[doc = "Reader of field `TXSTSFSTS`"]
pub type TXSTSFSTS_R = crate::R<bool, bool>;
#[doc = "Reader of field `PTXQ`"]
pub type PTXQ_R = crate::R<u8, u8>;
#[doc = "Reader of field `STSXSTSF`"]
pub type STSXSTSF_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bit 0 - Transmit Queue in Pause When this bit is high and the Rx flow control is enabled, it indicates that the Tx Queue is in the Pause condition (in the full-duplex only mode) because of the following: - Reception of the PFC packet for the priorities assigned to the Tx Queue when PFC is enabled - Reception of 802."]
    #[inline(always)]
    pub fn txqpaused(&self) -> TXQPAUSED_R {
        TXQPAUSED_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bits 1:2 - MTL Tx Queue Read Controller Status This field indicates the state of the Tx Queue Read Controller: 00: Idle state 01: Read state (transferring data to the MAC transmitter) 10: Waiting for pending Tx Status from the MAC transmitter 11: Flushing the Tx queue because of the Packet Abort request from the MAC."]
    #[inline(always)]
    pub fn trcsts(&self) -> TRCSTS_R {
        TRCSTS_R::new(((self.bits >> 1) & 0x03) as u8)
    }
    #[doc = "Bit 3 - MTL Tx Queue Write Controller Status When high, this bit indicates that the MTL Tx Queue Write Controller is active, and it is transferring the data to the Tx Queue."]
    #[inline(always)]
    pub fn twcsts(&self) -> TWCSTS_R {
        TWCSTS_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - MTL Tx Queue Not Empty Status When this bit is high, it indicates that the MTL Tx Queue is not empty and some data is left for transmission."]
    #[inline(always)]
    pub fn txqsts(&self) -> TXQSTS_R {
        TXQSTS_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - MTL Tx Status FIFO Full Status When high, this bit indicates that the MTL Tx Status FIFO is full."]
    #[inline(always)]
    pub fn txstsfsts(&self) -> TXSTSFSTS_R {
        TXSTSFSTS_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bits 16:18 - Number of Packets in the Transmit Queue This field indicates the current number of packets in the Tx Queue."]
    #[inline(always)]
    pub fn ptxq(&self) -> PTXQ_R {
        PTXQ_R::new(((self.bits >> 16) & 0x07) as u8)
    }
    #[doc = "Bits 20:22 - Number of Status Words in Tx Status FIFO of Queue This field indicates the current number of status in the Tx Status FIFO of this queue."]
    #[inline(always)]
    pub fn stsxstsf(&self) -> STSXSTSF_R {
        STSXSTSF_R::new(((self.bits >> 20) & 0x07) as u8)
    }
}
