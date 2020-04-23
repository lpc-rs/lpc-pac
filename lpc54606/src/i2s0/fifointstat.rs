#[doc = "Reader of register FIFOINTSTAT"]
pub type R = crate::R<u32, super::FIFOINTSTAT>;
#[doc = "Reader of field `TXERR`"]
pub type TXERR_R = crate::R<bool, bool>;
#[doc = "Reader of field `RXERR`"]
pub type RXERR_R = crate::R<bool, bool>;
#[doc = "Reader of field `TXLVL`"]
pub type TXLVL_R = crate::R<bool, bool>;
#[doc = "Reader of field `RXLVL`"]
pub type RXLVL_R = crate::R<bool, bool>;
#[doc = "Reader of field `PERINT`"]
pub type PERINT_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - TX FIFO error."]
    #[inline(always)]
    pub fn txerr(&self) -> TXERR_R {
        TXERR_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - RX FIFO error."]
    #[inline(always)]
    pub fn rxerr(&self) -> RXERR_R {
        RXERR_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Transmit FIFO level interrupt."]
    #[inline(always)]
    pub fn txlvl(&self) -> TXLVL_R {
        TXLVL_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Receive FIFO level interrupt."]
    #[inline(always)]
    pub fn rxlvl(&self) -> RXLVL_R {
        RXLVL_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Peripheral interrupt."]
    #[inline(always)]
    pub fn perint(&self) -> PERINT_R {
        PERINT_R::new(((self.bits >> 4) & 0x01) != 0)
    }
}
