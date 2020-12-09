#[doc = "Reader of register DMA_CHx_TXDESC_LIST_ADDR"]
pub type R = crate::R<u32, super::DMA_CHX_TXDESC_LIST_ADDR>;
#[doc = "Writer for register DMA_CHx_TXDESC_LIST_ADDR"]
pub type W = crate::W<u32, super::DMA_CHX_TXDESC_LIST_ADDR>;
#[doc = "Register DMA_CHx_TXDESC_LIST_ADDR `reset()`'s with value 0"]
impl crate::ResetValue for super::DMA_CHX_TXDESC_LIST_ADDR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `STL`"]
pub type STL_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `STL`"]
pub struct STL_W<'a> {
    w: &'a mut W,
}
impl<'a> STL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3fff_ffff << 2)) | (((value as u32) & 0x3fff_ffff) << 2);
        self.w
    }
}
impl R {
    #[doc = "Bits 2:31 - Start of transmit list This field contains the base address of the first in the Transmit list."]
    #[inline(always)]
    pub fn stl(&self) -> STL_R {
        STL_R::new(((self.bits >> 2) & 0x3fff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 2:31 - Start of transmit list This field contains the base address of the first in the Transmit list."]
    #[inline(always)]
    pub fn stl(&mut self) -> STL_W {
        STL_W { w: self }
    }
}
