#[doc = "Reader of register DMACR"]
pub type R = crate::R<u32, super::DMACR>;
#[doc = "Writer for register DMACR"]
pub type W = crate::W<u32, super::DMACR>;
#[doc = "Register DMACR `reset()`'s with value 0"]
impl crate::ResetValue for super::DMACR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RXDMAE`"]
pub type RXDMAE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RXDMAE`"]
pub struct RXDMAE_W<'a> {
    w: &'a mut W,
}
impl<'a> RXDMAE_W<'a> {
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
#[doc = "Reader of field `TXDMAE`"]
pub type TXDMAE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TXDMAE`"]
pub struct TXDMAE_W<'a> {
    w: &'a mut W,
}
impl<'a> TXDMAE_W<'a> {
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
impl R {
    #[doc = "Bit 0 - Receive DMA Enable. When this bit is set to one 1, DMA for the receive FIFO is enabled, otherwise receive DMA is disabled."]
    #[inline(always)]
    pub fn rxdmae(&self) -> RXDMAE_R {
        RXDMAE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Transmit DMA Enable. When this bit is set to one 1, DMA for the transmit FIFO is enabled, otherwise transmit DMA is disabled"]
    #[inline(always)]
    pub fn txdmae(&self) -> TXDMAE_R {
        TXDMAE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Receive DMA Enable. When this bit is set to one 1, DMA for the receive FIFO is enabled, otherwise receive DMA is disabled."]
    #[inline(always)]
    pub fn rxdmae(&mut self) -> RXDMAE_W {
        RXDMAE_W { w: self }
    }
    #[doc = "Bit 1 - Transmit DMA Enable. When this bit is set to one 1, DMA for the transmit FIFO is enabled, otherwise transmit DMA is disabled"]
    #[inline(always)]
    pub fn txdmae(&mut self) -> TXDMAE_W {
        TXDMAE_W { w: self }
    }
}
