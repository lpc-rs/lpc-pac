#[doc = "Reader of register IRQ"]
pub type R = crate::R<u32, super::IRQ>;
#[doc = "Writer for register IRQ"]
pub type W = crate::W<u32, super::IRQ>;
#[doc = "Register IRQ `reset()`'s with value 0"]
impl crate::ResetValue for super::IRQ {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RX_IRQ_ENABLE`"]
pub type RX_IRQ_ENABLE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RX_IRQ_ENABLE`"]
pub struct RX_IRQ_ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_IRQ_ENABLE_W<'a> {
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
#[doc = "Reader of field `TX_IRQ_ENABLE`"]
pub type TX_IRQ_ENABLE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TX_IRQ_ENABLE`"]
pub struct TX_IRQ_ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_IRQ_ENABLE_W<'a> {
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
#[doc = "Reader of field `RX_DEPTH_IRQ`"]
pub type RX_DEPTH_IRQ_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RX_DEPTH_IRQ`"]
pub struct RX_DEPTH_IRQ_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_DEPTH_IRQ_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | (((value as u32) & 0x0f) << 8);
        self.w
    }
}
#[doc = "Reader of field `TX_DEPTH_IRQ`"]
pub type TX_DEPTH_IRQ_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TX_DEPTH_IRQ`"]
pub struct TX_DEPTH_IRQ_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_DEPTH_IRQ_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | (((value as u32) & 0x0f) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - When 1, enables I2S receive interrupt."]
    #[inline(always)]
    pub fn rx_irq_enable(&self) -> RX_IRQ_ENABLE_R {
        RX_IRQ_ENABLE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - When 1, enables I2S transmit interrupt."]
    #[inline(always)]
    pub fn tx_irq_enable(&self) -> TX_IRQ_ENABLE_R {
        TX_IRQ_ENABLE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bits 8:11 - Set the FIFO level on which to create an irq request."]
    #[inline(always)]
    pub fn rx_depth_irq(&self) -> RX_DEPTH_IRQ_R {
        RX_DEPTH_IRQ_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - Set the FIFO level on which to create an irq request."]
    #[inline(always)]
    pub fn tx_depth_irq(&self) -> TX_DEPTH_IRQ_R {
        TX_DEPTH_IRQ_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - When 1, enables I2S receive interrupt."]
    #[inline(always)]
    pub fn rx_irq_enable(&mut self) -> RX_IRQ_ENABLE_W {
        RX_IRQ_ENABLE_W { w: self }
    }
    #[doc = "Bit 1 - When 1, enables I2S transmit interrupt."]
    #[inline(always)]
    pub fn tx_irq_enable(&mut self) -> TX_IRQ_ENABLE_W {
        TX_IRQ_ENABLE_W { w: self }
    }
    #[doc = "Bits 8:11 - Set the FIFO level on which to create an irq request."]
    #[inline(always)]
    pub fn rx_depth_irq(&mut self) -> RX_DEPTH_IRQ_W {
        RX_DEPTH_IRQ_W { w: self }
    }
    #[doc = "Bits 16:19 - Set the FIFO level on which to create an irq request."]
    #[inline(always)]
    pub fn tx_depth_irq(&mut self) -> TX_DEPTH_IRQ_W {
        TX_DEPTH_IRQ_W { w: self }
    }
}
