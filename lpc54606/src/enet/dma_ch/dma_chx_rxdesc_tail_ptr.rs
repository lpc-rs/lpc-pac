#[doc = "Reader of register DMA_CHx_RXDESC_TAIL_PTR"]
pub type R = crate::R<u32, super::DMA_CHX_RXDESC_TAIL_PTR>;
#[doc = "Writer for register DMA_CHx_RXDESC_TAIL_PTR"]
pub type W = crate::W<u32, super::DMA_CHX_RXDESC_TAIL_PTR>;
#[doc = "Register DMA_CHx_RXDESC_TAIL_PTR `reset()`'s with value 0"]
impl crate::ResetValue for super::DMA_CHX_RXDESC_TAIL_PTR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RDTP`"]
pub type RDTP_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `RDTP`"]
pub struct RDTP_W<'a> {
    w: &'a mut W,
}
impl<'a> RDTP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3fff_ffff << 2)) | (((value as u32) & 0x3fff_ffff) << 2);
        self.w
    }
}
impl R {
    #[doc = "Bits 2:31 - Receive Tail Pointer This field contains the tail pointer for the Rx ring."]
    #[inline(always)]
    pub fn rdtp(&self) -> RDTP_R {
        RDTP_R::new(((self.bits >> 2) & 0x3fff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 2:31 - Receive Tail Pointer This field contains the tail pointer for the Rx ring."]
    #[inline(always)]
    pub fn rdtp(&mut self) -> RDTP_W {
        RDTP_W { w: self }
    }
}
