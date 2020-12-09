#[doc = "Reader of register FIFOTH"]
pub type R = crate::R<u32, super::FIFOTH>;
#[doc = "Writer for register FIFOTH"]
pub type W = crate::W<u32, super::FIFOTH>;
#[doc = "Register FIFOTH `reset()`'s with value 0x001f_0000"]
impl crate::ResetValue for super::FIFOTH {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x001f_0000
    }
}
#[doc = "Reader of field `TX_WMARK`"]
pub type TX_WMARK_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `TX_WMARK`"]
pub struct TX_WMARK_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_WMARK_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0fff) | ((value as u32) & 0x0fff);
        self.w
    }
}
#[doc = "Reader of field `RX_WMARK`"]
pub type RX_WMARK_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `RX_WMARK`"]
pub struct RX_WMARK_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_WMARK_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0fff << 16)) | (((value as u32) & 0x0fff) << 16);
        self.w
    }
}
#[doc = "Reader of field `DMA_MTS`"]
pub type DMA_MTS_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DMA_MTS`"]
pub struct DMA_MTS_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_MTS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 28)) | (((value as u32) & 0x07) << 28);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:11 - FIFO threshold watermark level when transmitting data to card."]
    #[inline(always)]
    pub fn tx_wmark(&self) -> TX_WMARK_R {
        TX_WMARK_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 16:27 - FIFO threshold watermark level when receiving data to card."]
    #[inline(always)]
    pub fn rx_wmark(&self) -> RX_WMARK_R {
        RX_WMARK_R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
    #[doc = "Bits 28:30 - Burst size of multiple transaction; should be programmed same as DW-DMA controller multiple-transaction-size SRC/DEST_MSIZE."]
    #[inline(always)]
    pub fn dma_mts(&self) -> DMA_MTS_R {
        DMA_MTS_R::new(((self.bits >> 28) & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 0:11 - FIFO threshold watermark level when transmitting data to card."]
    #[inline(always)]
    pub fn tx_wmark(&mut self) -> TX_WMARK_W {
        TX_WMARK_W { w: self }
    }
    #[doc = "Bits 16:27 - FIFO threshold watermark level when receiving data to card."]
    #[inline(always)]
    pub fn rx_wmark(&mut self) -> RX_WMARK_W {
        RX_WMARK_W { w: self }
    }
    #[doc = "Bits 28:30 - Burst size of multiple transaction; should be programmed same as DW-DMA controller multiple-transaction-size SRC/DEST_MSIZE."]
    #[inline(always)]
    pub fn dma_mts(&mut self) -> DMA_MTS_W {
        DMA_MTS_W { w: self }
    }
}
