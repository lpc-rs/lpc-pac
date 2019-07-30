#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::SR {
    #[doc = r"Reads the contents of the register"]
    #[inline(always)]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = r"Reader of the field"]
pub type TFE_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type TNF_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type RNE_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type RFF_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type BSY_R = crate::FR<bool, bool>;
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Transmit FIFO Empty. This bit is 1 is the Transmit FIFO is empty, 0 if not."]
    #[inline(always)]
    pub fn tfe(&self) -> TFE_R {
        TFE_R::new((self.bits() & 0x01) != 0)
    }
    #[doc = "Bit 1 - Transmit FIFO Not Full. This bit is 0 if the Tx FIFO is full, 1 if not."]
    #[inline(always)]
    pub fn tnf(&self) -> TNF_R {
        TNF_R::new(((self.bits() >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Receive FIFO Not Empty. This bit is 0 if the Receive FIFO is empty, 1 if not."]
    #[inline(always)]
    pub fn rne(&self) -> RNE_R {
        RNE_R::new(((self.bits() >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Receive FIFO Full. This bit is 1 if the Receive FIFO is full, 0 if not."]
    #[inline(always)]
    pub fn rff(&self) -> RFF_R {
        RFF_R::new(((self.bits() >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Busy. This bit is 0 if the SPI controller is idle, 1 if it is currently sending/receiving a frame and/or the Tx FIFO is not empty."]
    #[inline(always)]
    pub fn bsy(&self) -> BSY_R {
        BSY_R::new(((self.bits() >> 4) & 0x01) != 0)
    }
}
