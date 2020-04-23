#[doc = "Reader of register SRAMBASE"]
pub type R = crate::R<u32, super::SRAMBASE>;
#[doc = "Writer for register SRAMBASE"]
pub type W = crate::W<u32, super::SRAMBASE>;
#[doc = "Register SRAMBASE `reset()`'s with value 0"]
impl crate::ResetValue for super::SRAMBASE {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `OFFSET`"]
pub type OFFSET_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `OFFSET`"]
pub struct OFFSET_W<'a> {
    w: &'a mut W,
}
impl<'a> OFFSET_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x007f_ffff << 9)) | (((value as u32) & 0x007f_ffff) << 9);
        self.w
    }
}
impl R {
    #[doc = "Bits 9:31 - Address bits 31:9 of the beginning of the DMA descriptor table. For 18 channels, the table must begin on a 512 byte boundary."]
    #[inline(always)]
    pub fn offset(&self) -> OFFSET_R {
        OFFSET_R::new(((self.bits >> 9) & 0x007f_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 9:31 - Address bits 31:9 of the beginning of the DMA descriptor table. For 18 channels, the table must begin on a 512 byte boundary."]
    #[inline(always)]
    pub fn offset(&mut self) -> OFFSET_W {
        OFFSET_W { w: self }
    }
}
