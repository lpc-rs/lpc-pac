#[doc = "Reader of register RSV"]
pub type R = crate::R<u32, super::RSV>;
#[doc = "Reader of field `RBC`"]
pub type RBC_R = crate::R<u16, u16>;
#[doc = "Reader of field `PPI`"]
pub type PPI_R = crate::R<bool, bool>;
#[doc = "Reader of field `RXDVSEEN`"]
pub type RXDVSEEN_R = crate::R<bool, bool>;
#[doc = "Reader of field `CESEEN`"]
pub type CESEEN_R = crate::R<bool, bool>;
#[doc = "Reader of field `RCV`"]
pub type RCV_R = crate::R<bool, bool>;
#[doc = "Reader of field `CRCERR`"]
pub type CRCERR_R = crate::R<bool, bool>;
#[doc = "Reader of field `LCERR`"]
pub type LCERR_R = crate::R<bool, bool>;
#[doc = "Reader of field `LOR`"]
pub type LOR_R = crate::R<bool, bool>;
#[doc = "Reader of field `ROK`"]
pub type ROK_R = crate::R<bool, bool>;
#[doc = "Reader of field `MULTICAST`"]
pub type MULTICAST_R = crate::R<bool, bool>;
#[doc = "Reader of field `BROADCAST`"]
pub type BROADCAST_R = crate::R<bool, bool>;
#[doc = "Reader of field `DRIBBLENIBBLE`"]
pub type DRIBBLENIBBLE_R = crate::R<bool, bool>;
#[doc = "Reader of field `CONTROLFRAME`"]
pub type CONTROLFRAME_R = crate::R<bool, bool>;
#[doc = "Reader of field `PAUSE`"]
pub type PAUSE_R = crate::R<bool, bool>;
#[doc = "Reader of field `UO`"]
pub type UO_R = crate::R<bool, bool>;
#[doc = "Reader of field `VLAN`"]
pub type VLAN_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bits 0:15 - Received byte count. Indicates length of received frame."]
    #[inline(always)]
    pub fn rbc(&self) -> RBC_R {
        RBC_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 16 - Packet previously ignored. Indicates that a packet was dropped."]
    #[inline(always)]
    pub fn ppi(&self) -> PPI_R {
        PPI_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - RXDV event previously seen. Indicates that the last receive event seen was not long enough to be a valid packet."]
    #[inline(always)]
    pub fn rxdvseen(&self) -> RXDVSEEN_R {
        RXDVSEEN_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Carrier event previously seen. Indicates that at some time since the last receive statistics, a carrier event was detected."]
    #[inline(always)]
    pub fn ceseen(&self) -> CESEEN_R {
        CESEEN_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Receive code violation. Indicates that received PHY data does not represent a valid receive code."]
    #[inline(always)]
    pub fn rcv(&self) -> RCV_R {
        RCV_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - CRC error. The attached CRC in the packet did not match the internally generated CRC."]
    #[inline(always)]
    pub fn crcerr(&self) -> CRCERR_R {
        CRCERR_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Length check error. Indicates the frame length field does not match the actual number of data items and is not a type field."]
    #[inline(always)]
    pub fn lcerr(&self) -> LCERR_R {
        LCERR_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Length out of range. Indicates that frame type/length field was larger than 1518 bytes. The EMAC doesn't distinguish the frame type and frame length, so, e.g. when the IP(0x8000) or ARP(0x0806) packets are received, it compares the frame type with the max length and gives the \"Length out of range\" error. In fact, this bit is not an error indication, but simply a statement by the chip regarding the status of the received frame."]
    #[inline(always)]
    pub fn lor(&self) -> LOR_R {
        LOR_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - Receive OK. The packet had valid CRC and no symbol errors."]
    #[inline(always)]
    pub fn rok(&self) -> ROK_R {
        ROK_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - The packet destination was a multicast address."]
    #[inline(always)]
    pub fn multicast(&self) -> MULTICAST_R {
        MULTICAST_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - The packet destination was a broadcast address."]
    #[inline(always)]
    pub fn broadcast(&self) -> BROADCAST_R {
        BROADCAST_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - Indicates that after the end of packet another 1-7 bits were received. A single nibble, called dribble nibble, is formed but not sent out."]
    #[inline(always)]
    pub fn dribblenibble(&self) -> DRIBBLENIBBLE_R {
        DRIBBLENIBBLE_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - The frame was a control frame."]
    #[inline(always)]
    pub fn controlframe(&self) -> CONTROLFRAME_R {
        CONTROLFRAME_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - The frame was a control frame with a valid PAUSE opcode."]
    #[inline(always)]
    pub fn pause(&self) -> PAUSE_R {
        PAUSE_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - Unsupported Opcode. The current frame was recognized as a Control Frame but contains an unknown opcode."]
    #[inline(always)]
    pub fn uo(&self) -> UO_R {
        UO_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - Frame's length/type field contained 0x8100 which is the VLAN protocol identifier."]
    #[inline(always)]
    pub fn vlan(&self) -> VLAN_R {
        VLAN_R::new(((self.bits >> 30) & 0x01) != 0)
    }
}
