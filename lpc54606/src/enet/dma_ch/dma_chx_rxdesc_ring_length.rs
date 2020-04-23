#[doc = "Reader of register DMA_CHx_RXDESC_RING_LENGTH"]
pub type R = crate::R<u32, super::DMA_CHX_RXDESC_RING_LENGTH>;
#[doc = "Writer for register DMA_CHx_RXDESC_RING_LENGTH"]
pub type W = crate::W<u32, super::DMA_CHX_RXDESC_RING_LENGTH>;
#[doc = "Register DMA_CHx_RXDESC_RING_LENGTH `reset()`'s with value 0"]
impl crate::ResetValue for super::DMA_CHX_RXDESC_RING_LENGTH {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RDRL`"]
pub type RDRL_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `RDRL`"]
pub struct RDRL_W<'a> {
    w: &'a mut W,
}
impl<'a> RDRL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03ff) | ((value as u32) & 0x03ff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:9 - Receive Ring Length This register sets the maximum number of Rx descriptors in the circular ring."]
    #[inline(always)]
    pub fn rdrl(&self) -> RDRL_R {
        RDRL_R::new((self.bits & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - Receive Ring Length This register sets the maximum number of Rx descriptors in the circular ring."]
    #[inline(always)]
    pub fn rdrl(&mut self) -> RDRL_W {
        RDRL_W { w: self }
    }
}
