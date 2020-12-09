#[doc = "Reader of register BYTCNT"]
pub type R = crate::R<u32, super::BYTCNT>;
#[doc = "Writer for register BYTCNT"]
pub type W = crate::W<u32, super::BYTCNT>;
#[doc = "Register BYTCNT `reset()`'s with value 0x0200"]
impl crate::ResetValue for super::BYTCNT {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0200
    }
}
#[doc = "Reader of field `BYTE_COUNT`"]
pub type BYTE_COUNT_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `BYTE_COUNT`"]
pub struct BYTE_COUNT_W<'a> {
    w: &'a mut W,
}
impl<'a> BYTE_COUNT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Number of bytes to be transferred; should be integer multiple of Block Size for block transfers."]
    #[inline(always)]
    pub fn byte_count(&self) -> BYTE_COUNT_R {
        BYTE_COUNT_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Number of bytes to be transferred; should be integer multiple of Block Size for block transfers."]
    #[inline(always)]
    pub fn byte_count(&mut self) -> BYTE_COUNT_W {
        BYTE_COUNT_W { w: self }
    }
}
