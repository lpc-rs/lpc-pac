#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::MIS {
    #[doc = r"Reads the contents of the register"]
    #[inline(always)]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = r"Reader of the field"]
pub type RORMIS_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type RTMIS_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type RXMIS_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type TXMIS_R = crate::FR<bool, bool>;
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - This bit is 1 if another frame was completely received while the RxFIFO was full, and this interrupt is enabled."]
    #[inline(always)]
    pub fn rormis(&self) -> RORMIS_R {
        RORMIS_R::new((self.bits() & 0x01) != 0)
    }
    #[doc = "Bit 1 - This bit is 1 if the Rx FIFO is not empty, has not been read for a time-out period, and this interrupt is enabled. The time-out period is the same for master and slave modes and is determined by the SSP bit rate: 32 bits at PCLK / (CPSDVSR X \\[SCR+1\\])."]
    #[inline(always)]
    pub fn rtmis(&self) -> RTMIS_R {
        RTMIS_R::new(((self.bits() >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - This bit is 1 if the Rx FIFO is at least half full, and this interrupt is enabled."]
    #[inline(always)]
    pub fn rxmis(&self) -> RXMIS_R {
        RXMIS_R::new(((self.bits() >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - This bit is 1 if the Tx FIFO is at least half empty, and this interrupt is enabled."]
    #[inline(always)]
    pub fn txmis(&self) -> TXMIS_R {
        TXMIS_R::new(((self.bits() >> 3) & 0x01) != 0)
    }
}
