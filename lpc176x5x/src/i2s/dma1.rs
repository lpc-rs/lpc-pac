#[doc = "Reader of register DMA1"]
pub type R = crate::R<u32, super::DMA1>;
#[doc = "Writer for register DMA1"]
pub type W = crate::W<u32, super::DMA1>;
#[doc = "Register DMA1 `reset()`'s with value 0"]
impl crate::ResetValue for super::DMA1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RX_DMA1_ENABLE`"]
pub type RX_DMA1_ENABLE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RX_DMA1_ENABLE`"]
pub struct RX_DMA1_ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_DMA1_ENABLE_W<'a> {
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
#[doc = "Reader of field `TX_DMA1_ENABLE`"]
pub type TX_DMA1_ENABLE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TX_DMA1_ENABLE`"]
pub struct TX_DMA1_ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_DMA1_ENABLE_W<'a> {
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
#[doc = "Reader of field `RX_DEPTH_DMA1`"]
pub type RX_DEPTH_DMA1_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RX_DEPTH_DMA1`"]
pub struct RX_DEPTH_DMA1_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_DEPTH_DMA1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | (((value as u32) & 0x0f) << 8);
        self.w
    }
}
#[doc = "Reader of field `TX_DEPTH_DMA1`"]
pub type TX_DEPTH_DMA1_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TX_DEPTH_DMA1`"]
pub struct TX_DEPTH_DMA1_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_DEPTH_DMA1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | (((value as u32) & 0x0f) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - When 1, enables DMA1 for I2S receive."]
    #[inline(always)]
    pub fn rx_dma1_enable(&self) -> RX_DMA1_ENABLE_R {
        RX_DMA1_ENABLE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - When 1, enables DMA1 for I2S transmit."]
    #[inline(always)]
    pub fn tx_dma1_enable(&self) -> TX_DMA1_ENABLE_R {
        TX_DMA1_ENABLE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bits 8:11 - Set the FIFO level that triggers a receive DMA request on DMA1."]
    #[inline(always)]
    pub fn rx_depth_dma1(&self) -> RX_DEPTH_DMA1_R {
        RX_DEPTH_DMA1_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - Set the FIFO level that triggers a transmit DMA request on DMA1."]
    #[inline(always)]
    pub fn tx_depth_dma1(&self) -> TX_DEPTH_DMA1_R {
        TX_DEPTH_DMA1_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - When 1, enables DMA1 for I2S receive."]
    #[inline(always)]
    pub fn rx_dma1_enable(&mut self) -> RX_DMA1_ENABLE_W {
        RX_DMA1_ENABLE_W { w: self }
    }
    #[doc = "Bit 1 - When 1, enables DMA1 for I2S transmit."]
    #[inline(always)]
    pub fn tx_dma1_enable(&mut self) -> TX_DMA1_ENABLE_W {
        TX_DMA1_ENABLE_W { w: self }
    }
    #[doc = "Bits 8:11 - Set the FIFO level that triggers a receive DMA request on DMA1."]
    #[inline(always)]
    pub fn rx_depth_dma1(&mut self) -> RX_DEPTH_DMA1_W {
        RX_DEPTH_DMA1_W { w: self }
    }
    #[doc = "Bits 16:19 - Set the FIFO level that triggers a transmit DMA request on DMA1."]
    #[inline(always)]
    pub fn tx_depth_dma1(&mut self) -> TX_DEPTH_DMA1_W {
        TX_DEPTH_DMA1_W { w: self }
    }
}
