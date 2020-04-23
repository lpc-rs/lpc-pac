#[doc = "Reader of register DMA_CHx_TXDESC_TAIL_PTR"]
pub type R = crate::R<u32, super::DMA_CHX_TXDESC_TAIL_PTR>;
#[doc = "Writer for register DMA_CHx_TXDESC_TAIL_PTR"]
pub type W = crate::W<u32, super::DMA_CHX_TXDESC_TAIL_PTR>;
#[doc = "Register DMA_CHx_TXDESC_TAIL_PTR `reset()`'s with value 0"]
impl crate::ResetValue for super::DMA_CHX_TXDESC_TAIL_PTR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `TDTP`"]
pub type TDTP_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `TDTP`"]
pub struct TDTP_W<'a> {
    w: &'a mut W,
}
impl<'a> TDTP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3fff_ffff << 2)) | (((value as u32) & 0x3fff_ffff) << 2);
        self.w
    }
}
impl R {
    #[doc = "Bits 2:31 - Transmit Tail Pointer This field contains the tail pointer for the Tx ring."]
    #[inline(always)]
    pub fn tdtp(&self) -> TDTP_R {
        TDTP_R::new(((self.bits >> 2) & 0x3fff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 2:31 - Transmit Tail Pointer This field contains the tail pointer for the Tx ring."]
    #[inline(always)]
    pub fn tdtp(&mut self) -> TDTP_W {
        TDTP_W { w: self }
    }
}
