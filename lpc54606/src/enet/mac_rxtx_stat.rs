#[doc = "Reader of register MAC_RXTX_STAT"]
pub type R = crate::R<u32, super::MAC_RXTX_STAT>;
#[doc = "Reader of field `TJT`"]
pub type TJT_R = crate::R<bool, bool>;
#[doc = "Reader of field `NCARR`"]
pub type NCARR_R = crate::R<bool, bool>;
#[doc = "Reader of field `LCARR`"]
pub type LCARR_R = crate::R<bool, bool>;
#[doc = "Reader of field `EXDEF`"]
pub type EXDEF_R = crate::R<bool, bool>;
#[doc = "Reader of field `LCOL`"]
pub type LCOL_R = crate::R<bool, bool>;
#[doc = "Reader of field `EXCOL`"]
pub type EXCOL_R = crate::R<bool, bool>;
#[doc = "Reader of field `RWT`"]
pub type RWT_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - PHY Interrupt Enable When this bit is set, it enables the assertion of the interrupt signal because of the setting of PHYIS bit in MAC Interrupt Status register Table 731."]
    #[inline(always)]
    pub fn tjt(&self) -> TJT_R {
        TJT_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - No Carrier When the DTXSTS bit is set in the MTL Operation Mode register Table 758, this bit indicates that the carrier signal from the PHY is not present at the end of preamble transmission."]
    #[inline(always)]
    pub fn ncarr(&self) -> NCARR_R {
        NCARR_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Loss of Carrier When the DTXSTS bit is set in the MTL Operation Mode register Table 758, this bit indicates that the loss of carrier occurred during packet transmission, that is, the PHY Carrier signal was inactive for one or more transmission clock periods during packet transmission."]
    #[inline(always)]
    pub fn lcarr(&self) -> LCARR_R {
        LCARR_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Excessive Deferral When the DTXSTS bit is set in the MTL Operation Mode register Table 758 and the DC bit is set in the MAC Configuration register Table 758, this bit indicates that the transmission ended because of excessive deferral of over 24,288 bit times (155,680 when Jumbo packet is enabled)."]
    #[inline(always)]
    pub fn exdef(&self) -> EXDEF_R {
        EXDEF_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Late Collision When the DTXSTS bit is set in the MTL Operation Mode register Table 758, this bit indicates that the packet transmission aborted because a collision occurred after the collision window (64 bytes including Preamble in MII mode)."]
    #[inline(always)]
    pub fn lcol(&self) -> LCOL_R {
        LCOL_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Excessive Collisions When the DTXSTS bit is set in the MTL Operation Mode register Table 758, this bit indicates that the transmission aborted after 16 successive collisions while attempting to transmit the current packet."]
    #[inline(always)]
    pub fn excol(&self) -> EXCOL_R {
        EXCOL_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Receive Watchdog Timeout This bit is set when a packet with length greater than 2,048 bytes is received (10,240 bytes when Jumbo Packet mode is enabled) and the WD bit is reset in the MAC Configuration register Table 722."]
    #[inline(always)]
    pub fn rwt(&self) -> RWT_R {
        RWT_R::new(((self.bits >> 8) & 0x01) != 0)
    }
}
