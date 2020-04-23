#[doc = "Reader of register FIFOSTAT"]
pub type R = crate::R<u32, super::FIFOSTAT>;
#[doc = "Writer for register FIFOSTAT"]
pub type W = crate::W<u32, super::FIFOSTAT>;
#[doc = "Register FIFOSTAT `reset()`'s with value 0x30"]
impl crate::ResetValue for super::FIFOSTAT {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x30
    }
}
#[doc = "Reader of field `TXERR`"]
pub type TXERR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TXERR`"]
pub struct TXERR_W<'a> {
    w: &'a mut W,
}
impl<'a> TXERR_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
#[doc = "Reader of field `RXERR`"]
pub type RXERR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RXERR`"]
pub struct RXERR_W<'a> {
    w: &'a mut W,
}
impl<'a> RXERR_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "Reader of field `PERINT`"]
pub type PERINT_R = crate::R<bool, bool>;
#[doc = "Reader of field `TXEMPTY`"]
pub type TXEMPTY_R = crate::R<bool, bool>;
#[doc = "Reader of field `TXNOTFULL`"]
pub type TXNOTFULL_R = crate::R<bool, bool>;
#[doc = "Reader of field `RXNOTEMPTY`"]
pub type RXNOTEMPTY_R = crate::R<bool, bool>;
#[doc = "Reader of field `RXFULL`"]
pub type RXFULL_R = crate::R<bool, bool>;
#[doc = "Reader of field `TXLVL`"]
pub type TXLVL_R = crate::R<u8, u8>;
#[doc = "Reader of field `RXLVL`"]
pub type RXLVL_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bit 0 - TX FIFO error. Will be set if a transmit FIFO error occurs. This could be an overflow caused by pushing data into a full FIFO, or by an underflow if the FIFO is empty when data is needed. Cleared by writing a 1 to this bit."]
    #[inline(always)]
    pub fn txerr(&self) -> TXERR_R {
        TXERR_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - RX FIFO error. Will be set if a receive FIFO overflow occurs, caused by software or DMA not emptying the FIFO fast enough. Cleared by writing a 1 to this bit."]
    #[inline(always)]
    pub fn rxerr(&self) -> RXERR_R {
        RXERR_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Peripheral interrupt. When 1, this indicates that the peripheral function has asserted an interrupt. The details can be found by reading the peripheral's STAT register."]
    #[inline(always)]
    pub fn perint(&self) -> PERINT_R {
        PERINT_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Transmit FIFO empty. When 1, the transmit FIFO is empty. The peripheral may still be processing the last piece of data."]
    #[inline(always)]
    pub fn txempty(&self) -> TXEMPTY_R {
        TXEMPTY_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Transmit FIFO not full. When 1, the transmit FIFO is not full, so more data can be written. When 0, the transmit FIFO is full and another write would cause it to overflow."]
    #[inline(always)]
    pub fn txnotfull(&self) -> TXNOTFULL_R {
        TXNOTFULL_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Receive FIFO not empty. When 1, the receive FIFO is not empty, so data can be read. When 0, the receive FIFO is empty."]
    #[inline(always)]
    pub fn rxnotempty(&self) -> RXNOTEMPTY_R {
        RXNOTEMPTY_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Receive FIFO full. When 1, the receive FIFO is full. Data needs to be read out to prevent the peripheral from causing an overflow."]
    #[inline(always)]
    pub fn rxfull(&self) -> RXFULL_R {
        RXFULL_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bits 8:12 - Transmit FIFO current level. A 0 means the TX FIFO is currently empty, and the TXEMPTY and TXNOTFULL flags will be 1. Other values tell how much data is actually in the TX FIFO at the point where the read occurs. If the TX FIFO is full, the TXEMPTY and TXNOTFULL flags will be 0."]
    #[inline(always)]
    pub fn txlvl(&self) -> TXLVL_R {
        TXLVL_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 16:20 - Receive FIFO current level. A 0 means the RX FIFO is currently empty, and the RXFULL and RXNOTEMPTY flags will be 0. Other values tell how much data is actually in the RX FIFO at the point where the read occurs. If the RX FIFO is full, the RXFULL and RXNOTEMPTY flags will be 1."]
    #[inline(always)]
    pub fn rxlvl(&self) -> RXLVL_R {
        RXLVL_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - TX FIFO error. Will be set if a transmit FIFO error occurs. This could be an overflow caused by pushing data into a full FIFO, or by an underflow if the FIFO is empty when data is needed. Cleared by writing a 1 to this bit."]
    #[inline(always)]
    pub fn txerr(&mut self) -> TXERR_W {
        TXERR_W { w: self }
    }
    #[doc = "Bit 1 - RX FIFO error. Will be set if a receive FIFO overflow occurs, caused by software or DMA not emptying the FIFO fast enough. Cleared by writing a 1 to this bit."]
    #[inline(always)]
    pub fn rxerr(&mut self) -> RXERR_W {
        RXERR_W { w: self }
    }
}
