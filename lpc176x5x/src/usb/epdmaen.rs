#[doc = "Writer for register EPDMAEN"]
pub type W = crate::W<u32, super::EPDMAEN>;
#[doc = "Register EPDMAEN `reset()`'s with value 0"]
impl crate::ResetValue for super::EPDMAEN {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `EP_DMA_EN0`"]
pub struct EP_DMA_EN0_W<'a> {
    w: &'a mut W,
}
impl<'a> EP_DMA_EN0_W<'a> {
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
#[doc = "Write proxy for field `EP_DMA_EN1`"]
pub struct EP_DMA_EN1_W<'a> {
    w: &'a mut W,
}
impl<'a> EP_DMA_EN1_W<'a> {
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
#[doc = "Write proxy for field `EP_DMA_EN`"]
pub struct EP_DMA_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> EP_DMA_EN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3fff_ffff << 2)) | (((value as u32) & 0x3fff_ffff) << 2);
        self.w
    }
}
impl W {
    #[doc = "Bit 0 - Control endpoint OUT (DMA cannot be enabled for this endpoint and the EP0_DMA_ENABLE bit value must be 0)."]
    #[inline(always)]
    pub fn ep_dma_en0(&mut self) -> EP_DMA_EN0_W {
        EP_DMA_EN0_W { w: self }
    }
    #[doc = "Bit 1 - Control endpoint IN (DMA cannot be enabled for this endpoint and the EP1_DMA_ENABLE bit must be 0)."]
    #[inline(always)]
    pub fn ep_dma_en1(&mut self) -> EP_DMA_EN1_W {
        EP_DMA_EN1_W { w: self }
    }
    #[doc = "Bits 2:31 - Endpoint xx(2 <= xx <= 31) DMA enable control bit. 0 = No effect. 1 = Enable the DMA operation for endpoint EPxx."]
    #[inline(always)]
    pub fn ep_dma_en(&mut self) -> EP_DMA_EN_W {
        EP_DMA_EN_W { w: self }
    }
}
