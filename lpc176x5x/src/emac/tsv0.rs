#[doc = "Reader of register TSV0"]
pub type R = crate::R<u32, super::TSV0>;
#[doc = "Reader of field `CRCERR`"]
pub type CRCERR_R = crate::R<bool, bool>;
#[doc = "Reader of field `LCE`"]
pub type LCE_R = crate::R<bool, bool>;
#[doc = "Reader of field `LOR`"]
pub type LOR_R = crate::R<bool, bool>;
#[doc = "Reader of field `DONE`"]
pub type DONE_R = crate::R<bool, bool>;
#[doc = "Reader of field `MULTICAST`"]
pub type MULTICAST_R = crate::R<bool, bool>;
#[doc = "Reader of field `BROADCAST`"]
pub type BROADCAST_R = crate::R<bool, bool>;
#[doc = "Reader of field `PACKETDEFER`"]
pub type PACKETDEFER_R = crate::R<bool, bool>;
#[doc = "Reader of field `EXDF`"]
pub type EXDF_R = crate::R<bool, bool>;
#[doc = "Reader of field `EXCOL`"]
pub type EXCOL_R = crate::R<bool, bool>;
#[doc = "Reader of field `LCOL`"]
pub type LCOL_R = crate::R<bool, bool>;
#[doc = "Reader of field `GIANT`"]
pub type GIANT_R = crate::R<bool, bool>;
#[doc = "Reader of field `UNDERRUN`"]
pub type UNDERRUN_R = crate::R<bool, bool>;
#[doc = "Reader of field `TOTALBYTES`"]
pub type TOTALBYTES_R = crate::R<u16, u16>;
#[doc = "Reader of field `CONTROLFRAME`"]
pub type CONTROLFRAME_R = crate::R<bool, bool>;
#[doc = "Reader of field `PAUSE`"]
pub type PAUSE_R = crate::R<bool, bool>;
#[doc = "Reader of field `BACKPRESSURE`"]
pub type BACKPRESSURE_R = crate::R<bool, bool>;
#[doc = "Reader of field `VLAN`"]
pub type VLAN_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - CRC error. The attached CRC in the packet did not match the internally generated CRC."]
    #[inline(always)]
    pub fn crcerr(&self) -> CRCERR_R {
        CRCERR_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Length check error. Indicates the frame length field does not match the actual number of data items and is not a type field."]
    #[inline(always)]
    pub fn lce(&self) -> LCE_R {
        LCE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Length out of range. Indicates that frame type/length field was larger than 1500 bytes. The EMAC doesn't distinguish the frame type and frame length, so, e.g. when the IP(0x8000) or ARP(0x0806) packets are received, it compares the frame type with the max length and gives the \"Length out of range\" error. In fact, this bit is not an error indication, but simply a statement by the chip regarding the status of the received frame."]
    #[inline(always)]
    pub fn lor(&self) -> LOR_R {
        LOR_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Transmission of packet was completed."]
    #[inline(always)]
    pub fn done(&self) -> DONE_R {
        DONE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Packet's destination was a multicast address."]
    #[inline(always)]
    pub fn multicast(&self) -> MULTICAST_R {
        MULTICAST_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Packet's destination was a broadcast address."]
    #[inline(always)]
    pub fn broadcast(&self) -> BROADCAST_R {
        BROADCAST_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Packet was deferred for at least one attempt, but less than an excessive defer."]
    #[inline(always)]
    pub fn packetdefer(&self) -> PACKETDEFER_R {
        PACKETDEFER_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Excessive Defer. Packet was deferred in excess of 6071 nibble times in 100 Mbps or 24287 bit times in 10 Mbps mode."]
    #[inline(always)]
    pub fn exdf(&self) -> EXDF_R {
        EXDF_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Excessive Collision. Packet was aborted due to exceeding of maximum allowed number of collisions."]
    #[inline(always)]
    pub fn excol(&self) -> EXCOL_R {
        EXCOL_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Late Collision. Collision occurred beyond collision window, 512 bit times."]
    #[inline(always)]
    pub fn lcol(&self) -> LCOL_R {
        LCOL_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Byte count in frame was greater than can be represented in the transmit byte count field in TSV1."]
    #[inline(always)]
    pub fn giant(&self) -> GIANT_R {
        GIANT_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Host side caused buffer underrun."]
    #[inline(always)]
    pub fn underrun(&self) -> UNDERRUN_R {
        UNDERRUN_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bits 12:27 - The total number of bytes transferred including collided attempts."]
    #[inline(always)]
    pub fn totalbytes(&self) -> TOTALBYTES_R {
        TOTALBYTES_R::new(((self.bits >> 12) & 0xffff) as u16)
    }
    #[doc = "Bit 28 - The frame was a control frame."]
    #[inline(always)]
    pub fn controlframe(&self) -> CONTROLFRAME_R {
        CONTROLFRAME_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - The frame was a control frame with a valid PAUSE opcode."]
    #[inline(always)]
    pub fn pause(&self) -> PAUSE_R {
        PAUSE_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - Carrier-sense method backpressure was previously applied."]
    #[inline(always)]
    pub fn backpressure(&self) -> BACKPRESSURE_R {
        BACKPRESSURE_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Frame's length/type field contained 0x8100 which is the VLAN protocol identifier."]
    #[inline(always)]
    pub fn vlan(&self) -> VLAN_R {
        VLAN_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
