#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::RIS {
    #[doc = r"Reads the contents of the register"]
    #[inline(always)]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = r"Reader of the field"]
pub type RORRIS_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type RTRIS_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type RXRIS_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type TXRIS_R = crate::FR<bool, bool>;
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - This bit is 1 if another frame was completely received while the RxFIFO was full. The ARM spec implies that the preceding frame data is overwritten by the new frame data when this occurs."]
    #[inline(always)]
    pub fn rorris(&self) -> RORRIS_R {
        RORRIS_R::new((self.bits() & 0x01) != 0)
    }
    #[doc = "Bit 1 - This bit is 1 if the Rx FIFO is not empty, and has not been read for a time-out period. The time-out period is the same for master and slave modes and is determined by the SSP bit rate: 32 bits at PCLK / (CPSDVSR X \\[SCR+1\\])."]
    #[inline(always)]
    pub fn rtris(&self) -> RTRIS_R {
        RTRIS_R::new(((self.bits() >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - This bit is 1 if the Rx FIFO is at least half full."]
    #[inline(always)]
    pub fn rxris(&self) -> RXRIS_R {
        RXRIS_R::new(((self.bits() >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - This bit is 1 if the Tx FIFO is at least half empty."]
    #[inline(always)]
    pub fn txris(&self) -> TXRIS_R {
        TXRIS_R::new(((self.bits() >> 3) & 0x01) != 0)
    }
}
