#[doc = "Reader of register STATE"]
pub type R = crate::R<u32, super::STATE>;
#[doc = "Reader of field `IRQ`"]
pub type IRQ_R = crate::R<bool, bool>;
#[doc = "Reader of field `DMAREQ1`"]
pub type DMAREQ1_R = crate::R<bool, bool>;
#[doc = "Reader of field `DMAREQ2`"]
pub type DMAREQ2_R = crate::R<bool, bool>;
#[doc = "Reader of field `RX_LEVEL`"]
pub type RX_LEVEL_R = crate::R<u8, u8>;
#[doc = "Reader of field `TX_LEVEL`"]
pub type TX_LEVEL_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bit 0 - This bit reflects the presence of Receive Interrupt or Transmit Interrupt. This is determined by comparing the current FIFO levels to the rx_depth_irq and tx_depth_irq fields in the IRQ register."]
    #[inline(always)]
    pub fn irq(&self) -> IRQ_R {
        IRQ_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - This bit reflects the presence of Receive or Transmit DMA Request 1. This is determined by comparing the current FIFO levels to the rx_depth_dma1 and tx_depth_dma1 fields in the DMA1 register."]
    #[inline(always)]
    pub fn dmareq1(&self) -> DMAREQ1_R {
        DMAREQ1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - This bit reflects the presence of Receive or Transmit DMA Request 2. This is determined by comparing the current FIFO levels to the rx_depth_dma2 and tx_depth_dma2 fields in the DMA2 register."]
    #[inline(always)]
    pub fn dmareq2(&self) -> DMAREQ2_R {
        DMAREQ2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bits 8:11 - Reflects the current level of the Receive FIFO."]
    #[inline(always)]
    pub fn rx_level(&self) -> RX_LEVEL_R {
        RX_LEVEL_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - Reflects the current level of the Transmit FIFO."]
    #[inline(always)]
    pub fn tx_level(&self) -> TX_LEVEL_R {
        TX_LEVEL_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
