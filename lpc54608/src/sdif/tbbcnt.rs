#[doc = "Reader of register TBBCNT"]
pub type R = crate::R<u32, super::TBBCNT>;
#[doc = "Writer for register TBBCNT"]
pub type W = crate::W<u32, super::TBBCNT>;
#[doc = "Register TBBCNT `reset()`'s with value 0"]
impl crate::ResetValue for super::TBBCNT {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `TRANS_FIFO_BYTE_COUNT`"]
pub type TRANS_FIFO_BYTE_COUNT_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `TRANS_FIFO_BYTE_COUNT`"]
pub struct TRANS_FIFO_BYTE_COUNT_W<'a> {
    w: &'a mut W,
}
impl<'a> TRANS_FIFO_BYTE_COUNT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Number of bytes transferred between Host/DMA memory and BIU FIFO."]
    #[inline(always)]
    pub fn trans_fifo_byte_count(&self) -> TRANS_FIFO_BYTE_COUNT_R {
        TRANS_FIFO_BYTE_COUNT_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Number of bytes transferred between Host/DMA memory and BIU FIFO."]
    #[inline(always)]
    pub fn trans_fifo_byte_count(&mut self) -> TRANS_FIFO_BYTE_COUNT_W {
        TRANS_FIFO_BYTE_COUNT_W { w: self }
    }
}
