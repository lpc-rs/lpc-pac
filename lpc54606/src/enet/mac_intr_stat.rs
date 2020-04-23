#[doc = "Reader of register MAC_INTR_STAT"]
pub type R = crate::R<u32, super::MAC_INTR_STAT>;
#[doc = "Reader of field `PHYIS`"]
pub type PHYIS_R = crate::R<bool, bool>;
#[doc = "Reader of field `PMTIS`"]
pub type PMTIS_R = crate::R<bool, bool>;
#[doc = "Reader of field `LPIIS`"]
pub type LPIIS_R = crate::R<bool, bool>;
#[doc = "Reader of field `TSIS`"]
pub type TSIS_R = crate::R<bool, bool>;
#[doc = "Reader of field `TXSTSIS`"]
pub type TXSTSIS_R = crate::R<bool, bool>;
#[doc = "Reader of field `RXSTSIS`"]
pub type RXSTSIS_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 3 - PHY Interrupt."]
    #[inline(always)]
    pub fn phyis(&self) -> PHYIS_R {
        PHYIS_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - PMT Interrupt Status."]
    #[inline(always)]
    pub fn pmtis(&self) -> PMTIS_R {
        PMTIS_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - LPI Interrupt Status."]
    #[inline(always)]
    pub fn lpiis(&self) -> LPIIS_R {
        LPIIS_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Timestamp interrupt status."]
    #[inline(always)]
    pub fn tsis(&self) -> TSIS_R {
        TSIS_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Transmit Status Interrupt."]
    #[inline(always)]
    pub fn txstsis(&self) -> TXSTSIS_R {
        TXSTSIS_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Receive Status Interrupt."]
    #[inline(always)]
    pub fn rxstsis(&self) -> RXSTSIS_R {
        RXSTSIS_R::new(((self.bits >> 14) & 0x01) != 0)
    }
}
